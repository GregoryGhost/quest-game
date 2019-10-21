use log::{debug, error, trace};
use logger::init_logger;

#[path = "logger.rs"]
pub mod logger;

extern crate parser_graphml;
use parser_graphml::parser::{read_graphml, Vertex, Edge};

use petgraph::graph::EdgeReference;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::io::stdin;

use std::fs::File;
use std::io::Read;


fn load_file(path: &str) -> String {
    use std::path::Path;
    println!("file exists: {}", Path::new(path).exists());
    let mut file = File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn main() {
    //TODO: нужно залогировать важные события в игре
    init_logger();

    const PATH: &str = "scenes-choices.graphml";

    let xml_doc = load_file(PATH);

    let graph = read_graphml(&xml_doc).expect("failed read graphml"); //TODO: здесь может быть можно переделать на mdo!

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
