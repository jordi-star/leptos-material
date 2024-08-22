//! leptos-material binds Material Web Components to Leptos Components
//! This library aims to stay as close to the original MWC component specs, meaning you should be able to use everything
//! as specified in their [docs](https://material-web.dev/).
//! leptos-material also tries to fill in the gaps for components that have not yet been implemented in material-web, such as Cards and Date Pickers.
//! Many components have a wide variety of attributes supported. For brevity, leptos-material does not completely expose all attributes,
//! instead, a component may support using `attr:attribute_name=value` syntax in the `view` macro. For example:
//! ```
//! <TextField
//! style=TextFieldStyle::Outlined
//! attr:type=InputType::Email
//! attr:label="Email"
//! />
//! ```

#![allow(non_snake_case)] // allow camel case component names
use leptos::{view, Attribute, Binding, IntoView};
use leptos_meta::*;
pub mod components;

/// Add required stylesheets and script tags to the page \<head\>.
/// This must be placed at the root of your main component for leptos-material components to work as intended.
/// For example, if you have a main component named `App`, your code will look like this:
/// ```
/// #[component]
/// fn App() -> impl IntoView {
/// 	view! {
/// 		<UseMaterialWebComponents/>
/// 		... // The rest of your site
/// 	}
/// }
/// ```
pub fn UseMaterialWebComponents() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap"
        />
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined"
        />
        <Style>"body { font-family = 'Roboto'; }"</Style>
        <Script>{include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/output_bundle.js"))}</Script>
        <Style>

            {#[cfg(feature = "card")] include_str!("components/css/card.css")}
            {#[cfg(feature = "datepicker")] include_str!("components/css/datepicker.css")}

        </Style>
    }
}
