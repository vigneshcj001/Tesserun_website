use crate::palette::CommandPalette;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

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
                on:click=move |event: web_sys::MouseEvent| {
                    if event.target().and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                        .is_some_and(|element| element.closest("a").ok().flatten().is_some()) {
                        set_open.set(false);
                    }
                }>
                <a href="#workspace">"Workspace"</a>
                <a href="#workflow">"How it works"</a>
                <a href="#questions">"FAQs"</a>
                <a href="https://github.com/vigneshcj001/Tesserun" target="_blank" rel="noopener noreferrer">
                    "GitHub "<span aria-hidden="true">"↗"</span>
                </a>
                <CommandPalette on_navigate=Callback::new(move |()| set_open.set(false))/>
            </nav>
        </div>
    }
}
