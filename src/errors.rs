use crate::parser::NodeId;

#[derive(Debug)]
pub enum Severity {
    Error,
    Note,
}

#[derive(Debug)]
pub struct SourceError {
    pub message: String,
    pub node_id: NodeId,
    pub severity: Severity,
}

pub enum ErrorKind {
    SourceError,
    NameBindingError,
    TypeError,
}

impl ErrorKind {
    pub fn code(&self) -> &str {
        match self {
            ErrorKind::SourceError => "nu::parser::source_error",
            ErrorKind::NameBindingError => "nu::parser::name_binding_error",
            ErrorKind::TypeError => "nu::parser::type_error",
        }
    }
}

pub struct ErrorLabel {
    msg: String,
    node_id: NodeId,
}

struct ParseError {
    pub kind: ErrorKind,
    pub labels: Vec<ErrorLabel>,
    pub help: Option<String>,
}

impl ParseError {
    pub fn new(kind: ErrorKind, msg: impl Into<String>, node_id: NodeId) -> Self {
        Self {
            kind,
            labels: vec![ErrorLabel { msg: msg.into(), node_id }],
            help: None
        }
    }

    pub fn help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    pub fn label(mut self, msg: impl Into<String>, node_id: NodeId) -> Self {
        self.labels.push(ErrorLabel { msg: msg.into(), node_id });
        self
    }

    pub fn type_mismatch(lhs_type: &str, lhs: NodeId, op_type: &str, op: NodeId, rhs_type: &str, rhs: NodeId) -> Self {
        Self::new(ErrorKind::TypeError, op_type, op).label(lhs_type, lhs).label(rhs_type, rhs)
    }
}
