use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::TargetCast;
use yew::{Callback, Component, Context, Html, classes, html};

#[derive(Debug, Clone)]
pub struct FileDetails {
    pub name: String,
    #[allow(unused)]
    file_type: String,
    pub data: Vec<u8>,
}

pub enum Msg {
    Loaded(FileDetails),
    Files(Option<web_sys::FileList>),
}

#[derive(yew::Properties, PartialEq)]
pub struct FileUploadProps {
    pub file_selected: Callback<FileDetails>,
}

pub struct FileUpload {
    readers: HashMap<String, FileReader>,
}

impl Component for FileUpload {
    type Message = Msg;
    type Properties = FileUploadProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                ctx.props().file_selected.emit(file);
                true
            }
            Msg::Files(files) => {
                for file in gloo::file::FileList::from(files.expect("files")).iter() {
                    let link = ctx.link().clone();
                    let name = file.name().clone();
                    let file_type = file.raw_mime_type();

                    let task = {
                        gloo::file::callbacks::read_as_bytes(file, move |res| {
                            link.send_message(Msg::Loaded(FileDetails {
                                data: res.expect("failed to read file"),
                                file_type,
                                name,
                            }))
                        })
                    };
                    self.readers.insert(file.name(), task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let noop_drag = Callback::from(|e: DragEvent| {
            e.prevent_default();
        });

        html! {
            <div class={classes!("submission-upload-wrapper")}>
                <label for="file-upload">
                    <div
                        id="drop-container"
                        class={classes!("submission-drop-container")}
                        ondrop={ctx.link().callback(|event: DragEvent| {
                            event.prevent_default();
                            Msg::Files(event.data_transfer().unwrap().files())
                        })}
                        ondragover={&noop_drag}
                        ondragenter={&noop_drag}
                    >
                        <span>{"Drop your Scratch submission here or click to select it"}</span>
                    </div>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="application/x.scratch.sb3"
                    style={"display: none"}
                    multiple={false}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::Files(input.files())
                    })}
                />
            </div>
        }
    }
}
