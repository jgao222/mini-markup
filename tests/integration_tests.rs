use anyhow::{anyhow, Result};
use mini_markup::{mxml_to_xml, xml_to_mxml};
use std::fs;

static TEST_DIR: &str = "./tests/files/";

macro_rules! test_file_conversion_to_xml {
    ( $test_name:ident, $file1:expr, $file2:expr ) => {
        #[test]
        fn $test_name() -> Result<()> {
            let source = fs::read_to_string($file1)?;
            let expected = fs::read_to_string($file2)?;
            let actual = mxml_to_xml(source)?;
            if actual == expected {
                Ok(())
            } else {
                Err(anyhow!("Test failed,\n{}\n!=\n{}", actual, expected))
            }
        }
    };
}

macro_rules! test_file_conversion_from_xml {
    ( $test_name:ident, $file1:expr, $file2:expr ) => {
        #[test]
        fn $test_name() -> Result<()> {
            let source = fs::read_to_string($file1)?;
            let expected = fs::read_to_string($file2)?;
            let actual = xml_to_mxml(source)?;
            if actual == expected {
                Ok(())
            } else {
                Err(anyhow!("Test failed,\n{}\n!=\n{}", actual, expected))
            }
        }
    };
}

// #[test]
// fn conversion_file_simple() -> Result<()> {
//     let source = fs::read_to_string("./tests/files/test1.txt")?;
//     let expected = fs::read_to_string("./tests/files/expected1.txt")?;
//     if mxml_to_xml(source)? == expected {
//         Ok(())
//     } else {
//         Err(anyhow!("Test failed"))
//     }
// }
test_file_conversion_to_xml!(
    file_simple,
    "./tests/files/test1.txt",
    "./tests/files/expected1.txt"
);

// #[test]
// fn conversion_file_with_escapes() -> Result<()> {
//     let source = fs::read_to_string("./tests/files/test2.txt")?;
//     let expected = fs::read_to_string("./tests/files/expected2.txt")?;
//     let actual = mxml_to_xml(source)?;
//     if actual == expected {
//         Ok(())
//     } else {
//         Err(anyhow!("Test failed, {} != {}", actual, expected))
//     }
// }
test_file_conversion_to_xml!(
    file_with_escapes,
    "./tests/files/test2.txt",
    "./tests/files/expected2.txt"
);

test_file_conversion_from_xml!(
    file_simple_from_xml,
    "./tests/files/expected1.txt",
    "./tests/files/test1.txt"
);

test_file_conversion_to_xml!(
    readme_test_to_xml,
    "./tests/files/test_readme.txt",
    "./tests/files/test_readme_expected.txt"
);

test_file_conversion_from_xml!(
    readme_test_from_xml,
    "./tests/files/test_readme_expected.txt",
    "./tests/files/test_readme.txt"
);

test_file_conversion_from_xml!(
    ignore_comments1,
    format!("{TEST_DIR}test_ignore_comments.txt"),
    format!("{TEST_DIR}test_ignore_comments_expected.txt")
);

test_file_conversion_from_xml!(
    ignore_comments2,
    format!("{TEST_DIR}test_ignore_comments2.txt"),
    format!("{TEST_DIR}test_ignore_comments2_expected.txt")
);