use mdo::option::bind;
use petgraph::graph::{Graph, NodeIndex};
use roxmltree::Node;

use crate::errors::*;
use crate::types::*;

/// Распарсить граф из GraphML формата.
///
/// # Errors
///
/// Если парсинг провалился, то возвращает ошибки парсинга [`errors::Error`].
pub fn read_graphml(xml_doc: &str) -> ResultGraphML {
    let doc = match roxmltree::Document::parse(xml_doc) {
        Ok(v) => v,
        Err(error) => return Err(Error::ParseXMLDocument(error)),
    };

    let (vertexes, edges): (Vec<GraphMLNode>, Vec<GraphMLNode>) =
        prepare_graphml(doc)?.into_iter().partition(|x| match x {
            GraphMLNode::Node(_) => true,
            _ => false,
        });

    format_graph(vertexes, edges)
}

#[derive(Debug)]
struct XmlEdge {
    source_id: String,
    target_id: String,
    text: String,
}

#[derive(Debug)]
enum GraphMLNode {
    Weight(XmlEdge),
    Node(Vertex),
}

fn prepare_graphml(doc: roxmltree::Document) -> Result<Vec<GraphMLNode>, Error> {
    const NODE: &str = "node";
    const EDGE: &str = "edge";
    const NODE_TEXT_ATTR_KEY: &str = "d3";
    const EDGE_TEXT_ATTR_KEY: &str = "d10";
    const VERTEX_SOURCE_ATTR_KEY: &str = "source";
    const VERTEX_TARGET_ATTR_KEY: &str = "target";

    let filtered_nodes = doc.root().descendants().filter(|node| node.is_element());

    let mut acc: Vec<GraphMLNode> = Vec::new();
    for node in filtered_nodes {
        match node.tag_name().name().trim() {
            NODE => {
                acc.push(GraphMLNode::Node(Vertex {
                    id: find_node_attr_by_key(&node, "id")?,
                    text: find_xml_node_text(&node, NODE_TEXT_ATTR_KEY)?.to_string(),
                }));
            }
            EDGE => {
                acc.push(GraphMLNode::Weight(XmlEdge {
                    source_id: find_node_attr_by_key(&node, VERTEX_SOURCE_ATTR_KEY)?,
                    target_id: find_node_attr_by_key(&node, VERTEX_TARGET_ATTR_KEY)?,
                    text: find_xml_node_text(&node, EDGE_TEXT_ATTR_KEY)
                        .unwrap_or("")
                        .to_string(),
                }));
            }
            _ => (),
        }
    }

    Ok(acc)
}

fn find_xml_node_text<'a>(node: &Node<'a, 'a>, attr_key: &str) -> Result<&'a str, Error> {
    const TAG_DATA: &str = "data";
    const ATTR_TAG_KEY: &str = "key";
    const TAG_LIST: &str = "List";
    const TAG_LABEL: &str = "Label";
    const TAG_LABEL_TEXT: &str = "Label.Text";

    let result = mdo! {
        data =<< node.children()
            .find(|x| {
                let found_key = match find_node_attr_by_key(x, ATTR_TAG_KEY) {
                    Ok(x) => x == attr_key,
                    _ => false,
                };

                x.tag_name().name() == TAG_DATA && found_key
            });
        l =<< data.children()
            .find(|x| x.tag_name().name() == TAG_LIST);
        lbl =<< l.children()
            .find(|x| x.tag_name().name() == TAG_LABEL);
        lbl_txt =<< lbl.children()
            .find(|x| x.tag_name().name() == TAG_LABEL_TEXT);

        ret lbl_txt.text()
    };

    result.ok_or(Error::PrepareGraphml(
        ErrorPrepareGraphML::NotFoundAttrByKey(attr_key.to_string()),
    ))
}

fn find_node_attr_by_key(node: &Node<'_, '_>, attr_key: &str) -> Result<String, Error> {
    node.attributes()
        .iter()
        .find(|a| a.name().contains(attr_key))
        .and_then(|a| Some(a.value().into()))
        .ok_or(Error::PrepareGraphml(
            ErrorPrepareGraphML::NotFoundAttrByKey(attr_key.to_string()),
        ))
}

fn format_graph<'a>(vertexes: Vec<GraphMLNode>, edges: Vec<GraphMLNode>) -> ResultGraphML<'a> {
    use std::collections::HashMap;

    let mut graph = Graph::<Vertex, Edge>::new();
    let mut vertex_indexes: HashMap<&String, NodeIndex> = HashMap::new();

    for vertex in &vertexes {
        if let GraphMLNode::Node(v) = vertex {
            vertex_indexes.insert(&v.id, graph.add_node(v.clone()));
        }
    }

    for edge in &edges {
        if let GraphMLNode::Weight(e) = edge {
            let edge = Edge {
                text: e.text.clone(),
            };

            let try_get_node_by_id = |id: &String| match vertex_indexes.get(id) {
                Some(node_index) => Ok(*node_index),
                None => {
                    return Err(Error::FormatGraph(ErrorFormatGraph::NotFoundNodeById(
                        id.to_string(),
                    )))
                }
            };
            graph.add_edge(
                try_get_node_by_id(&e.source_id)?,
                try_get_node_by_id(&e.target_id)?,
                edge,
            );
        }
    }

    Ok(graph)
}
