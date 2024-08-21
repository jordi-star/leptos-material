use leptos::{
    component,
    html::{self, Input},
    slot, view, Attribute, AttributeValue, Binding, Children, IntoAttribute, IntoView, NodeRef,
    ReadSignal, RwSignal, SignalGet,
};

#[derive(Default)]
pub enum SelectStyle {
    #[default]
    Filled,
    Outlined,
}

impl SelectStyle {
    pub fn get_element_name(&self) -> String {
        match self {
            SelectStyle::Filled => "md-filled-select",
            SelectStyle::Outlined => "md-outlined-select",
        }
        .into()
    }
}

/// [MWC Docs](https://material-web.dev/components/select/)
#[component]
pub fn Select(
    #[prop(optional)] style: SelectStyle,
    children: Children,
    // #[prop(optional)] text_field_icon: Option<TextFieldIcon>,
    // #[prop(optional)] props: Vec<TextFieldProperty>,
) -> impl IntoView {
    html::custom(html::Custom::new(style.get_element_name())).child(children())
}

/// [MWC Docs](https://material-web.dev/components/select/)
#[component]
pub fn SelectOption(
    #[prop(into)] value: String,
    #[prop(attrs)] attr: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <md-select-option value={ move || value }() {..attr}>
            {children()}
        </md-select-option>
    }
}
