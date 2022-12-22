//! Types for all standard HTML5 elements.

#![allow(non_camel_case_types)]

use crate::dom::{Node, TextNode};
use crate::types::*;
use crate::OutputType;
use axohtml_macros::declare_elements;

// Marker traits for element content groups

macro_rules! marker_trait {
    ($trait:ident) => {
        marker_trait!($trait, Node);
    };

    ($trait:ident, $parent:ident) => {
        pub trait $trait<T: OutputType + Send>: $parent<T> {}

        impl<T> IntoIterator for Box<dyn $trait<T>>
        where
            T: OutputType + Send,
        {
            type Item = Box<dyn $trait<T>>;
            type IntoIter = std::vec::IntoIter<Box<dyn $trait<T>>>;

            fn into_iter(self) -> Self::IntoIter {
                vec![self].into_iter()
            }
        }
    };
}

marker_trait!(MetadataContent);
marker_trait!(FlowContent);
marker_trait!(SectioningContent);
marker_trait!(HeadingContent);
// Phrasing content seems to be entirely a subclass of FlowContent
marker_trait!(PhrasingContent, FlowContent);
marker_trait!(EmbeddedContent);
marker_trait!(InteractiveContent);
marker_trait!(FormContent);
marker_trait!(SvgContent);
marker_trait!(ClipPathContent);
marker_trait!(DefsContent);
marker_trait!(FilterContent);

// Traits for elements that are more picky about their children
marker_trait!(DescriptionListContent);
marker_trait!(HGroupContent);
marker_trait!(MapContent);
marker_trait!(MediaContent); // <audio> and <video>
marker_trait!(SelectContent);
marker_trait!(TableContent);
marker_trait!(TableColumnContent);

