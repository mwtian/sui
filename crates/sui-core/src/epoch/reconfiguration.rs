// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::authority_active::ActiveAuthority;
use std::sync::atomic::Ordering;
use std::time::Duration;
use sui_types::committee::Committee;
use sui_types::crypto::PublicKeyBytes;
use sui_types::error::SuiResult;
use sui_types::messages::SignedTransaction;

impl<A> ActiveAuthority<A> {
    pub async fn start_epoch_change(&mut self) {
        self.state.halted.store(true, Ordering::SeqCst);
        while !self.state.batch_notifier.ticket_drained() {
            tokio::time::sleep(Duration::from_millis(50)).await;
            // TODO: Is it possible to have an infinite loop?
        }
    }

    pub async fn finish_epoch_change(&mut self) -> SuiResult {
        let sui_system_state = self.state.get_sui_system_state_object().await?;
        let next_epoch = sui_system_state.epoch + 1;
        let next_epoch_validators = &sui_system_state.validators.next_epoch_validators;
        let votes = next_epoch_validators
            .iter()
            .map(|metadata| {
                (
                    PublicKeyBytes::try_from(metadata.pubkey_bytes.as_ref())
                        .expect("Validity of public key bytes should be verified on-chain"),
                    metadata.next_epoch_stake,
                )
            })
            .collect();
        let new_committee = Committee::new(next_epoch, votes);
        self.state.insert_new_epoch_info(&new_committee)?;
        self.state.checkpoints.as_ref().unwrap().lock().committee = new_committee;
        // TODO: Update all committee in all components, potentially restart some authority clients.
        // Including: self.net, narwhal committee, anything else?
        // We should also reduce the amount of committee passed around.

        let advance_epoch_tx = SignedTransaction::new_change_epoch(
            next_epoch,
            0, // TODO: fill in storage_charge
            0, // TODO: fill in computation_charge
            self.state.name,
            &*self.state.secret,
        );
        self.state
            .change_epoch_tx
            .lock()
            .insert(self.state.name, advance_epoch_tx);

        // TODO: Now ask every validator in the committee for this signed tx.
        // Aggregate them to obtain a cert, execute the cert, and then start the new epoch.

        self.state.begin_new_epoch()?;
        Ok(())
    }
}
