use std::io::{self, prelude::*, BufReader};
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;

use super::loader::LoaderError;
use super::material::{CalCoreMaterial, Color, Map};

impl From<ParseIntError> for LoaderError {
    fn from(error: ParseIntError) -> Self {
        LoaderError::FormatError(format!(
            "Error converting string to int: {:?}",
            error.kind()
        ))
    }
}

impl From<ParseFloatError> for LoaderError {
    fn from(error: ParseFloatError) -> Self {
        LoaderError::FormatError(format!("Error converting string to float: {:?}", error))
    }
}

pub fn loadXmlCoreMaterial(filename: &PathBuf) -> Result<CalCoreMaterial, LoaderError> {
    // use

    let text = std::fs::read_to_string(filename)?;

    let file = std::fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    for token in xmlparser::Tokenizer::from(buf.as_str()) {
        match token {
            Ok(val) => match val {
                ElementStart => {}
            },
            Err(e) => return Err(LoaderError::FormatError(format!("XML Parse error"))),
        }
    }

    buf.clear();

    reader.read_to_string(&mut buf)?; //.map(|l| l.unwrap());

    let mut tokenizer = xmlparser::Tokenizer::from(buf.as_str());
    let material = loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::ElementStart {
                            prefix: _,
                            local,
                            span: _,
                        } => match local.as_str() {
                            "MATERIAL" => {
                                break parse_material(&mut tokenizer)?;
                            }
                            _ => {}
                        },
                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in file: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => return Err(LoaderError::FormatError(format!("XML Parse error"))),
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    };
    println!("Material: {material:?}");

    Ok(material)
}

fn parse_material(tokenizer: &mut xmlparser::Tokenizer) -> Result<CalCoreMaterial, LoaderError> {
    let mut nummaps: Option<i32> = None;
    let material = loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::ElementStart {
                            prefix: _,
                            local,
                            span: _,
                        } => {
                            return Err(LoaderError::FormatError(format!(
                                "Unexpect element start {} in MATERIAL",
                                local
                            )))
                        }
                        xmlparser::Token::Attribute {
                            prefix: _,
                            local,
                            value,
                            span,
                        } => match local.as_str() {
                            "NUMMAPS" => {
                                nummaps = Some(value.parse::<i32>()?);
                            }
                            _ => {
                                return Err(LoaderError::FormatError(format!(
                                    "Unexpect attribute '{}' in MATERIAL",
                                    span
                                )))
                            }
                        },
                        xmlparser::Token::ElementEnd { end, span } => match end {
                            xmlparser::ElementEnd::Open => {
                                let (ambient, diffuse, specular, shininess, maps) =
                                    parse_material_elements(tokenizer)?;
                                break CalCoreMaterial::new(
                                    ambient,
                                    diffuse,
                                    specular,
                                    shininess,
                                    maps,
                                );
                            }
                            xmlparser::ElementEnd::Close(_, element) => {
                                if !matches!(element.to_uppercase().as_str(), "MATERIAL") {}
                            }
                            xmlparser::ElementEnd::Empty => {}
                        },

                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in material: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(LoaderError::FormatError(format!(
                        "XML Parse error: {:?}",
                        e
                    )))
                }
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    };
    Ok(material)
}

fn parse_material_elements(
    tokenizer: &mut xmlparser::Tokenizer,
) -> Result<(Color, Color, Color, f32, Vec<Map>), LoaderError> {
    let mut ambient: Option<Color> = None;
    let mut diffuse: Option<Color> = None;
    let mut specular: Option<Color> = None;
    let mut shininess: Option<f32> = None;
    let mut vector_map: Option<Vec<Map>> = None;

    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::ElementStart {
                            prefix,
                            local,
                            span,
                        } => match local.as_str() {
                            "AMBIENT" => {
                                ambient = Some(parse_color(tokenizer, local.as_str())?);
                            }
                            "DIFFUSE" => {
                                diffuse = Some(parse_color(tokenizer, local.as_str())?);
                            }
                            "SPECULAR" => {
                                specular = Some(parse_color(tokenizer, local.as_str())?);
                            }
                            "SHININESS" => {
                                shininess = Some(parse_float(tokenizer, local.as_str())?);
                            }
                            "MAP" => {
                                if vector_map.is_none() {
                                    vector_map = Some(Vec::new());
                                }
                                let map = parse_string(tokenizer, local.as_str())?;
                                vector_map.as_mut().unwrap().push(Map::new(
                                     map,
                                     String::from("Diffuse Color"),
                                     0
                                ))
                            }
                            _ => {
                                return Err(LoaderError::FormatError(format!(
                                    "Unexpect element {} in MATERIAL",
                                    local
                                )))
                            }
                        },
                        xmlparser::Token::ElementEnd { end, span } => match end {
                            xmlparser::ElementEnd::Close(a, b) => break,
                            xmlparser::ElementEnd::Open => {}
                            xmlparser::ElementEnd::Empty => {
                                return Err(LoaderError::FormatError(format!(
                                    "XML material element empty"
                                )))
                            }
                        },
                        xmlparser::Token::Text { text: _ } => {}
                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in material elements: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(LoaderError::FormatError(format!(
                        "XML Parse error: {:?}",
                        e
                    )))
                }
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    }

    Ok((
        ambient.ok_or(LoaderError::FormatError(format!("XML Parse error")))?,
        diffuse.ok_or(LoaderError::FormatError(format!("XML Parse error")))?,
        specular.ok_or(LoaderError::FormatError(format!("XML Parse error")))?,
        shininess.ok_or(LoaderError::FormatError(format!("XML Parse error")))?,
        vector_map.unwrap_or(Vec::new())
    ))
}

