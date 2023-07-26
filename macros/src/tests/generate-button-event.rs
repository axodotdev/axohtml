fn html() {
    let mut element: axohtml::elements::button<String> = axohtml::elements::button::new();
    element.children.push(Box::new(axohtml::dom::TextNode::new("Click me".to_string())));
    element.events.r#click = Some("alert(1)".into());
    Box::new(element)
}
