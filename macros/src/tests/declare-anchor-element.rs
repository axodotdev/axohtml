pub struct Attrs_a {
    pub r#accesskey: Option<String>,
    pub r#aria_autocomplete: Option<String>,
    pub r#aria_checked: Option<crate::types::Bool>,
    pub r#aria_disabled: Option<crate::types::Bool>,
    pub r#aria_errormessage: Option<String>,
    pub r#aria_expanded: Option<crate::types::Bool>,
    pub r#aria_haspopup: Option<crate::types::Bool>,
    pub r#aria_hidden: Option<crate::types::Bool>,
    pub r#aria_invalid: Option<crate::types::Bool>,
    pub r#aria_label: Option<String>,
    pub r#aria_modal: Option<crate::types::Bool>,
    pub r#aria_multiline: Option<crate::types::Bool>,
    pub r#aria_multiselectable: Option<crate::types::Bool>,
    pub r#aria_orientation: Option<crate::types::AriaOrientation>,
    pub r#aria_placeholder: Option<String>,
    pub r#aria_pressed: Option<crate::types::Bool>,
    pub r#aria_readonly: Option<crate::types::Bool>,
    pub r#aria_required: Option<crate::types::Bool>,
    pub r#aria_selected: Option<crate::types::Bool>,
    pub r#aria_sort: Option<crate::types::AriaSort>,
    pub r#aria_valuemax: Option<isize>,
    pub r#aria_valuemin: Option<isize>,
    pub r#aria_valuenow: Option<isize>,
    pub r#aria_valuetext: Option<String>,
    pub r#autocapitalize: Option<String>,
    pub r#class: Option<crate::types::ClassList>,
    pub r#contenteditable: Option<crate::types::Bool>,
    pub r#contextmenu: Option<crate::types::Id>,
    pub r#dir: Option<crate::types::TextDirection>,
    pub r#download: Option<String>,
    pub r#draggable: Option<crate::types::Bool>,
    pub r#hidden: Option<crate::types::Bool>,
    pub r#href: Option<Uri>,
    pub r#hreflang: Option<LanguageTag>,
    pub r#id: Option<crate::types::Id>,
    pub r#is: Option<String>,
    pub r#lang: Option<crate::types::LanguageTag>,
    pub r#ping: Option<SpacedList<Uri>>,
    pub r#rel: Option<SpacedList<LinkType>>,
    pub r#role: Option<crate::types::Role>,
    pub r#style: Option<String>,
    pub r#tabindex: Option<isize>,
    pub r#target: Option<Target>,
    pub r#title: Option<String>,
    pub r#type: Option<Mime>,
}
pub struct a<T>
where
    T: crate::OutputType + Send,
{
    pub attrs: Attrs_a,
    pub data_attributes: Vec<(&'static str, String)>,
    pub aria_attributes: Vec<(&'static str, String)>,
    pub events: T::Events,
    pub children: Vec<Box<FlowContent<T>>>,
}
impl<T> a<T>
where
    T: crate::OutputType + Send,
{
    pub fn new() -> Self {
        a {
            events: T::Events::default(),
            attrs: Attrs_a {
                r#accesskey: None,
                r#aria_autocomplete: None,
                r#aria_checked: None,
                r#aria_disabled: None,
                r#aria_errormessage: None,
                r#aria_expanded: None,
                r#aria_haspopup: None,
                r#aria_hidden: None,
                r#aria_invalid: None,
                r#aria_label: None,
                r#aria_modal: None,
                r#aria_multiline: None,
                r#aria_multiselectable: None,
                r#aria_orientation: None,
                r#aria_placeholder: None,
                r#aria_pressed: None,
                r#aria_readonly: None,
                r#aria_required: None,
                r#aria_selected: None,
                r#aria_sort: None,
                r#aria_valuemax: None,
                r#aria_valuemin: None,
                r#aria_valuenow: None,
                r#aria_valuetext: None,
                r#autocapitalize: None,
                r#class: None,
                r#contenteditable: None,
                r#contextmenu: None,
                r#dir: None,
                r#download: None,
                r#draggable: None,
                r#hidden: None,
                r#href: None,
                r#hreflang: None,
                r#id: None,
                r#is: None,
                r#lang: None,
                r#ping: None,
                r#rel: None,
                r#role: None,
                r#style: None,
                r#tabindex: None,
                r#target: None,
                r#title: None,
                r#type: None,
            },
            data_attributes: Vec::new(),
            aria_attributes: Vec::new(),
            children: Vec::new(),
        }
    }
}
impl<T> crate::dom::Node<T> for a<T>
where
    T: crate::OutputType + Send,
{
    fn vnode(&'_ mut self) -> crate::dom::VNode<'_, T> {
        let mut attributes = Vec::new();
        if let Some(ref value) = self.attrs.r#accesskey {
            attributes.push(("accesskey", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_autocomplete {
            attributes.push(("aria_autocomplete", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_checked {
            attributes.push(("aria_checked", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_disabled {
            attributes.push(("aria_disabled", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_errormessage {
            attributes.push(("aria_errormessage", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_expanded {
            attributes.push(("aria_expanded", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_haspopup {
            attributes.push(("aria_haspopup", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_hidden {
            attributes.push(("aria_hidden", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_invalid {
            attributes.push(("aria_invalid", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_label {
            attributes.push(("aria_label", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_modal {
            attributes.push(("aria_modal", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_multiline {
            attributes.push(("aria_multiline", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_multiselectable {
            attributes.push(("aria_multiselectable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_orientation {
            attributes.push(("aria_orientation", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_placeholder {
            attributes.push(("aria_placeholder", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_pressed {
            attributes.push(("aria_pressed", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_readonly {
            attributes.push(("aria_readonly", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_required {
            attributes.push(("aria_required", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_selected {
            attributes.push(("aria_selected", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_sort {
            attributes.push(("aria_sort", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuemax {
            attributes.push(("aria_valuemax", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuemin {
            attributes.push(("aria_valuemin", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuenow {
            attributes.push(("aria_valuenow", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuetext {
            attributes.push(("aria_valuetext", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#autocapitalize {
            attributes.push(("autocapitalize", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#class {
            attributes.push(("class", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#contenteditable {
            attributes.push(("contenteditable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#contextmenu {
            attributes.push(("contextmenu", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#dir {
            attributes.push(("dir", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#download {
            attributes.push(("download", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#draggable {
            attributes.push(("draggable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#hidden {
            attributes.push(("hidden", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#href {
            attributes.push(("href", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#hreflang {
            attributes.push(("hreflang", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#id {
            attributes.push(("id", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#is {
            attributes.push(("is", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#lang {
            attributes.push(("lang", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#ping {
            attributes.push(("ping", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#rel {
            attributes.push(("rel", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#role {
            attributes.push(("role", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#style {
            attributes.push(("style", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#tabindex {
            attributes.push(("tabindex", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#target {
            attributes.push(("target", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#title {
            attributes.push(("title", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#type {
            attributes.push(("type", value.to_string()));
        }
        attributes.extend(self.data_attributes.clone());
        attributes.extend(self.aria_attributes.clone());
        let mut children = Vec::new();
        for child in &mut self.children {
            children.push(child.vnode());
        }
        crate::dom::VNode::Element(crate::dom::VElement {
            name: "a",
            attributes,
            events: &mut self.events,
            children,
        })
    }
}
impl<T> crate::dom::Element<T> for a<T>
where
    T: crate::OutputType + Send,
{
    fn name() -> &'static str {
        "a"
    }
    fn attribute_names() -> &'static [&'static str] {
        &[
            "accesskey",
            "aria_autocomplete",
            "aria_checked",
            "aria_disabled",
            "aria_errormessage",
            "aria_expanded",
            "aria_haspopup",
            "aria_hidden",
            "aria_invalid",
            "aria_label",
            "aria_modal",
            "aria_multiline",
            "aria_multiselectable",
            "aria_orientation",
            "aria_placeholder",
            "aria_pressed",
            "aria_readonly",
            "aria_required",
            "aria_selected",
            "aria_sort",
            "aria_valuemax",
            "aria_valuemin",
            "aria_valuenow",
            "aria_valuetext",
            "autocapitalize",
            "class",
            "contenteditable",
            "contextmenu",
            "dir",
            "download",
            "draggable",
            "hidden",
            "href",
            "hreflang",
            "id",
            "is",
            "lang",
            "ping",
            "rel",
            "role",
            "style",
            "tabindex",
            "target",
            "title",
            "type",
        ]
    }
    fn required_children() -> &'static [&'static str] {
        &[]
    }
    fn attributes(&self) -> Vec<(&'static str, String)> {
        let mut out = Vec::new();
        if let Some(ref value) = self.attrs.r#accesskey {
            out.push(("accesskey", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_autocomplete {
            out.push(("aria_autocomplete", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_checked {
            out.push(("aria_checked", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_disabled {
            out.push(("aria_disabled", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_errormessage {
            out.push(("aria_errormessage", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_expanded {
            out.push(("aria_expanded", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_haspopup {
            out.push(("aria_haspopup", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_hidden {
            out.push(("aria_hidden", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_invalid {
            out.push(("aria_invalid", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_label {
            out.push(("aria_label", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_modal {
            out.push(("aria_modal", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_multiline {
            out.push(("aria_multiline", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_multiselectable {
            out.push(("aria_multiselectable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_orientation {
            out.push(("aria_orientation", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_placeholder {
            out.push(("aria_placeholder", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_pressed {
            out.push(("aria_pressed", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_readonly {
            out.push(("aria_readonly", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_required {
            out.push(("aria_required", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_selected {
            out.push(("aria_selected", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_sort {
            out.push(("aria_sort", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuemax {
            out.push(("aria_valuemax", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuemin {
            out.push(("aria_valuemin", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuenow {
            out.push(("aria_valuenow", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#aria_valuetext {
            out.push(("aria_valuetext", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#autocapitalize {
            out.push(("autocapitalize", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#class {
            out.push(("class", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#contenteditable {
            out.push(("contenteditable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#contextmenu {
            out.push(("contextmenu", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#dir {
            out.push(("dir", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#download {
            out.push(("download", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#draggable {
            out.push(("draggable", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#hidden {
            out.push(("hidden", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#href {
            out.push(("href", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#hreflang {
            out.push(("hreflang", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#id {
            out.push(("id", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#is {
            out.push(("is", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#lang {
            out.push(("lang", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#ping {
            out.push(("ping", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#rel {
            out.push(("rel", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#role {
            out.push(("role", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#style {
            out.push(("style", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#tabindex {
            out.push(("tabindex", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#target {
            out.push(("target", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#title {
            out.push(("title", value.to_string()));
        }
        if let Some(ref value) = self.attrs.r#type {
            out.push(("type", value.to_string()));
        }
        for (key, value) in &self.data_attributes {
            out.push((key, value.to_string()));
        }
        for (key, value) in &self.aria_attributes {
            out.push((key, value.to_string()));
        }
        out
    }
}
impl<T> FlowContent<T> for a<T>
where
    T: crate::OutputType + Send,
{}
impl<T> PhrasingContent<T> for a<T>
where
    T: crate::OutputType + Send,
{}
impl<T> InteractiveContent<T> for a<T>
where
    T: crate::OutputType + Send,
{}
impl<T> std::fmt::Display for a<T>
where
    T: crate::OutputType + Send,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<{}", "a")?;
        if let Some(ref value) = self.attrs.r#accesskey {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "accesskey", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_autocomplete {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_autocomplete", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_checked {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_checked", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_disabled {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_disabled", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_errormessage {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_errormessage", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_expanded {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_expanded", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_haspopup {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_haspopup", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_hidden {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_hidden", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_invalid {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_invalid", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_label {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_label", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_modal {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_modal", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_multiline {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_multiline", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_multiselectable {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_multiselectable", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_orientation {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_orientation", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_placeholder {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_placeholder", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_pressed {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_pressed", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_readonly {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_readonly", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_required {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_required", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_selected {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_selected", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_sort {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_sort", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_valuemax {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_valuemax", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_valuemin {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_valuemin", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_valuenow {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_valuenow", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#aria_valuetext {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "aria_valuetext", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#autocapitalize {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "autocapitalize", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#class {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "class", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#contenteditable {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "contenteditable", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#contextmenu {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "contextmenu", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#dir {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "dir", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#download {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "download", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#draggable {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "draggable", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#hidden {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "hidden", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#href {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "href", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#hreflang {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "hreflang", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#id {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "id", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#is {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "is", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#lang {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "lang", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#ping {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "ping", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#rel {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "rel", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#role {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "role", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#style {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "style", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#tabindex {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "tabindex", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#target {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "target", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#title {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "title", value)?;
            }
        }
        if let Some(ref value) = self.attrs.r#type {
            let value = crate::escape_html_attribute(value.to_string());
            if !value.is_empty() {
                write!(f, " {}=\"{}\"", "type", value)?;
            }
        }
        for (key, value) in &self.data_attributes {
            write!(
                f, " data-{}=\"{}\"", key, crate ::escape_html_attribute(value
                .to_string())
            )?;
        }
        for (key, value) in &self.aria_attributes {
            write!(
                f, " aria-{}=\"{}\"", key, crate ::escape_html_attribute(value
                .to_string())
            )?;
        }
        write!(f, "{}", self.events)?;
        write!(f, ">")?;
        for child in &self.children {
            child.fmt(f)?;
        }
        write!(f, "</{}>", "a")
    }
}
impl<T> IntoIterator for a<T>
where
    T: crate::OutputType + Send,
{
    type Item = a<T>;
    type IntoIter = std::vec::IntoIter<a<T>>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}
impl<T> IntoIterator for Box<a<T>>
where
    T: crate::OutputType + Send,
{
    type Item = Box<a<T>>;
    type IntoIter = std::vec::IntoIter<Box<a<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}
