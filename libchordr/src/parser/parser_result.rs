use super::{Metadata, Node};

pub struct ParserResult {
    pub metadata: Metadata,
    pub node: Node,
}

impl ParserResult {
    pub fn new(node: Node, metadata: Metadata) -> Self {
        Self { metadata, node }
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
