use crate::rules::character::Character;
use leptos::prelude::*;

#[component]
pub fn StatsView() -> impl IntoView {
    let state = expect_context::<Store<Character>>();

    view! {
        <div class="stats-view">
            <div class="stat-item">
                <label for="health">"Health: "</label>
                <input
                    type="number"
                    id="health"
                    prop:value=move || state.get().health.to_string()
                    on:input:target=move |ev| {
                        let value = ev.target().value();
                        if let Ok(num) = value.parse::<i32>() {
                            state.update(|char| {
                                char.health = num;
                            });
                        }
                    }
                />
            </div>
            <div class="stat-item">
                <label for="stability">"Stability: "</label>
                <input
                    type="number"
                    id="stability"
                    prop:value=move || state.get().stability.to_string()
                    on:input:target=move |ev| {
                        let value = ev.target().value();
                        if let Ok(num) = value.parse::<i32>() {
                            state.update(|char| {
                                char.stability = num;
                            });
                        }
                    }
                />
            </div>
        </div>
    }
}
