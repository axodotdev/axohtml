# Changelog

## 0.5.0 - 2023-03-29

### 🛠️  Fixes

- **✨ Fix broken links and naming issues from fork transition- [adrianheine], [pr29]**

    This PR helps clean up a variety of references and links that weren't caught
    when the project transitioned from a fork of the `typed-html` crate.

### 🌿 Maintenance

- **✨ Dependency gardening - [ashleygwilliams], [pr32]/[pr33]**

    General dependency maintenance with two notable actions:

    - replace `ansi_term` with `console` to match the rest of the axo toolchain
    - update `lalrpop` to 0.19.9 (latest release) to address warning

[adrianheine]: https://github.com/adrianheine
[pr29]: https://github.com/axodotdev/axohtml/pull/29
[pr32]: https://github.com/axodotdev/axohtml/pull/32
[pr33]: https://github.com/axodotdev/axohtml/pull/33

## 0.4.1 - 2023-01-24

### 🛠️ Fixes

- **✨ Fix capitalization for Permissions-Policy meta tag- [ashleygwilliams], [pr26]**

  This PR updates the capitalization of the Permissions Policy header from
  `Permissions-policy` to `Permissions-Policy`.

[pr26]: https://github.com/axodotdev/axohtml/pull/26
[ashleygwilliams]: https://github.com/ashleygwilliams

## 0.4.0 - 2023-01-24

### 🎁 Features

- **✨ Add support for Permissions-Policy meta tag- [SaraVieira], [pr23]**

  This pr adds support for using the [`Permissions-Policy` meta tag](https://www.w3.org/TR/permissions-policy-1/) that is used for defining a set of browser APIs you do not wish your website to have.

[pr23]: https://github.com/axodotdev/axohtml/pull/23

## 0.3.0 - 2023-01-02

### 🎁 Features

- **✨ More robust `aria` type checking - [SaraVieira], [i2]/[pr12], [i3]/[pr11]**

  `aria-sort` and `aria-orientation` now offer more robust type checking following the guidelines of MDN you can see in their pages:

  - [`aria-sort`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort)
  - [`aria-orientation`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation)

[i2]: https://github.com/axodotdev/axohtml/issues/2
[pr12]: https://github.com/axodotdev/axohtml/pull/12
[i3]: https://github.com/axodotdev/axohtml/issues/3
[pr11]: https://github.com/axodotdev/axohtml/pull/11

- **✨ Add twitter SEO tag support - [SaraVieira], [pr17]**

  Add support for meta tags used for twitter cards as showed in [their docs](https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/markup)

[pr17]: https://github.com/axodotdev/axohtml/pull/17

### 🛠️ Fixes

- **✨ Data Attributes now work with more than one hyphen - [SaraVieira], [pr10]**

  Our support for `data` attributes was limited in the way that it only supported one hyphen in said attributes, well, no more, use as many hyphens as your heart pleases

[pr10]: https://github.com/axodotdev/axohtml/pull/10

- **✨ Allow `script` tags in HTML - [SaraVieira], [pr10]**

  We now allow you to add script tags in the HTML after the body as the HTML standards also allow

- **✨ Allow unescaped text in`script`- [SaraVieira], [pr14]**

  Until now we were escaping the text passed down to the `script` tag and in the `script` tag is the only place we are sure we don't want to escape that so that's fixed and you can add `script` tags with unescaped text inside

[pr14]: https://github.com/axodotdev/axohtml/pull/14

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
