use anyhow::{anyhow, Result};
use mini_markup::mxml_to_xml;
use std::fs;

#[test]
fn conversion_file_simple() -> Result<()> {
    let source = fs::read_to_string("./tests/files/test1.txt")?;
    let expected = fs::read_to_string("./tests/files/expected1.txt")?;
    if mxml_to_xml(source)? == expected {
        Ok(())
    } else {
        Err(anyhow!("Test failed"))
    }
}

#[test]
fn conversion_file_with_escapes() -> Result<()> {
    let source = fs::read_to_string("./tests/files/test2.txt")?;
    let expected = fs::read_to_string("./tests/files/expected2.txt")?;
    let actual = mxml_to_xml(source)?;
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("Test failed, {} != {}", actual, expected))
    }
}
