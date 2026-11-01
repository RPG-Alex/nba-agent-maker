use yew::prelude::*;

use crate::{
    rules::{
        backgrounds::{self, Background}, character::{self, Character},
    }, views::field_update,
};

#[component]
pub(crate) fn AgentInfo() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    html! {
        <section id="agent-info">
        {
            if character.editable {
                html!(
                    <div>
                        <div>
                            <span>{"General Points Spent: "}{character.general_points_spent}</span>
                        </div>
                        <div>
                            <span>{"Investigative Points Spent: "}{character.investigative_points_spent}</span>
                        </div>
                    </div>
                )
            } else {
                html!()
            }
        }
        <NameField />
        <HandlerField />
        <BackgroundInput />
        </section>
    }
}


#[component]
fn NameField() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    let on_name_input = field_update(character.clone(), |character, value: String| {
        character.agent_name = value;
    });
    html!(
        <div id="name">
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
    )
}

#[component]
fn HandlerField() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    let on_handler_input = field_update(character.clone(), |character, value: String| {
        character.handler = value;
    });
    html!(
        <div id="handler">
                    <label for="handler">{"Handler: "}</label>
                {
                    if character.editable {
                        html!(
                            <input type="text" id="handler" value={character.handler.clone()} oninput={on_handler_input} />
                        )
                    } else {
                        html!(
                            <span id="handler">
                            {&character.handler}
                            </span>
                        )
                    }
                }
        </div>
    )
}

#[component]
fn BackgroundInput() -> Html {
    let character =
        use_context::<UseStateHandle<Character>>().expect("Character context not found");

    let backgrounds = Background::backgrounds();

    // let on_background_input = field_update(character.clone(), |character, value: String| {
    //     character.background = value;
    // });
    html!(
        <div id="background">
                    <label for="background">{"Background: "}</label>
                {
                    if character.editable {
                        html!(
                            <select id="background">
                                {
                                    for backgrounds.iter().map(|background| {
                                        html!(
                                            <option value={background.to_string()}>{background.to_string()}</option>
                                        )
                                    })
                                }
                                <option value="custom">{"Custom"}</option>
                            </select>
                        )
                    } else {
                        html!(
                            <span id="background">
                            
                            </span>
                        )
                    }
                }
        </div>
    )
}