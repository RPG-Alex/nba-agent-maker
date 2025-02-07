mod rules;
mod views;

use leptos::prelude::*;
use reactive_stores::Store;
use rules::character::*;
use views::{agent_info::*, general_skills_info::*};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        provide_context(Store::new(Character::default()));
        view! {
            <div id="character-sheet">
                <AgentInfo />
                <GeneralSkillsInfo />
                <AgentState />
            </div>
        }
    })
}

//for debugging the bad way
#[component]
fn AgentState() -> impl IntoView {
    let state = expect_context::<Store<Character>>();
    view! {
        <p>"Here is the agent name:" {move || state.agent_name().get()}</p>
        <p>"Here is the agent MOS:" {move || state.get().get_mos()}</p>
        <p>"Here is the agent Drive:" {move || state.drive().get()}</p>
        <p>"Here is the agent Handler:" {move || state.handler().get()}</p>
        <p>"Here is the agent  Rating in Skill: " {move || state.general_skills().get().network.rating}</p>
    }
}
