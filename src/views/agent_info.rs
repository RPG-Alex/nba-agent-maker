use crate::rules::{
    character::Character,
    drives::get_drives,
};
use leptos::prelude::*;

#[component]
pub fn AgentInfo() -> impl IntoView {
    // agent field togglers
    let (name, set_name) = signal(true);
    let (handler, set_handler) = signal(true);
    let (role, set_role) = signal(true);
    let (background, set_background) = signal(String::new());

    let state = expect_context::<Store<Character>>();
    let drive = get_drives();

    let investigative_points = RwSignal::new(0);
    //let general_points = RwSignal::new(state.general_points().get());

    view! {
            <section id="agent-info">
            // <div>
            //     <div><span> "General Points Spent: " {move || {state.general_points().get() - general_points.get()}} </span></div>
            //     <div><span> "Investigative Points Spent: " {move || {investigative_points.get()}} </span></div>
            // </div>
            <div>
                <label for="agent-name">"Agent Name: "</label>
                {
                    move || {
                        if name.get() {
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
                                                set_name.set(false);
                                            }
                                        }
                                    />
                            }
                            .into_any()
                        } else {
                            view! {
                                    <span>{state.agent_name().get()} " "</span>
                                    <button
                                        on:click=move |_| set_name.set(true)
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
                <label for="drive">"Drive: "</label>
                {
                    move || {
                        view! {
                            <select
                                id="drive"
                                bind:value=state.drive()
                                on:change=move |ev| {
                                    state.drive().set(event_target_value(&ev));
                                }
                            >
                                <option value="">"-- Select Drive --"</option>
                                {
                                    drive.iter().map(|&d| {
                                        view! {
                                            <option value=d>{d}</option>
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
                <label for="handler">"Handler: "</label>
                {
                    move || {
                        if handler.get() {
                            view! {
                                <input
                                    type="text"
                                    id="handler"
                                    prop:value=state.handler().get()
                                    on:input:target=move |ev| {
                                        state.handler().set(ev.target().value());
                                    }
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            set_handler.set(false);
                                        }
                                    }
                                />
                            }
                            .into_any()
                        } else {
                            view! {
                                <span>{state.handler().get()} " "</span>
                                <button
                                    on:click=move |_| set_handler.set(true)
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
                <label for="professional-role">"Professional Role: "</label>
                {
                    move || {
                        if role.get() {
                            view! {
                                <input
                                    type="text"
                                    id="professional-role"
                                    prop:value=state.professional_role().get()
                                    on:input:target=move |ev| {
                                        state.professional_role().set(ev.target().value());
                                    }
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            set_role.set(false);
                                        }
                                    }
                                />
                            }
                            .into_any()
                        } else {
                            view! {
                                <span>{state.professional_role().get()} " "</span>
                                <button
                                    on:click=move |_| set_role.set(true)
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
                <label for="backgrounds">"Background(s): "</label>
                {
                    move || {
                        view! {
                            <input
                                type="text"
                                id="backgrounds"
                                prop:value=background.get()
                                on:input:target=move |ev| {
                                    set_background.set(ev.target().value());
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        let bg = background.get();
                                        if !bg.is_empty() {
                                            investigative_points.set(investigative_points.get()+ 18);
                                            state.backgrounds().update(|bgs| bgs.push(bg.clone()));
                                            set_background.set("".to_string())
                                        }
                                    }
                                }
                            />
                        }
                        .into_any()
                    }
                }
                    {move || {
                        state.backgrounds().get().iter().map(|bg| {
                            let bg_clone = bg.clone();
                            view! {
                                <span>{bg_clone.clone()} " "</span>
                                <button
                                    on:click=move |_| {
                                        investigative_points.set(investigative_points.get()- 18);
                                        state.backgrounds().update(|bgs| bgs.retain(|b| b != &bg_clone));
                                    }
                                    title="Remove"
                                >
                                    "✖"
                                </button>
                            }
                            .into_any()
                        }).collect::<Vec<_>>()
                    }}
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
