mod rules;
// mod views;

use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::rules::character::Character;

const STORAGE_KEY: &str = "character-sheet";


#[component]
fn App() -> Html {
    // load from storage if available, else default
    let character = use_state_eq(|| {
        LocalStorage::get::<Character>(STORAGE_KEY).unwrap_or_default()
    });
    // save changes to storage
    {
        let character = (*character).clone();
        use_effect_with(character, move |character| {
            LocalStorage::set(STORAGE_KEY, character).expect("failed to save character");
        });
    }
    html! {
        {character.agent_name.clone()}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
