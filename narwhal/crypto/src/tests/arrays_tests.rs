// Copyright(C) 2021, Mysten Labs
// SPDX-License-Identifier: Apache-2.0
use super::*;
use serde_test::{assert_tokens, Token};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Foo(#[serde(with = "arrays_serde")] [u8; 64]);

#[test]
fn test_array_serde() {
    let foo = Foo([0u8; 64]);
    assert_tokens(
        &foo,
        &[
            Token::NewtypeStruct { name: "Foo" },
            Token::Tuple { len: 64 },
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::TupleEnd,
        ],
    );
}
