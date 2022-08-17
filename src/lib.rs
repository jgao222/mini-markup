/// Library functions for the mxml-conversion program
use std::collections::HashSet;
use anyhow::{bail, Result};


// see https://html.spec.whatwg.org/multipage/syntax.html#void-elements
// !DOCTYPE is also included, since in raw form it looks like an unclosed html tag
pub const HTML_VOID_ELEMENTS: [&str; 14] = ["!DOCTYPE", "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track", "wbr"];
// we will need to construct this into a set every time it is needed, is that worth it?

/// Convert from MXML (curly brackets) into XML (end tags)
/// # Params
/// source - the source MXML as a String
pub fn mxml_to_xml(source: String) -> Result<String> {
    let scopes_converted = mxml_scopes_to_xml(source, HashSet::new())?;
    let escapes_converted = replace_bracket_escapes(scopes_converted);
    Ok(escapes_converted)
}

/// Convert XML to MXML
/// # Params
/// source - the source XML as a String
pub fn xml_to_mxml(source: String) -> Result<String> {
    xml_scopes_to_mxml(source, HashSet::new())
}

/// Convert MXML to HTML, being aware of HTML5 void elements
/// # Params
/// source - the source MXML as a string
pub fn mxml_to_html(source: String) -> Result<String> {
    let scopes_converted = mxml_scopes_to_xml(source, HashSet::from(HTML_VOID_ELEMENTS))?;
    let escapes_converted = replace_bracket_escapes(scopes_converted);
    Ok(escapes_converted)
}

/// Converts HTML to MXML, being aware of HTML5 void elements
/// # Params
/// source - the source HTML as a String
pub fn html_to_mxml(source: String) -> Result<String> {
    xml_scopes_to_mxml(source, HashSet::from(HTML_VOID_ELEMENTS))
}

/// converts xml scopes to mxml
/// # Params
/// source - xml text
/// void_element_tags - a set of tags which are allowed to be empty-element tags without `/>` at the end
fn xml_scopes_to_mxml(source: String, void_element_tags: HashSet<&str>) -> Result<String> {
    // do a similar thing, parsing tags into stack
    // append ` {` after every opening tag
    // upon encountering end tag w/ same tag name as top of stack which should be always?
    // just replace with `}`

    // How to solve problem of self-closing tags?
    // - new solution, get the names of the tags which are allowed to not close themselves
    // - but still be empty-element tags and check for those

    // or just focus on converting well-formed XML? XML requires self-closing tags
    // to end in `/>`, so it would be possible to check
    // not so easy with HTML though.

    let mut tag_name_stack: Vec<String> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut out = String::new();

    let mut in_closing_tag = false;
    let mut in_comment = false;
    for (index, char) in chars.iter().enumerate() {
        // if inside comment override everything
        if in_comment {
            out.push(*char);
            if *char == '>' && chars[index - 1] == '-' && chars[index - 2] == '-' {
                in_comment = false;
            }
            continue;
        }
        // in empty element tag case do default case, no need to open or close any scopes
        if *char == '>' && chars[index - 1] != '/' {
            if in_closing_tag {
                out.push('}'); // the end of the scope
                in_closing_tag = false;
            } else if let Some(XMLTag::Start(tag_name)) = find_tag_name_at(&chars, index) {
                // if the tag is void-element (allowed to be empty element without closing `/>`)
                // don't add it to stack of tags that need to be closed, don't add curly brace
                if !void_element_tags.contains(tag_name.as_str()) {
                    tag_name_stack.push(tag_name);
                    out.push_str("> {"); // yes this is hardcoded
                    // TODO make whitespace customizable?
                } else {
                    out.push('>');
                }
            } else {
                bail!(format!(
                    "Couldn't find well-formed tag around position {}",
                    index
                ))
            }
        } else if *char == '<' {
            if chars[index + 1] == '/' {
                // this assumes all end tags start with `</` and there is no whitespace between < and /
                // TODO check if this is a reasonable assumption based on XML spec
                if let Some(XMLTag::End(tag_name)) = find_tag_name_at(&chars, index) {
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
                    bail!(format!(
                        "Couldn't find well-formed tag around position {}",
                        index
                    ))
                }
            } else if chars[index + 1..index + 4].iter().collect::<String>() == "!--" {
                in_comment = true;
                out.push(*char);
            } else {
                out.push(*char);
            }
        } else if !in_closing_tag {
            out.push(*char);
        }
    }
    if !tag_name_stack.is_empty() {
        bail!("Unmatched start tag(s) present")
    }

    Ok(out)
}

