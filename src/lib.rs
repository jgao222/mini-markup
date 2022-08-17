/// Library functions for the mxml-conversion program
use anyhow::{bail, Result};
use Tag::*;

/// Convert a file from MXML (curly brackets) into XML (end tags)
/// # Params
/// source - the source MXML file as a String
pub fn mxml_to_xml(source: String) -> Result<String> {
    let scopes_converted = mxml_scopes_to_xml(source)?;
    let escapes_converted = replace_bracket_escapes(scopes_converted);
    Ok(escapes_converted)
}

/// Convert a file from XML to MXML
/// # Params
/// source - the source XML file as a String
pub fn xml_to_mxml(source: String) -> Result<String> {
    // do a similar thing, parsing tags into stack
    // append ` {` after every opening tag
    // upon encountering end tag w/ same tag name as top of stack which should be always?
    // just replace with `}`

    // How to solve problem of self-closing tags?
    // - they won't have a corresponding end tag
    // - shouldn't put an opening scope `{` after it
    // - can't tell if a tag is self-closing? need to parse until we should find
    // - an end tag? then decide to put brackets or not? will be inefficient
    // - if we just insert, will be more complicated if we do recursively

    // solution - two pass
    // 1st pass put all end tags, in order, into stack
    // second pass, upon encountered every opening tag, check the top end tag
    // and if it matches it should be the corresponding one?
    // won't directly work, need to reverse order of closing tags in the level of nesting
    // otherwise matching the first opening one to the last closing one will happen

    // or just focus on converting well-formed XML? XML requires self-closing tags
    // to end in `/>`, so it would be possible to check
    // not so easy with HTML though.

    let mut tag_name_stack: Vec<String> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut out = String::new();

    let mut in_closing_tag = false;
    for (index, char) in chars.iter().enumerate() {
        // in empty element tag case do default case, no need to open or close any scopes
        if *char == '>' && chars[index - 1] != '/' {
            if in_closing_tag {
                out.push('}');  // the end of the scope
                in_closing_tag = false;
            } else if let Some(StartTag(tag_name)) = find_tag_name_at(&chars, index) {
                tag_name_stack.push(tag_name);
                out.push_str("> {");  // yes this is hardcoded
                                             // TODO make whitespace customizable?
            } else {
                bail!(format!("Couldn't find well-formed tag around position {}", index))
            }
        } else if *char == '<' && chars[index + 1] == '/' {
            // this assumes all end tags start with `</` and there is no whitespace between < and /
            // TODO check if this is a reasonable assumption based on XML spec
            if let Some(EndTag(tag_name)) = find_tag_name_at(&chars, index) {
                if let Some(matched_name) = tag_name_stack.pop() {
                    if tag_name == matched_name {
                        in_closing_tag = true;
                    } else {
                        bail!(format!("Mismatched end tag at position {index}"))
                    }
                } else {
                    bail!(format!("Extra unmatched end tag at position {index}"))
                }
            } else {
                bail!(format!("Couldn't find well-formed tag around position {}", index))
            }

        } else {
            if !in_closing_tag {
                out.push(*char);
            }
        }
    }

    Ok(out)
}

fn mxml_scopes_to_xml(source: String) -> Result<String> {
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
                    //       Something to consider might be just leaving unrecognized brackets in
                    bail!(format!("Couldn't find a tag before the scope at {}", index))
                }
                // we just found a tag, so remove whitespace b/w opening tag and scope opener
                out = out.trim().to_string(); // this may be inefficient
            }
            '}' => {
                if let Some(tag) = tag_stack.pop() {
                    out.push_str(format!("</{}>", tag).as_str());
                } else {
                    // also need to solve this with the TODO above
                    bail!("Unmatched closing tag");
                }
            }
            _ => out.push(*char),
        }
    }

    Ok(out)
}

/// Replaces the custom mxml escape codes `&lbrkt;` and `&rbrkt;` with the characters
/// `{` and `}` respectively.
fn replace_bracket_escapes(source: String) -> String {
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
fn find_tag_name_before_scope(chars: &[char], index: usize) -> Option<String> {
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
    // otherwise go ahead and find the tag name, which should be the first segment of characters
    // after the opening `<`, before the first space (or the closing `>` if no space/attrs)
    match find_tag_name_at(chars, index) {
        Some(StartTag(s)) => Some(s),  // only take start tags
        _ => None  // reject if it was a self-closing or end tag!
    }
}

/// Find the name of the tag given either the index of the starting angle bracket
/// or the ending angle bracket
/// # Params
/// chars - a slice of characters to search in for the tag name
/// index - the starting index to look from
/// # Preconditions
/// `index` is the index of either the starting `<` or ending `>` of the tag
fn find_tag_name_at(chars: &[char], index: usize) -> Option<Tag> {
    let mut index = index;
    if chars[index] == '>' {
        while index > 0 && chars[index] != '<' {
            index -= 1;
        }
    }
    if chars[index] != '<' {
        return None;
    }
    let mut index_space = index;
    while !chars[index_space].is_whitespace() && chars[index_space] != '>' {
        index_space += 1;
    }
    if chars[index + 1] == '/' {
        Some(EndTag(chars[index + 2..index_space].iter().collect()))
    } else if chars[index_space - 1] == '/' {
        Some(EmptyElementTag(chars[index + 1..index_space - 1].iter().collect()))
    } else {
        Some(StartTag(chars[index + 1..index_space].iter().collect()))
    }
}

enum Tag {
    StartTag(String),
    EndTag(String),
    EmptyElementTag(String),
}


#[cfg(test)]
mod tests {
    use crate::{find_tag_name_before_scope, mxml_scopes_to_xml, replace_bracket_escapes};

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
        let chars: Vec<char> = "<banana> {".chars().collect();
        assert_eq!("banana", find_tag_name_before_scope(&chars, 9).unwrap());
    }

    #[test]
    fn find_tag_name_with_attrs() {
        let chars: Vec<char> =
            "<div style=\"border-width: 10px, border-radius: 2px\" on_click=stuff> {"
                .chars()
                .collect();
        let index = chars.len() - 1;
        assert_eq!("div", find_tag_name_before_scope(&chars, index).unwrap());
    }

    #[test]
    fn find_tag_name_none_simple() {
        let chars: Vec<char> = "text {".chars().collect();
        assert!(find_tag_name_before_scope(&chars, 5).is_none());
    }
}
