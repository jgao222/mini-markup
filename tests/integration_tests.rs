use anyhow::{anyhow, Result};
use mini_markup::{html_to_mxml, mxml_to_html, mxml_to_xml, xml_to_mxml};
use std::fs;

static TEST_DIR: &str = "./tests/files/";

macro_rules! test_file_conversion {
    ( $test_name:ident, $test_fn:ident, $file1:expr, $file2:expr ) => {
        #[test]
        fn $test_name() -> Result<()> {
            let source = fs::read_to_string(format!("{TEST_DIR}{}", $file1))?;
            let expected = fs::read_to_string(format!("{TEST_DIR}{}", $file2))?;
            let actual = $test_fn(source)?;
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
test_file_conversion!(
    file_simple,
    mxml_to_xml,
    "test1.txt",
    "expected1.txt"
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
test_file_conversion!(
    file_with_escapes,
    mxml_to_xml,
    "test2.txt",
    "expected2.txt"
);

test_file_conversion!(
    file_simple_from_xml,
    xml_to_mxml,
    "expected1.txt",
    "test1.txt"
);

test_file_conversion!(
    readme_test_to_xml,
    mxml_to_xml,
    "test_readme.txt",
    "test_readme_expected.txt"
);

test_file_conversion!(
    readme_test_from_xml,
    xml_to_mxml,
    "test_readme_expected.txt",
    "test_readme.txt"
);

test_file_conversion!(
    ignore_comments1,
    xml_to_mxml,
    "test_ignore_comments.txt",
    "test_ignore_comments_expected.txt"
);

test_file_conversion!(
    ignore_comments2,
    mxml_to_xml,
    "test_ignore_comments2.txt",
    "test_ignore_comments2_expected.txt"
);

// the conversion on a real file looks correct at this moment, so this is a regression test
test_file_conversion!(
    resume_regression_test,
    html_to_mxml,
    "resume.html",
    "resume.mxml"
);

test_file_conversion!(
    resume_regression_test2,
    mxml_to_html,
    "resume.mxml",
    "resume.html"
);
