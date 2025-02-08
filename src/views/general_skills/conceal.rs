use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn Conceal() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="conceal">
            <label for="conceal-mos">"Conceal"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let conceal_value = move || state.get().general_skills.conceal.rating;
                    let filled = move || r_i <= conceal_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.conceal.rating = r_i;
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
                                        char.general_skills.conceal.rating = 0;
                                    });
                                }
                            >"|_|"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.conceal.rating = r_i;
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
                id="conceal-mos"
                prop:checked=move || state.get().general_skills.conceal.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Conceal");
                    });
                }
            />
        </div>
    }
}
