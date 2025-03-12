use crate::parser::{AstNode, NodeId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RedirectionSource {
    Stdout,
    Stderr,
    StdoutAndStderr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RedirectionTarget {
    File { expr: NodeId, append: bool },
    Pipe { expr: NodeId },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipelineRedirection {
    Single {
        source: RedirectionSource,
        target: RedirectionTarget,
    },
    Separate {
        out: RedirectionTarget,
        err: RedirectionTarget,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipelineElement {
    pub inp_pipe: Option<NodeId>, // input pipe to the element, TODO: Is this necessary?
    pub expr: NodeId,
    pub redirection: Option<PipelineRedirection>,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub elements: Vec<PipelineElement>,
}

// TODO: How to call this?
#[derive(Debug, Clone)]
pub enum Sentence {
    Pipeline(NodeId),  // AstNode::Pipeline
    Statement(NodeId), // AstNode::Statement
}

#[derive(Debug, Clone)]
pub struct Block {
    pub sentences: Vec<Sentence>,
}

impl Block {
    pub fn new(sentences: Vec<Sentence>) -> Block {
        Block { sentences }
    }
}

pub fn get_redirection_src_tgt(
    redirection_node: &AstNode,
    expr: NodeId,
) -> (RedirectionSource, RedirectionTarget) {
    match redirection_node {
        AstNode::RedirectOutToFile => (
            RedirectionSource::Stdout,
            RedirectionTarget::File {
                expr,
                append: false,
            },
        ),
        AstNode::RedirectOutToFileAppend => (
            RedirectionSource::Stdout,
            RedirectionTarget::File { expr, append: true },
        ),
        AstNode::RedirectErrToFile => (
            RedirectionSource::Stderr,
            RedirectionTarget::File {
                expr,
                append: false,
            },
        ),
        AstNode::RedirectErrToFileAppend => (
            RedirectionSource::Stderr,
            RedirectionTarget::File { expr, append: true },
        ),
        AstNode::RedirectErrToPipe => (RedirectionSource::Stderr, RedirectionTarget::Pipe { expr }),
        AstNode::RedirectOutErrToFile => (
            RedirectionSource::StdoutAndStderr,
            RedirectionTarget::File {
                expr,
                append: false,
            },
        ),
        AstNode::RedirectOutErrToFileAppend => (
            RedirectionSource::StdoutAndStderr,
            RedirectionTarget::File { expr, append: true },
        ),
        AstNode::RedirectOutErrToPipe => (
            RedirectionSource::StdoutAndStderr,
            RedirectionTarget::Pipe { expr },
        ),
        _ => panic!("redirection is not redirection"),
    }
}
