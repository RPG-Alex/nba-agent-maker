use yew::prelude::*;

use crate::views::investigative_abilities::academic::{
    accounting::*, archeology::*, architecture::*, art_history::*, criminology::*, diagnoses::*,
    history::*, human_terrain::*, languages::*, law::*, military_science::*, occult_studies::*,
    research::*, vampirology::*,
};

#[component]
pub fn AcademicAbilities() -> Html {
    html! {
        <div id="academic">
            <header>{"Academic Abilities"}</header>
            <Accounting />
            <Archeology />
        </div>
    }
}
