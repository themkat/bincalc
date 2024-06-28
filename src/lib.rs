use leptos::{
    component, create_effect, create_signal, event_target_value, view, Callable, Callback, For, IntoView, SignalGet, SignalSet
};
use radix_fmt::radix;

// TODO: maybe some sort of sorting could be user friendly?

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal("0".to_string());
    let (base, set_base) = create_signal(16);
    let (valid, set_valid) = create_signal(true);

    view! {
        <div class="mx-auto sm:w-[80vw] flex flex-col items-center ">
            <BaseSelector
                selected_bases=move |_| {
                    let mut result = vec![base.get()];
                    result
                        .extend(
                            (2_u32..=32_u32)
                                .filter(|elem| base.get() != *elem)
                                .collect::<Vec<u32>>(),
                        );
                    result
                }

                on_selected=move |selected_base| {
                    let current_input = input.get();
                    set_input
                        .set(
                            format!(
                                "{:#}",
                                radix(
                                    u32::from_str_radix(current_input.as_str(), base.get())
                                        .unwrap_or(0),
                                    selected_base as u8,
                                ),
                            ),
                        );
                    set_base.set(selected_base);
                }

                callback_on_btn=false
            />
            <input
                type="text"
                class="border-dashed border-2 text-right"
                prop:value=input
                on:input=move |ev| {
                    let number = event_target_value(&ev);
                    set_input.set(number.clone());
                    set_valid.set(number_is_valid(number, base.get()));
                }
            />

            <label>"Valid: " {valid}</label>
            <OutputList number=move |_| {
                u32::from_str_radix(input.get().as_str(), base.get()).unwrap_or(0)
            }/>
        </div>
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
            <For
                each=move || selected_bases.get()
                key=move |base| format!("{}-{}", number.call(NothingNess {}), base)
                children=move |base| {
                    view! {
                        <li class="p-1.5 hover:outline">
                            {base_name(base)} ": "
                            {format!("{:#}", radix(number.call(NothingNess {}), base as u8))}
                        </li>
                    }
                }
            />

        </ul>
        <BaseSelector
            selected_bases=move |_| {
                (2..=32).filter(|base| !selected_bases.get().contains(base)).collect::<Vec<u32>>()
            }

            on_selected=move |base| {
                set_selected_bases
                    .set(selected_bases.get().into_iter().chain(vec![base].into_iter()).collect())
            }

            callback_on_btn=true
        />
    }
}

#[component]
fn BaseSelector(
    #[prop(into)] selected_bases: Callback<NothingNess, Vec<u32>>,
    #[prop(into)] on_selected: Callback<u32>,
    callback_on_btn: bool,
    #[prop(default = "flex items-center py-3.5".to_string())] class: String,
) -> impl IntoView {
    let (base, set_base) = create_signal(3_u32);

    create_effect(move |_| {
        let bases = selected_bases.call(NothingNess {});
        set_base.set(*bases.first().unwrap_or(&0));
    });
    
    view! {
        <div class=class>
            <select
                name="Base"
                class="border-solid border-2 rounded-lg"
                on:change=move |ev| {
                    let new_base = event_target_value(&ev).parse().unwrap();
                    set_base.set(new_base);
                    if !callback_on_btn {
                        on_selected.call(new_base);
                    }
                }

                prop:value=move || base.get().to_string()
            >
                <For
                    each=move || selected_bases.call(NothingNess {})
                    key=|base| base.clone()
                    children=move |base| {
                        view! { <option value=base>{base_name(base)}</option> }
                    }
                />

            </select>
            {move || {
                if callback_on_btn {
                    view! {
                        <button
                            class="border-solid border-2 rounded-lg px-1 mx-1.5"
                            on:click=move |_| {
                                on_selected.call(base.get());
                            }
                        >

                            "Add"
                        </button>
                    }
                        .into_any()
                } else {
                    view! { <p></p> }.into_any()
                }
            }}

        </div>
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
