use super::{scenario::Scenario, step::Step};
use leptos::prelude::*;

#[component]
pub fn DevelopDemo(
    scenario: RwSignal<Scenario>,
    brief: RwSignal<String>,
    started: RwSignal<bool>,
    step: RwSignal<Step>,
) -> impl IntoView {
    let role = RwSignal::new(0_usize);
    let source = RwSignal::new(false);
    view! {
        <section class="demo-view">
            <div class="tour-kicker">"YOUR IDEA. A LITTLE MORE DIRECTION."</div>
            <h3>"What will you create?"</h3>
            <p class="tour-description">"Edit the brief. Explore the roles. See where an idea could go."</p>
            <div class="brief-box interactive-brief">
                <label class="visually-hidden" for="demo-brief">"Your project idea"</label>
                <textarea id="demo-brief" maxlength="600" rows="3" prop:value=move || brief.get()
                    on:input=move |event| { brief.set(event_target_value(&event)); started.set(false); }></textarea>
                <div class="brief-bottom"><span>"Local walkthrough · No AI calls"</span>
                    <button class="mock-button" disabled=move || brief.get().trim().is_empty()
                        on:click=move |_| { started.set(true); role.set(0); source.set(false); }>
                        {move || if started.get() { "Restart preview ↺" } else { "Preview the workflow ↗" }}
                    </button>
                </div>
            </div>
            <Show when=move || started.get()>
                <div class="demo-result">
                    <p class="demo-result-label" role="status">"EXAMPLE WORKFLOW · CHOOSE A ROLE"</p>
                    <div class="agent-switcher" role="group" aria-label="Explore agent roles">
                        {["Planner", "Developer", "Reviewer"].into_iter().enumerate().map(move |(index, name)| view! {
                            <button aria-pressed=move || (role.get() == index).to_string()
                                on:click=move |_| role.set(index)><span class="agent-avatar" aria-hidden="true">{name.chars().next().unwrap_or('A').to_string()}</span>{name}</button>
                        }).collect_view()}
                    </div>
                    <div class="role-output">
                        {move || match role.get() {
                            0 => view! { <div><h4>"Start with a considered plan."</h4><ol>{scenario.get().plan().into_iter().map(|line| view! { <li>{line}</li> }).collect_view()}</ol></div> }.into_any(),
                            1 => view! { <div><h4>"Make the source your own."</h4><p>"The desktop Developer creates editable static source. This tour shows a small hand-authored example for your selected starter."</p><div class="file-chips"><span>"index.html"</span><span>"styles.css"</span><span>"app.js"</span></div><button class="demo-text-button" aria-expanded=move || source.get().to_string() aria-controls="source-example" on:click=move |_| source.update(|value| *value = !*value)>{move || if source.get() { "Hide example ↑" } else { "Peek at example source ↗" }}</button><div id="source-example" hidden=move || !source.get()><pre class="source-example"><code>{move || scenario.get().markup()}</code></pre></div></div> }.into_any(),
                            _ => view! { <div><h4>"Leave room for a second look."</h4><p>"An independent reviewer advises on the generated source. Static checks add another perspective; running and testing the application is still your next step."</p><span class="review-badge">"Runtime behavior: not verified"</span></div> }.into_any(),
                        }}
                    </div>
                    <p class="preview-footnote">"Examples follow the selected starter, not an AI interpretation of your edited brief."</p>
                    <button class="demo-next" on:click=move |_| { step.set(Step::Check); super::focus_panel(); }>"Explore the review "<span>"→"</span></button>
                </div>
            </Show>
            <Show when=move || !started.get()><div class="preview-team"><span class="preview-agent"><span class="agent-avatar">"P"</span>"Plan with purpose"</span><span class="preview-agent"><span class="agent-avatar">"D"</span>"Make it yours"</span><span class="preview-agent"><span class="agent-avatar">"R"</span>"Take a second look"</span></div></Show>
        </section>
    }
}
