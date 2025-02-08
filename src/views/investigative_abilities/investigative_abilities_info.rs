use crate::views::investigative_abilities::{
    academic_abilities::*, interpersonal_abilities::*, technical_abilities::*,
};
use leptos::prelude::*;

#[component]
pub fn InvestigativeSkillsInfo() -> impl IntoView {
    view! {
        <div class="investigative-skills">
            <AcademicAbilities />
            <InterpersonalAbilities />
            <TechnicalAbilities />
        </div>
    }
}
