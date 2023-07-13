fn main() {
    println!("This crate is not meant to be run, it only serves as a compilation test.");
}

#[test]
fn ui() {
    extern crate version_check;

    if !version_check::is_feature_flaggable().unwrap_or(false) {
        return;
    }

    extern crate compiletest_rs as compiletest;

    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from("cases"),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

#[test]
fn test_subtree() {
    use axohtml::html;
    use axohtml::dom::DOMTree;

    let output: DOMTree<String> = html!(
        <h1>"This is HTML"</h1>
    );

    assert_eq!(
        output.to_string(),
        "<h1>This is HTML</h1>"
    )
}

#[test]
fn test_button_onclick() {
    use axohtml::html;
    use axohtml::dom::DOMTree;

    let output: DOMTree<String> = html!(
        <button onclick="alert(1)">"Click me"</button>
        : String
    );

    assert_eq!(
        output.to_string(),
        "<button onclick=\"alert&#x28;1&#x29;\">Click me</button>"
    )
}

#[test]
fn test_flowcontent_function() {
    use axohtml::html;
    use axohtml::dom::DOMTree;
    use axohtml::elements::FlowContent;

    fn content() -> Box<dyn FlowContent<String>> {
        return html!(
            <h1>
                "This is HTML"
            </h1>
        )
    }

    let output: DOMTree<String> = html!(
        <div>
            { content() }
        </div>
    );

    assert_eq!(
        output.to_string(),
        "<div><h1>This is HTML</h1></div>"
    )
}

#[test]
fn test_phrasingcontent_function() {
    use axohtml::{html, text};
    use axohtml::dom::DOMTree;
    use axohtml::elements::PhrasingContent;

    fn italicized(content: &str) -> Box<dyn PhrasingContent<String>> {
        return html!(
            <i>
                { text!(content) }
            </i>
        )
    }

    let output: DOMTree<String> = html!(
        <p>
            "This is "
            { italicized("HTML") }
        </p>
    );

    assert_eq!(
        output.to_string(),
        "<p>This is <i>HTML</i></p>"
    )
}

#[test]
fn test_template_function() {
    use axohtml::html;
    use axohtml::dom::DOMTree;
    use axohtml::elements::FlowContent;

    fn layout(body: impl FnOnce() -> Box<dyn FlowContent<String>>) -> DOMTree<String> {
        return html!(
            <section>
                <h1>"Header"</h1>
                { body() }
            </section>
        )
    }

    let output: DOMTree<String> = layout(|| {html!(
        <div>
            <h2>"Subheading"</h2>
            <p>"This is content!"</p>
        </div>
    )});

    assert_eq!(
        output.to_string(),
        "<section><h1>Header</h1><div><h2>Subheading</h2><p>This is content!</p></div></section>"
    )
}

#[test]
fn test_inline_script() {
    use axohtml::{html, unsafe_text};
    use axohtml::dom::DOMTree;

    let output: DOMTree<String> = html!(
        <script>
            {unsafe_text!(r#"alert(1);"#)}
        </script>
    );

    assert_eq!(
        output.to_string(),
        "<script>alert(1);</script>"
    )
}
