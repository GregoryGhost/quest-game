use crate::types::*;
use petgraph::graph::Graph;

/// Результат парсинга графа из GraphML формата.
pub type ResultGraphML<'a> = Result<Graph<Vertex, Edge>, Error>;

/// Ошибки парсинга графа из GraphML формата.
#[derive(Debug)]
pub enum Error {
    /// Ошибки этапа парсинга графа из XML данных.
    ParseXMLDocument(roxmltree::Error),
    /// Ошибки этапа подготовки распарсенного графа
    ///     в промежуточное представление графа.
    PrepareGraphml(ErrorPrepareGraphML),
    /// Ошибки этапа форматирования GraphML представления графа
    ///     в Petgraph граф.
    FormatGraph(ErrorFormatGraph),
}

/// Ошибки этапа подготовления графа из GraphML формата.
#[derive(Debug)]
pub enum ErrorPrepareGraphML {
    /// Не найдена атрибут по ключу.
    NotFoundAttrByKey(String),
}

/// Ошибки этапа форматирования GraphML представления графа в Petgraph граф.
#[derive(Debug)]
pub enum ErrorFormatGraph {
    /// Не найдена нода графа по переданному идентификатору.
    NotFoundNodeById(String),
}
