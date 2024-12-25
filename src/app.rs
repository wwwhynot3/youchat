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
            <h1 class="flex justify-center items-center font-bold underline text3-3xl">
                "Wait! Wait! Wait!"
            </h1>
            <LeptosDemo />
        </main>
    }
}

#[component]
fn LeptosDemo() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::from("What's up buddy?"));

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }
            #[derive(Serialize, Deserialize, Clone)]
            struct TestStore {
                pub key: String,
                pub value: String,
            }
            let args = TestStore {
                key: "test".to_string(),
                value: "test".to_string(),
            };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            set_greet_msg.set("Loading...".to_string());
            // let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = serde_wasm_bindgen::from_value::<Result<(), String>>(
                invoke("write_to_store", args.clone()).await,
            )
            .unwrap();
            set_greet_msg.set("Saving...".to_string());
            let res = serde_wasm_bindgen::from_value::<Result<Option<String>, String>>(
                invoke("read_from_store", args).await,
            )
            .unwrap();
            set_greet_msg.set("Reading...".to_string());
            set_greet_msg.set(res.unwrap().unwrap());
        });
    };
    view! {
        <h1>"Welcome to Tauri + Leptos"</h1>
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://docs.rs/leptos/" target="_blank">
                <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo" />
            </a>
        </div>
        <p>"Click on the Tauri and Leptos logos to learn more."</p>

        <form class="row" on:submit=greet>
            <input id="greet-input" placeholder="Enter a name..." on:input=update_name />
            <button type="submit">"Greet"</button>
        </form>
        <p>{move || greet_msg.get()}</p>
    }
}
