use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ImprovePostArgs {
    content: String,
}

#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <svg
            class="animate-spin h-5 w-5 text-blue-500 mx-auto"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
        >
            <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
            ></circle>
            <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
            ></path>
        </svg>
    }
}


#[component]
pub fn CreatePost() -> impl IntoView {
    let (post, set_post) = signal(String::new());
    let (improved_post, set_improved_post) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    let generate_improved_post = move |_| {
        let post_content = post.get().to_string();
        set_is_loading.set(true);
        spawn_local(async move {
            if !post_content.is_empty() {
                let args = serde_wasm_bindgen::to_value(&ImprovePostArgs { content: post_content.clone() }).unwrap();
                let res = invoke("improve_post", args).await;

                if let Some(new_post) = res.as_string() {
                    set_improved_post.set(new_post);
                }
            }
            set_is_loading.set(false);
        });
    };

    let clear_improved_text = move |_| {
        set_improved_post.set(String::new());
        set_post.set(String::new());
    };

    view! {
        <div class="p-4">
        <textarea
            class="w-full h-64 p-2 border rounded dark:bg-gray-800"
            placeholder="Write your post here..."
            prop:value=post
            on:input=move |e| set_post.set(event_target_value(&e))
        />

        <div class="flex space-x-4 mt-2">
        <button
            class="px-4 py-2 bg-blue-500 text-white rounded"
            on:click=generate_improved_post
            disabled=move || is_loading.get()
        >
            {move || if is_loading.get() { "Loading..." } else { "Ask AI" }}
        </button>

        <button
            class="px-4 py-2 bg-blue-500 text-white rounded"
            on:click=clear_improved_text
            
        >
         "Clear"
        </button>
        </div>
        <div
            class="mt-4 p-4 border rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
            on:click=move |_| set_post.set(improved_post.get())
        >
        {move || if is_loading.get() { view! {<LoadingSpinner /> }.into_any()} else { let html_content = improved_post
            .get()
            .replace("\n", "<br>")
            .replace("<think>", r#"<think><span class="italic text-sm">"#)
            .replace("</think>", "</span></think>");   view! { <div inner_html={html_content}></div> }.into_any()}}
        </div>
    </div>
    }
}
