use proc_macro2::{Ident, Span, TokenStream};

use crate::map::StringyMap;

pub fn required_children(element: &str) -> &[&str] {
    match element {
        "html" => &["head", "body"],
        "head" => &["title"],
        _ => &[],
    }
}

pub fn global_attrs(span: Span) -> StringyMap<Ident, TokenStream> {
    let mut attrs = StringyMap::new();
    {
        let mut insert =
            |key, value: &str| attrs.insert(Ident::new(key, span), value.parse().unwrap());

        insert("id", "crate::types::Id");
        insert("class", "crate::types::ClassList");

        insert("accesskey", "String");
        insert("autocapitalize", "String");
        insert("contenteditable", "crate::types::Bool");
        insert("contextmenu", "crate::types::Id");
        insert("dir", "crate::types::TextDirection");
        insert("draggable", "crate::types::Bool");
        insert("hidden", "crate::types::Bool");
        insert("is", "String");
        insert("lang", "crate::types::LanguageTag");
        insert("role", "crate::types::Role");
        insert("style", "String");
        insert("tabindex", "isize");
        insert("title", "String");

        // ARIA
        insert("aria_autocomplete", "String");
        insert("aria_checked", "crate::types::Bool");
        insert("aria_disabled", "crate::types::Bool");
        insert("aria_errormessage", "String");
        insert("aria_expanded", "crate::types::Bool");
        insert("aria_haspopup", "crate::types::Bool");
        insert("aria_hidden", "crate::types::Bool");
        insert("aria_invalid", "crate::types::Bool");
        insert("aria_label", "String");
        insert("aria_modal", "crate::types::Bool");
        insert("aria_multiline", "crate::types::Bool");
        insert("aria_multiselectable", "crate::types::Bool");
        insert("aria_orientation", "String"); // TODO Only supports some values https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation
        insert("aria_placeholder", "String");
        insert("aria_pressed", "crate::types::Bool");
        insert("aria_readonly", "crate::types::Bool");
        insert("aria_required", "crate::types::Bool");
        insert("aria_selected", "crate::types::Bool");
        insert("aria_placeholder", "String");
        insert("aria_sort", "String"); // TODO only supports some values
        insert("aria_valuemax", "isize");
        insert("aria_valuemin", "isize");
        insert("aria_valuenow", "isize");
        insert("aria_valuetext", "String");

        // FIXME XML attrs missing
    }
    attrs
}

pub static SELF_CLOSING: &[&str] = &[
    "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link",
    "meta", "param", "source", "track", "wbr",
];
