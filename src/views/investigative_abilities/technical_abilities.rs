use crate::rules::character::Character;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn TechnicalAbilities() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    let total = 3;
    view! {
        <div id="technical">
            <header>"Technical Abilities"</header>

            <div class="ability" id="chemistry">
                <label for="chemistry">"Chemistry"</label>
                <div class="bubbles">
                    { move || (1..=total).map(|i| {
                        let r_i = total - i + 1;
                        let rating = move || state.get().investigative_abilities.chemistry.rating;
                        let filled = move || r_i <= rating();
                        if r_i == 1 {
                            view! {
                                <span
                                    class="bubble"
                                    data-value={r_i.to_string()}
                                    on:click=move |_| {
                                        state.update(|char| {
                                            char.investigative_abilities.chemistry.rating = r_i;
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
                                            char.investigative_abilities.chemistry.rating = 0;
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
                                            char.investigative_abilities.chemistry.rating = r_i;
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

            <div class="ability" id="cryptography">
                <label for="cryptography">"Cryptography"</label>
                <div class="bubbles">
                    { move || (1..=total).map(|i| {
                        let r_i = total - i + 1;
                        let rating = move || state.get().investigative_abilities.cryptography.rating;
                        let filled = move || r_i <= rating();
                        if r_i == 1 {
                            view! {
                                <span
                                    class="bubble"
                                    data-value={r_i.to_string()}
                                    on:click=move |_| {
                                        state.update(|char| {
                                            char.investigative_abilities.cryptography.rating = r_i;
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
                                            char.investigative_abilities.cryptography.rating = 0;
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
                                            char.investigative_abilities.cryptography.rating = r_i;
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
        </div>
    }
}
