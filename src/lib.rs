use leptos::{
    component, create_signal, event_target_value, svg::view, view, For, IntoView, SignalGet,
    SignalSet,
};

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal::<String>(String::new());
    let (base, set_base) = create_signal(16);
    let (valid, set_valid) = create_signal(true);

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

#[component]
fn OutputList() -> impl IntoView {
    let (selected_bases, set_selected_bases) = create_signal(vec![2, 10, 16]);
    // TODO: plus sign at the bottom to add a new one

    view! {
        <p>"Results will end up here"</p>
        <ul>
            <For each=move || selected_bases.get()
            key=|base| base.clone()
            children=move |base| {
                view! {
                    <p>{base_name(base)} ": "</p>
                }
            }/>
        </ul>
    }
}

fn number_is_valid(digits: String, base: u32) -> bool {
    u64::from_str_radix(&digits, base).is_ok()
}

fn base_name(base: u32) -> Option<String> {
    match base {
        2 => Some("Binary".to_string()),
        8 => Some("Octal".to_string()),
        10 => Some("Decimal".to_string()),
        16 => Some("Hexadecimal".to_string()),
        2..=32 => Some(format!("Base{base}")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    // TODO: just verify number validity for many different numbers

    mod base_name {
        use crate::base_name;
        use paste::paste;

        macro_rules! base_name_test {
            ($name:ident with base $base:expr) => {
                paste! {
                    #[test]
                    fn [<base_name_ $name:lower _test>]() {
                        assert_eq!(Some(stringify!($name).to_string()), base_name($base));
                    }
                }
            };
        }

        base_name_test!(Binary with base 2);
        base_name_test!(Octal with base 8);
        base_name_test!(Decimal with base 10);
        base_name_test!(Hexadecimal with base 16);
        base_name_test!(Base3 with base 3);
        base_name_test!(Base32 with base 32);

        #[test]
        fn base_name_invalid_test() {
            assert!(base_name(0).is_none());
            assert!(base_name(1).is_none());
            assert!(base_name(33).is_none());
            assert!(base_name(69).is_none());
        }
    }
}
