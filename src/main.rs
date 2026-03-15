use codee::{string::JsonSerdeCodec, *};
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
use rules::character::Character;
mod rules;
mod views;
use views::head::MyHead;

#[component]
fn Sheet() -> impl IntoView {
    let (state, set_state, _) = use_local_storage::<Character, JsonSerdeCodec>("character-data");
    let site_name = "Rusted NBA: A Rust Based Nights Black Agents' Character Creator!";
    view! { <MyHead/>}
}
fn main() {
    mount_to_body(Sheet);
}