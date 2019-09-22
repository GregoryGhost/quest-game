use petgraph::graph::{NodeIndex, UnGraph};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::string::FromUtf8Error;

fn main() {
    match read_graphml("test.xml") {
        // match read_graphml("саси нло))) .graphml") {
        Ok(graphml) => {
            println!("graph: {:?}", graphml);
        }
        Err(error) => println!("{:?}", error),
    }
}

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

fn get_node_by_id<'a>(nodes: &'a Vec<GraphMLNode>, search_node_id: &String) -> Option<&'a Vertex> {
    let found_node = nodes.iter().find(|graph_ml_node| {
        if let GraphMLNode::Node(vertex) = graph_ml_node {
            vertex.id == (*search_node_id)
        } else {
            false
        }
    });

    match found_node {
        Some(GraphMLNode::Node(vertex)) => Some(vertex),
        _ => None,
    }
}

fn read_graphml(path: &'static str) -> Result<UnGraph<Vertex, Edge>, &'static str> {
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

            let mut graph = UnGraph::<Vertex, Edge>::new_undirected();

            for xml_node in &xml_vertexes {
                if let GraphMLNode::Node(vertex) = xml_node {
                    graph.add_node(vertex.clone());
                }
            }

            for xml_node in &xml_edges {
                if let GraphMLNode::Weight(xml_edge) = xml_node {
                    let edge = Edge {
                        source: get_node_by_id(&xml_vertexes, &xml_edge.source_id)
                            .expect("got vertex")
                            .clone(),
                        target: get_node_by_id(&xml_vertexes, &xml_edge.target_id)
                            .expect("got vertex")
                            .clone(),
                        text: xml_edge.text.clone(),
                    };
                    graph.add_edge(NodeIndex::new(0), NodeIndex::new(0), edge);
                }
            }

            Ok(graph)
        }
        Err(_) => Err("error read xml"),
    }
}
