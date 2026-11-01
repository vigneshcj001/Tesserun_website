use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let (open, set_open) = signal(false);
    let button = NodeRef::<leptos::html::Button>::new();

    view! {
        <div class="navigation-region" on:keydown=move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" && open.get_untracked() {
                set_open.set(false);
                if let Some(button) = button.get() { let _ = button.focus(); }
            }
        }>
            <button node_ref=button class="menu-toggle"
                aria-label=move || if open.get() { "Close navigation" } else { "Open navigation" }
                aria-expanded=move || open.get().to_string() aria-controls="navigation"
                on:click=move |_| set_open.update(|value| *value = !*value)>
                <span></span><span></span>
            </button>
            <nav id="navigation" aria-label="Main navigation" class:open=move || open.get()
                on:click=move |_| set_open.set(false)>
                <a href="#workspace">"Workspace"</a>
                <a href="#workflow">"How it works"</a>
                <a href="#questions">"FAQs"</a>
                <a href="https://github.com/vigneshcj001/Tesserun" target="_blank" rel="noopener noreferrer">
                    "GitHub "<span aria-hidden="true">"↗"</span>
                </a>
            </nav>
        </div>
    }
}
