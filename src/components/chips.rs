use leptos::{component, html, Attribute, Children, IntoAttribute, IntoView};

#[derive(Default)]
pub enum ChipType {
    #[default]
    Assist,
    Filter,
    Input,
    Suggestion,
}

impl ChipType {
    pub fn get_element_name(&self) -> String {
        match self {
            ChipType::Assist => "md-assist-chip",
            ChipType::Filter => "md-filter-chip",
            ChipType::Input => "md-input-chip",
            ChipType::Suggestion => "md-suggestion-chip",
        }
        .into()
    }
}

/// [MWC Docs](https://material-web.dev/components/chip/#chip-sets)
#[component]
pub fn Chipset(children: Children) -> impl IntoView {
    leptos::view! { <md-chip-set>{children()}</md-chip-set> }
}

/// [MWC Docs](https://material-web.dev/components/chip/)
#[component]
pub fn Chip(
    #[prop(optional)] chip_type: ChipType,
    #[prop(attrs)] attr: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    html::custom(html::Custom::new(chip_type.get_element_name()))
        .attrs(attr)
        .child(children())
}
