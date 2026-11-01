use leptos::prelude::*;

#[component]
pub fn DownloadCard() -> impl IntoView {
    let guide = RwSignal::new(false);
    let requested = RwSignal::new(false);
    view! {
        <div class="download-card-top"><img src="assets/mark.svg" width="45" height="45" alt=""/><span>"Tesserun for Windows"<small>"v0.3.0 "<span>"·"</span>" x64 "<span>"·"</span>" Early release"</small></span><span class="platform-symbol" aria-hidden="true">"⊞"</span></div>
        <a class="button button-primary" href="downloads/tesserun-0.3.0-windows-x64.exe" download="tesserun-0.3.0-windows-x64.exe"
            on:click=move |_| { requested.set(true); guide.set(true); }>
            {move || if requested.get() { "Download again" } else { "Download Tesserun" }}<span aria-hidden="true">"↓"</span>
        </a>
        <div class="download-meta"><span>"Portable .exe · 5.7 MB"</span><a href="downloads/SHA256SUMS.txt" target="_blank" rel="noopener">"SHA-256 ↗"</a></div>
        <Show when=move || requested.get()><p class="download-requested" role="status">"Download requested. Your browser handles saving the file. Here’s how to get started."</p></Show>
        <button class="setup-toggle" aria-expanded=move || guide.get().to_string() aria-controls="setup-guide" on:click=move |_| guide.update(|value| *value = !*value)>
            <span>"First time? Your first three steps"</span><span aria-hidden="true">{move || if guide.get() { "−" } else { "+" }}</span>
        </button>
        <div id="setup-guide" class="setup-guide" hidden=move || !guide.get()>
            <ol><li><span>"01"</span><div><strong>"Make a little room."</strong><p>"Save the portable app in a writable folder, then open it."</p></div></li><li><span>"02"</span><div><strong>"Choose your connection."</strong><p>"Open Settings. Select a provider and model, then enter your API key or connect local Ollama."</p></div></li><li><span>"03"</span><div><strong>"Bring your first idea."</strong><p>"Describe a project in Development. Review the source and test it before publishing."</p></div></li></ol>
        </div>
        <p class="release-notice">"Unsigned early release. Tested on the developer’s Windows machine; compatibility on other PCs is not yet verified."</p>
        <a class="release-link" href="https://github.com/vigneshcj001/Tesserun" target="_blank" rel="noopener noreferrer">"Prefer to build from source? "<span>"↗"</span></a>
    }
}
