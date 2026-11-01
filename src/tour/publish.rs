use leptos::prelude::*;

#[component]
pub fn PublishDemo() -> impl IntoView {
    let cloud = RwSignal::new(false);
    view! {
        <section class="demo-view">
            <div class="tour-kicker">"YOUR WORK. YOUR NEXT MOVE."</div><h3>"Where will it go?"</h3>
            <p class="tour-description">"Keep exploring locally, or publish a static website."</p>
            <div class="publish-switch" role="group" aria-label="Explore release options">
                <button aria-pressed=move || (!cloud.get()).to_string() on:click=move |_| cloud.set(false)>"↓ Export locally"</button>
                <button aria-pressed=move || cloud.get().to_string() on:click=move |_| cloud.set(true)>"↗ Publish a website"</button>
            </div>
            <div class="publish-detail">
                {move || if cloud.get() { view! {
                    <div><span class="release-path-label">"VERCEL · STATIC WEBSITES"</span><h4>"A deliberate step into the world."</h4><ol><li>"Configure your Vercel connection in the desktop app."</li><li>"Save your source and review the findings."</li><li>"Confirm publishing, then open the returned URL."</li></ol><p class="preview-footnote">"This tour does not create a deployment. Generated backends and databases are not supported by this publisher."</p></div>
                }.into_any() } else { view! {
                    <div><span class="release-path-label">"A LOCAL SOURCE BUNDLE"</span><h4>"Take it from here."</h4><div class="export-tree"><span>"▱ your-project/"</span><span>"├ index.html"</span><span>"├ styles.css"</span><span>"├ app.js"</span><span>"├ tesserun-review.json"</span><span>"└ tesserun-export.json"</span></div><p class="preview-footnote">"Example bundle. Your generated files and review reports are exported together."</p></div>
                }.into_any() }}
            </div>
            <a class="demo-next" href="#download">"Try it in Tesserun "<span>"↓"</span></a>
        </section>
    }
}
