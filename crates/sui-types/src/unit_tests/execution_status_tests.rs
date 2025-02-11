// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::execution_status::ExecutionFailureStatus;
#[test]
fn enforce_order_test() {
    let new_map = ExecutionFailureStatus::order_to_variant_map();
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "exec_failure_status.yaml"]);
    let existing_map: std::collections::BTreeMap<usize, String> =
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();

    // Check that the new map includes the existing map in order
    for (pos, val) in existing_map {
        match new_map.get(&pos) {
            None => {
                panic!("Enum variant {} has been removed. Not allowed: enum must be backward compatible.", val);
            }
            Some(new_val) if new_val == &val => continue,
            Some(new_val) => {
                panic!("Enum variant {val} has been swapped with {new_val} at position {pos}. Not allowed: enum must be backward compatible.");
            }
        }
    }
}
