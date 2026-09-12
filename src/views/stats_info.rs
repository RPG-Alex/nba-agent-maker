use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::rules::character::Character;

#[component]
pub(crate) fn StatsInfo() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    // message for inputs
    let health_message = use_state(String::new);
    let stability_message = use_state(String::new);

    let on_health_input = {
        let health_message = health_message.clone();
        let character = character.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();

            if let Ok(health) = input.value().parse::<i8>() {
                let mut updated = (*character).clone();
                updated.health = health;
                character.set(updated);
                health_message.set(String::new());
            } else {
                input.set_value(&character.health.to_string());

                health_message.set(
                    "What's the point of a health stat if you're just going to cheese it?"
                        .to_string(),
                );
            }
        })
    };

    let on_stability_input = {
        let stability_message = stability_message.clone();
        let character = character.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();

            if let Ok(stability) = input.value().parse::<i8>() {
                let mut updated = (*character).clone();
                updated.stability = stability;
                character.set(updated);

                stability_message.set(String::new());
            } else {
                input.set_value(&character.stability.to_string());

                stability_message.set("Is stability a joke to you?!".to_string());
            }
        })
    };

    html! {
        <div class="stats-view">
            <div class="stat-item">
                <label for="health">{"Health: "}</label>
                <input
                    type="number"
                    id="health"
                    value={character.health.to_string()}
                    oninput={on_health_input}
                />
                <span>{(*health_message).clone()}</span>
            </div>

            <div class="stat-item">
                <label for="stability">{"Stability: "}</label>
                <input
                    type="number"
                    id="stability"
                    value={character.stability.to_string()}
                    oninput={on_stability_input}
                />
                <span>{(*stability_message).clone()}</span>
            </div>
        </div>
    }
}
