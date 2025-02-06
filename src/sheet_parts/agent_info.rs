use crate::character::{Character, CharacterStoreFields};
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn AgentInfo() -> impl IntoView {
    let (editing, set_editing) = signal(true);
    let state = expect_context::<Store<Character>>();
    view! {
            <section id="agent-info">
                <div>
                    <label for="agent-name">"Agent Name: "</label>
                    {
                        move || {
                            if editing.get() {
                                view! {
                                        <input
                                            type="text"
                                            id="agent-name"
                                            prop:value=state.agent_name().get()
                                            on:input:target=move |ev| {
                                                state.agent_name().set(ev.target().value());
                                                }
                                            on:keydown=move |ev| {
                                                if ev.key() == "Enter" {
                                                    set_editing.set(false);
                                                }
                                            }
                                        />
                                }
                                .into_any()
                            } else {
                                view! {
                                        <span>{state.agent_name().get()} " "</span>
                                        <button
                                            on:click=move |_| set_editing.set(true)
                                            title="Edit"
                                        >
                                            "✎"
                                        </button>
                                }
                                .into_any()
                            }
                        }
                    }
            </div>
            <div>
                <label for="mos">"MOS"</label>
                <input type="text" id="mos"/>
            </div>
            <div>
                <label for="drive">"Drive"</label>
                <input type="text" id="drive"/>
            </div>
            <div>
                <label for="handler">"Handler"</label>
                <input type="text" id="handler"/>
            </div>
            <div>
                <label for="professional-role">"Professional Role"</label>
                <input type="text" id="professional-role"/>
            </div>
            <div>
                <label for="backgrounds">"Background(s)"</label>
                <input type="text" id="backgrounds"/>
            </div>
            <div>
                <label for="symbol">"Symbol"</label>
                <input type="text" id="symbol"/>
            </div>
            <div>
                <label for="solace">"Solace"</label>
                <input type="text" id="solace"/>
            </div>
            <div>
                <label for="safety">"Safety"</label>
                <input type="text" id="safety"/>
            </div>
            <div>
                <label for="heat-level">"Heat Level"</label>
                <input type="number" id="heat-level"/>
            </div>
            <div>
                <label for="build-points">"Build Points"</label>
                <input type="number" id="build-points"/>
            </div>
        </section>
    }
}

/*
|    |    |    |    |    |
|----|----|----|----|----|
| ✝  | ☥  | ☦  | ⛧  | ☠  |
| ⚰  | 🦇  | 🌙  | ⛤  | ⚜  |
| 🔮  | ☾  | ♏  | 🩸  | ∴  |
| ♆  | 𓂀  | ☀  | ☪  | ✡  |
| 🕯  | 🔻  | 🔺  | 🕷  | 🕸  |
| 🏺  | ⚗  | 🜏  | 🜍  | 🜔  |
| 🜊  | 🜋  | 🜎  | 🜙  | 🜛  |
| ☾🌑☽  | 🌘🌑🌒  | ☽☉☾  | 𖤐  | ⛥  |
| ⛥⛧⛤  | ☩  | ☬  | ✵  | ✶  |
| ⚖  | ⚒  | 🜎⚝🜎  | 🜠  | 🜢  |

### **Edit Button Suggestions**
Here are suggested symbols for the edit button for each field based on their themes:

- **Agent Name:** ✎ *(Pencil for name entry)*
- **MOS:** ⚒ *(Hammer & Pick, representing profession/work)*
- **Drive:** 🔥 *(Fire, symbolizing passion and motivation)*
- **Handler:** 🕵 *(Detective emoji, fitting for espionage/mystery themes)*
- **Professional Role:** ⚔ *(Crossed swords, representing roles in combat/espionage)*
- **Backgrounds:** 🏺 *(Ancient vase, symbolizing history and past experiences)*
- **Symbol:** ⛧ *(Inverted pentagram, emphasizing mysticism & secret societies)*
- **Solace:** ☾ *(Crescent moon, representing comfort, spirituality, or solitude)*
- **Safety:** 🛡 *(Shield, representing protection and security)*
- **Heat Level:** 🔻 *(Downward triangle, indicating increasing danger or tension)*
- **Build Points:** 🩸 *(Blood drop, symbolizing sacrifice or resource allocation in a dark setting)*

This way, each edit button matches the thematic essence of the corresponding field. Let me know if you'd like adjustments! 🦇
*/
