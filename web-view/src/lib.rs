use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct SceneModel {
    console: ConsoleService,
    description: String,
    choices: Vec<(String, Vec<String>)>,
    current_scene_id: usize,
}

pub enum Msg {
    Choice(usize),
}

impl Component for SceneModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let choices = vec![
            (
                "Scene1 bla-bla".to_string(),
                vec!["choice1".to_string(), "choice1".to_string()],
            ),
            (
                "Scene2 bla-bal".to_string(),
                vec![
                    "choice2".to_string(),
                    "choice2".to_string(),
                    "asdfs".to_string(),
                ],
            ),
        ];
        SceneModel {
            console: ConsoleService::new(),
            description: choices[0].0.to_string(),
            choices: choices,
            current_scene_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Choice(i) => {
                self.description = self.choices[i].0.to_string();
                self.current_scene_id = i;
            }
        }
        true
    }
}

impl Renderable<SceneModel> for SceneModel {
    fn view(&self) -> Html<Self> {
        let view_message = |i: usize| {
            let msg = self.choices[self.current_scene_id].1[i].clone();
            html! {
                <button class="quest-game__scene-choice" onclick=|_| Msg::Choice(i)>
                    { format!("{}.{}", i+1, msg) }
                </button>
            }
        };

        html! {
            <div class="quest-game">
                <div class="quest-game__scene-description">{self.description.clone()}</div>
                <div class="quest-game__scene-choices">
                        { for (0..self.choices[self.current_scene_id].1.len()).map(view_message) }
                </div>
            </div>
        }
    }
}
