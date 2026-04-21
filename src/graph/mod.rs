use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
}

pub fn connect(edges: &mut Vec<GraphEdge>, from: impl Into<String>, to: impl Into<String>) {
    edges.push(GraphEdge {
        from: from.into(),
        to: to.into(),
    });
}
