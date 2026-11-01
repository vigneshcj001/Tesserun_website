use leptos::prelude::*;
use wasm_bindgen::JsCast;

const PROVIDERS: [(&str, &str, &str, &str); 5] = [
    (
        "OpenAI",
        "◉",
        "YOUR CLOUD CONNECTION",
        "Choose a model and enter your OpenAI API key in Settings. Your brief and relevant source are sent to the provider you choose.",
    ),
    (
        "Anthropic",
        "✳",
        "YOUR CLOUD CONNECTION",
        "Connect your Anthropic API key and choose a model in Settings. Planning, development, and review use your selected connection.",
    ),
    (
        "Gemini",
        "✦",
        "YOUR CLOUD CONNECTION",
        "Use your Gemini API key and chosen model. Tesserun keeps the same development workflow while you choose the provider behind it.",
    ),
    (
        "Groq",
        "↯",
        "YOUR CLOUD CONNECTION",
        "Enter your Groq API key and select a supported model. Provider availability, model access, and usage charges depend on your account.",
    ),
    (
        "Ollama",
        "♧",
        "A LOCAL MODEL CONNECTION",
        "Run Ollama on your computer and select an installed model. Tesserun connects to a loopback endpoint; no cloud provider key is required for this connection.",
    ),
];

#[component]
pub fn ProviderExplorer() -> impl IntoView {
    let selected = RwSignal::new(0_usize);
    view! {
        <div class="provider-heading"><div><span class="section-kicker">"ONE WORKSPACE. YOUR CONNECTION."</span><h2>"Choose what "<span class="serif">"powers you."</span></h2></div><p>"Explore the supported providers. Connect your choice in the desktop app."</p></div>
        <div class="provider-tabs" role="tablist" aria-label="Explore model providers">
            {PROVIDERS.into_iter().enumerate().map(move |(index, (name, icon, _, _))| view! {
                <button id=format!("provider-{index}") role="tab" aria-controls="provider-description"
                    aria-selected=move || (selected.get() == index).to_string() tabindex=move || if selected.get() == index { 0 } else { -1 }
                    on:click=move |_| selected.set(index)
                    on:keydown=move |event: web_sys::KeyboardEvent| {
                        let next = match event.key().as_str() { "ArrowRight" => Some((index + 1) % 5), "ArrowLeft" => Some((index + 4) % 5), "Home" => Some(0), "End" => Some(4), _ => None };
                        if let Some(next) = next { event.prevent_default(); selected.set(next); if let Some(element) = document().get_element_by_id(&format!("provider-{next}")) && let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() { let _ = element.focus(); } }
                    }><span aria-hidden="true">{icon}</span>{name}</button>
            }).collect_view()}
        </div>
        <div id="provider-description" class="provider-description" role="tabpanel" tabindex="0" aria-labelledby=move || format!("provider-{}", selected.get())>
            <div><span class="provider-connection-label">{move || PROVIDERS[selected.get()].2}</span><p>{move || PROVIDERS[selected.get()].3}</p></div>
            <a href="#download">"Make it your workspace "<span>"↗"</span></a>
        </div>
        <p class="provider-fine-print">"This explorer does not connect to providers. Cloud API usage may incur charges. Keys entered in Tesserun stay in process memory."</p>
    }
}
