use crate::rules::character::Character;
use leptos::prelude::*;

#[component]
pub fn HandToHand() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="hand-to-hand">
            <label for="hand-to-hand-mos">"Hand-to-Hand"</label>
            <div class="bubbles">
                {move || (1..=20).map(|i| {
                    let total = 20;
                    let r_i = total - i + 1;
                    let hand_to_hand_value = move || state.get().general_skills.hand_to_hand.rating;
                    let filled = move || r_i <= hand_to_hand_value();

                    if r_i == 1 {
                        view! {
                            <span
                                class="bubble"
                                data-value={r_i.to_string()}
                                on:click=move |_| {
                                    state.update(|char| {
                                        char.general_skills.hand_to_hand.rating = r_i;
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
                                        char.general_skills.hand_to_hand.rating = 0;
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
                                        char.general_skills.hand_to_hand.rating = r_i;
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
                                        char.general_skills.hand_to_hand.rating = r_i;
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
                id="hand-to-hand-mos"
                prop:checked=move || state.get().general_skills.hand_to_hand.is_mos
                on:change=move |_| {
                    state.update(|char| {
                        char.general_skills.set_mos("Hand-to-Hand");
                    });
                }
            />
        </div>
    }
}
