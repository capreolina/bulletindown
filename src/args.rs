use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        help = "The dialect/flavour of BBCode to emit.",
        short,
        long,
        required = true,
        arg_enum,
        value_parser
    )]
    pub dialect: Dialect,
    #[clap(
        help = "A path to the input Markdown file. Defaults to stdin.",
        short,
        long,
        value_parser
    )]
    pub input: Option<String>,
    #[clap(
        help = "A path to the output BBCode file. Defaults to stdout.",
        short,
        long,
        value_parser
    )]
    pub output: Option<String>,
    #[clap(
        help = "Enable non-CommonMark (GFM) table syntax.",
        short,
        long,
        value_parser
    )]
    pub tables: bool,
    #[clap(
        help = "Enable non-CommonMark (GFM) footnote syntax.",
        short,
        long,
        value_parser
    )]
    pub footnotes: bool,
    #[clap(
        help = "Enable non-CommonMark (GFM) strikethrough syntax.",
        short,
        long,
        value_parser
    )]
    pub strikethrough: bool,
    #[clap(
        help = "Allow non-CommonMark (GFM) tasklist syntax.",
        long,
        value_parser
    )]
    pub tasklists: bool,
    #[clap(help = "Enable “smart punctuation”.", long, value_parser)]
    pub smart_punctuation: bool,
}

#[derive(ArgEnum, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Xenforo,
    Proboards,
}
