//! <div align="center">
//!   <h1>🌗🚀 Dioxus</h1>
//!   <p>
//!     <strong>A concurrent, functional, virtual DOM for Rust</strong>
//!   </p>
//! </div>
//!
//! # Resources
//!
//! This overview is provides a brief introduction to Dioxus. For a more in-depth guide, make sure to check out:
//! - [Getting Started](https://dioxuslabs.com/getting-started)
//! - [Book](https://dioxuslabs.com/book)
//! - [Reference](https://dioxuslabs.com/reference)
//! - [Community Examples](https://github.com/DioxusLabs/community-examples)
//!
//! # Overview and Goals
//!
//! Dioxus makes it easy to quickly build complex user interfaces with Rust. Any Dioxus app can run in the web browser,
//! as a desktop app, as a mobile app, or anywhere else provided you build the right renderer.
//!
//! Dioxus is heavily inspired by React, supporting many of the same concepts:
//!
//! - Hooks for state
//! - VirtualDom & diffing
//! - Concurrency, fibers, and asynchronous rendering
//! - JSX-like templating syntax
//!
//! If you know React, then you know Dioxus.
//!
//! Dioxus is *substantially* more performant than many of the other Rust UI libraries (Yew/Percy) and is *significantly* more performant
//! than React - roughly competitve with InfernoJS.
//!
//! Remember: Dioxus is a library for declaring interactive user interfaces - it is not a dedicated renderer. Most 1st party renderers for Dioxus currently only support web technologies.
//!
//! ## Brief Overview
//!
//! All Dioxus apps are built by composing functions that take in a `Scope` which is generic over some `Properties` and return an `Element`.
//! A `Scope` holds relevant state data for the the currently-rendered component.
//!
//! To launch an app, we use the `launch` method for the specific renderer we want to use. In the launch function, we pass the app's `Component`.
//!
//! ```rust, ignore
//! use dioxus::prelude::*;
//!
//! fn main() {
//!     dioxus::desktop::launch(app);
//! }
//!
//! fn app(cx: Scope) -> Element {
//!     cx.render(rsx!("hello world!"))
//! }
//! ```
//!
//! ## Elements & your first component
//!
//! To assemble UI trees with Diouxs, you need to use the `render` function on
//! something called `LazyNodes`. To produce `LazyNodes`, you can use the `rsx!`
//! macro or the NodeFactory API. For the most part, you want to use the `rsx!`
//! macro.
//!
//! Any element in `rsx!` can have attributes, listeners, and children. For
//! consistency, we force all attributes and listeners to be listed *before*
//! children.
//!
//! ```rust, ignore
//! let value = "123";
//!
//! rsx!(
//!     div {
//!         class: "my-class {value}",                  // <--- attribute
//!         onclick: move |_| log::info!("clicked!"),   // <--- listener
//!         h1 { "hello world" },                       // <--- child
//!     }
//! )
//! ```
//!
//! The rsx macro accepts attributes in "struct form" and then will parse the rest
//! of the body as child elements and rust expressions. Any rust expression that
//! implements `IntoIterator<Item = impl IntoVNode>` will be parsed as a child.
//!
//! ```rust, ignore
//! rsx!(
//!     div {
//!         (0..10).map(|_| rsx!(span { "hello world" }))
//!     }
//! )
//!
//! ```
//!
//! Used within components, the rsx! macro must be rendered into an `Element` with
//! the `render` function on Scope.
//!
//! If we want to omit the boilerplate of `cx.render`, we can simply pass in
//! `cx` as the first argument of rsx. This is sometimes useful when we need to
//! render nodes in match statements.
//!
//! ```rust, ignore
//! fn example(cx: Scope) -> Element {
//!
//!     // both of these are equivalent
//!     cx.render(rsx!("hello world"))
//!
//!     rsx!(cx, "hello world!")
//! }
//! ```
//!
//! Putting everything together, we can write a simple component that a list of
//! elements:
//!
//! ```rust, ignore
//! fn app(cx: Scope) -> Element {
//!     let name = "dave";
//!     cx.render(rsx!(
//!         h1 { "Hello, {name}!" }
//!         div {
//!             class: "my-class",
//!             id: "my-id",
//!
//!             (0..5).map(|i| rsx!(
//!                 div { key: "{i}"
//!                     "FizzBuzz: {i}"
//!                 }
//!             ))
//!
//!         }
//!     ))
//! }
//! ```
//!
//! ## Components
//!
//! We can compose these function components to build a complex app. Each new
//! component we design must take some Properties. For components with no explicit
//! properties, we can use the `()` type or simply omit the type altogether.
//!
//! In Dioxus, all properties are memoized by default!
//!
//! ```rust, ignore
//! fn App(cx: Scope) -> Element {
//!     cx.render(rsx!(
//!         Header {
//!             title: "My App",
//!             color: "red",
//!         }
//!     ))
//! }
//! ```
//!
//! Our `Header` component takes in a `title` and a `color` property, which we
//! delcare on an explicit `HeaderProps` struct.
//!
//! ```rust, ignore
//! // The `Props` derive macro lets us add additional functionality to how props are interpreted.
//! #[derive(Props, PartialEq)]
//! struct HeaderProps {
//!     title: String,
//!     color: String,
//! }
//!
//! fn Header(cx: Scope<HeaderProps>) -> Element {
//!     cx.render(rsx!(
//!         div {
//!             background_color: "{cx.props.color}"
//!             h1 { "{cx.props.title}" }
//!         }
//!     ))
//! }
//! ```
//!
//! Components may use the `inline_props` macro to completely inline the props
//! definition into the function arguments.
//!
//! ```rust, ignore
//! #[inline_props]
//! fn Header(cx: Scope, title: String, color: String) -> Element {
//!     cx.render(rsx!(
//!         div {
//!             background_color: "{color}"
//!             h1 { "{title}" }
//!         }
//!     ))
//! }
//! ```
//!
//! Components may also borrow data from their parent component. We just need to
//! attach some lifetimes to the props struct.
//! > Note: we don't need to derive `PartialEq` for borrowed props since they cannot be memoized.
//!
//! ```rust, ignore
//! #[derive(Props)]
//! struct HeaderProps<'a> {
//!     title: &'a str,
//!     color: &'a str,
//! }
//!
//! fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
//!     cx.render(rsx!(
//!         div {
//!             background_color: "{cx.props.color}"
//!             h1 { "{cx.props.title}" }
//!         }
//!     ))
//! }
//! ```
//!
//! Components that beging with an uppercase letter may be called through
//! traditional curly-brace syntax like so:
//!
//! ```rust, ignore
//! rsx!(
//!     Header { title: "My App" }
//! )
//! ```
//!
//! Alternatively, if your components begin with a lowercase letter, you can use
//! the function call syntax:
//!
//! ```rust, ignore
//! rsx!(
//!     header( title: "My App" )
//! )
//! ```
//!
//! ## Hooks
//!
//! While components are reusable forms of UI elements, hooks are reusable forms
//! of logic. Hooks provide us a way of retrieving state from `Scope` and using
//! it to render UI elements.
//!
//! By convention, all hooks are functions that should start with `use_`. We can
//! use hooks to define state and modify it from within listeners.
//!
//! ```rust, ignore
//! fn app(cx: Scope) -> Element {
//!     let name = use_state(&cx, || "world");
//!
//!     rsx!(cx, "hello {name}!")
//! }
//! ```
//!
//! Hooks are sensitive to how they are used. To use hooks, you must abide by the
//! ["rules of hooks" (borrowed from react)](https://reactjs.org/docs/hooks-rules.html):
//! - Functions with "use_" should not be called in callbacks
//! - Functions with "use_" should not be called out of order
//! - Functions with "use_" should not be called in loops or conditionals
//!
//! In a sense, hooks let us add a field of state to our component without declaring
//! an explicit struct. However, this means we need to "load" the struct in the right
//! order. If that order is wrong, then the hook will pick the wrong state and panic.
//!
//! Most hooks you'll write are simply composition of other hooks:
//!
//! ```rust, ignore
//! fn use_username(cx: &ScopeState, id: Uuid) -> bool {
//!     let users = use_context::<Users>(cx);
//!     users.get(&id).map(|user| user.logged_in).ok_or(false)
//! }
//! ```
//!  
//! To create entirely new foundational hooks, we can use the `use_hook` method on ScopeState.
//!
//! ```rust, ignore
//! fn use_mut_string(cx: &ScopeState) -> &mut String {
//!     cx.use_hook(|_| "Hello".to_string())
//! }
//! ```
//!
//! If you want to extend Dioxus with some new functionality, you'll probably want to implement a new hook from scratch.
//!
//! ## Putting it all together
//!
//! Using components, templates, and hooks, we can build a simple app.
//!
//! ```rust, ignore
//! use dioxus::prelude::*;
//!
//! fn main() {
//!     dioxus::desktop::launch(App);
//! }
//!
//! fn App(cx: Scope) -> Element {
//!     let mut count = use_state(&cx, || 0);
//!
//!     cx.render(rsx!(
//!         div { "Count: {count}" }
//!         button { onclick: move |_| count += 1, "Increment" }
//!         button { onclick: move |_| count -= 1, "Decrement" }
//!     ))
//! }
//! ```
//!
//! ## Features
//!
//! This overview doesn't cover everything. Make sure to check out the tutorial and reference guide on the official
//! website for more details.
//!
//! Beyond this overview, Dioxus supports:
//! - Server-side rendering
//! - Concurrent rendering (with async support)
//! - Web/Desktop/Mobile support
//! - Pre-rendering and rehydration
//! - Fragments, Portals, and Suspense
//! - Inline-styles
//! - Custom event handlers
//! - Custom elements
//! - Basic fine-grained reactivity (IE SolidJS/Svelte)
//! - and more!
//!
//! Good luck!
//!
//! ## Inspiration, Resources, Alternatives and Credits
//!
//! Dioxus is inspired by:
//! - React: for its hooks, concurrency, suspense
//! - Dodrio: for its research in bump allocation, double buffering, and diffing architecture
//!
//! Alternatives to Dioxus include:
//! - Yew: supports function components and web, but no SSR, borrowed data, or bump allocation. Rather slow at times.
//! - Percy: supports function components, web, ssr, but lacks in state management
//! - Sycamore: supports function components, web, ssr, but closer to SolidJS than React
//! - MoonZoom/Seed: opionated in the Elm model (message, update) - no hooks
//!
//! We've put a lot of work into making Dioxus ergonomic and *familiar*.
//! Our target audience is TypeSrcipt developers looking to switch to Rust for the web - so we need to be comparabale to React.

pub use dioxus_core as core;

#[cfg(feature = "hooks")]
pub use dioxus_hooks as hooks;

#[cfg(feature = "router")]
pub use dioxus_router as router;

#[cfg(feature = "ssr")]
pub use dioxus_ssr as ssr;

#[cfg(feature = "web")]
pub use dioxus_web as web;

#[cfg(feature = "desktop")]
pub use dioxus_desktop as desktop;

// #[cfg(feature = "mobile")]
// pub use dioxus_mobile as mobile;

pub mod events {
    #[cfg(feature = "html")]
    pub use dioxus_html::{on::*, KeyCode};
}

pub mod prelude {
    pub use dioxus_core::prelude::*;
    pub use dioxus_core_macro::{format_args_f, inline_props, rsx, Props, Routable};
    pub use dioxus_elements::{GlobalAttributes, SvgAttributes};
    pub use dioxus_hooks::*;
    pub use dioxus_html as dioxus_elements;
}
