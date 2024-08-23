use leptos::{component, slot, view, Children, ChildrenFn, IntoView};

use crate::components::elevation::Elevation;

#[derive(Default, PartialEq)]
pub enum CardStyle {
    #[default]
    Elevated,
    Filled,
    Outlined,
}

impl CardStyle {
    pub fn get_class_name(&self) -> String {
        match self {
            CardStyle::Elevated => "elevated",
            CardStyle::Filled => "filled",
            CardStyle::Outlined => "outlined",
        }
        .into()
    }
}

/// This is a native Leptos component.
/// See [material.io](https://m3.material.io/components/cards/overview).
#[component]
pub fn Card(
    style: CardStyle,
    #[prop(optional)] headline: Option<Headline>,
    children: Children,
) -> impl IntoView {
    let elevation = style.eq(&CardStyle::Elevated).then(Elevation);
    let classes = format!("leptos-material-card {}", style.get_class_name());
    let headline_view = headline.map(|value| {
        view! { <div class="leptos-material-card-headline">{(value.children)()}</div> }
    });
    view! { <div class=classes>{elevation} {headline_view} {children()}</div> }
}

/// From [material.io](https://m3.material.io/components/cards/guidelines#001f3b36-abe2-42dd-90cd-958a465377fb): Headline text often communicates the subject of the card, such as the name of a photo album or article.
#[slot]
pub struct Headline {
    children: ChildrenFn,
}
