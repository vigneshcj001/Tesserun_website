use super::step::Step;
use leptos::prelude::*;

#[component]
pub fn TourContent(step: RwSignal<Step>) -> impl IntoView {
    move || {
        match step.get() {
        Step::Develop => view! {
            <div class="tour-kicker">"A GOOD IDEA IS A GREAT PLACE TO START"</div>
            <h3>"What will you create?"</h3>
            <p class="tour-description">"A little direction. A team of agents. Something worth building."</p>
            <div class="brief-box">
                <p>"A calm, minimal dashboard for keeping track of my daily habits."<br/>"Make it feel personal, with a little room to grow."</p>
                <div class="brief-bottom"><span>"Your idea, in your words."</span><span class="mock-button">"Run agent team ↗"</span></div>
            </div>
            <div class="preview-team">
                <span class="preview-agent"><span class="agent-avatar">"P"</span>"Planner"</span>
                <span class="preview-agent"><span class="agent-avatar">"D"</span>"Developer"</span>
                <span class="preview-agent"><span class="agent-avatar">"R"</span>"Reviewer"</span>
            </div>
            <p class="preview-footnote">"Your provider. Your model. One considered workflow."</p>
        }.into_any(),
        Step::Check => view! {
            <div class="tour-kicker">"A LITTLE MORE CLARITY BEFORE THE NEXT STEP"</div>
            <h3>"Take a closer look."</h3>
            <p class="tour-description">"Source checks and advisory review, together in one place."</p>
            <div class="check-list">
                <div><span>"Allowed source files"</span><span class="check-ok">"✓ Passed"</span></div>
                <div><span>"Duplicate paths & source limits"</span><span class="check-ok">"✓ Passed"</span></div>
                <div><span>"Independent agent review"</span><span>"Advisory findings"</span></div>
                <div><span>"Runtime & behavior tests"</span><span class="check-pending">"Not run"</span></div>
            </div>
            <p class="preview-footnote">"Example report. Static checks do not verify runtime behavior."</p>
        }.into_any(),
        Step::Deploy => view! {
            <div class="tour-kicker">"SOMETHING YOU MADE. SOMEWHERE TO GO."</div>
            <h3>"Ready for the next step?"</h3>
            <p class="tour-description">"Keep building locally, or share your static website with the world."</p>
            <div class="deploy-options">
                <div><strong>"↓ Export source"</strong><p>"Your files, a review report, and a manifest. All in one local bundle."</p></div>
                <div><strong>"↗ Publish to Vercel"</strong><p>"Connect your account and confirm before publishing a static website."</p></div>
            </div>
            <p class="preview-footnote">"Publishing requires a Vercel token and your explicit confirmation."</p>
        }.into_any(),
    }
    }
}
