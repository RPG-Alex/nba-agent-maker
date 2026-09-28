use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::{Callback, Html, InputEvent, MouseEvent, TargetCast, UseStateHandle, component, html, use_context, use_state};

use crate::rules::character::{self, Character};

pub(crate) mod agent_info;
pub(crate) mod general_skills;
pub(crate) mod investigative_abilities;
pub(crate) mod stats_info;

pub(crate) fn field_update<Field, F>(
    character_state: UseStateHandle<Character>,
    setter: F,
) -> Callback<InputEvent>
where
    F: Fn(&mut Character, Field) + 'static,
    Field: FromStr + 'static,
{
    Callback::from(move |event: InputEvent| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let Ok(new_value) = input.value().parse::<Field>() else {
            return;
        };

        let mut updated = (*character_state).clone();
        setter(&mut updated, new_value);
        character_state.set(updated);
    })
}

#[component]
pub(crate) fn AgentEditable() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    let on_editable_change = {
        let character = character.clone();
        Callback::from(move |_: MouseEvent| {
        let mut updated = (*character).clone();
        updated.editable = !updated.editable;
        character.set(updated);
    })};


    html! {
        <div id="editable">
        {
            if character.editable {
                html!(
                    <button onclick={on_editable_change}>{"Save"}</button>
                )
            } else {
                html!(
                    <button onclick={on_editable_change}>{"Updated Info"}</button>
                )
            }
        }
        </div>
    }
}
