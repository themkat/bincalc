use leptos::{
    component, create_signal, event_target_value, view, Callable, Callback, For, IntoView,
    ReadSignal, SignalGet, SignalSet,
};
use radix_fmt::radix;

// TODO: might be useful to limit the input somehow? hex numbers suddenly get big in binary lol :smile:
//       would be fun to find a general way to calculate a number like that :D

// TODO: maybe some sort of sorting could be user friendly?

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal::<String>(String::new());
    let (base, set_base) = create_signal(16);
    let (valid, set_valid) = create_signal(true);

    view! {
        <input type="text"
            prop:value=input
            on:input=move |ev| {
                let number = event_target_value(&ev);
                set_input.set(number.clone());
                set_valid.set(number_is_valid(number, base.get()));
            } />
        <label>"Valid: " {valid}</label>
        <OutputList number=move |_| u32::from_str_radix(input.get().as_str(), base.get()).unwrap_or(0) />
    }
}

// placeholder struct
struct NothingNess;

#[component]
fn OutputList(#[prop(into)] number: Callback<NothingNess, u32>) -> impl IntoView {
    // TODO: maybe change to set. even if we only render each once due to the clever keys, we can probably still fill memory
    let (selected_bases, set_selected_bases) = create_signal(vec![2, 10, 16]);

    view! {
        <ul>
            <For each=move || selected_bases.get()
            key=move |base| format!("{}-{}", number.call(NothingNess {}), base)
            children=move |base| {
                view! {
                    <li>{base_name(base)} ": " {format!("{:#}", radix(number.call(NothingNess {}), base as u8))}</li>
                }
            }/>
        </ul>
        <BaseSelector selected_bases=selected_bases
            on_selected=move |base| {
                set_selected_bases.set(selected_bases.get().into_iter().chain(vec![base].into_iter()).collect())
            }/>
    }
}

#[component]
fn BaseSelector(
    selected_bases: ReadSignal<Vec<u32>>,
    #[prop(into)] on_selected: Callback<u32>,
) -> impl IntoView {
    let (base, set_base) = create_signal(3_u32);

    view! {
        <select name="Base" on:change=move |ev| {
            set_base.set(event_target_value(&ev).parse().unwrap());
        }
                prop:value=move || base.get().to_string()
        >
            <For each=move || {
                (2..=32).filter(|base| !selected_bases.get().contains(base)).collect::<Vec<u32>>()
            }
                 key=|base| base.clone()
                 children=move |base| {
                     view! {
                         <option value=base>{base_name(base)}</option>
                     }
                 }
             />
        </select>
        <button on:click=move |_| {
            on_selected.call(base.get());
        }>"Add"</button>
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
        12 => Some("Duodecimal".to_string()),
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
        base_name_test!(Duodecimal with base 12);
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
