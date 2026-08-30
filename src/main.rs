mod rules;
mod views;

use yew::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::{rules::character::Character, views::agent_info};
use views::stats_info::StatsView;
const STORAGE_KEY: &str = "character-sheet";

type CharacterContext = UseStateHandle<Character>;

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
        <ContextProvider<CharacterContext> context={character.clone()}>
            <StatsView />
            {" total health: " }{character.health}
        </ContextProvider<CharacterContext>>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
