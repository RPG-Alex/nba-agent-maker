use crate::rules::character::Character;
use leptos::prelude::*;

#[component]
pub fn InterpersonalAbilities() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    let total = 3;
    view! {
        <div id="interpersonal">
        <header>"Interpersonal Abilities"</header>

        <div class="ability" id="bs-detector">
            <label for="bs-detector">"Bullshit Detector"</label>
            <div class="bubbles">
                { move || (1..=total).map(|i| {
                    let r_i = total - i + 1;
                    let rating = move || state.get().investigative_abilities.bullshit_detector.rating;
                    let filled = move || r_i <= rating();
                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.investigative_abilities.bullshit_detector.rating = r_i;
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
                                        char.investigative_abilities.bullshit_detector.rating = 0;
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
                                        char.investigative_abilities.bullshit_detector.rating = r_i;
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


        <div class="ability" id="negotiation">
            <label for="negotiation">"Negotiation"</label>
            <div class="bubbles">
                { move || (1..=total).map(|i| {
                    let r_i = total - i + 1;
                    let rating = move || state.get().investigative_abilities.negotiation.rating;
                    let filled = move || r_i <= rating();
                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.investigative_abilities.negotiation.rating = r_i;
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
                                        char.investigative_abilities.negotiation.rating = 0;
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
                                        char.investigative_abilities.negotiation.rating = r_i;
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
