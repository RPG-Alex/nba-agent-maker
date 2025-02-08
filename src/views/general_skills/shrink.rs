use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn Shrink() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="shrink">
            <label for="shrink-mos">"Shrink"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let shrink_value = move || state.get().general_skills.shrink.rating;
                    let filled = move || r_i <= shrink_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.shrink.rating = r_i;
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
                                        char.general_skills.shrink.rating = 0;
                                    });
                                }
                            >" ✘"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.shrink.rating = r_i;
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
                id="shrink-mos"
                prop:checked=move || state.get().general_skills.shrink.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Shrink");
                    });
                }
            />
        </div>
    }
}
