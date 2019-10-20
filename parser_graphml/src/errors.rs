use crate::types::*;
use petgraph::graph::Graph;

pub type ResultGraphML<'a> = Result<Graph<Vertex, Edge>, Error>;

#[derive(Debug)]
pub enum Error {
    ParseXMLDocument(roxmltree::Error),
    PrepareGraphml(ErrorPrepareGraphML),
    FormatGraph(ErrorFormatGraph),
}

#[derive(Debug)]
pub enum ErrorPrepareGraphML {
    NotFoundAttrByKey(String),
}

#[derive(Debug)]
pub enum ErrorFormatGraph {
    NotFoundNodeById(String),
}
