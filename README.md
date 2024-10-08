# leptos-material
leptos-material is a Leptos component wrapper for [Material Web Components](https://material-web.dev/) (MWC), along with some extra components to fill in the gaps.

This library aims to stay close to the original MWC component specs, meaning you should be able to use everything
as specified in their [docs](https://material-web.dev/).
leptos-material also implements some components not yet implemented in MWC, such as Cards and Date Pickers.

Many components have a wide variety of attributes supported. For brevity, leptos-material does not completely expose all attributes,
instead, many components use `attr:attribute_name=value` syntax in the `view` macro to add attributes. For example:
 ```html
 <TextField
 style=TextFieldStyle::Outlined
 attr:type=InputType::Email
 attr:label="Email"
>
```
## Installing
To install `leptos-material`, run the following command from your project directory:
```
cargo add leptos-material
```
Or, to use the latest repository updates in your project, add
```
leptos-material = { git = "https://github.com/jordi-star/leptos-material" }
```
to your Cargo.toml

## Using
Use the `UseMaterialWebComponents` component at the root of your main component to initialize MWC.
For example, if you have a main component named `App`, your code will look like this:
```rust
#[component]
fn App() -> impl IntoView {
	view! {
		<UseMaterialWebComponents/>
		... // The rest of your site
	}
}
```
Then, enable the features for any components you'd like to use.
For example, to use the `Icon` component, you'd enable the `icon` feature.
Finally, you can use the component in your code.
```rust
#[component]
fn App() -> impl IntoView {
	view! {
		<UseMaterialWebComponents/>
		<h1>{"Hello from leptos-material!"}</h1>
		<Icon name="mood" />
	}
}
```


## Features
Every component is gated behind a feature. This library will bundle any needed MWC components into a single JS file for use with `UseMaterialWebComponents`. By default, leptos-material uses the `full` feature, which will include all MWC components.
To reduce compile time and save webpage load time, [disable default-features](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) and only add features related to the components you need.

## `nightly` Note
This library does not use `nightly` Rust to allow this library to work in projects that are using the stable Rust toolchain.
Using Leptos with `nightly` Rust allows function-call syntax for Signals, instead of using `.get()` and `.set()`. This functionality is not essential to this library, so leptos-material will stick to stable Rust for the foreseeable future.

## Work in progress
This library is a work in progress, and does not currently implement *all* MWC components.
Here's a list of all currently implemented feature-flags:
* checkbox
* textfield
* button
* icon
* iconbutton
* card
* elevation
* progress
* datepicker
* select
* chips