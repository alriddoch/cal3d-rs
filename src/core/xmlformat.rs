use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use super::loader::LoaderError;
use super::material::CalCoreMaterial;

pub fn loadXmlCoreMaterial(filename: &PathBuf) -> Result<CalCoreMaterial, LoaderError> {
    // use

    let text = std::fs::read_to_string(filename)?;

    let file = std::fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    println!("Foo {}", buf);

    for token in xmlparser::Tokenizer::from(buf.as_str()) {
        match token {
            Ok(val) => match val {
                ElementStart => {}
            },
            Err(e) => return Err(LoaderError::FormatError(format!("XML Parse error"))),
        }
        println!("{:?}", token);
    }

    buf.clear();

    reader.read_to_string(&mut buf)?; //.map(|l| l.unwrap());

    println!("Bar {}", buf);

    let mut tokenizer = xmlparser::Tokenizer::from(buf.as_str());
    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => match val {
                    xmlparser::Token::ElementStart {
                        prefix,
                        local,
                        span,
                    } => match local.as_str() {
                        "MATERIAL" => {
                            parse_material(&mut tokenizer)?;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Err(e) => return Err(LoaderError::FormatError(format!("XML Parse error"))),
            },
            None => {}
        }
    }

    todo!();
}

fn parse_material(tokenizer: &mut xmlparser::Tokenizer) -> Result<(), LoaderError> {
    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => match val {
                    xmlparser::Token::ElementStart {
                        prefix,
                        local,
                        span,
                    } => match local.as_str() {
                        "MATERIAL" => {
                            // parse_material(tokenizer);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Err(e) => return Err(LoaderError::FormatError(format!("XML Parse error"))),
            },
            None => {}
        }
    }
}
