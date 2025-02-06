use leptos::prelude::*;
mod sheet_parts;
use sheet_parts::agent_info::*;
mod character;
use character::*;
use reactive_stores::Store;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        provide_context(Store::new(Character::default()));
        view! {
            <div id="character-sheet">
                <AgentInfo />
                <AgentState />
            </div>
        }
    })
}

// for debugging the bad way
#[component]
fn AgentState() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    view! {
        <p>"Here is the agenet name:" {move || state.agent_name().get()}</p>
    }
}
