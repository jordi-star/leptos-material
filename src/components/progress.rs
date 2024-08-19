use leptos::{component, html, view, Children, IntoView};
/// [MWC Docs](https://material-web.dev/components/progress/)
#[component]
pub fn CircularProgress(#[prop(default = true)] indeterminate: bool) -> impl IntoView {
    view! { <md-circular-progress indeterminate=indeterminate></md-circular-progress> }
}

/// [MWC Docs](https://material-web.dev/components/progress/)
#[component]
pub fn LinearProgress(#[prop(default = true)] indeterminate: bool) -> impl IntoView {
    view! { <md-linear-progress indeterminate=indeterminate></md-linear-progress> }
}
