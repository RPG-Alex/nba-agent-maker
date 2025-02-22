use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn Accounting() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    let total = 3;
    view! {
        <div class="ability" id="accounting">
            <label for="accounting">"Accounting"</label>
            <div class="bubbles">
                { move || (1..=total).map(|i| {
                    let r_i = total - i + 1;
                    let rating = move || state.get().investigative_abilities.accounting.rating;
                    let filled = move || r_i <= rating();
                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.investigative_abilities.accounting.rating = r_i;
                                    });
                                }
                            >
                                { if filled() { "●" } else { "○" } }
                            </span>
                            <span
                                class="bubble"
                                data-value="0"
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.investigative_abilities.accounting.rating = 0;
                                    });
                                }
                            >
                                " ✘"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.investigative_abilities.accounting.rating = r_i;
                                    });
                                }
                            >
                                { if filled() { "●" } else { "○" } }
                            </span>
                        }.into_any()
                    }
                }).collect::<Vec<_>>() }
            </div>
        </div>
    }
}
