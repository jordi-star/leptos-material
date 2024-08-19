use leptos::{component, html, Attribute, Children, IntoAttribute, IntoView};

#[derive(PartialEq, Clone, Default)]
pub enum ButtonType {
    #[default] // Default is `Submit` to be consistent with the HTML standard.
    Submit,
    Button,
    Reset,
}

impl From<ButtonType> for String {
    fn from(value: ButtonType) -> Self {
        match value {
            ButtonType::Button => "button",
            ButtonType::Reset => "reset",
            ButtonType::Submit => "submit",
        }
        .into()
    }
}

impl IntoAttribute for ButtonType {
    fn into_attribute(self) -> Attribute {
        Attribute::String(String::from(self).into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Attribute::String(String::from(*self).into())
    }
}

/// Styles correspond to the 5 button types.
/// This is used in the Button component to determine what element tag to use.
#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Elevated,
    Filled,
    FilledTonal,
    Outlined,
    Text,
}

impl ButtonStyle {
    /// Get corrosponding element tag from enum value.
    pub fn get_element_name(&self) -> String {
        match self {
            ButtonStyle::Elevated => "md-elevated-button",
            ButtonStyle::Filled => "md-filled-button",
            ButtonStyle::FilledTonal => "md-filled-tonal-button",
            ButtonStyle::Outlined => "md-outlined-button",
            ButtonStyle::Text => "md-text-button",
        }
        .into()
    }
}

/// [MWC Docs](https://material-web.dev/components/button/)
#[component]
pub fn Button(
    #[prop(optional)] style: ButtonStyle,
    #[prop(optional)] button_type: ButtonType,
    #[prop(attrs)] attr: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    html::custom(html::Custom::new(style.get_element_name()))
        .attrs(attr)
        .attr("type", button_type.into_attribute())
        .child(children())
}
