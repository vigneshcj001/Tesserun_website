mod download;
mod navigation;
mod palette;
mod providers;
mod tour;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    console_error_panic_hook::set_once();
    let document = document();

    // Static content remains usable before WASM loads, or when it is disabled.
    // The interactive regions are enhanced without delaying the static page.
    if let Some(element) = document.get_element_by_id("navigation-root") {
        element.set_text_content(None);
        leptos::mount::mount_to(element.unchecked_into(), navigation::Navigation).forget();
    }
    if let Some(element) = document.get_element_by_id("workspace-tour") {
        element.set_text_content(None);
        leptos::mount::mount_to(element.unchecked_into(), tour::WorkspaceTour).forget();
    }
    if let Some(element) = document.get_element_by_id("models") {
        element.set_text_content(None);
        leptos::mount::mount_to(element.unchecked_into(), providers::ProviderExplorer).forget();
    }
    if let Some(element) = document.get_element_by_id("download-interaction") {
        element.set_text_content(None);
        leptos::mount::mount_to(element.unchecked_into(), download::DownloadCard).forget();
    }
}
