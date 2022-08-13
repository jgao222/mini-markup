/// Library functions for the mxml-conversion program
use anyhow::{Result, bail};

pub fn mxml_to_xml(source: String) -> Result<String> {
    let scopes_converted = mxml_scopes_to_xml(source)?;
    let escapes_converted = replace_bracket_escapes(scopes_converted);
    Ok(escapes_converted)
}

pub fn mxml_scopes_to_xml(source: String) -> Result<String> {
    // TODO this also won't work if there are curly braces in other parts of the html file,
    // like if a CSS file is inlined in stylesheet tag, or if JavaScript is inlined in a script tag
    let mut tag_stack: Vec<String> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut out = String::new();

    for (index, char) in chars.iter().enumerate() {
        match char {
            '{' => {
                if let Some(tag_name) = find_tag_name_before_scope(&chars, index) {
                    tag_stack.push(tag_name);
                } else {
                    // TODO: this currently disallows empty scopes, when we could potentially
                    //       consider replacing via default value or nothing at all
                    bail!(format!("Couldn't find a tag before the scope at {}", index))
                }
                // we just found a tag, so remove whitespace b/w opening tag and scope opener
                out = out.trim().to_string();  // this may be inefficient
            },
            '}' => {
                if let Some(tag)= tag_stack.pop() {
                    out.push_str(format!("</{}>", tag).as_str());
                } else {
                    // also need to solve this with the TODO above
                    bail!("Unmatched closing tag");
                }
            },
            _ => out.push(*char),
        }
    }

    Ok(out)
}

/// Replaces the custom mxml escape codes `&lbrkt;` and `&rbrkt;` with the characters
/// `{` and `}` respectively.
pub fn replace_bracket_escapes(source: String) -> String {
    // a manual implementation could be more efficient in one pass as opposed to probably two
    source.replace("&lbrkt;", "{").replace("&rbrkt;", "}")
}

/// finds the name of the tag appearing before the scope opened by a bracket at
/// `index`.
/// # Params
/// `chars`: Vector of characters to search
/// `index`: index into the given vector, as the position of a `{` character
/// # Panics
/// If `chars[index]` is not `{`, panics
fn find_tag_name_before_scope(chars: &Vec<char>, index: usize) -> Option<String> {
    assert!(chars[index] == '{');
    let mut index = index - 1;
    while index > 0 && chars[index].is_whitespace() {
        index -= 1;
    }
    // the first non whitespace character upon walking backwards should be the tag closing right angle bracket
    if chars[index] != '>' {
        // this indicates either unclosed tag before the scope, or no tag before the scope
        return None;
    }
    // check if this is a self-closing tag, for html5 self closing is optional so it must be checked against on its own
    // if the tag before this scope opens is self-closing, it is not associated with this scope, and
    // so this scope should be handled like a tag-less scope
    if chars[index - 1] == '/' {
        return None;
    }
    // otherwise go ahead and find the tag name, which should be the first segment of characters
    // after the opening `<`, before the first space (or the closing `>` if no space/attrs)
    let mut index_space = index;
    while index > 0 && chars[index] != '<' {
        if chars[index].is_whitespace() {
            index_space = index;
        }
        index -= 1;
    }
    if chars[index] != '<' {
        // something weird happened, we got to start of file without finding tag opener
        return None;
    }
    Some(chars[index+1..index_space].iter().collect())
}


#[cfg(test)]
mod tests {
    use crate::{replace_bracket_escapes, mxml_scopes_to_xml, find_tag_name_before_scope};

    #[test]
    fn bracket_escapes_single() {
        let source = "&lbrkt;".to_string();
        assert_eq!("{", replace_bracket_escapes(source))
    }

    #[test]
    fn bracket_escapes_multiple() {
        let source = "&rbrkt;  &lbrkt; &lbrkt;".to_string();
        assert_eq!("}  { {", replace_bracket_escapes(source))
    }

    #[test]
    fn bracket_escapes_with_other_characters() {
        let source = "&lbrkt; abcdefg&rbrkt;".to_string();
        assert_eq!("{ abcdefg}", replace_bracket_escapes(source))
    }

    #[test]
    fn conversion_simple() {
        let source = "<tagname> {}";
        let expected = "<tagname></tagname>";
        assert_eq!(mxml_scopes_to_xml(source.into()).unwrap(), expected);
    }

    #[test]
    fn find_tag_name_simple() {
        let chars = "<banana> {".chars().collect();
        assert_eq!("banana", find_tag_name_before_scope(&chars, 9).unwrap());
    }

    #[test]
    fn find_tag_name_with_attrs() {
        let chars: Vec<char> = "<div style=\"border-width: 10px, border-radius: 2px\" on_click=stuff> {".chars().collect();
        let index = chars.len() - 1;
        assert_eq!("div", find_tag_name_before_scope(&chars, index).unwrap());
    }

    #[test]
    fn find_tag_name_none_simple() {
        let chars = "text {".chars().collect();
        assert!(find_tag_name_before_scope(&chars, 5).is_none());
    }
}