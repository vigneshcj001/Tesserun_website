use super::step::Step;
use leptos::prelude::*;

#[component]
pub fn ReviewDemo(step: RwSignal<Step>) -> impl IntoView {
    let selected = RwSignal::new(0_usize);
    const CHECKS: [(&str, &str, &str); 3] = [
        (
            "Source constraints",
            "Inspect",
            "The desktop app checks allowed file names, duplicate paths, file count, and total source size. These checks do not execute the application.",
        ),
        (
            "Independent AI review",
            "Advisory",
            "A separate Reviewer request examines the generated source. Treat its findings as advice to evaluate, rather than proof that behavior is correct.",
        ),
        (
            "Runtime & behavior",
            "Not run",
            "Tesserun currently does not execute generated applications or run behavior tests. Open and test your application before publishing it.",
        ),
    ];
    view! {
        <section class="demo-view">
            <div class="tour-kicker">"CLARITY BEFORE THE NEXT STEP"</div><h3>"Know what was checked."</h3>
            <p class="tour-description">"Choose a review layer to understand what it tells you."</p>
            <div class="review-layers" role="group" aria-label="Review layers">
                {CHECKS.into_iter().enumerate().map(move |(index, (name, badge, _))| view! {
                    <button aria-pressed=move || (selected.get() == index).to_string() on:click=move |_| selected.set(index)>
                        <span class="review-layer-index">{format!("0{}", index + 1)}</span><strong>{name}</strong><span class="layer-badge">{badge}</span><span>"↗"</span>
                    </button>
                }).collect_view()}
            </div>
            <div class="review-explanation" aria-live="polite"><h4>{move || CHECKS[selected.get()].0}</h4><p>{move || CHECKS[selected.get()].2}</p></div>
            <button class="demo-next" on:click=move |_| { step.set(Step::Deploy); super::focus_panel(); }>"Find your next step "<span>"→"</span></button>
        </section>
    }
}
