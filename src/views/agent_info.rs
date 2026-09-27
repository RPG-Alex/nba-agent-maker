use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    rules::{character::Character, drives::get_drives},
    views::field_update,
};

#[component]
pub(crate) fn AgentInfo() -> Html {
    let drives = get_drives();
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    let on_name_input = field_update(character.clone(), |character, value: String| {
        character.agent_name = value;
    });

    let on_handler_input = Callback::from(move |event: InputEvent| {});

    let on_role_input = Callback::from(move |event: InputEvent| {});

    let on_background_input = Callback::from(move |event: InputEvent| {});

    let on_investigative_points_input = Callback::from(move |event: InputEvent| {});

    let on_general_points_input = Callback::from(move |event: InputEvent| {});

    let on_agent_info_update = Callback::from(move |event: InputEvent| {});

    let agent_info_editable = use_state(|| false);

    html! {
        <section id="agent-info">
            <div>
                <div>
                    <span>{"General Points Spent: "}{character.general_points_spent}</span>
                </div>
                <div><span>{"Investigative Points Spent: "}{character.investigative_points_spent}</span></div>
            </div>
            <div>
                <label for="agent-name">{"Agent Name: "}</label>
                {
                    if character.editable {
                        html!(
                            <input type="text" id="agent-name" value={character.agent_name.clone()} oninput={on_name_input} />
                        )
                    } else {
                        html!(
                                <span id="agent-name">
                                    {&character.agent_name}
                                </span>
                        )
                    }
                }

            </div>
        </section>
    }
}
