use crate::views::general_skills::{
    athletics::*, conceal::*, cover::*, digital_intrusion::*, disguise::*, driving::*,
    explosive_devices::*, filch::*, gambling::*, hand_to_hand::*, infiltration::*, mechanics::*,
    medic::*, network::*, piloting::*, preparedness::*, sense_trouble::*, shooting::*, shrink::*,
    surveillance::*, weapons::*,
};

use leptos::prelude::*;
#[component]
pub fn GeneralSkillsInfo() -> impl IntoView {
    view! {
        <section id="general-skills">
            <Athletics />
            <Conceal />
            <Cover />
            <DigitalIntrusion />
            <Disguise />
            <Driving />
            <ExplosiveDevices />
            <Filch />
            <Gambling />
            <HandToHand />
            <Infiltration />
            <Mechanics />
            <Medic />
            <Network />
            <Piloting />
            <Preparedness />
            <SenseTrouble />
            <Shooting />
            <Shrink />
            <Surveillance />
            <Weapons />
        </section>
    }
}
