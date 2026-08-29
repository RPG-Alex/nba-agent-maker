mod rules;
// mod views;

use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::rules::character::Character;

const STORAGE_KEY: &str = "character-sheet";


#[component]
fn App() -> Html {
    let character = use_state_eq(|| {
        LocalStorage::get::<Character>(STORAGE_KEY).unwrap_or_default()
    });
    html! {
        {character.agent_name.clone()}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
