use codee::*;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
use rules::character::Character;

mod rules;

#[component]
fn Sheet() -> impl IntoView {
    let (state, set_state, _) = use_local_storage::<Character, JsonSerdeCodec>("character-data");
}

fn main() {
    mount_to_body(Sheet);
}