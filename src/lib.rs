/// Library functions for the mxml-conversion program
use anyhow::{Result};

pub fn mxml_string_to_xml(source: String) -> Result<String> {
    // let mut stack = Vec::<String>::new();

    // let mut out = String::new();

    // let mut idx: usize = 0;
    // let chars: Vec<char> = f.chars().collect();

    // let mut found_tag: bool = false; // when we find a tag parse until find closing
    // let mut found_tag_name: bool = false;
    // let mut tag_needs_block: bool = false;

    // let mut cur_tag_name: String = "".to_string();

    // let mut tag_start_index: Option<usize> = Option::None;

    // while idx < chars.len() {
    //     match parse_symbol(chars[idx]) {
    //         Some(symbol) => match symbol {
    //             Symbols::OpenArrow => {
    //                 if chars[idx - 1] == '\\' {
    //                     continue;
    //                 }
    //                 if found_tag {
    //                     handle_bad_input();
    //                 }
    //                 found_tag = true;
    //                 tag_start_index = Option::Some(idx);
    //             },
    //             Symbols::CloseArrow => {
    //                 if chars[idx - 1] == '\\' {
    //                     continue;
    //                 }
    //                 if !found_tag {
    //                     handle_bad_input();
    //                 }
    //                 found_tag = false;
    //                 tag_needs_block = true;
    //                 cur_tag_name = "".to_string();

    //                 stack.push("</".to_owned() + &cur_tag_name + ">");
    //                 let tag: &String = &chars[tag_start_index.expect("bad")..idx+1].iter().collect();
    //                 out.push_str(tag);
    //             },
    //             Symbols::OpenBrace => {
    //                 if tag_needs_block { tag_needs_block = false; }
    //             },
    //             Symbols::CloseBrace => {
    //                 out.push_str(&stack.pop().unwrap());
    //             },
    //             Symbols::Space => {
    //                 if !found_tag_name { // if the first space we saw after finding a tag opening
    //                     cur_tag_name = chars[tag_start_index.expect("This really wasn't supposed to happen!") + 1..idx].iter().collect();
    //                     found_tag_name = true;
    //                 }
    //             },
    //         },
    //         None => {
    //             idx += 1;
    //             out.push(chars[idx]);
    //             continue;
    //         },
    //     }
    // }
    todo!()
}

/// Replaces the custom mxml escape codes `&lbrkt;` and `&rbrkt;` with the characters
/// `{` and `}` respectively.
pub fn replace_bracket_escapes(source: String) -> String {
    // a manual implementation could be more efficient in one pass as opposed to probably two
    source.replace("&lbrkt;", "{").replace("&rbrkt;", "}")
}

enum Symbols {
    OpenArrow,
    CloseArrow,
    OpenBrace,
    CloseBrace,
    Space,
}

/**
 * Parses characters that we are interested in
 * might be redundant, since we are parsing into types and then checking
 * the types again afterwards
 */
fn parse_symbol(c: char) -> Option<Symbols> {
    if c == '<' {
        return Option::Some(Symbols::OpenArrow);
    } else if c == '>' {
        return Option::Some(Symbols::CloseArrow);
    } else if c == '{' {
        return Option::Some(Symbols::OpenBrace);
    } else if c == '}' {
        return Option::Some(Symbols::CloseBrace);
    } else if c == ' ' {
        return Option::Some(Symbols::Space);
    }
    return Option::None;
}


#[cfg(test)]
mod tests {
    use crate::{replace_bracket_escapes, mxml_string_to_xml};

    #[test]
    fn bracket_escapes_single() {
        let source = "&lbrkt;".to_string();
        assert_eq!("{", replace_bracket_escapes(source))
    }

    #[test]
    fn bracket_escapes_multiple() {
        let source = "&rbrkt;  &lbrkt; &lbrkt".to_string();
        assert_eq!("}  { {".to_string(), replace_bracket_escapes(source))
    }

    #[test]
    fn bracket_escapes_with_other_characters() {
        let source = "&lbrkt; abcdefg&rbrkt;".to_string();
        assert_eq!("{ abcdefg}".to_string(), replace_bracket_escapes(source))
    }

    #[test]
    fn conversion_simple() {
        let source = "<tagname> {}";
        let expected = "<tagname></tagname>";
        assert_eq!(mxml_string_to_xml(source.into()).unwrap(), expected);
    }
}