use codee::{string::JsonSerdeCodec};
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use rules::character::Character;
use serde::{Deserialize, Serialize};
mod rules;
mod views;
use views::head::MyHead;

#[component]
fn Sheet() -> impl IntoView {
    view! { <MyHead/>}
}
fn main() {
    let (state, set_state, _) = use_local_storage::<Character, JsonSerdeCodec>("character-data");
    mount_to_body(Sheet);
}
