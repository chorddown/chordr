use super::{MetaInformation, Node};

pub struct ParserResult {
    pub meta: MetaInformation,
    pub node: Node,
}

impl ParserResult {
    pub fn new(node: Node, meta: MetaInformation) -> Self {
        Self { meta, node }
    }

    pub fn node(self) -> Node {
        self.node
    }

    pub fn node_as_ref(&self) -> &Node {
        &self.node
    }

    pub fn meta(&self) -> MetaInformation {
        self.meta.clone()
    }

    pub fn meta_as_ref(&self) -> &MetaInformation {
        &self.meta
    }
}
