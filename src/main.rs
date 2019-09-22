use petgraph::graph::{NodeIndex, UnGraph};
use quick_xml::events::Event;
use quick_xml::Reader;
use roxmltree::Node;
use std::fs::File;
use std::io::Read;
use std::process;
use std::string::FromUtf8Error;

fn main() {
    // match read_graphml_xquery("test.xml") {
    match read_graphml_xquery("саси нло))) .graphml") {
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

fn read_graphml_xquery(path: &'static str) -> ResultGraphML {
    let xml_doc = load_file(path);
    let doc = match roxmltree::Document::parse(&xml_doc) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}.", e);
            process::exit(1);
        }
    };
    let nodes: Vec<GraphMLNode> = doc
        .root()
        .descendants()
        .filter(|node| if node.is_element() { true } else { false })
        .fold(Vec::new(), |mut acc, node| {
            match node.tag_name().name().trim() {
                "node" => {
                    acc.push(GraphMLNode::Node(Vertex {
                        id: find_node_attr_by_key2(&node, "id").expect("find vertex id"),
                        text: find_xml_node_text(&node, "d3")
                            .expect(format!("found node text {:?}", node).as_ref())
                            .to_string(),
                    }));
                    acc
                }
                "edge" => {
                    acc.push(GraphMLNode::Weight(XmlEdge {
                        source_id: find_node_attr_by_key2(&node, "source").expect("got source id"),
                        target_id: find_node_attr_by_key2(&node, "target").expect("got target id"),
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
            let found_key = find_node_attr_by_key2(x, "key")
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

type XmlReader = Reader<std::io::BufReader<std::fs::File>>;

type ResultGraphML = Result<UnGraph<Vertex, Edge>, &'static str>;

struct XmlEntryIterator {
    reader: XmlReader,
}

impl XmlEntryIterator {
    pub fn new(reader: XmlReader) -> XmlEntryIterator {
        XmlEntryIterator { reader: reader }
    }
}

impl Iterator for XmlEntryIterator {
    type Item = Result<GraphMLNode, quick_xml::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        let mut xml_nodes: Vec<XmlNode> = Vec::new();
        let mut graphml_nodes: Vec<GraphMLNode> = Vec::new();

        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"node" => {
                        let node = GraphMLNode::Node(Vertex {
                            id: find_node_attr_by_key(&e, "id").expect("find node id"),
                            text: String::from("nop"),
                        });

                        println!("{:?}", node);

                        graphml_nodes.push(node);
                    }
                    b"edge" => {
                        let edge = GraphMLNode::Weight(XmlEdge {
                            source_id: find_node_attr_by_key(&e, "source").expect("got source id"),
                            target_id: find_node_attr_by_key(&e, "target").expect("got target id"),
                            text: String::from("nop"),
                        });

                        println!("{:?}", edge);

                        graphml_nodes.push(edge);
                    }
                    b"data" => {
                        match find_node_attr_by_key(&e, "key")
                            .expect("got data key")
                            .as_bytes()
                        {
                            b"d3" => {
                                xml_nodes.push(XmlNode::NodeData());
                            }
                            b"d10" => {
                                xml_nodes.push(XmlNode::WeightData());
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                },
                Ok(Event::CData(e)) => match graphml_nodes.last_mut() {
                    Some(GraphMLNode::Node(t)) => {
                        //TODO: для получения текста нужно найти тег дочерний тег data с ключом "d3" и текст находиться в List->Label->Label.Text [CDATA]
                        t.text = e
                            .unescape_and_decode(&self.reader)
                            .expect("Error content tag");
                        println!("node text: {:?}", t.text);
                    }
                    Some(GraphMLNode::Weight(t)) => {
                        //TODO: для получения текста нужно найти тег дочерний тег data с ключом "d10" и текст находиться в Data->List->Label->Label.Text [CDATA]

                        t.text = e
                            .unescape_and_decode(&self.reader)
                            .expect("Error content tag");
                        println!("weight text: {:?}", t.text);
                    }
                    _ => (),
                },
                Ok(Event::End(e)) => match e.name() {
                    b"node" | b"edge" => match graphml_nodes.pop() {
                        Some(tag) => return Some(Ok(tag)),
                        None => return None,
                    },
                    _ => (),
                },
                Err(e) => return Some(Err(e)),
                Ok(Event::Eof) => {
                    break;
                }
                _ => {}
            }
        }

        None
    }
}

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
enum XmlNode {
    Weight(XmlEdge),
    Node(Vertex),
    NodeData(),
    WeightData(),
}

#[derive(Debug)]
enum GraphMLNode {
    Weight(XmlEdge),
    Node(Vertex),
}

#[deprecated]
fn find_node_attr_by_key(
    node: &quick_xml::events::BytesStart<'_>,
    attr_key: &'static str,
) -> Result<String, FromUtf8Error> {
    let found_node_id = node
        .attributes()
        .find(|a| {
            let k = a.as_ref().expect("get node attribute").key.into();
            String::from_utf8(k)
                .expect("get attr by key")
                .contains(attr_key)
        })
        .expect("got found node attribute")
        .expect("got found node");
    let val = String::from_utf8(found_node_id.value.into());
    val
}

fn find_node_attr_by_key2(node: &Node<'_, '_>, attr_key: &'static str) -> Option<String> {
    let found_node_attr = node
        .attributes()
        .iter()
        .find(|a| a.name().contains(attr_key))
        .and_then(|x| Some(x.value().to_string()));

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

#[deprecated]
fn read_graphml(path: &'static str) -> ResultGraphML {
    let reader = Reader::from_file(path);

    match reader {
        Ok(buf_reader) => {
            let xml_document = XmlEntryIterator::new(buf_reader);
            let (xml_vertexes, xml_edges): (Vec<GraphMLNode>, Vec<GraphMLNode>) = xml_document
                .map(|x| x.expect("get ok xml node value"))
                .partition(|x| match x {
                    GraphMLNode::Node(_) => true,
                    _ => false,
                });

            format_graph(xml_vertexes, xml_edges)
        }
        Err(_) => Err("error read xml"),
    }
}

fn format_graph(vertexes: Vec<GraphMLNode>, edges: Vec<GraphMLNode>) -> ResultGraphML {
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
