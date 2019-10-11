use log::{debug, error, trace};
use logger::init_logger;
use mdo::option::bind;
use petgraph::graph::EdgeReference;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use roxmltree::Node;
use std::fs::File;
use std::io::stdin;
use std::io::Read;

#[path = "logger.rs"]
pub mod logger;

#[macro_use]
extern crate mdo;

fn main() {
    //TODO: нужно залогировать важные события в игре
    init_logger();

    const PATH: &str = "scenes-choices.graphml";

    let graph = read_graphml(PATH).expect("failed read graphml"); //TODO: здесь может быть можно переделать на mdo!

    start_game(&graph);
}

fn start_game(graph: &Graph<Vertex, Edge>) {
    //TODO: заменить expect на match с error!(???)
    const EXIT_CODE: usize = 0;

    let mut input = String::new();
    let mut number: usize;

    let mut vertex_ix: NodeIndex;
    vertex_ix = graph
        .node_indices()
        .take(1)
        .next()
        .expect("failed get first vertex in graph");

    let mut out_edges: Vec<EdgeReference<'_, Edge>>;

    loop {
        input.clear();
        println!("Сцена: {}", graph[vertex_ix].text);
        out_edges = graph
            .edges_directed(vertex_ix, Direction::Outgoing)
            .collect();

        if out_edges.is_empty() {
            println!("Больше нету действий, выхожу...");
            break;
        }

        println!("Выберите действие: ");
        let mut i = 1;
        for edge in &out_edges {
            println!("{}. {}", i, edge.weight().text.clone());
            i += 1;
        }
        match stdin().read_line(&mut input) {
            Ok(_) => {
                number = match input.trim_end().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("Введен некорректный номер.");
                        continue;
                    }
                };

                if number == EXIT_CODE {
                    println!("Получен код выхода - выхожу...");
                    break;
                }

                if 1 <= number && number <= i {
                    trace!(
                        "Получен верный номер варианта {}",
                        number
                    );
                    trace!("out edges count: {}", out_edges.len());
                    trace!("out edges: {:?}", out_edges);

                    if let Some(found_vertex_ix) = out_edges.get(usize::from(number - 1)) {
                        vertex_ix = found_vertex_ix.target();
                    } else {
                        println!("Не получилось получить вариант");
                    }
                    debug!("vertex_ix {:?}", graph[vertex_ix]);
                } else {
                    println!(
                        "Введен неверный номер варианта, попробуйте снова."
                    );
                }
            }
            Err(e) => {
                error!("error {:?}", e);
                continue;
            }
        }
    }
}

fn load_file(path: &str) -> String {
    use std::path::Path;
    println!("file exists: {}", Path::new(path).exists());
    let mut file = File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn read_graphml(path: &str) -> ResultGraphML {
    const NODE: &str = "node";
    const EDGE: &str = "edge";
    const NODE_TEXT_ATTR_KEY: &str = "d3";
    const EDGE_TEXT_ATTR_KEY: &str = "d10";
    const VERTEX_SOURCE_ATTR_KEY: &str = "source";
    const VERTEX_TARGET_ATTR_KEY: &str = "target";

    let xml_doc = load_file(path);
    let doc = match roxmltree::Document::parse(&xml_doc) {
        Ok(v) => v,
        Err(_) => return Err("Error parse xml document"),
    };

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

    let (vertexes, edges): (Vec<GraphMLNode>, Vec<GraphMLNode>) =
        nodes.into_iter().partition(|x| match x {
            GraphMLNode::Node(_) => true,
            _ => false,
        });

    format_graph(vertexes, edges)
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

type ResultGraphML<'a> = Result<Graph<Vertex, Edge>, &'static str>;

#[derive(Debug, Clone)]
struct Vertex {
    id: String,
    text: String,
}

#[derive(Debug, Clone)]
struct Edge {
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

            let try_get_node_by_id = |id| *vertex_indexes.get(id).expect("got node by id");

            graph.add_edge(
                try_get_node_by_id(&e.source_id),
                try_get_node_by_id(&e.target_id),
                edge,
            );
        }
    }

    Ok(graph)
}
