use crate::views::investigative_abilities::academic::{
    accounting::*, archeology::*, architecture::*, art_history::*, criminology::*, diagnoses::*,
    history::*, human_terrain::*, languages::*, law::*, military_science::*, occult_studies::*,
    research::*, vampirology::*,
};
use leptos::prelude::*;

#[component]
pub fn AcademicAbilities() -> impl IntoView {
    view! {
        <div id="academic">
            <header>"Academic Abilities"</header>
            <Accounting />
            <Archeology />
        </div>
    }
}
