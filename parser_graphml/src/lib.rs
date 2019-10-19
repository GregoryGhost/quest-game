#[macro_use]
extern crate mdo;

pub mod parser_graphml {
    use mdo::option::bind;
    use petgraph::graph::{Graph, NodeIndex};
    use roxmltree::Node;

    pub fn read_graphml(xml_doc: &str) -> ResultGraphML {
        let doc = match roxmltree::Document::parse(xml_doc) {
            Ok(v) => v,
            Err(error) => return Err(Error::ParseXMLDocument(error)),
        };

        let (vertexes, edges): (Vec<GraphMLNode>, Vec<GraphMLNode>) =
            prepare_graphml(doc).into_iter().partition(|x| match x {
                GraphMLNode::Node(_) => true,
                _ => false,
            });

        format_graph(vertexes, edges)
    }

    fn prepare_graphml(doc: roxmltree::Document) -> Vec<GraphMLNode> {
        const NODE: &str = "node";
        const EDGE: &str = "edge";
        const NODE_TEXT_ATTR_KEY: &str = "d3";
        const EDGE_TEXT_ATTR_KEY: &str = "d10";
        const VERTEX_SOURCE_ATTR_KEY: &str = "source";
        const VERTEX_TARGET_ATTR_KEY: &str = "target";

        let nodes: Vec<GraphMLNode> = doc
            .root()
            .descendants()
            .filter(|node| node.is_element())
            .fold(Vec::new(), |mut acc, node| {
                match node.tag_name().name().trim() {
                    NODE => {
                        acc.push(GraphMLNode::Node(Vertex {
                            id: find_node_attr_by_key(&node, "id").expect("find vertex id"),
                            text: find_xml_node_text(&node, NODE_TEXT_ATTR_KEY)
                                .expect(format!("found node text {:?}", node).as_ref())
                                .to_string(),
                        }));
                        acc
                    }
                    EDGE => {
                        acc.push(GraphMLNode::Weight(XmlEdge {
                            source_id: find_node_attr_by_key(&node, VERTEX_SOURCE_ATTR_KEY)
                                .expect("got source id"),
                            target_id: find_node_attr_by_key(&node, VERTEX_TARGET_ATTR_KEY)
                                .expect("got target id"),
                            text: find_xml_node_text(&node, EDGE_TEXT_ATTR_KEY)
                                .map_or(Some("".to_string()), |x| Some(x.to_string()))
                                .unwrap(),
                        }));
                        acc
                    }
                    _ => acc,
                }
            });
        nodes
    }

    fn find_xml_node_text<'a>(node: &Node<'a, 'a>, attr_key: &str) -> Option<&'a str> {
        const TAG_DATA: &str = "data";
        const ATTR_TAG_KEY: &str = "key";
        const TAG_LIST: &str = "List";
        const TAG_LABEL: &str = "Label";
        const TAG_LABEL_TEXT: &str = "Label.Text";

        mdo! {
            data =<< node.children()
                .find(|x| {
                    let found_key = find_node_attr_by_key(x, ATTR_TAG_KEY)
                        .and_then(|x| if x == attr_key { Some(true) } else { None })
                        .is_some();

                    x.tag_name().name() == TAG_DATA && found_key
                });
            l =<< data.children()
                .find(|x| x.tag_name().name() == TAG_LIST);
            lbl =<< l.children()
                .find(|x| x.tag_name().name() == TAG_LABEL);
            lbl_txt =<< lbl.children()
                .find(|x| x.tag_name().name() == TAG_LABEL_TEXT);

            ret lbl_txt.text()
        }
    }

    pub type ResultGraphML<'a> = Result<Graph<Vertex, Edge>, Error>;

    pub enum Error {
        ParseXMLDocument(roxmltree::Error),
        PrepareGraphml(),
        FormatGraph(ErrorFormatGraph),
    }

    pub enum ErrorFormatGraph {
        NotFoundNodeById(String),
    }

    #[derive(Debug, Clone)]
    pub struct Vertex {
        pub id: String,
        pub text: String,
    }

    #[derive(Debug, Clone)]
    pub struct Edge {
        pub text: String,
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

    fn find_node_attr_by_key(node: &Node<'_, '_>, attr_key: &str) -> Option<String> {
        node.attributes()
            .iter()
            .find(|a| a.name().contains(attr_key))
            .and_then(|x| Some(x.value().into()))
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
}
