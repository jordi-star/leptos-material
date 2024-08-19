use leptos::{component, html, Attribute, Children, IntoAttribute, IntoView};

use super::button::ButtonType;

#[derive(Default)]
pub enum IconButtonStyle {
    #[default]
    Icon,
    Filled,
    FilledTonal,
    Outlined,
}

impl IconButtonStyle {
    pub fn get_element_name(&self) -> String {
        match self {
            IconButtonStyle::Icon => "md-icon-button",
            IconButtonStyle::Filled => "md-filled-icon-button",
            IconButtonStyle::FilledTonal => "md-filled-tonal-icon-button",
            IconButtonStyle::Outlined => "md-outlined-icon-button",
        }
        .into()
    }
}

/// [MWC Docs](https://material-web.dev/components/icon-button/)
#[component]
pub fn IconButton(
    #[prop(optional)] style: IconButtonStyle,
    #[prop(optional)] button_type: ButtonType,
    #[prop(attrs)] attr: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    html::custom(html::Custom::new(style.get_element_name()))
        .attrs(attr)
        .attr("type", button_type.into_attribute())
        .child(children())
}
