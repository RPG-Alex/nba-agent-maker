use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn Preparedness() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="preparedness">
            <label for="preparedness-mos">"Preparedness"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let preparedness_value = move || state.get().general_skills.preparedness.rating;
                    let filled = move || r_i <= preparedness_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.preparedness.rating = r_i;
                                    });
                                }
                            >
                                {if filled() { "●" } else { "○" }}
                            </span>
                            <span
                                class="bubble"
                                data-value={0.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.preparedness.rating = 0;
                                    });
                                }
                            >" ✘"
                            </span>
                        }.into_any()
                    } else if r_i == 8 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.preparedness.rating = r_i;
                                    });
                                }
                            >
                               {if filled() { "⬤" } else { "⭕" }}
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.preparedness.rating = r_i;
                                    });
                                }
                            >
                                {if filled() { "●" } else { "○" }}
                            </span>
                        }.into_any()
                    }
                }).collect::<Vec<_>>()}
            </div>
            <input
                type="checkbox"
                id="preparedness-mos"
                prop:checked=move || state.get().general_skills.preparedness.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Preparedness");
                    });
                }
            />
        </div>
    }
}
