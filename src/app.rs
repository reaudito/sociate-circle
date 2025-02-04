use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use crate::router::RouterApp;
use web_sys::{window, HtmlElement};


#[component]
pub fn App() -> impl IntoView {
    view! { <RouterApp /> }
}
