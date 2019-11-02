use parser_graphml::parser::*;
use petgraph::graph::EdgeReference;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

pub struct SceneModel {
    console: ConsoleService,
    description: String,
    current_scene_id: NodeIndex,
    graph: Graph<Vertex, Edge>,
}

pub enum QuestMsg {
    Choice(usize),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub graph: Graph<Vertex, Edge>,
}

impl SceneModel {
    fn new(console: ConsoleService, graph: Graph<Vertex, Edge>) -> SceneModel {
        let first_scene: NodeIndex = match graph.node_indices().take(1).next() {
            Some(vertex) => vertex,
            None => {
                const MSG: &str =
                    "Не удалось получить первую вершину в графе";
                panic!(MSG);
            }
        };

        SceneModel {
            console: console,
            graph: graph.clone(),
            description: graph[first_scene].text.clone(),
            current_scene_id: first_scene,
        }
    }

    fn get_choices(&self) -> Vec<EdgeReference<'_, Edge>> {
        self.graph
            .edges_directed(self.current_scene_id, Direction::Outgoing)
            .collect()
    }
}

impl Component for SceneModel {
    type Message = QuestMsg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SceneModel::new(ConsoleService::new(), props.graph.clone())
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            QuestMsg::Choice(number) => {
                if let Some(found_vertex_ix) = self.get_choices().get(number - 1) {
                    self.current_scene_id = found_vertex_ix.target();
                    self.description = self.graph[self.current_scene_id].text.clone();
                } else {
                    println!("Не получилось получить вариант");
                }
            }
        }
        true
    }
}

impl Renderable<SceneModel> for SceneModel {
    fn view(&self) -> Html<Self> {
        let choices = self.get_choices();

        let view_message = |i: usize| {
            let msg = choices[i].weight().text.clone();
            html! {
                <button class="quest-game__scene-choice" onclick=|_| QuestMsg::Choice(i)>
                    { format!("{}.{}", i+1, msg) }
                </button>
            }
        };

        html! {
            <div class="quest-game">
                <div class="quest-game__scene-description">{self.description.clone()}</div>
                <div class="quest-game__scene-choices">
                        { for (0..choices.len()).map(view_message) }
                </div>
            </div>
        }
    }
}
