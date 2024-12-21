use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <div class="top-nav">
                <h1>"WeChat"</h1>
            </div>
            <div class="main-content">
                <div class="chat-list">
                    <div class="chat-item">
                        <div class="chat-info">
                            <h2>"Contact"</h2>
                            <p>"Last Contact..."</p>
                        </div>
                    </div>
                </div>
                <div class="chat-window">
                    <div class="chat-header">
                        <h2>"Chats"</h2>
                    </div>
                    <div class="chat-messages"></div>
                    <div class="chat-input">
                        <input type="text" placeholder="Type a message..." />
                        <button>"Send"</button>
                    </div>
                </div>
            </div>

        </main>
    }
}
