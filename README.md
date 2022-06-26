# bulletindown

Smol program to convert [Markdown](https://en.wikipedia.org/wiki/Markdown) to
[BBCode](https://en.wikipedia.org/wiki/BBCode), in
[XenForo](https://en.wikipedia.org/wiki/XenForo) or
[ProBoards](https://en.wikipedia.org/wiki/ProBoards) style.

## installation

Requires [the Rust toolchain](https://rustup.rs/).

```bash
git clone 'https://codeberg.org/deer/bulletindown.git'
cd bulletindown
cargo run --release
```

## usage

```text
USAGE:
    bulletindown [OPTIONS] --dialect <DIALECT>

OPTIONS:
    -d, --dialect <DIALECT>
            The dialect/flavour of BBCode to emit.

            [possible values: xenforo, proboards]

    -e, --encoding-warnings
            Warn when emitting non-UCS-2 characters (specifically, any
            codepoints U+fffe or larger) in the XenForo dialect. Some XenForo
            implementations will discard(!) these characters, leading to broken
            output. Warnings are printed to stderr.

    -f, --footnotes
            Enable non-CommonMark (GFM) footnote syntax.

    -h, --help
            Print help information

    -i, --input <INPUT>
            A path to the input Markdown file. Defaults to stdin.

    -o, --output <OUTPUT>
            A path to the output BBCode file. Defaults to stdout.

    -s, --strikethrough
            Enable non-CommonMark (GFM) strikethrough syntax.

        --smart-punctuation
            Enable “smart punctuation”.

    -t, --tables
            Enable non-CommonMark (GFM) table syntax.

        --tasklists
            Allow non-CommonMark (GFM) tasklist syntax.

    -V, --version
            Print version information
```

## legal

[Dual-licenced](https://en.wikipedia.org/wiki/Multi-licensing) under the terms
of the [Apache Licence version
2\.0](https://www.apache.org/licenses/LICENSE-2.0) and the [Expat
Licence](https://en.wikipedia.org/wiki/MIT_License).
