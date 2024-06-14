use std::marker::PhantomData;

use crate::node::{DefaultNode, Node, SomePacketHandler};

pub struct Open;
pub struct Terminated;

pub trait PipelineState {}
impl PipelineState for Open {}
impl PipelineState for Terminated {}

pub struct NodeBuilder {
    name: String,
    handler: SomePacketHandler,
}

impl NodeBuilder {
    pub fn new<T: Into<String>>(name: T, handler: SomePacketHandler) -> Self {
        Self {
            name: name.into(),
            handler,
        }
    }

    fn build(self) -> Box<dyn Node> {
        DefaultNode::from_handler(self.name, self.handler)
    }
}

pub struct PipelineBuilder<T: PipelineState> {
    nodes: Vec<NodeBuilder>,
    _state: PhantomData<T>,
}

impl Default for PipelineBuilder<Open> {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineBuilder<Open> {
    pub fn new() -> Self {
        PipelineBuilder::<Open> {
            nodes: Vec::new(),
            _state: PhantomData,
        }
    }
}

impl<T: PipelineState> PipelineBuilder<T> {
    pub fn build(self) -> Box<dyn Node> {
        let mut prev_node: Option<Box<dyn Node>> = None;
        for node_builder in self.nodes.into_iter().rev() {
            let mut node = node_builder.build();
            if let Some(prev_node) = prev_node {
                node.attach(prev_node);
            }
            prev_node = Some(node);
        }
        prev_node.unwrap()
    }
}

impl PipelineBuilder<Open> {
    pub fn attach<T: Into<String>, U: Into<SomePacketHandler>>(
        mut self,
        name: T,
        handler: U,
    ) -> PipelineBuilder<Open> {
        self.nodes.push(NodeBuilder::new(name, handler.into()));
        self
    }

    pub fn demux<T: Into<String>, U: Into<SomePacketHandler>>(
        mut self,
        name: T,
        handler: U,
    ) -> PipelineBuilder<Terminated> {
        self.nodes.push(NodeBuilder::new(name, handler.into()));
        PipelineBuilder::<Terminated> {
            nodes: self.nodes,
            _state: PhantomData::<Terminated>,
        }
    }
}
