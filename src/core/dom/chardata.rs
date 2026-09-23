//! This module defines some interfaces related to `CharacterData`.

use super::{Node, NodeType};

/// `CharacterData` is an abstract interface defined at [DOM Standard](https://dom.spec.whatwg.org/#interface-characterdata).
///
/// NOTE (puppy patch): the trait is kept so that this module mirrors the DOM
/// Standard, but it is not used as a bound yet, hence the `dead_code` allowance.
#[allow(dead_code)]
pub trait CharacterData {}

/// `Text` is a kind of `Node`.
#[derive(Debug, PartialEq)]
pub struct Text {
    pub data: String,
}
impl Text {
    pub fn new(text: String) -> Box<Node> {
        Box::new(Node {
            node_type: NodeType::Text(Text { data: text }),
            children: vec![],
        })
    }
}
impl CharacterData for Text {}
