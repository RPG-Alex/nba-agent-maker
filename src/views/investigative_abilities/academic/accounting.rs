use yew::prelude::*;

#[component]
pub fn Accounting() -> Html {
    let total = 3;
    html! {
    }        
}

// <div class="ability" id="accounting">
//             <label for="accounting">"Accounting"</label>
//             <div class="bubbles">
//                 { move || (1..=total).map(|i| {
//                     let r_i = total - i + 1;
//                     let rating = move || state.get().investigative_abilities.accounting.rating;
//                     let filled = move || r_i <= rating();
//                     if r_i == 1 {
//                         view! {
//                             <span
//                                 class="bubble"
//                                 data-value={r_i.to_string()}
//                                 on:click=move |_| {
//                                     state.update(|char| {
//                                         char.investigative_abilities.accounting.rating = r_i;
//                                     });
//                                 }
//                             >
//                                 { if filled() { "●" } else { "○" } }
//                             </span>
//                             <span
//                                 class="bubble"
//                                 data-value="0"
//                                 on:click=move |_| {
//                                     state.update(|char| {
//                                         char.investigative_abilities.accounting.rating = 0;
//                                     });
//                                 }
//                             >
//                                 " ✘"
//                             </span>
//                         }.into_any()
//                     } else {
//                         view! {
//                             <span
//                                 class="bubble"
//                                 data-value={r_i.to_string()}
//                                 on:click=move |_| {
//                                     state.update(|char| {
//                                         char.investigative_abilities.accounting.rating = r_i;
//                                     });
//                                 }
//                             >
//                                 { if filled() { "●" } else { "○" } }
//                             </span>
//                         }.into_any()
//                     }
//                 }).collect::<Vec<_>>() }
//             </div>
//         </div>
//     }