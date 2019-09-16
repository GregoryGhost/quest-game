use petgraph::graph::{NodeIndex, UnGraph};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::num::ParseIntError;

type XmlReader = Reader<std::io::BufReader<std::fs::File>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct XmlNode {
    text: Option<String>,
    tag: String,
    attributes: Option<Vec<String>>,
}

struct XmlEntryIterator {
    reader: XmlReader,
}

impl XmlEntryIterator {
    pub fn new(reader: XmlReader) -> XmlEntryIterator {
        XmlEntryIterator { reader: reader }
    }
}

impl Iterator for XmlEntryIterator {
    type Item = Result<XmlNode, quick_xml::Error>;

    fn next(&mut self) -> Option<quick_xml::Result<XmlNode>> {
        let mut buf = Vec::new();
        let mut nodes: Vec<XmlNode> = Vec::new();

        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"node" | b"edge" => {
                        //TODO: нужно переписать как-то по другому
                        let node = XmlNode {
                            text: None,
                            tag: String::from(std::str::from_utf8(e.name()).unwrap()),
                            attributes: None,
                        };
                        nodes.push(node);
                    }
                    _ => {}
                },
                Ok(Event::Text(e)) => match nodes.last() {
                    Some(t) => {
                        nodes.push(XmlNode {
                            text: Some(
                                e.unescape_and_decode(&self.reader)
                                    .expect("Error content tag"),
                            ),
                            ..(*t).clone() //TODO: нужно переписать на Box<str>
                        });
                    }
                    _ => {}
                },
                Ok(Event::End(e)) => match e.name() {
                    b"node" | b"edge" => match nodes.pop() {
                        Some(node) => return Some(Ok(node)),
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
    id: u64,
    text: String,
}

#[derive(Debug, Clone)]
struct Edge {
    source: Vertex,
    target: Vertex,
    text: String,
}

#[derive(Debug, Clone)]
enum Tag {
    Weight(Edge),
    Node(Vertex),
}

fn find_node_id(
    node: &quick_xml::events::BytesStart<'_>,
    attr_id: &'static str,
) -> Result<u64, ParseIntError> {
    let t = node
        .attributes()
        .find(|a| {
            let k = a.as_ref().expect("get node attribute").key.into();
            String::from_utf8(k)
                .expect("get attr by key")
                .contains(attr_id)
        })
        .expect("got found node attribute")
        .expect("got found node");
    let val = String::from_utf8(t.value.into()).expect("get value node");
    val.parse::<u64>()
}

fn get_node_by_tag(tags: &Vec<Tag>, search_node_id: u64) -> Option<&Vertex> {
    println!("tags: {:?}", tags);
    let found_node = tags
        .iter()
        .find(|tag| {
            println!("node by tag: {:?}", tag);
            if let Tag::Node(vertex) = tag {
                vertex.id == search_node_id
            } else {
                false
            }
        })
        .expect("got found node by tag");
    match found_node {
        Tag::Node(vertex) => Some(vertex),
        _ => None,
    }
}

#[deprecated]
fn read_graphml(path: &'static str) -> Result<UnGraph<Vertex, Edge>, &'static str> {
    let reader1 = Reader::from_file(path);

    match reader1 {
        Ok(mut reader) => {
            let mut buf = Vec::new();
            let mut current_tags: Vec<Tag> = Vec::new();
            let mut nodes: Vec<Tag> = Vec::new();
            let mut graph = UnGraph::<Vertex, Edge>::new_undirected();

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"node" => {
                            let node = Tag::Node(Vertex {
                                id: find_node_id(&e, "id").expect("find node id"),
                                text: String::from("nop"),
                            });
                            println!("{:?}", node);
                            current_tags.push(node);
                        }
                        b"edge" => {
                            let source = get_node_by_tag(
                                &nodes,
                                find_node_id(&e, "source").expect("find source"),
                            )
                            .expect("get source");
                            let target = get_node_by_tag(
                                &nodes,
                                find_node_id(&e, "target").expect("find target"),
                            )
                            .expect("get target");
                            let edge = Tag::Weight(Edge {
                                source: source.clone(),
                                target: target.clone(),
                                text: String::from("nop"),
                            });

                            current_tags.push(edge);
                        }
                        _ => (),
                    },
                    Ok(Event::Text(e)) => match current_tags.last() {
                        Some(Tag::Node(t)) => {
                            graph.add_node(Vertex {
                                text: e.unescape_and_decode(&reader).expect("Error content tag"),
                                ..*t
                            });
                        }
                        Some(Tag::Weight(t)) => {
                            graph.add_edge(NodeIndex::new(0), NodeIndex::new(0), t.clone());
                        }
                        _ => (),
                    },
                    Ok(Event::End(e)) => match e.name() {
                        b"node" | b"edge" => {
                            let tag = current_tags.pop();
                            match tag {
                                Some(t) => {
                                    if let Tag::Node(_) = t {
                                        nodes.push(t);
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    },
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
                buf.clear();
            }
            Ok(graph.clone())
        }
        Err(_) => Err("error read xml"),
    }
}

fn main() {
    //1.read xml
    //2.make graph from xml - manual
    //3.print graph

    match read_graphml2("test.xml") {
        Ok(graphml) => {
            println!("graph: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}

fn find_node_id2(node: &XmlNode, attr_id: &'static str) -> Result<u64, ParseIntError> {
    unimplemented!();
}

fn get_node_by_tag2(tags: &Vec<XmlNode>, search_node_id: u64) -> Option<&Vertex> {
    unimplemented!();
}

fn read_graphml2(path: &'static str) -> Result<UnGraph<Vertex, Edge>, &'static str> {
    let reader = Reader::from_file(path);

    match reader {
        Ok(buf_reader) => {
            let xml_document = XmlEntryIterator::new(buf_reader);
            let (xml_vertexes, xml_edges): (Vec<XmlNode>, Vec<XmlNode>) = xml_document
                .filter(|x| {
                    if let Ok(y) = x {
                        if y.tag == "node" || y.tag == "edge" {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .map(|x| x.expect("get ok xml node value"))
                .partition(|x| x.tag == "node");

            let mut graph = UnGraph::<Vertex, Edge>::new_undirected();
            for xml_vertex in xml_vertexes {
                graph.add_node(Vertex {
                    id: find_node_id2(&xml_vertex, "id").expect("find node id"),
                    text: xml_vertex.text.expect("get node text"),
                });
            }

            for xml_edge in &xml_edges {
                let source_vertex = get_node_by_tag2(
                    xml_edges.as_ref(),
                    find_node_id2(&xml_edge, "source").expect("find source vertex"),
                ).expect("get source vertex");
                let target_vertex = get_node_by_tag2(
                    xml_edges.as_ref(),
                    find_node_id2(&xml_edge, "target").expect("find target vertex"),
                ).expect("get target vertex");
                let edge = Edge {
                    source: source_vertex.clone(),
                    target: target_vertex.clone(),
                    text: (*xml_edge).clone().text.expect("get edge text"),//TODO: нужно сделать через Deref
                };
                graph.add_edge(NodeIndex::new(0), NodeIndex::new(0), edge);
            }

            Ok(graph)
        }
        Err(_) => Err("error read xml"),
    }
}
