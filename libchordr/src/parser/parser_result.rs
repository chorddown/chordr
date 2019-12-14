use super::{Meta, Node};

pub struct ParserResult {
    meta: Meta,
    node: Node,
}

impl ParserResult {
    pub(super) fn new(node: Node, meta: Meta) -> Self {
        Self { meta, node }
    }

    pub fn node(self) -> Node {
        self.node
    }

    pub fn node_as_ref(&self) -> &Node {
        &self.node
    }

    pub fn meta(&self) -> Meta {
        self.meta.clone()
    }
}
