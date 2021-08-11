use std::{fs::File, io};

use anyhow::Result;

use crate::{util::exit_document_errors, DebugArguments};
use s3wf2::parser::Parser;

pub(crate) fn subcommand_debug(args: DebugArguments) -> Result<()> {
    let (mut stdin, mut file);
    let source: &mut dyn io::Read = match args.input.as_deref() {
        None | Some("-") => {
            stdin = io::stdin();
            &mut stdin
        }
        Some(filename) => {
            file = io::BufReader::new(File::open(filename)?);
            &mut file
        }
    };

    let mut text = String::new();
    source.read_to_string(&mut text)?;

    let parser = Parser::new();
    match parser.parse(&text) {
        Ok(document) => {
            println!("{}", document);
            Ok(())
        }
        Err(errors) => {
            exit_document_errors(&errors);
        }
    }
}
