use petgraph::graph::{NodeIndex, UnGraph};
use roxmltree::Node;
use std::fs::File;
use std::io::Read;

fn main() {
    match read_graphml("саси нло))) .graphml") {
        Ok(graphml) => {
            println!("graph: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}

fn load_file(path: &str) -> String {
    let mut file = File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn read_graphml(path: &'static str) -> ResultGraphML {
    let xml_doc = load_file(path);
    let doc = match roxmltree::Document::parse(&xml_doc) {
        Ok(v) => v,
        Err(_) => return Err("Error parse xml document"),
    };

    let nodes: Vec<GraphMLNode> = doc
        .root()
        .descendants()
        .filter(|node| if node.is_element() { true } else { false })
        .fold(Vec::new(), |mut acc, node| {
            match node.tag_name().name().trim() {
                "node" => {
                    acc.push(GraphMLNode::Node(Vertex {
                        id: find_node_attr_by_key(&node, "id").expect("find vertex id"),
                        text: find_xml_node_text(&node, "d3")
                            .expect(format!("found node text {:?}", node).as_ref())
                            .to_string(),
                    }));
                    acc
                }
                "edge" => {
                    acc.push(GraphMLNode::Weight(XmlEdge {
                        source_id: find_node_attr_by_key(&node, "source").expect("got source id"),
                        target_id: find_node_attr_by_key(&node, "target").expect("got target id"),
                        text: find_xml_node_text(&node, "d10")
                            .map_or(Some("".to_string()), |x| Some(x.to_string()))
                            .unwrap(),
                    }));
                    acc
                }
                _ => acc,
            }
        });

    let (vertexes, edges): (Vec<GraphMLNode>, Vec<GraphMLNode>) =
        nodes.into_iter().partition(|x| match x {
            GraphMLNode::Node(_) => true,
            _ => false,
        });

    format_graph(vertexes, edges)
}

fn find_xml_node_text<'a>(node: &Node<'a, 'a>, attr_key: &'static str) -> Option<&'a str> {
    //TODO: нужно переписать, слишком большая вложенность
    node.children()
        .into_iter()
        .find(|x| {
            let found_key = find_node_attr_by_key(x, "key")
                .and_then(|x| if x == attr_key { Some(true) } else { None })
                .is_some();
            if x.tag_name().name() == "data" && found_key {
                true
            } else {
                false
            }
        })
        .and_then(|x| {
            x.children()
                .find(|x| {
                    if x.tag_name().name() == "List" {
                        true
                    } else {
                        false
                    }
                })
                .and_then(|x1| {
                    x1.children()
                        .find(|x| {
                            if x.tag_name().name() == "Label" {
                                true
                            } else {
                                false
                            }
                        })
                        .and_then(|x2| {
                            x2.children()
                                .find(|x| {
                                    if x.tag_name().name() == "Label.Text" {
                                        true
                                    } else {
                                        false
                                    }
                                })
                                .and_then(|x3| x3.text())
                        })
                })
        })
}

type ResultGraphML<'a> = Result<UnGraph<Vertex, Edge>, &'static str>;

#[derive(Debug, Clone)]
struct Vertex {
    id: String,
    text: String,
}

#[derive(Debug, Clone)]
struct Edge {
    source: Vertex,
    target: Vertex,
    text: String,
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

fn find_node_attr_by_key(node: &Node<'_, '_>, attr_key: &'static str) -> Option<String> {
    let found_node_attr = node
        .attributes()
        .iter()
        .find(|a| a.name().contains(attr_key))
        .and_then(|x| Some(x.value().into()));

    found_node_attr
}

fn get_node_by_id<'a>(nodes: &'a Vec<GraphMLNode>, search_node_id: &String) -> Option<&'a Vertex> {
    nodes
        .iter()
        .find(|graph_ml_node| {
            if let GraphMLNode::Node(vertex) = graph_ml_node {
                vertex.id == (*search_node_id)
            } else {
                false
            }
        })
        .and_then(|found_node| {
            if let GraphMLNode::Node(vertex) = found_node {
                Some(vertex)
            } else {
                None
            }
        })
}

fn format_graph<'a>(vertexes: Vec<GraphMLNode>, edges: Vec<GraphMLNode>) -> ResultGraphML<'a> {
    let mut graph = UnGraph::<Vertex, Edge>::new_undirected();

    for vertex in &vertexes {
        if let GraphMLNode::Node(v) = vertex {
            graph.add_node(v.clone());
        }
    }

    for edge in &edges {
        if let GraphMLNode::Weight(e) = edge {
            let edge = Edge {
                source: get_node_by_id(&vertexes, &e.source_id)
                    .expect("got vertex")
                    .clone(),
                target: get_node_by_id(&vertexes, &e.target_id)
                    .expect("got vertex")
                    .clone(),
                text: e.text.clone(),
            };
            graph.add_edge(NodeIndex::new(0), NodeIndex::new(0), edge);
        }
    }

    Ok(graph)
}
