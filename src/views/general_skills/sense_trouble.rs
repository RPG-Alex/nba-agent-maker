use crate::rules::character::Character;
use leptos::prelude::*;

#[component]
pub fn SenseTrouble() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="sense-trouble">
            <label for="sense-trouble-mos">"Sense Trouble"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let sense_trouble_value = move || state.get().general_skills.sense_trouble.rating;
                    let filled = move || r_i <= sense_trouble_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.sense_trouble.rating = r_i;
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
                                        char.general_skills.sense_trouble.rating = 0;
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
                                        char.general_skills.sense_trouble.rating = r_i;
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
                                        char.general_skills.sense_trouble.rating = r_i;
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
                id="sense-trouble-mos"
                prop:checked=move || state.get().general_skills.sense_trouble.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Sense Trouble");
                    });
                }
            />
        </div>
    }
}
