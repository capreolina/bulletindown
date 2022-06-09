use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        help = "The dialect/flavour of BBCode to emit.",
        short,
        long,
        required = true,
        arg_enum
    )]
    pub dialect: Dialect,
    #[clap(
        help = "A path to the input Markdown file. Defaults to stdin.",
        short,
        long
    )]
    pub input: Option<String>,
    #[clap(
        help = "A path to the output BBCode file. Defaults to stdout.",
        short,
        long
    )]
    pub output: Option<String>,
    #[clap(help = "Enable non-CommonMark (GFM) table syntax.", short, long)]
    pub tables: bool,
    #[clap(
        help = "Enable non-CommonMark (GFM) footnote syntax.",
        short,
        long
    )]
    pub footnotes: bool,
    #[clap(
        help = "Enable non-CommonMark (GFM) strikethrough syntax.",
        short,
        long
    )]
    pub strikethrough: bool,
    #[clap(help = "Allow non-CommonMark (GFM) tasklist syntax.", long)]
    pub tasklists: bool,
    #[clap(help = "Enable “smart punctuation”.", long)]
    pub smart_punctuation: bool,
}

#[derive(ArgEnum, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Xenforo,
    Proboards,
}
