#![recursion_limit = "128"]
//! This crate provides the `html!` macro for building fully type checked HTML
//! documents inside your Rust code using roughly [JSX] compatible syntax.
//!
//! This crate is a fork of the great [Bodil Stokke's] [typed-html] crate. Opted
//! for a fork instead of maintainership because not currently intending to use or
//! maintain the Wasm compatibility (for now).
//!
//! [Bodil Stokke's]: https://github.com/bodil
//! [typed-html]: https://github.com/bodil/typed-html
//!
//! # Quick Preview
//!
//! ```
//! # #![recursion_limit = "128"]
//! # use axohtml::html;
//! # use axohtml::dom::{DOMTree, VNode};
//! # use axohtml::types::Metadata;
//! # fn main() {
//! let mut doc: DOMTree<String> = html!(
//!     <html>
//!         <head>
//!             <title>"Hello Axo"</title>
//!             <meta name=Metadata::Author content="Axo Developer Co."/>
//!         </head>
//!         <body>
//!             <h1>">o_o<"</h1>
//!             <p class="official">
//!                 "The tool company for tool companies"
//!             </p>
//!             { (0..3).map(|_| html!(
//!                 <p class="emphasis">
//!                     ">o_o<"
//!                 </p>
//!             )) }
//!             <p class="citation-needed">
//!                 "Every company should be a developer experience company."
//!             </p>
//!         </body>
//!     </html>
//! );
//! let doc_str = doc.to_string();
//! # }
//! ```
//!
//! # Syntax
//!
//! This macro largely follows [JSX] syntax, but with some differences:
//!
//! * Text nodes must be quoted, because there's only so much Rust's tokeniser can
//!   handle outside string literals. So, instead of `<p>Hello</p>`, you need to
//!   write `<p>"Hello"</p>`. (The parser will throw an error asking you to do this
//!   if you forget.)
//! * Element attributes will accept simple Rust expressions, but the parser has
//!   its limits, as it's not a full Rust parser. You can use literals,
//!   variables, dotted properties, type constructors and single function or
//!   method calls. If you use something the parser isn't currently capable of
//!   handling, it will complain. You can put braces or parentheses around the
//!   expression if the parser doesn't understand
//!   it. You can use any Rust code inside a brace or parenthesis block.
//!
//! # Valid HTML5
//!
//! The macro will only accept valid HTML5 tags, with no tags or attributes marked
//! experimental or obsolete. If it won't accept something you want it to accept, we
//! can discuss it over a pull request (experimental tags and attributes, in
//! particular, are mostly omitted just for brevity, and you're welcome to implement
//! them).
//!
//! The structure validation is simplistic by necessity, as it defers to the type
//! system: a few elements will have one or more required children, and any element
//! which accepts children will have a restriction on the type of the children,
//! usually a broad group as defined by the HTML spec. Many elements have
//! restrictions on children of children, or require a particular ordering of
//! optional elements, which isn't currently validated.
//!
//! # Attribute Values
//!
//! Brace blocks in the attribute value position should return the expected type for
//! the attribute. The type checker will complain if you return an unsupported type.
//! You can also use literals or a few simple Rust expressions as attribute values
//! (see the Syntax section above).
//!
//! The `html!` macro will add an [`.into()`][Into::into] call to the value
//! expression, so that you can use any type that has an [`Into<A>`] trait
//! defined for the actual attribute type `A`.
//!
//! As a special case, if you use a string literal, the macro will instead use the
//! [`FromStr<A>`][std::str::FromStr] trait to try and parse the string literal into the
//! expected type. This is extremely useful for eg. CSS classes, letting you type
//! `class="css-class-1 css-class-2"` instead of going to the trouble of
//! constructing a [`SpacedSet<Class>`][types::SpacedSet]. The big caveat for this:
//! currently, the macro is not able to validate the string at compile time, and the
//! conversion will panic at runtime if the string is invalid.
//!
//! ## Example
//!
//! ```
//! # use std::convert::{TryFrom, TryInto};
//! # use axohtml::html;
//! # use axohtml::dom::DOMTree;
//! # use axohtml::types::{Class, SpacedSet};
//! # fn main() -> Result<(), &'static str> {
//! let classList: SpacedSet<Class> = ["foo", "bar", "baz"].try_into()?;
//! # let doc: DOMTree<String> =
//! html!(
//!     <div>
//!         <div class="foo bar baz" />         // parses a string literal
//!         <div class=["foo", "bar", "baz"] /> // uses From<[&str, &str, &str]>
//!         <div class=classList />             // uses a variable in scope
//!         <div class={                        // evaluates a code block
//!             SpacedSet::try_from(["foo", "bar", "baz"])?
//!         } />
//!     </div>
//! )
//! # ; Ok(()) }
//! ```
//!
//! # Generated Nodes
//!
//! Brace blocks in the child node position are expected to return an
//! [`IntoIterator`] of [`DOMTree`][dom::DOMTree]s. You can return single
//! elements or text nodes, as they both implement `IntoIterator` for themselves.
//! The macro will consume this iterator at runtime and insert the generated nodes
//! as children in the expected position.
//!
//! ## Example
//!
//! ```
//! # use axohtml::{html, text};
//! # use axohtml::dom::DOMTree;
//! # fn main() {
//! # let doc: DOMTree<String> =
//! html!(
//!     <ul>
//!         { (1..=5).map(|i| html!(
//!             <li>{ text!("{}", i) }</li>
//!         )) }
//!     </ul>
//! )
//! # ;}
//! ```
//!
//! # Rendering
//!
//! You have two options for actually producing something useful from the DOM tree
//! that comes out of the macro.
//!
//! ## Render to a string
//!
//! The DOM tree data structure implements [`Display`], so you can call
//! [`to_string()`][std::string::ToString::to_string] on it to render it to a [`String`]. If you
//! plan to do this, the type of the tree should be [`DOMTree<String>`][dom::DOMTree] to
//! ensure you're not using any event handlers that can't be printed.
//!
//! ```
//! # use axohtml::html;
//! # use axohtml::dom::DOMTree;
//! # fn main() {
//! let doc: DOMTree<String> = html!(
//!     <p>"Hello Axo"</p>
//! );
//! let doc_str = doc.to_string();
//! assert_eq!("<p>Hello Axo</p>", doc_str);
//! # }
//! ```
//!
//! ## Render to a virtual DOM
//!
//! The DOM tree structure also implements a method called `vnode()`, which renders
//! the tree to a tree of [`VNode`][dom::VNode]s, which is a mirror of the generated tree
//! with every attribute value rendered into `String`s. You can walk this virtual
//! DOM tree and pass it on to your favourite virtual DOM system.
//!
//! # License
//!
//! Copyright 2018 Bodil Stokke, 2022 Axo Developer Co.
//!
//! This software is subject to the terms of the Mozilla Public License, v. 2.0. If
//! a copy of the MPL was not distributed with this file, You can obtain one at
//! <http://mozilla.org/MPL/2.0/>.
//!
//! [JSX]: https://reactjs.org/docs/introducing-jsx.html

pub extern crate htmlescape;

use std::fmt::Display;

pub use axohtml_macros::html;

pub mod dom;
pub mod elements;
pub mod events;
pub mod types;

/// Marker trait for outputs
pub trait OutputType {
    /// The type that contains events for this output.
    type Events: Default + Display + Send;
    /// The type of event targets for this output.
    type EventTarget: Send;
    /// The type that's returned from attaching an event listener to a target.
    type EventListenerHandle: Send;
}

/// String output
impl OutputType for String {
    type Events = events::Events<String>;
    type EventTarget = ();
    type EventListenerHandle = ();
}

pub fn escape_html_attribute(html_attr: String) -> String {
    // Even though the code is quoting the variables with a double quote, escape all known quoting chars
    html_attr
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
        .replace('`', "&#96;")
}
