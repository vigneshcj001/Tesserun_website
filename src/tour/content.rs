use super::{
    develop::DevelopDemo, publish::PublishDemo, review::ReviewDemo, scenario::Scenario, step::Step,
};
use leptos::prelude::*;

#[component]
pub fn TourContent(
    step: RwSignal<Step>,
    scenario: RwSignal<Scenario>,
    brief: RwSignal<String>,
    started: RwSignal<bool>,
) -> impl IntoView {
    move || match step.get() {
        Step::Develop => {
            view! { <DevelopDemo scenario=scenario brief=brief started=started step=step/> }
                .into_any()
        }
        Step::Check => view! { <ReviewDemo step=step/> }.into_any(),
        Step::Deploy => view! { <PublishDemo/> }.into_any(),
    }
}
