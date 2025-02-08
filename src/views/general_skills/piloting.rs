use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn Piloting() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="piloting">
            <label for="piloting-mos">"Piloting"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let piloting_value = move || state.get().general_skills.piloting.rating;
                    let filled = move || r_i <= piloting_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.piloting.rating = r_i;
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
                                        char.general_skills.piloting.rating = 0;
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
                                        char.general_skills.piloting.rating = r_i;
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
                id="piloting-mos"
                prop:checked=move || state.get().general_skills.piloting.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Piloting");
                    });
                }
            />
        </div>
    }
}
