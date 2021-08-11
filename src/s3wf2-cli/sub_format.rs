use std::{fs::File, io, io::prelude::*, time::Instant};

use anyhow::{bail, Result};

use crate::{util::exit_document_errors, FormatArguments};
use s3wf2::{
    document::Document,
    emitter::{console::ConsoleEmitter, html::HtmlEmitter, Emit},
    parser::Parser,
};

pub(crate) fn subcommand_format(args: FormatArguments) -> Result<()> {
    let stdin;
    let mut source: Box<dyn io::Read> = match args.input.as_deref() {
        None | Some("-") => {
            stdin = io::stdin();
            Box::new(io::BufReader::new(stdin.lock()))
        }
        Some(filename) => Box::new(io::BufReader::new(File::open(filename)?)),
    };
    let mut text = String::new();
    source.read_to_string(&mut text)?;

    let parser = Parser::new();
    let parse_started = Instant::now();
    let document = match parser.parse(&text) {
        Ok(document) => document,
        Err(errors) => {
            exit_document_errors(&errors);
        }
    };
    let parse_time = parse_started.elapsed();
    if args.verbose {
        use ansi_term::Color::Green;
        eprintln!(
            "{} It took {}ms.",
            Green.bold().paint("Parse succeeded."),
            parse_time.as_millis()
        );
    }

    match args.format_type.as_deref() {
        Some("html") | None => {
            emit_html(&document, args.output)?;
        }
        Some("console") => {
            emit_console(&document)?;
        }
        Some(otherwise) => bail!("Unknown format type: {}", otherwise),
    }

    Ok(())
}

fn emit_html(document: &Document, output: Option<String>) -> Result<()> {
    let mut emitter = HtmlEmitter::new(4);
    match output.as_deref() {
        None | Some("-") => {
            let stdout = io::stdout();
            let mut writer = io::BufWriter::with_capacity(1 << 16, stdout.lock());
            emitter.emit(&mut writer, document)?;
            writer.flush()?;
        }
        Some(file) => {
            let mut writer = io::BufWriter::with_capacity(1 << 16, File::create(file)?);
            emitter.emit(&mut writer, document)?;
            writer.flush()?;
        }
    }
    Ok(())
}

fn emit_console(document: &Document) -> Result<()> {
    let mut emitter = ConsoleEmitter::new();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::with_capacity(1 << 16, stdout.lock());
    emitter.emit(&mut writer, document)?;
    writer.flush()?;
    Ok(())
}
