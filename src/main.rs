use petgraph::graph::{NodeIndex, UnGraph};
use quick_xml::events::Event;
use quick_xml::Reader;

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

fn read_graphml(path: &'static str) -> Result<UnGraph<Vertex, Edge>, &'static str> {
    let reader1 = Reader::from_file(path);

    //TODO: переписать на итератор, возвращающий просто xml-ноды, имеющие атрибуты и т.д.
    match reader1 {
        Ok(mut reader) => {
            let mut buf = Vec::new();
            let mut current_tags: Vec<Tag> = Vec::new();
            let mut graph = UnGraph::<Vertex, Edge>::new_undirected();

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"node" => {
                            let t = e.attributes().find(|a| {
                                let k = a.as_ref().unwrap().key.into();
                                String::from_utf8(k).unwrap().contains("id")
                            });
                            let val = String::from_utf8(t.unwrap().unwrap().value.into()).unwrap();
                            let node = Tag::Node(Vertex {
                                id: val.parse::<u64>().unwrap(),
                                text: String::from("nop"),
                            });
                            println!("{:?}", node);
                            current_tags.push(node);
                        }
                        b"edge" => {
                            let source_id = {
                                let t = e
                                    .attributes()
                                    .find(|a| {
                                        let k = a.as_ref().unwrap().key.into();
                                        String::from_utf8(k).unwrap().contains("source")
                                    })
                                    .unwrap()
                                    .unwrap();
                                let val = String::from_utf8(t.value.into()).unwrap();
                                val.parse::<u64>().unwrap()
                            };

                            let target_id = {
                                let t = e
                                    .attributes()
                                    .find(|a| {
                                        let k = a.as_ref().unwrap().key.into();
                                        String::from_utf8(k).unwrap().contains("target")
                                    })
                                    .unwrap()
                                    .unwrap();
                                let val = String::from_utf8(t.value.into()).unwrap();
                                val.parse::<u64>().unwrap()
                            };
                            let source = {
                                let s = current_tags
                                    .iter()
                                    .find(|tag| {
                                        if let Tag::Node(a) = tag {
                                            a.id == source_id
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap();
                                match s {
                                    Tag::Node(a) => a,
                                    _ => unreachable!(),
                                }
                            };
                            let target = {
                                let s = current_tags
                                    .iter()
                                    .find(|tag| {
                                        if let Tag::Node(a) = tag {
                                            a.id == target_id
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap();
                                match s {
                                    Tag::Node(a) => a,
                                    _ => unreachable!(),
                                }
                            };
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
                            let _ = current_tags.pop();
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

    match read_graphml("test.xml") {
        Ok(graphml) => {
            println!("graph: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}
