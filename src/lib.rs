use leptos::{component, create_signal, event_target_value, view, IntoView, SignalGet, SignalSet};

#[component]
pub fn App() -> impl IntoView {
    // TODO: any state here?
    // TODO: base as well?
    let (input, set_input) = create_signal::<String>(String::new());
    let (base, set_base) = create_signal(16);
    let (valid, set_valid) = create_signal(true);

    // todo: layout
    //         ability to select number system
    //         input on top
    //         output list with + button to add one
    view! {
        <input type="text"
            prop:value=input
            on:input=move |ev| {
                // TODO: validate?

                let number = event_target_value(&ev);
                set_input.set(number.clone());
                set_valid.set(number_is_valid(number, base.get()));
            } />
        <label>"Valid: " {valid}</label>
        <OutputList />
    }
}

// TODO: a ouputs component? Where we can add or remove rows?
#[component]
fn OutputList() -> impl IntoView {
    // TODO: signal for result types?
    // TODO: plus sign at the bottom to add a new one

    view! {
        <p>"Results will end up here"</p>
    }
}

fn number_is_valid(digits: String, base: u32) -> bool {
    u64::from_str_radix(&digits, base).is_ok()
}

#[cfg(test)]
mod tests {
    // TODO: just verify number validity for many different numbers
}
