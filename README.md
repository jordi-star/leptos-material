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
Every component is gated behind a feature. By default, this library will include all component's MWC code at compile time.
To reduce compile time and save webpage load time, [disable default-features](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) and only add features related to the components you need.

## `nightly` Note
This library does not use `nightly` Rust to allow this library to work in projects that are using the stable Rust toolchain.
Using Leptos with `nightly` Rust allows function-call syntax for Signals, instead of using `.get()` and `.set()`. This functionality is not essential to this library, so `leptos-material` will stick to Stable Rust for the foreseeable future.