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

## Features
Every component is gated behind a feature. This library only bundles what you need for each component used. When a feature for a component is added, leptos-material will automatically bundle that component's MWC code at compile time.

<!-- | Component     | Status
| ------------- |:-------------:|
|       | |