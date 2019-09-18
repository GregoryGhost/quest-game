use petgraph::graph::{NodeIndex, UnGraph};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::string::FromUtf8Error;

type XmlReader = Reader<std::io::BufReader<std::fs::File>>;

struct XmlEntryIterator {
    reader: XmlReader,
}

impl XmlEntryIterator {
    pub fn new(reader: XmlReader) -> XmlEntryIterator {
        XmlEntryIterator { reader: reader }
    }
}

impl Iterator for XmlEntryIterator {
    type Item = Result<Tag, quick_xml::Error>;

    fn next(&mut self) -> Option<quick_xml::Result<Tag>> {
        let mut buf = Vec::new();
        let mut tags: Vec<Tag> = Vec::new();

        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                        b"node" => {
                            let node = Tag::Node(Vertex {
                                id: find_node_id(&e, "id").expect("find node id"),
                                text: String::from("nop"),
                            });

                            println!("{:?}", node);

                            tags.push(node);
                        }
                        b"edge" => {
                            //TODO: какая-то проблема с нахождением вершин по ид
                            let source = get_node_by_tag(
                                &tags,
                                &find_node_id(&e, "source").expect("find source"),
                            )
                            .expect("get source");

                            println!("{:?}", source);

                            let target = get_node_by_tag(
                                &tags,
                                &find_node_id(&e, "target").expect("find target"),
                            )
                            .expect("get target");

                            println!("{:?}", target);

                            let edge = Tag::Weight(Edge {
                                source: source.clone(),
                                target: target.clone(),
                                text: String::from("nop"),
                            });

                            println!("{:?}", edge);

                            tags.push(edge);
                        }
                        _ => (),
                    },
                Ok(Event::Text(e)) => match tags.last_mut() {
                        Some(Tag::Node(t)) => {
                            t.text = e.unescape_and_decode(&self.reader).expect("Error content tag");
                        },
                        Some(Tag::Weight(t)) => {
                            t.text = e.unescape_and_decode(&self.reader).expect("Error content tag");
                        },
                        _ => (),
                    },
                Ok(Event::End(e)) => match e.name() {
                    b"node" | b"edge" => match tags.pop() {
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Vertex {
    id: String,
    text: String,
}

#[derive(Debug, Clone, Eq,PartialEq)]
struct Edge {
    source: Vertex,
    target: Vertex,
    text: String,
}

#[derive(Debug, Clone, Eq,PartialEq)]
enum Tag {
    Weight(Edge),
    Node(Vertex),
}

fn find_node_id(
    node: &quick_xml::events::BytesStart<'_>,
    attr_id: &'static str,
) -> Result<String, FromUtf8Error> {
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
    let val = String::from_utf8(t.value.into());
    val
}

fn get_node_by_tag<'a>(tags: &'a Vec<Tag>, search_node_id: &String) -> Option<&'a Vertex> {
    println!("tags: {:?}", tags);
    let found_node = tags
        .iter()
        .find(|tag| {
            println!("node by tag: {:?}", tag);
            if let Tag::Node(vertex) = tag {
                vertex.id == (*search_node_id)
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
                                &find_node_id(&e, "source").expect("find source"),
                            )
                            .expect("get source");
                            let target = get_node_by_tag(
                                &nodes,
                                &find_node_id(&e, "target").expect("find target"),
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
                                id: t.id.clone()
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

    // match read_graphml2("test.xml") {
        match read_graphml2("саси нло))) .graphml") {
        Ok(graphml) => {
            println!("graph: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}


fn read_graphml2(path: &'static str) -> Result<UnGraph<Vertex, Edge>, &'static str> {
    let reader = Reader::from_file(path);

    match reader {
        Ok(buf_reader) => {
            let xml_document = XmlEntryIterator::new(buf_reader);
            let (tag_vertexes, tag_edges): (Vec<Tag>, Vec<Tag>) = xml_document
                .map(|x| x.expect("get ok xml node value"))
                .partition(|x| match x { Tag::Node(_) => true, _ => false});

            let mut graph = UnGraph::<Vertex, Edge>::new_undirected();
            for tag_edge in tag_edges {
                if let Tag::Weight(edge) = tag_edge {
                    graph.add_edge(NodeIndex::new(0), NodeIndex::new(0), edge);
                }
            }

            for tag_vertex in tag_vertexes {
                if let Tag::Node(vertex) = tag_vertex {
                    graph.add_node(vertex);
                }
            }

            Ok(graph)
        }
        Err(_) => Err("error read xml"),
    }
}
