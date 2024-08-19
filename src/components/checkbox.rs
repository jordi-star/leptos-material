use leptos::{
    component, logging, view, IntoView, ReadSignal, Signal, SignalGet, SignalGetUntracked,
    SignalSet, SignalSetter, SignalUpdate, WriteSignal,
};

/// [MWC Docs](https://material-web.dev/components/checkbox/)
#[component]
pub fn Checkbox(
    #[prop(optional, into)] get_checked: Option<Signal<bool>>,
    #[prop(optional, into)] set_checked: Option<SignalSetter<bool>>,
) -> impl IntoView {
    view! {
        <md-checkbox
            prop:checked=move || match get_checked {
                Some(value) => value(),
                None => false,
            }

            // On mouse down is more responsive than onclick.
            on:mousedown=move |_| {
                if let Some(getter) = get_checked {
                    if let Some(setter) = set_checked {
                        setter.set(!getter.get());
                    }
                }
            }
        >
        </md-checkbox>
    }
}
