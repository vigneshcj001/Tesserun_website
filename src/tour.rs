mod content;
mod develop;
mod publish;
mod review;
mod scenario;
mod step;

use content::TourContent;
use leptos::prelude::*;
use scenario::Scenario;
use step::Step;

pub fn focus_panel() {
    use wasm_bindgen::JsCast;
    if let Some(element) = document().get_element_by_id("tour-panel")
        && let Some(element) = element.dyn_ref::<web_sys::HtmlElement>()
    {
        let _ = element.focus();
    }
}

#[component]
pub fn WorkspaceTour() -> impl IntoView {
    let step = RwSignal::new(Step::Develop);
    let scenario = RwSignal::new(Scenario::Habits);
    let brief = RwSignal::new(Scenario::Habits.brief().to_owned());
    let started = RwSignal::new(false);

    view! {
        <div class="preview-stage">
            <div class="stage-grid" aria-hidden="true"></div>
            <div class="demo-toolbar"><div><span class="live-demo-dot"></span><strong>"THE INTERACTIVE STUDIO"</strong><span>"Pick a starting point"</span></div>
                <div class="scenario-picker" role="group" aria-label="Choose a starter idea">
                    {Scenario::ALL.into_iter().map(move |item| view! {
                        <button aria-pressed=move || (scenario.get() == item).to_string() on:click=move |_| {
                            scenario.set(item); brief.set(item.brief().to_owned()); started.set(false); step.set(Step::Develop);
                        }>{item.label()}</button>
                    }).collect_view()}
                </div>
            </div>
            <div class="app-window">
                <div class="window-bar">
                    <span class="window-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                    <span>"Tesserun "<span class="window-bar-muted">"/ a space for your next idea"</span></span>
                    <span class="window-version">"v0.3.0"</span>
                </div>
                <div class="app-layout">
                    <aside class="app-sidebar" aria-label="Studio demo navigation">
                        <div class="preview-brand"><img src="/assets/mark.svg" alt="" width="23" height="23"/>"tesserun."</div>
                        <button class="preview-new" aria-label="Reset studio demo" on:click=move |_| { scenario.set(Scenario::Habits); brief.set(Scenario::Habits.brief().to_owned()); started.set(false); step.set(Step::Develop); }>"+ "<span>"New idea"</span></button>
                        <span class="sidebar-label">"WORKSPACE"</span>
                        {Step::ALL.into_iter().map(move |item| view! {
                            <button class="side-item" class:active=move || step.get() == item aria-label=item.breadcrumb() aria-pressed=move || (step.get() == item).to_string() on:click=move |_| step.set(item)>
                                {item.icon()}<span>{item.breadcrumb()}</span>
                            </button>
                        }).collect_view()}
                        <span class="side-item desktop-only-feature">"◌ "<span>"AI chat · Desktop app"</span></span>
                        <div class="sidebar-bottom"><span class="status-dot"></span>"Native. Local. Yours."</div>
                    </aside>
                    <div class="app-content" id="tour-panel" role="tabpanel"
                        aria-labelledby=move || step.get().tab_id() tabindex="0">
                        <div class="app-breadcrumb">"Workspace "<span>"/"</span>
                            <strong>{move || step.get().breadcrumb()}</strong>
                            <span class="local-badge"><span class="status-dot"></span>"Local workspace"</span>
                        </div>
                        <div class="tour-body" id="tour-content"><TourContent step=step scenario=scenario brief=brief started=started/></div>
                    </div>
                </div>
            </div>
            <span class="stage-label">"LESS FRICTION. MORE FLOW."</span>
        </div>
        <div class="tour-controls">
            <div class="tabs" role="tablist" aria-label="Explore the workspace">
                {Step::ALL.into_iter().map(move |item| view! { <TourTab item=item step=step/> }).collect_view()}
            </div>
            <p>"A hands-on walkthrough "<span>"·"</span>" No model requests"</p>
        </div>
    }
}

#[component]
fn TourTab(item: Step, step: RwSignal<Step>) -> impl IntoView {
    view! {
        <button id=item.tab_id() role="tab" aria-controls="tour-panel"
            aria-label=format!("{} {}", item.number(), item.label())
            aria-selected=move || (step.get() == item).to_string()
            tabindex=move || if step.get() == item { 0 } else { -1 }
            on:click=move |_| step.set(item)
            on:keydown=move |event: web_sys::KeyboardEvent| {
                if let Some(next) = item.for_key(&event.key()) {
                    event.prevent_default();
                    step.set(next);
                    if let Some(element) = document().get_element_by_id(next.tab_id()) {
                        use wasm_bindgen::JsCast;
                        if let Some(button) = element.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = button.focus();
                        }
                    }
                }
            }>
            {item.number()}<span>{item.label()}</span>
        </button>
    }
}