declare_elements! {
    html {
        xmlns: Uri,
    } with [head, body];
    head with [title] MetadataContent;
    body with FlowContent;

    // Metadata
    base {
        href: Uri,
        target: Target,
    } in [MetadataContent];
    link {
        as: Mime,
        crossorigin: CrossOrigin,
        href: Uri,
        hreflang: LanguageTag,
        media: String, // FIXME media query
        rel: LinkType,
        sizes: String, // FIXME
        title: String, // FIXME
        type: Mime,
    } in [MetadataContent];
    meta {
        charset: String, // FIXME IANA standard names
        content: String,
        http_equiv: HTTPEquiv,
        name: Metadata,
        // non standard, uses https://en.wikipedia.org/wiki/RDFa
        property: MetadataProperties,
    } in [MetadataContent];
    style {
        type: Mime,
        media: String, // FIXME media query
        nonce: Nonce,
        title: String, // FIXME
    } in [MetadataContent] with TextNode;
    title in [MetadataContent] with TextNode;

    // Flow
    a {
        download: String,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedList<LinkType>,
        target: Target,
        type: Mime,
    } in [FlowContent, PhrasingContent, InteractiveContent, SvgContent] with FlowContent;
    abbr in [FlowContent, PhrasingContent] with PhrasingContent;
    address in [FlowContent] with FlowContent;
    article in [FlowContent, SectioningContent] with FlowContent;
    aside in [FlowContent, SectioningContent] with FlowContent;
    audio {
        autoplay: Bool,
        controls: Bool,
        crossorigin: CrossOrigin,
        loop: Bool,
        muted: Bool,
        preload: Preload,
        src: Uri,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    b in [FlowContent, PhrasingContent] with PhrasingContent;
    bdo in [FlowContent, PhrasingContent] with PhrasingContent;
    bdi in [FlowContent, PhrasingContent] with PhrasingContent;
    blockquote {
        cite: Uri,
    } in [FlowContent] with FlowContent;
    br in [FlowContent, PhrasingContent];
    button {
        autofocus: Bool,
        disabled: Bool,
        form: Id,
        formaction: Uri,
        formenctype: FormEncodingType,
        formmethod: FormMethod,
        formnovalidate: Bool,
        formtarget: Target,
        name: Id,
        type: ButtonType,
        value: String,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    canvas {
        height: usize,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with FlowContent;
    cite in [FlowContent, PhrasingContent] with PhrasingContent;
    code in [FlowContent, PhrasingContent] with PhrasingContent;
    data {
        value: String,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    datalist in [FlowContent, PhrasingContent] with option;
    del {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    details {
        open: Bool,
    } in [FlowContent, SectioningContent, InteractiveContent] with [summary] FlowContent;
    dfn in [FlowContent, PhrasingContent] with PhrasingContent;
    div in [FlowContent] with FlowContent;
    dl in [FlowContent] with DescriptionListContent;
    em in [FlowContent, PhrasingContent] with PhrasingContent;
    embed {
        height: usize,
        src: Uri,
        type: Mime,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent];
    // FIXME the legend attribute should be optional
    fieldset in [FlowContent, SectioningContent, FormContent] with [legend] FlowContent;
    // FIXME the figcaption attribute should be optional
    figure in [FlowContent, SectioningContent] with [figcaption] FlowContent;
    footer in [FlowContent] with FlowContent;
    form {
        accept-charset: SpacedList<CharacterEncoding>,
        action: Uri,
        autocomplete: OnOff,
        enctype: FormEncodingType,
        method: FormMethod,
        name: Id,
        novalidate: Bool,
        target: Target,
    } in [FlowContent] with FlowContent;
    h1 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h2 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h3 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h4 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h5 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    h6 in [FlowContent, HeadingContent, HGroupContent] with PhrasingContent;
    header in [FlowContent] with FlowContent;
    hgroup in [FlowContent, HeadingContent] with HGroupContent;
    hr in [FlowContent];
    i in [FlowContent, PhrasingContent] with PhrasingContent;
    iframe {
        allow: FeaturePolicy,
        allowfullscreen: Bool,
        allowpaymentrequest: Bool,
        height: usize,
        name: Id,
        referrerpolicy: ReferrerPolicy,
        sandbox: SpacedSet<Sandbox>,
        src: Uri,
        srcdoc: Uri,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent] with FlowContent;
    img {
        alt: String,
        crossorigin: CrossOrigin,
        decoding: ImageDecoding,
        height: usize,
        ismap: Bool,
        sizes: SpacedList<String>, // FIXME it's not really just a string
        src: Uri,
        srcset: String, // FIXME this is much more complicated
        usemap: String, // FIXME should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent];
    input {
        accept: String,
        alt: String,
        autocomplete: String,
        autofocus: Bool,
        capture: String,
        checked: Bool,
        disabled: Bool,
        form: Id,
        formaction: Uri,
        formenctype: FormEncodingType,
        formmethod: FormDialogMethod,
        formnovalidate: Bool,
        formtarget: Target,
        height: isize,
        list: Id,
        max: String,
        maxlength: usize,
        min: String,
        minlength: usize,
        multiple: Bool,
        name: Id,
        pattern: String,
        placeholder: String,
        readonly: Bool,
        required: Bool,
        size: usize,
        spellcheck: Bool,
        src: Uri,
        step: String,
        tabindex: usize,
        type: InputType,
        value: String,
        width: isize,
    } in [FlowContent, FormContent, PhrasingContent];
    ins {
        cite: Uri,
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with FlowContent;
    kbd in [FlowContent, PhrasingContent] with PhrasingContent;
    label {
        for: Id,
        form: Id,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with PhrasingContent;
    main in [FlowContent] with FlowContent;
    map {
        name: Id,
    } in [FlowContent, PhrasingContent] with MapContent;
    mark in [FlowContent, PhrasingContent] with PhrasingContent;
    // TODO the <math> element
    meter {
        value: isize,
        min: isize,
        max: isize,
        low: isize,
        high: isize,
        optimum: isize,
        form: Id,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    nav in [FlowContent, SectioningContent] with FlowContent;
    noscript in [MetadataContent, FlowContent, PhrasingContent] with Node;
    object {
        data: Uri,
        form: Id,
        height: usize,
        name: Id,
        type: Mime,
        typemustmatch: Bool,
        usemap: String, // TODO should be a fragment starting with '#'
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent, InteractiveContent, FormContent] with param;
    ol {
        reversed: Bool,
        start: isize,
        type: OrderedListType,
    } in [FlowContent] with li;
    output {
        for: SpacedSet<Id>,
        form: Id,
        name: Id,
    } in [FlowContent, PhrasingContent, FormContent] with PhrasingContent;
    p in [FlowContent] with PhrasingContent;
    pre in [FlowContent] with PhrasingContent;
    progress {
        max: f64,
        value: f64,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    q {
        cite: Uri,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ruby in [FlowContent, PhrasingContent] with PhrasingContent;
    s in [FlowContent, PhrasingContent] with PhrasingContent;
    samp in [FlowContent, PhrasingContent] with PhrasingContent;
    script {
        async: Bool,
        crossorigin: CrossOrigin,
        defer: Bool,
        integrity: Integrity,
        nomodule: Bool,
        nonce: Nonce,
        src: Uri,
        text: String,
        type: String, // TODO could be an enum
    } in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with TextNode;
    section in [FlowContent, SectioningContent] with FlowContent;
    select {
        autocomplete: String,
        autofocus: Bool,
        disabled: Bool,
        form: Id,
        multiple: Bool,
        name: Id,
        required: Bool,
        size: usize,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with SelectContent;
    small in [FlowContent, PhrasingContent] with PhrasingContent;
    span in [FlowContent, PhrasingContent] with PhrasingContent;
    strong in [FlowContent, PhrasingContent] with PhrasingContent;
    sub in [FlowContent, PhrasingContent] with PhrasingContent;
    sup in [FlowContent, PhrasingContent] with PhrasingContent;
    table in [FlowContent] with TableContent;
    template in [MetadataContent, FlowContent, PhrasingContent, TableColumnContent] with Node;
    textarea {
        autocomplete: OnOff,
        autofocus: Bool,
        cols: usize,
        disabled: Bool,
        form: Id,
        maxlength: usize,
        minlength: usize,
        name: Id,
        placeholder: String,
        readonly: Bool,
        required: Bool,
        rows: usize,
        spellcheck: BoolOrDefault,
        wrap: Wrap,
    } in [FlowContent, PhrasingContent, InteractiveContent, FormContent] with TextNode;
    time {
        datetime: Datetime,
    } in [FlowContent, PhrasingContent] with PhrasingContent;
    ul in [FlowContent] with li;
    var in [FlowContent, PhrasingContent] with PhrasingContent;
    video {
        autoplay: Bool,
        controls: Bool,
        crossorigin: CrossOrigin,
        height: usize,
        loop: Bool,
        muted: Bool,
        preload: Preload,
        playsinline: Bool,
        poster: Uri,
        src: Uri,
        width: usize,
    } in [FlowContent, PhrasingContent, EmbeddedContent] with MediaContent;
    wbr in [FlowContent, PhrasingContent];

    // Non-group elements
    area {
        alt: String,
        coords: String, // TODO could perhaps be validated
        download: Bool,
        href: Uri,
        hreflang: LanguageTag,
        ping: SpacedList<Uri>,
        rel: SpacedSet<LinkType>,
        shape: AreaShape,
        target: Target,
    } in [MapContent];
    caption in [TableContent] with FlowContent;
    col {
        span: usize,
    };
    colgroup {
        span: usize,
    } in [TableContent] with col;
    dd in [DescriptionListContent] with FlowContent;
    dt in [DescriptionListContent] with FlowContent;
    figcaption with FlowContent;
    legend with PhrasingContent;
    li {
        value: isize,
    } with FlowContent;
    option {
        disabled: Bool,
        label: String,
        selected: Bool,
        value: String,
    } in [SelectContent] with TextNode;
    optgroup {
        disabled: Bool,
        label: String,
    } in [SelectContent] with option;
    param {
        name: String,
        value: String,
    };
    source {
        src: Uri,
        type: Mime,
    } in [MediaContent];
    summary with PhrasingContent;
    tbody in [TableContent] with tr;
    td {
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
    } in [TableColumnContent] with FlowContent;
    tfoot in [TableContent] with tr;
    th {
        abbr: String,
        colspan: usize,
        headers: SpacedSet<Id>,
        rowspan: usize,
        scope: TableHeaderScope,
    } in [TableColumnContent] with FlowContent;
    thead in [TableContent] with tr;
    tr in [TableContent] with TableColumnContent;
    track {
        default: Bool,
        kind: VideoKind,
        label: String,
        src: Uri,
        srclang: LanguageTag,
    } in [MediaContent];

    svg {
        height: String,
        width: String,
        preserveAspectRatio: PreserveAspectRatio,
        viewBox: String,
        x: String,
        y: String,
        xmlns: String,
        clip-path: String,
        clip-rule: String,
        color: String,
        color-interpolation: String,
        color-rendering: String,
        cursor: String,
        display: String,
        fill: String,
        fill-opacity: String,
        fill-rule: String,
        filter: String,
        mask: String,
        opacity: String,
        pointer-events: String,
        shape-rendering: String,
        stroke: String,
        stroke-dasharray: String,
        stroke-dashoffset: String,
        stroke-linecap: String,
        stroke-linejoin: String,
        stroke-miterlimit: String,
        stroke-opacity: String,
        stroke-width: String,
        transform: String,
        vector-effect: String,
        visibility: String
    } in [FlowContent] with SvgContent;
    path {
        d: String,
        pathLength: usize,
    } in [SvgContent, ClipPathContent, DefsContent];
    circle {
        cx: String,
        pathLength: usize,
        cy: String,
        r: String
    } in [SvgContent, ClipPathContent, DefsContent];
    clipPath {
        clipPathUnits: ClipPathIUnits,
    } in [SvgContent] with ClipPathContent;
    defs in [SvgContent] with DefsContent;
    desc in [SvgContent] with PhrasingContent;

    ellipse {
        cx: String,
        pathLength: usize,
        cy: String,
        rx: String,
        ry: String
    } in [SvgContent, ClipPathContent, DefsContent];
    feBlend {
        _in: String, // TODO OMG FUCK THIS
        in2: usize,
        mode: Blends
    } in [SvgContent, ClipPathContent, DefsContent, FilterContent];
    filter {
        x: String,
        y: String,
        width: String,
        height: String,
        filterUnits: ClipPathIUnits,
        primitiveUnits: ClipPathIUnits,
    } in [SvgContent, ClipPathContent, DefsContent] with FilterContent;
    // Don't @ me
    blink in [FlowContent, PhrasingContent] with PhrasingContent;
    marquee {
        behavior: String, // FIXME enum
        bgcolor: String, // FIXME colour
        direction: String, // FIXME direction enum
        height: String, // FIXME size
        hspace: String, // FIXME size
        loop: isize,
        scrollamount: usize,
        scrolldelay: usize,
        truespeed: Bool,
        vspace: String, // FIXME size
        width: String, // FIXME size
    } in [FlowContent, PhrasingContent] with PhrasingContent;
}

#[test]
fn test_data_attributes() {
    use crate as axohtml;
    use crate::{dom::DOMTree, html};

    let frag: DOMTree<String> = html!(<div data-id="1234">"Boo!"</div>);

    assert_eq!("<div data-id=\"1234\">Boo!</div>", frag.to_string());
}
#[test]
fn test_meta_tags() {
    use crate as axohtml;
    use crate::{dom::DOMTree, html};

    let frag: DOMTree<String> = html!(<meta property="og:url" content="http://example.com"/>
    );

    assert_eq!(
        "<meta content=\"http://example.com\" property=\"og:url\"/>",
        frag.to_string()
    );
}

#[test]
fn test_aria() {
    use crate as axohtml;
    use crate::{dom::DOMTree, html};

    let frag: DOMTree<String> = html!(<div aria_hidden="true" aria_label="hello" />
    );

    assert_eq!(
        "<div aria-hidden=\"true\" aria-label=\"hello\"></div>",
        frag.to_string()
    );
}

#[test]
fn test_svg() {
    use crate as axohtml;
    use crate::{dom::DOMTree, html};

    let frag: DOMTree<String> = html!(
    <svg width="29.018px" height="29.018px" viewBox="0 -0.59 29.018 29.018" id="_25_-_Star" xmlns="http://www.w3.org/2000/svg">
    <path id="_25_-_Star-2" d="M13.645,4.01l-2.057,6.334a1.013,1.013,0,0,1-.962.7H3.967a2.475,2.475,0,0,0-1.456,4.478L7.9,19.435a1.011,1.011,0,0,1,.367,1.131L6.208,26.9a2.476,2.476,0,0,0,3.81,2.768l5.388-3.914a1.012,1.012,0,0,1,1.188,0l5.388,3.914a2.476,2.476,0,0,0,3.81-2.768l-2.058-6.333a1.011,1.011,0,0,1,.367-1.131l5.388-3.914a2.475,2.475,0,0,0-1.456-4.478H21.374a1.013,1.013,0,0,1-.962-.7L18.355,4.01a2.477,2.477,0,0,0-4.71,0Zm1.9.618a.475.475,0,0,1,.9,0l2.058,6.334a3.012,3.012,0,0,0,2.864,2.081h6.659a.475.475,0,0,1,.28.86l-5.387,3.914a3.011,3.011,0,0,0-1.094,3.367l2.058,6.333a.476.476,0,0,1-.733.532L17.77,24.135a3.011,3.011,0,0,0-3.54,0L8.843,28.049a.476.476,0,0,1-.733-.532l2.058-6.333a3.011,3.011,0,0,0-1.094-3.367L3.687,13.9a.475.475,0,0,1,.28-.86h6.659a3.012,3.012,0,0,0,2.864-2.081l2.058-6.334Z" transform="translate(-1.491 -2.3)" fill-rule="evenodd"/>
    </svg>
    );

    assert_eq!(
        "<div aria-hidden=\"true\" aria-label=\"hello\"></div>",
        frag.to_string()
    );
}
