use leptos::{component, view, IntoView};

/// Icon element. See [Material Symbols](https://fonts.google.com/icons) for icon options.
/// Icon names are `snake_case`.
#[component]
pub fn Icon(
    /// Icon name. See [Material Symbols](https://fonts.google.com/icons) for icon options.
    #[prop(into)]
    name: String,
    #[prop(optional, into)] element_slot: String,
) -> impl IntoView {
    view! { <md-icon slot=element_slot>{name}</md-icon> }
}
