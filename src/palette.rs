use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

mod destinations;
use destinations::{Destination, matching};

#[component]
pub fn CommandPalette(on_navigate: Callback<()>) -> impl IntoView {
    let dialog = NodeRef::<leptos::html::Dialog>::new();
    let search = NodeRef::<leptos::html::Input>::new();
    let query = RwSignal::new(String::new());
    let open = move || {
        if let Some(dialog) = dialog.get() {
            let _ = dialog.show_modal();
            if let Some(input) = search.get() {
                let _ = input.focus();
            }
        }
    };
    let listener =
        leptos::leptos_dom::helpers::window_event_listener(leptos::ev::keydown, move |event| {
            if (event.ctrl_key() || event.meta_key()) && event.key().eq_ignore_ascii_case("k") {
                event.prevent_default();
                if let Some(dialog) = dialog.get() {
                    if dialog.open() {
                        dialog.close();
                    } else {
                        open();
                    }
                }
            }
        });
    on_cleanup(move || listener.remove());
    let matches = move || matching(&query.get());
    view! {
        <button class="command-trigger" aria-label="Search website — Control or Command K" title="Jump to a section (Ctrl/Cmd + K)" on:click=move |_| open()><span aria-hidden="true">"⌕"</span><span class="command-trigger-label">"Quick jump"</span><kbd>"K"</kbd></button>
        <Portal><dialog class="command-dialog" node_ref=dialog aria-label="Navigate Tesserun"
            on:close=move |_| query.set(String::new())
            on:click=move |event| {
                if event.target().is_some_and(|target| target.dyn_ref::<web_sys::HtmlDialogElement>().is_some())
                    && let Some(dialog) = dialog.get() {
                    dialog.close();
                }
            }>
            <div class="command-inner">
                <div class="command-search"><span aria-hidden="true">"⌕"</span>
                    <input node_ref=search aria-label="Find a section" placeholder="Where would you like to go?" autocomplete="off"
                        prop:value=move || query.get() on:input=move |event| query.set(event_target_value(&event))
                        on:keydown=move |event: web_sys::KeyboardEvent| {
                            if event.key() == "Enter" || event.key() == "ArrowDown" {
                                event.prevent_default();
                                if let Ok(Some(link)) = document().query_selector(".command-dialog .command-result")
                                    && let Some(link) = link.dyn_ref::<web_sys::HtmlElement>() {
                                        if event.key() == "Enter" { link.click(); } else { let _ = link.focus(); }
                                }
                            }
                        }/>
                    <button aria-label="Close navigation search" class="command-close" on:click=move |_| { if let Some(dialog) = dialog.get() { dialog.close(); } }>"esc"</button>
                </div>
                <div class="command-results">
                    {move || if matches().is_empty() { view! { <p class="command-empty" role="status">"No matching section. Try “download” or “models”."</p> }.into_any() } else {
                        matches().into_iter().map(move |Destination { name, description, href, icon, .. }| view! {
                            <a class="command-result" href=href on:click=move |_| { if let Some(dialog) = dialog.get() { dialog.close(); } on_navigate.run(()); }><span class="command-icon" aria-hidden="true">{icon}</span><span><strong>{name}</strong><small>{description}</small></span><span class="command-arrow" aria-hidden="true">"↗"</span></a>
                        }).collect_view().into_any()
                    }}
                </div>
                <div class="command-footer"><span>"A little shortcut to your next step."</span><span>"↓ to focus · Enter to go"</span></div>
            </div>
        </dialog></Portal>
    }
}
