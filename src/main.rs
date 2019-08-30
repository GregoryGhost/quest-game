use log::debug;
use petgraph::Graph;
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
    Weight(Vertex),
    Node(Vertex),
}

fn read_graphml(path: &'static str) -> Result<Graph<Vertex, String>, &'static str> {
    let reader1 = Reader::from_file(path);
    //TODO: переписать на итератор, возвращающий просто xml-ноды, имеющие атрибуты и т.д.
    match reader1 {
        Ok(mut reader) => {
            let mut buf = Vec::new();
            let mut current_tags: Vec<Tag> = Vec::new();
            let mut graph = Graph::<Vertex, String>::new();

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"node" => {
                            let t = e.attributes()
                                .find(|a| {
                                    let k = a.as_ref().unwrap().key.into();
                                    return String::from_utf8(k).unwrap().contains("id")
                                });
                            let val = String::from_utf8(t.unwrap().unwrap().value.into()).unwrap();
                            let node = Tag::Node(Vertex {
                                id: val.parse::<u64>().unwrap(),
                                text: String::from("nop"),
                            });
                            println!("{:?}", node);
                            current_tags.push(node);
                        }
                        // b"edge" => {
                        //     current_tags.push(Tag::Weight);
                        // }
                        _ => (),
                    },
                    Ok(Event::Text(e)) => match current_tags.last() {
                        Some(Tag::Node(t)) => {
                            graph.add_node(
                                Vertex{text: e.unescape_and_decode(&reader).expect("Error content tag"),..*t},
                            );
                        }
                        // Some(Tag::Weight) => (),
                        _ => (),
                    },
                    Ok(Event::End(e)) => match e.name() {
                        b"node" => {
                            let _ = current_tags.pop();
                        }
                        // b"edge" => {
                        //     let _ = current_tags.pop();
                        // }
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
            println!("nodes: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}

fn make_graph() -> Graph<&'static str, &'static str> {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);
    deps.clone()
}
