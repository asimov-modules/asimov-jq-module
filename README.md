# ASIMOV JQ Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-jq-module.svg)](https://crates.io/crates/asimov-jq-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-jq-module.svg)](https://pypi.org/project/asimov-jq-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-jq-module.svg)](https://rubygems.org/gems/asimov-jq-module)

[ASIMOV] module for JSON transformation using the [jq] filter language.

## ✨ Features

- Transforms JSON inputs using the [jq] programming language.
- Supports values from environment variables using the `env.USER` syntax.
- Loads environment variables from `.env` (aka dotenv) files.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U asimov-jq-module
```

### Installation from RubyGems

```bash
gem install asimov-jq-module
```

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
[Python]: https://python.org
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
[jq]: https://en.wikipedia.org/wiki/Jq_(programming_language)
