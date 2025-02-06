use leptos::prelude::*;

#[component]
pub fn Languages() -> impl IntoView {
    view! {
        <section id="languages">
            <h3>"Languages Spoken"</h3>
            <div>
                <label for="language-1">"Language 1"</label>
                <input type="text" id="language-1"/>
            </div>
        </section>
    }
}
