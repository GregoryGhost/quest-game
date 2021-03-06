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
    fisrt_scene_id: NodeIndex,
}

pub enum QuestMsg {
    Choice(usize),
    ReloadToFirstScene,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub graph: Graph<Vertex, Edge>,
}

impl SceneModel {
    fn new(mut console: ConsoleService, graph: Graph<Vertex, Edge>) -> SceneModel {
        let (desc, first_scene_id) = SceneModel::init_scene_model(&graph, &mut console);

        SceneModel {
            console: console,
            graph: graph,
            description: desc,
            current_scene_id: first_scene_id,
            fisrt_scene_id: first_scene_id,
        }
    }

    fn init_scene_model(
        graph: &Graph<Vertex, Edge>,
        console: &mut ConsoleService,
    ) -> (String, NodeIndex) {
        let first_scene_id: NodeIndex = match graph.node_indices().take(1).next() {
            Some(vertex) => vertex,
            None => {
                const MSG: &str = "Не удалось получить первую вершину в графе";
                console.log(MSG);
                panic!(MSG);
            }
        };

        let desc = SceneModel::get_scene_desc(graph, first_scene_id);

        (desc, first_scene_id)
    }

    fn get_choices(&self) -> Vec<EdgeReference<'_, Edge>> {
        self.graph
            .edges_directed(self.current_scene_id, Direction::Outgoing)
            .collect()
    }

    fn get_scene_desc(graph: &Graph<Vertex, Edge>, scene_id: NodeIndex) -> String {
        graph[scene_id].text.clone()
    }

    fn get_scene_description(&self, scene_id: NodeIndex) -> String {
        SceneModel::get_scene_desc(&self.graph, scene_id)
    }
}

impl Component for SceneModel {
    type Message = QuestMsg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SceneModel::new(ConsoleService::new(), props.graph.clone())
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.graph = _props.graph;
        let (desc, first_scene_id) = SceneModel::init_scene_model(&self.graph, &mut self.console);
        self.description = desc;
        self.fisrt_scene_id = first_scene_id;
        self.current_scene_id = self.fisrt_scene_id;

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            QuestMsg::Choice(number) => {
                if let Some(found_vertex_ix) = self.get_choices().get(number) {
                    self.current_scene_id = found_vertex_ix.target();
                    self.description = self.get_scene_description(self.current_scene_id);
                } else {
                    let msg = &format!("Не удалось получить вариант по номеру: {:?}", number);
                    self.console.log(msg);
                    panic!(msg.clone());
                }
            }
            QuestMsg::ReloadToFirstScene => {
                self.current_scene_id = self.fisrt_scene_id;
                self.description = self.get_scene_description(self.current_scene_id);
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
                <button class="quest-game__scene-choice btn" augmented-ui="tl-clip br-clip exe" onclick=|_| QuestMsg::Choice(i)>
                    { format!("{}.{}", i+1, msg) }
                </button>
            }
        };

        html! {
            <div class="quest-game">
                <div class="quest-game__menu">
                    <button class="quest-game__reload-game" onclick=|_| QuestMsg::ReloadToFirstScene > { "Начать сначала" } </button>
                </div>
                <div class="quest-game__scene">
                    <div class="quest-game__scene-description" augmented-ui="tl-clip t-clip tr-clip r-clip br-clip b-clip bl-clip l-clip exe">{self.description.clone()}</div>
                    <div class="quest-game__scene-choices">
                            { for (0..choices.len()).map(view_message) }
                    </div>
                </div>
            </div>
        }
    }
}
