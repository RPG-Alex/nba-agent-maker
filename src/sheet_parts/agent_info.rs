use crate::{
    character::{Character, CharacterStoreFields},
    rules::general_skills::*,
};
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn AgentInfo() -> impl IntoView {
    let (editing, set_editing) = signal(true);
    let state = expect_context::<Store<Character>>();
    let general_skills = gen_skills_list();
    view! {
            <section id="agent-info">
            <div>
                <label for="agent-name">"Agent Name: "</label>
                {
                    move || {
                        if editing.get() {
                            view! {
                                    <input
                                        type="text"
                                        id="agent-name"
                                        prop:value=state.agent_name().get()
                                        on:input:target=move |ev| {
                                            state.agent_name().set(ev.target().value());
                                            }
                                        on:keydown=move |ev| {
                                            if ev.key() == "Enter" {
                                                set_editing.set(false);
                                            }
                                        }
                                    />
                            }
                            .into_any()
                        } else {
                            view! {
                                    <span>{state.agent_name().get()} " "</span>
                                    <button
                                        on:click=move |_| set_editing.set(true)
                                        title="Edit"
                                    >
                                        "✎"
                                    </button>
                            }
                            .into_any()
                        }
                    }
                }
            </div>
            <div>
                <label for="mos">"MOS: "</label>
                {
                    move || {
                            view! {
                                <select
                                    id="mos"
                                    bind:value=state.mos()
                                    on:change=move |ev| {
                                        state.mos().set(event_target_value(&ev));
                                    }
                                >
                                    <option value="">"-- Select MOS --"</option>
                                    {
                                        general_skills.iter().map(|skill| {
                                            view! {
                                                <option value=skill.clone()>{skill.clone()}</option>
                                            }
                                            .into_any()
                                        }).collect::<Vec<_>>()
                                    }
                                </select>
                            }
                            .into_any()
                    }
                }
            </div>
            <div>
                <label for="drive">"Drive"</label>
                <input type="text" id="drive"/>
            </div>
            <div>
                <label for="handler">"Handler"</label>
                <input type="text" id="handler"/>
            </div>
            <div>
                <label for="professional-role">"Professional Role"</label>
                <input type="text" id="professional-role"/>
            </div>
            <div>
                <label for="backgrounds">"Background(s)"</label>
                <input type="text" id="backgrounds"/>
            </div>
            <div>
                <label for="symbol">"Symbol"</label>
                <input type="text" id="symbol"/>
            </div>
            <div>
                <label for="solace">"Solace"</label>
                <input type="text" id="solace"/>
            </div>
            <div>
                <label for="safety">"Safety"</label>
                <input type="text" id="safety"/>
            </div>
            <div>
                <label for="heat-level">"Heat Level"</label>
                <input type="number" id="heat-level"/>
            </div>
            <div>
                <label for="build-points">"Build Points"</label>
                <input type="number" id="build-points"/>
            </div>
        </section>
    }
}