/// converts mxml scopes to xml
/// # Params
/// source - xml text
/// void_element_tags - a set of tags which are allowed to be empty-element tags without `/>` at the end
fn mxml_scopes_to_xml(source: String, void_element_tags: HashSet<&str>) -> Result<String> {
    // TODO this also won't work if there are curly braces in other parts of the html file,
    // like if a CSS file is inlined in stylesheet tag, or if JavaScript is inlined in a script tag
    let mut tag_stack: Vec<String> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut out = String::new();

    let mut in_comment = false;

    for (index, char) in chars.iter().enumerate() {
        // if inside comment override everything
        if in_comment {
            out.push(*char);
            if *char == '>' && chars[index - 1] == '-' && chars[index - 2] == '-' {
                in_comment = false;
            }
            continue;
        }
        match char {
            '{' => {
                if let Some(tag_name) = find_tag_name_before_scope(&chars, index) {
                    // TODO: currently handling void element tags before scopes as invalid,
                    //       but in the future might be able to just ignore curly braces
                    //       that don't have valid preceding tags, and just leave them in
                    //       which would fix a few of the issues and undefined behaviors

                    // all that has to be done for now though for mxml -> html is to bail here
                    println!("{tag_name}"); // TODO println
                    if void_element_tags.contains(tag_name.as_str()) {
                        bail!(format!("Invalid tag before scope at {index}"))
                    }
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
            '<' => {
                if chars[index + 1..index + 4].iter().collect::<String>() == "!--" {
                    in_comment = true;
                }
                out.push(*char);
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
        Some(XMLTag::Start(s)) => Some(s), // only take start tags
        _ => None,                         // reject if it was a self-closing or end tag!
    }
}

/// Find the name of the tag given either the index of the starting angle bracket
/// or the ending angle bracket
/// # Params
/// chars - a slice of characters to search in for the tag name
/// index - the starting index to look from
/// # Preconditions
/// `index` is the index of either the starting `<` or ending `>` of the tag
fn find_tag_name_at(chars: &[char], index: usize) -> Option<XMLTag> {
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
        Some(XMLTag::End(chars[index + 2..index_space].iter().collect()))
    } else if chars[index_space - 1] == '/' {
        Some(XMLTag::EmptyElement(
            chars[index + 1..index_space - 1].iter().collect(),
        ))
    } else {
        Some(XMLTag::Start(
            chars[index + 1..index_space].iter().collect(),
        ))
    }
}

enum XMLTag {
    Start(String),
    End(String),
    EmptyElement(String),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;

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
        assert_eq!(mxml_scopes_to_xml(source.into(), HashSet::new()).unwrap(), expected);
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

    #[test]
    fn xml_to_mxml_simple() {
        let source = "<tagname></tagname>";
        let expected = "<tagname> {}";
        assert_eq!(xml_scopes_to_mxml(source.into(), HashSet::new()).unwrap(), expected);
    }

    #[test]
    fn mxml_to_html_error_simple() {
        let source = "<img src=\"totally_real_src.png\"> {}";
        let result = mxml_to_html(source.into());
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn html_to_mxml_ignore_void() {
        let source = "<img src=\"abc\">\n<div>\n    text\n</div>";
        let expected = "<img src=\"abc\">\n<div> {\n    text\n}";
        assert_eq!(html_to_mxml(source.into()).unwrap(), expected);
    }

    #[test]
    fn html_to_mxml_ignore_properly_closed() {
        let source = "<notag src=\"abc\"/>\n<div>\n    text\n</div>";
        let expected = "<notag src=\"abc\"/>\n<div> {\n    text\n}";
        assert_eq!(html_to_mxml(source.into()).unwrap(), expected);
    }

    #[test]
    fn xml_to_mxml_comments_ignored() {
        let source = "<!-- <this would=\"be a tag\"></this but it's in a comment> -->";
        assert_eq!(xml_to_mxml(source.into()).unwrap(), source);
    }

    #[test]
    fn mxml_to_xml_comments_ignored() {
        let source = "<!-- <this would be valid mxml> { but it's in a comment } -->";
        assert_eq!(mxml_to_xml(source.into()).unwrap(), source);
    }
}
