# Changelog

## 0.2.0 - 2022-12-19

### 🎁 Features

- **✨ New Attribute - `aria`** - [SaraVieira]

    [`aria` attributes] are critical to making the web more accessible to
    everyone, but most importantly, people with disabilities. These were a to-do
    item from the original project, and so we to-did them. At least most of
    them. There are a [few open issues] if you'd like to help us complete the
    implementation.

[`aria` attributes]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA
[few open issues]: https://github.com/axodotdev/axohtml/issues?q=is%3Aissue+is%3Aopen+aria

- **✨ New Attribute - `meta:property`** - [SaraVieira]

    If you ask the internet why `meta` tags have a `property` attribute that 
    isn't in the spec, you won't get great answers. Although not formally
    specified in HTML5, `property` attributes in `meta` tags are important for
    SEO and [the Open Graph Protocol]. They *are* documented in [RDFa] which is
    a formal W3C recommendation.

    It is outside the scope of this project to standardize standards bodies. We
    needed to support the `property` attribute, and so we did.

[SaraVieira]: https://github.com/SaraVieira
[the Open Graph Protocol]: https://ogp.me/
[RDFa]: https://en.wikipedia.org/wiki/RDFa

## 0.1.0 - 2022-12-16

Forked project, removed `dodrio` and `stdweb` features; initial release.
