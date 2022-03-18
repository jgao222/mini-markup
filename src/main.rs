use std::{fs, process::exit};

struct Settings {
    file_path: String,
    verbose: bool,
}

struct Printer<'a> {
    settings: &'a Settings,
}

impl Settings {
    fn parse_settings(args: &Vec<String>) -> Settings {
        Settings { file_path: args[0].clone(), verbose: args[1].eq("-v") }
    }
}

impl<'a> Printer<'a> {
    fn print(&self, s: &str) {
        // print if not silenced
        if self.settings.verbose {
            println!("{s}");
        }
    }
}

enum Symbols {
    OpenArrow,
    CloseArrow,
    OpenBrace,
    CloseBrace,
    Space,
}

fn main() {
    println!("I do nothing!");
    let args: Vec<String> = std::env::args().collect();

    let settings = Settings::parse_settings(&args[1..].to_vec());
    let p = Printer {settings: &settings};

    p.print("Reading file.");

    let f = fs::read_to_string(&settings.file_path).expect("failed to read file");

    p.print("File read");
    p.print("Constructing stack");

    let mut stack = std::vec::Vec::<String>::new();

    let mut out = String::new();

    let mut idx: usize = 0;
    let chars: Vec<char> = f.chars().collect();

    let mut found_tag: bool = false; // when we find a tag parse until find closing
    let mut found_tag_name: bool = false;
    let mut tag_needs_block: bool = false;

    let mut cur_tag_name: String = "".to_string();

    let mut tag_start_index: Option<usize> = Option::None;

    while idx < chars.len() {
        match parse_symbol(chars[idx]) {
            Some(symbol) => match symbol {
                Symbols::OpenArrow => {
                    if chars[idx - 1] == '\\' {
                        continue;
                    }
                    if found_tag {
                        handle_bad_input();
                    }
                    found_tag = true;
                    tag_start_index = Option::Some(idx);
                },
                Symbols::CloseArrow => {
                    if chars[idx - 1] == '\\' {
                        continue;
                    }
                    if !found_tag {
                        handle_bad_input();
                    }
                    found_tag = false;
                    tag_needs_block = true;
                    cur_tag_name = "".to_string();

                    stack.push("</".to_owned() + &cur_tag_name + ">");
                    let tag: &String = &chars[tag_start_index.expect("bad")..idx+1].iter().collect();
                    out.push_str(tag);
                },
                Symbols::OpenBrace => {
                    if tag_needs_block { tag_needs_block = false; }
                },
                Symbols::CloseBrace => {
                    out.push_str(&stack.pop().unwrap());
                },
                Symbols::Space => {
                    if !found_tag_name { // if the first space we saw after finding a tag opening
                        cur_tag_name = chars[tag_start_index.expect("This really wasn't supposed to happen!") + 1..idx].iter().collect();
                        found_tag_name = true;
                    }
                },
            },
            None => {
                idx += 1;
                out.push(chars[idx]);
                continue;
            },
        }
    }


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

fn handle_bad_input() {
    println!("Bad input format - Operation failed.");
    exit(1);
}

