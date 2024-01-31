// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use solang_parser::{
    parse,
    pt::{ContractPart, SourceUnitPart},
};
use solang_parser::pt::{Comment, Loc};

#[test]
fn test_solang_parser() {
    use solang_parser::{
        parse,
        pt::{ContractPart, SourceUnitPart},
    };

    let (tree, comments) = parse(
        r#"
        contract flipper {
            bool private value;

            /// Constructor that initializes the `bool` value to the given `init_value`.
            constructor(bool initvalue) {
                value = initvalue;
            }

            /// A message that can be called on instantiated contracts.
            /// This one flips the value of the stored `bool` from `true`
            /// to `false` and vice versa.
            function flip() public {
                value = !value;
            }

            /// Simply returns the current value of our `bool`.
            function get() public view returns (bool) {
                return value;
            }
        }
        "#,
        0,
    )
    .unwrap();

    for part in &tree.0 {
        match part {
            SourceUnitPart::ContractDefinition(def) => {
                println!("found contract {:?}", def.name);
                for part in &def.parts {
                    match part {
                        ContractPart::VariableDefinition(def) => {
                            println!("variable {:?}", def.name);
                        }
                        ContractPart::FunctionDefinition(def) => {
                            println!("function {:?}", def.name);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    for comment in comments {
        let (loc, _comment) = match comment {
            Comment::Line(loc, _comment) => { (loc, _comment) }
            Comment::Block(loc, _comment) => { (loc, _comment) }
            Comment::DocLine(loc, _comment) => { (loc, _comment) }
            Comment::DocBlock(loc, _comment) => { (loc, _comment) }
        };
        let (l1, l2, l3) = match loc {
            Loc::File(a, b, c) => { (a,b,c) }
            _ => { unimplemented!() }
        };
        println!("Loc: {}:{}:{}, Comment: {}", l1, l2, l3, _comment);
    }
}
