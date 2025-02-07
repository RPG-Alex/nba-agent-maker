use crate::rules::character::{Character, CharacterStoreFields};
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn InvestigativeSkillsInfo() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    let total = 3;
    view! {
        <div class="investigative-skills">
            <div class="academic">
                <header>"Academic Abilities"</header>

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
                                        "|_|"
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

                <div class="ability" id="archaeology">
                    <label for="archaeology">"Archaeology"</label>
                    <div class="bubbles">
                        { move || (1..=total).map(|i| {
                            let r_i = total - i + 1;
                            let rating = move || state.get().investigative_abilities.archaeology.rating;
                            let filled = move || r_i <= rating();
                            if r_i == 1 {
                                view! {
                                    <span
                                        class="bubble"
                                        data-value={r_i.to_string()}
                                        on:click=move |_| {
                                            state.update(|char| {
                                                char.investigative_abilities.archaeology.rating = r_i;
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
                                                char.investigative_abilities.archaeology.rating = 0;
                                            });
                                        }
                                    >
                                        "|_|"
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span
                                        class="bubble"
                                        data-value={r_i.to_string()}
                                        on:click=move |_| {
                                            state.update(|char| {
                                                char.investigative_abilities.archaeology.rating = r_i;
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

            <div class="interpersonal">
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
                                        "|_|"
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
                                        "|_|"
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
            <div class="technical">
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
                                        "|_|"
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
                                        "|_|"
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
        </div>
    }
}
