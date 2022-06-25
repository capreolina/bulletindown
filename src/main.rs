mod args;
mod convert;

use anyhow::Result;
use clap::Parser;
use pulldown_cmark::Options;
use std::{
    fs::File,
    io::{self, prelude::*},
};

fn main() -> Result<()> {
    // Process command line arguments.
    let args = args::Args::parse();
    let markdown_opts = {
        let mut opts = Options::empty();

        if args.tables {
            opts.insert(Options::ENABLE_TABLES);
        }
        if args.footnotes {
            opts.insert(Options::ENABLE_FOOTNOTES);
        }
        if args.strikethrough {
            opts.insert(Options::ENABLE_STRIKETHROUGH);
        }
        if args.tasklists {
            opts.insert(Options::ENABLE_TASKLISTS);
        }
        if args.smart_punctuation {
            opts.insert(Options::ENABLE_SMART_PUNCTUATION);
        }

        opts
    };

    // Read contents of input file into memory.
    let mut input_string = String::new();
    if let Some(input_path) = args.input {
        File::open(input_path)?.read_to_string(&mut input_string)?;
    } else {
        io::stdin().lock().read_to_string(&mut input_string)?;
    }

    // Perform the actual conversion.
    let output_string = convert::convert(
        input_string,
        args.dialect,
        args.encoding_warnings,
        markdown_opts,
    )?;
    let output_str = output_string.trim();

    // Write to the output file.
    if let Some(output_path) = args.output {
        let mut output = File::create(output_path)?;
        output.write_all(output_str.as_bytes())?;
        output.write_all(b"\n")?;
    } else {
        let mut output = io::stdout().lock();
        output.write_all(output_str.as_bytes())?;
        output.write_all(b"\n")?;
        output.flush()?;
    }

    Ok(())
}
