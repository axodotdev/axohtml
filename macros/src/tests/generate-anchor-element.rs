fn html() {
    let mut element = axohtml::elements::a::new();
    element
        .attrs
        .r#href = Some(
        "https://www.w3schools.com"
            .parse()
            .unwrap_or_else(|err| {
                eprintln!(
                    "ERROR: <a href=\"https://www.w3schools.com\"> failed to parse attribute value: {}\nERROR: rebuild with nightly to print source location",
                    err
                );
                panic!("failed to parse string literal");
            }),
    );
    element
        .children
        .push(Box::new(axohtml::dom::TextNode::new("Visit W3Schools.com!".to_string())));
    Box::new(element)
}
