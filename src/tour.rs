mod content;
mod step;

use content::TourContent;
use leptos::prelude::*;
use step::Step;

#[component]
pub fn WorkspaceTour() -> impl IntoView {
    let step = RwSignal::new(Step::Develop);

    view! {
        <div class="preview-stage">
            <div class="stage-grid" aria-hidden="true"></div>
            <div class="app-window">
                <div class="window-bar">
                    <span class="window-dots" aria-hidden="true"><i></i><i></i><i></i></span>
                    <span>"Tesserun "<span class="window-bar-muted">"/ a space for your next idea"</span></span>
                    <span class="window-version">"v0.3.0"</span>
                </div>
                <div class="app-layout">
                    <aside class="app-sidebar" aria-hidden="true">
                        <div class="preview-brand"><img src="/assets/mark.svg" alt="" width="23" height="23"/>"tesserun."</div>
                        <div class="preview-new">"+ "<span>"New project"</span></div>
                        <span class="sidebar-label">"WORKSPACE"</span>
                        {Step::ALL.into_iter().map(move |item| view! {
                            <span class="side-item" class:active=move || step.get() == item>
                                {item.icon()}<span>{item.breadcrumb()}</span>
                            </span>
                        }).collect_view()}
                        <span class="side-item">"◌ "<span>"AI chat"</span></span>
                        <div class="sidebar-bottom"><span class="status-dot"></span>"Native. Local. Yours."</div>
                    </aside>
                    <div class="app-content" id="tour-panel" role="tabpanel"
                        aria-labelledby=move || step.get().tab_id() tabindex="0">
                        <div class="app-breadcrumb">"Workspace "<span>"/"</span>
                            <strong>{move || step.get().breadcrumb()}</strong>
                            <span class="local-badge"><span class="status-dot"></span>"Local workspace"</span>
                        </div>
                        <div class="tour-body" id="tour-content"><TourContent step=step/></div>
                    </div>
                </div>
            </div>
            <span class="stage-label">"LESS FRICTION. MORE FLOW."</span>
        </div>
        <div class="tour-controls">
            <div class="tabs" role="tablist" aria-label="Explore the workspace">
                {Step::ALL.into_iter().map(move |item| view! { <TourTab item=item step=step/> }).collect_view()}
            </div>
            <p>"Interactive product tour "<span>"·"</span>" Illustrative content"</p>
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
