use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::services::ConsoleService;
use yew::{html, ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};

#[path = "./file_upload_sample.rs"]
pub mod file_upload;

#[path = "./lib.rs"]
pub mod quest_game;

use file_upload::FileModel;
use quest_game::SceneModel;

pub struct RootView {
    loaded_graph_file: Option<String>,
}

pub enum RootMsg {
    LoadGraph(String),
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
            //TODO: здесь также должна быть парсинг и обработка ошибок парсинга графа
            //  при этом нужно уведомить пользователя о том, что файл не был распарсен и вы вести в лог подробную ошибку.
            //TODO: компонент игровой сцены должен принимать распарсенный граф и уже с ним работать.
            html! {
                <div>
                    <SceneModel graph=graph_file />
                </div>
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