fn parse_color(tokenizer: &mut xmlparser::Tokenizer, element: &str) -> Result<Color, LoaderError> {
    let mut color: Option<Color> = None;

    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::Text { text } => {
                            let parts = text.split(" ").collect::<Vec<&str>>();
                            if parts.len() != 4 {
                                return Err(LoaderError::FormatError(format!(
                                    "XML {element} has wrong number of values: {text}"
                                )));
                            }

                            color = Some(Color::new(
                                parts[0].parse::<u8>()?,
                                parts[1].parse::<u8>()?,
                                parts[2].parse::<u8>()?,
                                parts[3].parse::<u8>()?,
                            ));
                        }
                        xmlparser::Token::ElementEnd { end, span } => match end {
                            xmlparser::ElementEnd::Close(a, b) => break,
                            xmlparser::ElementEnd::Open => {}
                            xmlparser::ElementEnd::Empty => {
                                return Err(LoaderError::FormatError(format!(
                                    "XML {} element empty",
                                    element
                                )))
                            }
                        },
                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in {element} color: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(LoaderError::FormatError(format!(
                        "XML Parse error: {:?}",
                        e
                    )))
                }
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    }

    Ok(color.ok_or(LoaderError::FormatError(format!("")))?)
}

fn parse_float(tokenizer: &mut xmlparser::Tokenizer, element: &str) -> Result<f32, LoaderError> {
    let mut float: Option<f32> = None;

    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::Text { text } => {
                            float = Some(text.parse::<f32>()?);
                        }
                        xmlparser::Token::ElementEnd { end, span } => match end {
                            xmlparser::ElementEnd::Close(a, b) => break,
                            xmlparser::ElementEnd::Open => {}
                            xmlparser::ElementEnd::Empty => {
                                return Err(LoaderError::FormatError(format!(
                                    "XML {} element empty",
                                    element
                                )))
                            }
                        },
                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in {element} float: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(LoaderError::FormatError(format!(
                        "XML Parse error: {:?}",
                        e
                    )))
                }
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    }

    Ok(float.ok_or(LoaderError::FormatError(format!("")))?)
}

fn parse_string(tokenizer: &mut xmlparser::Tokenizer, element: &str) -> Result<String, LoaderError> {
    let mut string: Option<String> = None;
    loop {
        let next = tokenizer.next();
        match next {
            Some(token) => match token {
                Ok(val) => {
                    match val {
                        xmlparser::Token::Text { text } => {
                            string = Some(text.to_string());
                        }
                        xmlparser::Token::ElementEnd { end, span } => match end {
                            xmlparser::ElementEnd::Close(a, b) => break,
                            xmlparser::ElementEnd::Open => {}
                            xmlparser::ElementEnd::Empty => {
                                return Err(LoaderError::FormatError(format!(
                                    "XML {} element empty",
                                    element
                                )))
                            }
                        },
                        _ => {
                            return Err(LoaderError::FormatError(format!(
                                "XML error in {element} string: {:?}",
                                val
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(LoaderError::FormatError(format!(
                        "XML Parse error: {:?}",
                        e
                    )))
                }
            },
            None => {
                return Err(LoaderError::FormatError(format!(
                    "Unexpected end of XML file"
                )))
            }
        }
    }

    Ok(string.ok_or(LoaderError::FormatError(format!("")))?)
}
