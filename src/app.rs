use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use dominator::DomBuilder;
pub use dominator::{clone, events, html, svg, with_node, Dom};
pub use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use web_sys::HtmlInputElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, name: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke)]
    async fn invoke2(cmd: &str);
}

#[derive(Clone, Serialize, Deserialize)]
struct Args {
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct App {
    greet: Mutable<Option<String>>,
    input: Mutable<String>
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            greet: Mutable::new(None),
            input: Mutable::new(String::new())
        })
    }
    
    pub fn render(app: Arc<Self>) -> Dom {
        let state = app.clone();
        // let new_msg =
        //     invoke("greet", to_value(&GreetArgs { name: &name.get() }).unwrap()).await;
        html!("div", {
            .child(html!("div", {
                .class(["body"])
                .child(html!("div", {
                    .text("Welcome to the Tauri + Dominator starter template.")
                }))
            }))
            .child(html!("div", {
                .children(&mut [
                    html!("input" => HtmlInputElement, {
                        .prop_signal("value", state.input.signal_cloned())
    
                        .with_node!(element => {
                            .event(clone!(app => move |_: events::Input| {
                                log::info!("input: {}", element.value());
                                app.input.set(element.value());
                            }))
                        })
                    }),
    
                    html!("button", {
                        .text("Greet")
                        .event(clone!(app => move |_: events::Click| {
                            let app = app.clone();
                            let input = app.input.lock_ref();
    
                            if *input == "" {
                                app.clone().greet.set(None);
    
                            } else {

                                let input = input.to_string();
                                let args = Args {name: input.clone()};
                                let app = app.clone();
                                spawn_local(async move {
                                    let name = invoke("greet", to_value(&args).unwrap()).await;
                                    app.greet.set(name.as_string());
                                });
                            }
                        }))
                    }),
    
                    html!("div", {
                        .text_signal(app.greet.signal_ref(|greet| format!("{}", greet.clone().unwrap_or_default())))
                    }),
                    
                    html!("button", {
                        .text("Greet2")
                        .event(clone!(app => move |_: events::Click| {
                            let app = app.clone();
                            let input = app.input.lock_ref();
    
                            if *input == "" {
                                app.clone().greet.set(None);
    
                            } else {

                                let input = input.to_string();
                                let args = Args {name: input.clone()};
                                let app = app.clone();
                                spawn_local(async move {
                                    invoke2("greet2").await;
                                    // app.greet.set(name.as_string());
                                });
                            }
                        }))
                    }),
    
                    html!("div", {
                        .text_signal(app.greet.signal_ref(|greet| format!("{}", greet.clone().unwrap_or_default())))
                    }),
                    
                ])
            }))
        })
    }
}
