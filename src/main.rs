use petgraph::Graph;
use petgraph_graphml;
use log::debug;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
enum Tag {
    Tag3,
    Tag2
}

fn main() {
    //1.read xml
    //2.make graph from xml - manual
    //3.print graph
    //let graph = make_graph();
    let reader1 = Reader::from_file("test.xml");

    match reader1 {
        Ok(mut reader) => {
            let mut buf = Vec::new();
            let mut tags = Vec::new();
            let mut current_tags: Vec<Tag> = Vec::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        match e.name() {
                            b"tag3" => {
                                current_tags.push(Tag::Tag3);
                            },
                            _ => ()
                        }
                    },
                    Ok(Event::Text(e)) => {
                        match current_tags.last() {
                            Some(_) => tags.push(e.unescape_and_decode(&reader).expect("Error content tag")),
                            None => (),
                        }
                    },
                    Ok(Event::End(e)) => {
                        match e.name() {
                            b"tag3" => {
                                let _ = current_tags.pop();
                            },
                            _ => ()
                        }
                    }
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
                buf.clear();
            }
            println!("current_tag: {:?}", current_tags);
            println!("Tags3 events: {:?}", tags);
        },
        Err(error) => println!("{}", error),
    }
}

fn make_graph() -> Graph<&'static str, &'static str> {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    deps.extend_with_edges(&[
        (pg, fb), (pg, qc),
        (qc, rand), (rand, libc), (qc, libc),
    ]);
    deps.clone()
}