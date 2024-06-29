use bincalc::App;
use leptos::{document, mount_to, view, IntoView, wasm_bindgen::JsCast};

fn main() {
    // lazy, so not error checking. know it is there lol
    let document = document();
    let element = document.get_element_by_id("container").unwrap().unchecked_into();
    mount_to(element, || {
        view! { <App/> }
    })
}
