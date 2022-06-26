use crate::args::Dialect;
use anyhow::{anyhow, bail, Result};
use html_escape::decode_html_entities_to_string;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

static MULTILINE_SUMMARY: &str =
    "A `<summary>` element (including its contents) must be all on a single \
line";

pub fn convert<S: AsRef<str>>(
    input: S,
    dialect: Dialect,
    encoding_warnings: bool,
    markdown_opts: Options,
) -> Result<String> {
    let input = input.as_ref();

    // Set up the Markdown parser.
    let parser = Parser::new_ext(input, markdown_opts);

    // We record BBCode output into this in-memory buffer.
    let mut output = String::with_capacity(input.len());

    // This state machine just iterates through the events pulled from the
    // Markdown parser.
    //
    // `start_li` is a bit of state required to handle items in (ordered or
    // unordered) lists, so that we can emit both `[li]` _and_ `[/li]`, if the
    // BBCode dialect requires this. `start_fn` is a similar thing for footnote
    // definitions.
    let mut start_li = false;
    let mut start_fn = false;
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    if start_li || start_fn {
                        start_li = false;
                        start_fn = false;
                    } else {
                        output.push('\n')
                    }
                }
                // We can ignore the fragment ID and the element’s classes,
                // respectively; BBCode will have nothing to do with such
                // information.
                Tag::Heading(lvl, _, _) => {
                    // We emulate actual headers (`<h1>`, `<h2>`, etc.) by
                    // increasing font size, making the text bold, and
                    // underlining the text.
                    output.push_str(match dialect {
                        Dialect::Xenforo => "\n[size=\"",
                        Dialect::Proboards => "\n\n[font size=\"",
                    });
                    output.push(match lvl {
                        HeadingLevel::H1 => '7',
                        HeadingLevel::H2 => '6',
                        HeadingLevel::H3 => '5',
                        HeadingLevel::H4 => '4',
                        _ => '3',
                    });
                    output.push_str("\"][b][u]");
                }
                Tag::BlockQuote => output.push_str(match dialect {
                    Dialect::Xenforo => "[quote]",
                    Dialect::Proboards => "[blockquote]",
                }),
                // We ignore the specified code language, if any.
                Tag::CodeBlock(_) => output.push_str(match dialect {
                    Dialect::Xenforo => "[code]",
                    Dialect::Proboards => "\n[pre]",
                }),
                Tag::List(ord) => output.push_str(if ord.is_some() {
                    // It might seem weird that we don’t inspect the value
                    // inside of `ord`, but AFAIK, no BBCode implementations
                    // properly implement a “starting number” for `<ol>`s.
                    match dialect {
                        Dialect::Xenforo => "[list=1]",
                        Dialect::Proboards => "\n[ol]",
                    }
                } else {
                    match dialect {
                        Dialect::Xenforo => "[list]",
                        Dialect::Proboards => "\n[ul]",
                    }
                }),
                Tag::Item => {
                    start_li = true;
                    output.push_str(match dialect {
                        Dialect::Xenforo => "\n[*]",
                        Dialect::Proboards => "\n[li]",
                    });
                }
                Tag::FootnoteDefinition(fnid) => {
                    start_fn = true;

                    // We do our best to emulate a footnote definition...
                    output.push_str("\n\u{231c}"); // ⌜
                    output.push_str(&fnid);
                    output.push_str("\u{231d}: ");
                }
                // We ignore alignment indicators for tables, because again,
                // BBCode cannot do anything with this information.
                Tag::Table(_) => output.push_str("[table]"),
                Tag::TableHead => output.push_str(match dialect {
                    Dialect::Xenforo => "[tr]",
                    Dialect::Proboards => "\n  [thead][tr]",
                }),
                Tag::TableRow => output.push_str("[tr]"),
                Tag::TableCell => output.push_str("[td]"),
                Tag::Emphasis => output.push_str("[i]"),
                Tag::Strong => output.push_str("[b]"),
                Tag::Strikethrough => output.push_str("[s]"),
                // Link type and anchor title, respectively, don’t matter...
                Tag::Link(_, url, _) => {
                    output.push_str(match dialect {
                        Dialect::Xenforo => "[url=",
                        Dialect::Proboards => "[a href=\"",
                    });
                    output.push_str(&url);
                    output.push_str(match dialect {
                        Dialect::Xenforo => "]",
                        Dialect::Proboards => "\"]",
                    });
                }
                // Link type still don’t matter.
                Tag::Image(_, url, title) => match dialect {
                    Dialect::Xenforo => {
                        output.push_str("[img]");
                        output.push_str(&url);
                        output.push_str("[/img]");
                    }
                    Dialect::Proboards => {
                        output.push_str("[img src=\"");
                        output.push_str(&url);
                        output.push_str("\" alt=\"");
                        output.push_str(&title);
                        output.push_str("\"]");
                    }
                },
            },
            Event::End(tag) => match tag {
                Tag::Paragraph => output.push('\n'),
                Tag::Heading(_, _, _) => output.push_str(match dialect {
                    Dialect::Xenforo => "[/u][/b][/size]\n",
                    Dialect::Proboards => "[/u][/b][/font]\n\n",
                }),
                Tag::BlockQuote => output.push_str(match dialect {
                    Dialect::Xenforo => "[/quote]",
                    Dialect::Proboards => "[/blockquote]",
                }),
                Tag::CodeBlock(_) => output.push_str(match dialect {
                    Dialect::Xenforo => "[/code]\n",
                    Dialect::Proboards => "[/pre]\n",
                }),
                Tag::List(ord) => output.push_str(match dialect {
                    Dialect::Xenforo => "\n[/list]",
                    Dialect::Proboards => {
                        if ord.is_some() {
                            "\n[/ol]"
                        } else {
                            "\n[/ul]"
                        }
                    }
                }),
                Tag::Item => {
                    // A smol hack to make the whitespace around list items not
                    // get goof’d up.
                    output.truncate(output.trim_end().len());

                    match dialect {
                        Dialect::Xenforo => (),
                        Dialect::Proboards => output.push_str("[/li]"),
                    }
                }
                Tag::FootnoteDefinition(_) => output.push('\n'),
                // Once again, ignoring table column alignments...
                Tag::Table(_) => output.push_str(match dialect {
                    Dialect::Xenforo => "[/table]",
                    Dialect::Proboards => "\n  [/tbody]\n[/table]",
                }),
                Tag::TableHead => output.push_str(match dialect {
                    Dialect::Xenforo => "[/tr]",
                    Dialect::Proboards => "[/tr][/thead]\n  [tbody]",
                }),
                Tag::TableRow => output.push_str("[/tr]"),
                Tag::TableCell => output.push_str("[/td]"),
                Tag::Emphasis => output.push_str("[/i]"),
                Tag::Strong => output.push_str("[/b]"),
                Tag::Strikethrough => output.push_str("[/s]"),
                Tag::Link(_, _, _) => output.push_str(match dialect {
                    Dialect::Xenforo => "[/url]",
                    Dialect::Proboards => "[/a]",
                }),
                // No need to handle the end of an image element; the handler
                // for the start of an image element (as seen above) does all
                // of the work.
                Tag::Image(_, _, _) => (),
            },
            Event::Text(s) => output.push_str(&s),
            Event::Code(s) => {
                // Yes, the fact that we specify the font as `Courier New` to
                // implement inline “code” elements for XenForo is deeply
                // unfortunate. But I don’t know of any better way.
                output.push_str(match dialect {
                    Dialect::Xenforo => "[font=Courier New]",
                    Dialect::Proboards => "[tt]",
                });
                output.push_str(&s);
                output.push_str(match dialect {
                    Dialect::Xenforo => "[/font]",
                    Dialect::Proboards => "[/tt]",
                });
            }
            Event::Html(s) => {
                match s.as_ref().trim() {
                    // Some particular HTML elements have known translations:
                    "<del>" => output.push_str("[s]"),
                    "</del>" => output.push_str("[/s]"),
                    "<sup>" => output.push_str("[sup]"),
                    "</sup>" => output.push_str("[/sup]"),
                    "<sub>" => output.push_str("[sub]"),
                    "</sub>" => output.push_str("[/sub]"),
                    "<b>" => output.push_str("[b]"),
                    "</b>" => output.push_str("[/b]"),
                    "<i>" => output.push_str("[i]"),
                    "</i>" => output.push_str("[/i]"),
                    "<blockquote>" => output.push_str(match dialect {
                        Dialect::Xenforo => "[quote]",
                        Dialect::Proboards => "[blockquote]",
                    }),
                    "</blockquote>" => output.push_str(match dialect {
                        Dialect::Xenforo => "[/quote]",
                        Dialect::Proboards => "[/blockquote]",
                    }),
                    "<details>" => match dialect {
                        Dialect::Xenforo => output.push_str("\n[spoiler="),
                        // ProBoards doesn’t have details/spoiler elements
                        // AFAIK, so we just skip it.
                        Dialect::Proboards => eprintln!(
                            "[[WARN]] ProBoards doesn’t support `<details>`",
                        ),
                    },
                    s_trimmed if s_trimmed.starts_with("<br") => {
                        let mut is_br = true;

                        if let Some(c) = s_trimmed.chars().nth(3) {
                            if !c.is_whitespace() && c != '/' && c != '>' {
                                is_br = false;
                            } else {
                                output.push('\n');
                            }
                        } else {
                            is_br = false;
                        }

                        if !is_br {
                            eprintln!(
                                "[[WARN]] Unrecognised HTML tag: {s_trimmed}",
                            );
                            // Interpret it literally...
                            output.push_str(&s);
                        }
                    }
                    s_trimmed if s_trimmed.starts_with("<summary") => {
                        match dialect {
                            Dialect::Xenforo => {
                                if !s_trimmed.ends_with("</summary>") {
                                    bail!(MULTILINE_SUMMARY);
                                }

                                decode_html_entities_to_string(
                                    &s_trimmed
                                        .split(&['<', '>'][..])
                                        .nth(2)
                                        .ok_or_else(|| {
                                        anyhow!(MULTILINE_SUMMARY)
                                    })?,
                                    &mut output,
                                );
                                output.push(']');
                            }
                            Dialect::Proboards => (),
                        }
                    }
                    "</details>" => match dialect {
                        Dialect::Xenforo => output.push_str("[/spoiler]\n"),
                        Dialect::Proboards => (),
                    },
                    _ => {
                        // Any HTML elements that start with `<!` are assumed
                        // to be comments of some kind.
                        if !s.starts_with("<!") {
                            eprintln!("[[WARN]] Unrecognised HTML tag: {s}");
                            // This isn’t a comment, so we assume that this
                            // “HTML element” is not an HTML element at all,
                            // and is meant to be interpreted literally!
                            output.push_str(&s);
                        }
                    }
                }
            }
            Event::FootnoteReference(fnid) => {
                // We do our best to emulate a footnote marker...
                match dialect {
                    Dialect::Xenforo => (),
                    Dialect::Proboards => output.push_str("[sup]"),
                }
                output.push('\u{231c}'); // ⌜
                output.push_str(&fnid);
                output.push('\u{231d}'); // ⌝
                match dialect {
                    Dialect::Xenforo => (),
                    Dialect::Proboards => output.push_str("[/sup]"),
                }
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push('\n'),
            Event::Rule => output.push_str(match dialect {
                // Shitty hack for XenForo LMAO
                Dialect::Xenforo => "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                Dialect::Proboards => "\n[hr]\n",
            }),
            Event::TaskListMarker(checked) => {
                output.push(if checked {
                    '\u{2611}' // BALLOT BOX WITH CHECK
                } else {
                    '\u{2610}' // BALLOT BOX
                });
                output.push('\u{00a0}'); // NO-BREAK SPACE
            }
        }
    }

    if encoding_warnings {
        match dialect {
            Dialect::Xenforo => {
                for c in output.chars() {
                    if c >= '\u{fffe}' {
                        eprintln!(
                            "[[WARN]] Non-UCS-2 character in output: '{c}' \
                             (U+{:x})",
                            u32::from(c),
                        );
                    }
                }
            }
            Dialect::Proboards => (),
        }
    }

    Ok(output)
}
