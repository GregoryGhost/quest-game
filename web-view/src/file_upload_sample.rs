use yew::prelude::{
    html, Callback, ChangeData, Component, ComponentLink, Html, Properties, Renderable,
    ShouldRender,
};
use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::services::ConsoleService;

pub struct FileModel {
    link: ComponentLink<FileModel>,
    reader: ReaderService,
    console: ConsoleService,
    tasks: Vec<ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
    onloaded: Callback<String>,
    title: &'static str,
    error: Option<&'static str>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: &'static str,
    #[props(required)]
    pub onloaded: Callback<String>,
}

type Chunks = bool;

pub enum FileMsg {
    Loaded(FileData),
    Chunk(FileChunk),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
}

impl Component for FileModel {
    type Message = FileMsg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileModel {
            reader: ReaderService::new(),
            link,
            tasks: vec![],
            files: vec![],
            by_chunks: false,
            console: ConsoleService::new(),
            title: props.title,
            onloaded: props.onloaded,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FileMsg::Loaded(file) => match std::str::from_utf8(&file.content) {
                Ok(info) => {
                    let data = info.to_owned();
                    self.files.push(data.clone());
                    self.onloaded.emit(data.clone());
                }
                Err(e) => {
                    const MSG: &str = "Не удалось прочитать выбранный файл";
                    self.error = Some(MSG);
                    self.console.log(&format!("Msg: {}. Panic: {}", MSG, e));
                }
            },
            FileMsg::Chunk(chunk) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
            }
            FileMsg::Files(files, chunks) => {
                for file in files.into_iter() {
                    let task = {
                        if chunks {
                            let callback = self.link.send_back(FileMsg::Chunk);
                            self.reader.read_file_by_chunks(file, callback, 10)
                        } else {
                            let callback = self.link.send_back(FileMsg::Loaded);
                            self.reader.read_file(file, callback)
                        }
                    };
                    self.tasks.push(task);
                }
            }
            FileMsg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
            }
        }
        true
    }
}

impl Renderable<FileModel> for FileModel {
    fn view(&self) -> Html<Self> {
        let flag = self.by_chunks;
        html! {
            //TODO: добавить уведомление провальности загрузки файла.
            <div class="file-upload__container">
                <div>
                    <input class="file-upload" type="file" onchange=|value| {
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                result.extend(files);
                            }
                            FileMsg::Files(result, flag)
                        } style="display:none" id="file_input"/>
                    <label class="file-upload__label" for="file_input">{ self.title }</label>
                    <div class="file-upload__error">{ if let Some(error) = self.error {error} else { "" } }</div>
                </div>
                <div class="file-upload__options">
                    <label class="file-upload__by-chunks_label">{ "By chunks" }</label>
                    <input class="file-upload__by-chunks-checkbox" type="checkbox" checked=flag onclick=|_| FileMsg::ToggleByChunks />
                </div>
                // NOTE: нужно только для отладки
                // TODO: нужно перенести в лог при отладке
                // <ul>
                //     { for self.files.iter().map(|f| self.view_file(f)) }
                // </ul>
            </div>
        }
    }
}

impl FileModel {
    fn view_file(&self, data: &str) -> Html<Self> {
        html! {
            <li>{ data }</li>
        }
    }
}
