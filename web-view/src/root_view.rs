use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::services::ConsoleService;
use yew::{html, ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};

#[path = "./file_upload_sample.rs"]
pub mod file_upload;

#[path = "./lib.rs"]
pub mod quest_game;

use file_upload::FileModel;
use quest_game::SceneModel;

use parser_graphml::parser::*;

pub struct RootView {
    loaded_graph_file: Option<String>,
}

pub enum RootMsg {
    LoadGraph(String)
}

impl Component for RootView {
    type Message = RootMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        RootView {
            loaded_graph_file: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMsg::LoadGraph(graph_file) => {
                self.loaded_graph_file = Some(graph_file);
            }
        }
        true
    }
}

impl Renderable<RootView> for RootView {
    fn view(&self) -> Html<Self> {
        if let Some(graph_file) = &self.loaded_graph_file {
            match read_graphml(&graph_file) {
                Ok(graph) => {
                    html! {
                        <div class="game-container">
                            <div class="game__menu">
                                <div class="game__load-new-scene">
                                    <FileModel title="Загрузить файл игры" onloaded=|graph_file| RootMsg::LoadGraph(graph_file) />
                                </div>
                            </div>
                            <div class="game__scene">
                                <SceneModel graph=graph />
                            </div>
                        </div>
                    }
                }
                Err(e) => {
                    const MSG: &str = "Ошибка парсинга графа из GraphML формата. Попробуйте заново загрузить файл.";
                    //TODO: писать еще ошибку в лог.
                    html! {
                        <div class="error">{MSG}</div>
                    }
                }
            }
        } else {
            html! {
                <div>
                    <FileModel title="Загрузить файл игры" onloaded=|graph_file| RootMsg::LoadGraph(graph_file) />
                </div>
            }
        }
    }
}
