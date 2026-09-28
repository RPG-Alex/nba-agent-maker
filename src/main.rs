mod rules;
mod views;

use gloo_storage::{LocalStorage, Storage};
use rules::character::Character;
use views::{agent_info::AgentInfo, stats_info::StatsInfo};
use yew::prelude::*;

use self::views::AgentEditable;

const STORAGE_KEY: &str = "character-sheet";

type CharacterContext = UseStateHandle<Character>;

#[component]
fn App() -> Html {
    // load from storage if available, else default
    let character =
        use_state_eq(|| LocalStorage::get::<Character>(STORAGE_KEY).unwrap_or_default());
    // save changes to storage
    {
        let character = (*character).clone();
        use_effect_with(character, move |character| {
            LocalStorage::set(STORAGE_KEY, character).expect("failed to save character");
        });
    }
    html! {
        <div id="character-sheet">
        <ContextProvider<CharacterContext> context={character.clone()}>
            <AgentInfo />
            <StatsInfo />
            <AgentEditable />
        </ContextProvider<CharacterContext>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
