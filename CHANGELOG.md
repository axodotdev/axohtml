# Changelog

## 0.3.0 - 2023-01-02

### 🎁 Features

- **✨ More robust `aria` type checking - [SaraVieira], [I2]/[PR12], [I3]/[PR11]**

  `aria-sort` and `aria-orientation` now offer more robust type checking following the guidelines of MDN you can see in their pages:

  - [`aria-sort`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort)
  - [`aria-orientation`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation)

[I2]: https://github.com/axodotdev/axohtml/issues/2
[PR12]: https://github.com/axodotdev/axohtml/pull/12
[I3]: https://github.com/axodotdev/axohtml/issues/3
[PR11]: https://github.com/axodotdev/axohtml/pull/11

- **✨ Add twitter SEO tag support - [SaraVieira], [PR17]**

  Add support for meta tags used for twitter cards as showed in [their docs](https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/markup)

[PR17]: https://github.com/axodotdev/axohtml/pull/17

### 🛠️ Fixes

- **✨ Data Attributes now work with more than one hyphen - [SaraVieira], [PR10]**

  Our support for `data` attributes was limited in the way that it only supported one hyphen in said attributes, well, no more, use as many hyphens as your heart pleases

[PR10]: https://github.com/axodotdev/axohtml/pull/10

- **✨ Allow `script` tags in HTML - [SaraVieira], [PR10]**

  We now allow you to add script tags in the HTML after the body as the HTML standards also allow

- **✨ Allow unescaped text in`script`- [SaraVieira], [PR14]**

  Until now we were escaping the text passed down to the `script` tag and in the `script` tag is the only place we are sure we don't want to escape that so that's fixed and you can add `script` tags with unescaped text inside

[PR14]: https://github.com/axodotdev/axohtml/pull/14

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
  SEO and [the Open Graph Protocol]. They _are_ documented in [RDFa] which is
  a formal W3C recommendation.

  It is outside the scope of this project to standardize standards bodies. We
  needed to support the `property` attribute, and so we did.

[saravieira]: https://github.com/SaraVieira
[the open graph protocol]: https://ogp.me/
[rdfa]: https://en.wikipedia.org/wiki/RDFa

## 0.1.0 - 2022-12-16

Forked project, removed `dodrio` and `stdweb` features; initial release.
