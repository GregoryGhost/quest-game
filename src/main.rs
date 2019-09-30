use mdo::option::bind;
use petgraph::graph::EdgeReference;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use roxmltree::Node;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate mdo;

fn main() {
    const PATH: &str = "саси нло))) .graphml";

    let graph = read_graphml(PATH).expect("failed read graphml");

    start_game(&graph);
}

fn start_game(graph: &UnGraph<Vertex, Edge>) {
    const EXIT_CODE: u8 = 0;

    let mut input = String::new();
    let mut number: u8;

    let mut vertex_ix: NodeIndex;
    vertex_ix = graph.node_indices().take(1).next().unwrap();

    let mut out_edges: Vec<EdgeReference<'_, Edge>>;

    loop {
        input.clear();
        println!("Сцена: {}", graph[vertex_ix].text);
        out_edges = graph
            .edges_directed(vertex_ix, Direction::Outgoing)
            .filter(|x| x.weight().source.id == graph[vertex_ix].id)
            .collect();
        println!("Выберите действие: ");
        let mut i = 1;
        for edge in &out_edges {
            println!("{}. {}", i, edge.weight().target.text.clone()); //TODO: тут должен быть edge.weight().text
            i += 1;
        }
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                number = match input.trim_end().parse::<u8>() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("Введен некорректный номер.");
                        continue;
                    }
                };

                if number == EXIT_CODE {
                    break;
                }

                if 1 <= number && number <= i {
                    println!("Получен верный номер варианта");
                    println!("out edges count: {}", out_edges.len());
                    println!("out edges: {:?}", out_edges);
                    if let Some(found_vertex_ix) = out_edges.get((i - 1) as usize) {
                        //TODO: не получается получить NodeIx
                        //  потому, что все сформированные ребра содержат NodeIndex(0).
                        vertex_ix = found_vertex_ix.target();
                    } else {
                        vertex_ix = out_edges.last().unwrap().target();
                        println!("Не получилось получить вариант");
                    }
                    println!("vertex_ix {:?}", graph[vertex_ix]);
                } else {
                    println!(
                        "Введен неверный номер варианта, попробуйте снова."
                    );
                }
            }
            Err(e) => {
                println!("error {:?}", e);
                continue;
            }
        }
    }
    //0. Вывести текст сцены и варианты ответа для данной сцены (текст ребер);
    //1. Считать номер варианта ответа;
    //2. Найти номер варианта ответа в графе;
    //2.1 Получить по найденному варианту следующую сцену;
    //2.2 повторить с шага 0.
    //3. Если номер варианта ответа не считался, то попросить ввести правильный ответ пользователя.
}

fn load_file(path: &str) -> String {
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

fn find_node_attr_by_key(node: &Node<'_, '_>, attr_key: &str) -> Option<String> {
    node.attributes()
        .iter()
        .find(|a| a.name().contains(attr_key))
        .and_then(|x| Some(x.value().into()))
}

fn get_node_by_id<'a>(nodes: &'a [GraphMLNode], search_node_id: &str) -> Option<&'a Vertex> {
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
