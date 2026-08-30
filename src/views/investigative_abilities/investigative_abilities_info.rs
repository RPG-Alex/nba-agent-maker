use yew::prelude::*;
use crate::views::investigative_abilities::{
    academic_abilities::*, interpersonal_abilities::*, technical_abilities::*,
};

#[component]
pub fn InvestigativeSkillsInfo() -> Html {
    html! {
        <section id="investigative-skills">
            <AcademicAbilities />
            <InterpersonalAbilities />
            <TechnicalAbilities />
        </section>
    }
}
