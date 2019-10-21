/// Вершина графа.
#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: String,
    pub text: String,
}

/// Ребро графа.
#[derive(Debug, Clone)]
pub struct Edge {
    pub text: String,
}
