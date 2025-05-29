# ASIMOV JQ Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-jq-module)](https://crates.io/crates/asimov-jq-module)

[ASIMOV] module for JSON transformation using the [jq] filter language.

## ✨ Features

- Transforms JSON inputs using the [jq] programming language.
- Supports values from environment variables using the `env.USER` syntax.
- Loads environment variables from `.env` (aka dotenv) files.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-jq-module
```

## 👉 Examples

### Transforming JSON Input

```bash
asimov-jq-runner filter.jq < input.json > output.json
```

## 📚 Reference

### Installed Binaries

- `asimov-jq-runner`: filters JSON from standard input to standard output

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-jq-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-jq-module&text=asimov-jq-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-jq-module&title=asimov-jq-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-jq-module&t=asimov-jq-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-jq-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-jq-module)

[ASIMOV]: https://github.com/asimov-platform
[jq]: https://en.wikipedia.org/wiki/Jq_(programming_language)
