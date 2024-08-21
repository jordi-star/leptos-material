use leptos::{
    component,
    html::{self, Input},
    slot, view, Attribute, AttributeValue, Binding, Children, IntoAttribute, IntoView, NodeRef,
    RwSignal, SignalGet,
};

use super::icon::{Icon, IconProps};

#[derive(PartialEq, Clone, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Number,
    Password,
    Search,
    TelephoneNumber,
    Url,
    Textarea,
}

impl From<InputType> for String {
    fn from(value: InputType) -> Self {
        match value {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Search => "search",
            InputType::TelephoneNumber => "tel",
            InputType::Url => "url",
            InputType::Textarea => "textarea",
        }
        .into()
    }
}

impl IntoAttribute for InputType {
    fn into_attribute(self) -> Attribute {
        Attribute::String(String::from(self).into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Attribute::String(String::from(*self).into())
    }
}

pub enum TextFieldIconPlacement {
    Leading,
    Trailing,
}

#[slot]
pub struct TextFieldIcon {
    placement: TextFieldIconPlacement,
    /// Icon Name
    #[prop(into)]
    name: String,
}

#[derive(Default)]
pub enum TextFieldStyle {
    #[default]
    Filled,
    Outlined,
}

impl TextFieldStyle {
    pub fn get_element_name(&self) -> String {
        match self {
            TextFieldStyle::Filled => "md-filled-text-field",
            TextFieldStyle::Outlined => "md-outlined-text-field",
        }
        .into()
    }
}

/// [MWC Docs](https://material-web.dev/components/text-field/)
#[component]
pub fn TextField(
    #[prop(optional)] style: TextFieldStyle,
    #[prop(optional)] value: RwSignal<String>,
    #[prop(attrs)] attr: Vec<(&'static str, Attribute)>,
    #[prop(optional)] text_field_icon: Option<TextFieldIcon>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let mut html = html::custom(html::Custom::new(style.get_element_name()))
        .attrs(attr)
        .prop("value", move || value.get());
    if let Some(icon) = text_field_icon {
        let icon_element = Icon(IconProps {
            name: icon.name,
            element_slot: match icon.placement {
                TextFieldIconPlacement::Leading => "leading-icon",
                TextFieldIconPlacement::Trailing => "trailing-icon",
            }
            .into(),
        });
        html = html.child(icon_element);
    }
    if let Some(child) = children {
        html = html.child(child())
    }
    html
}
