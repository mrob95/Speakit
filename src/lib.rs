use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn _split_symbol(symbol: &str, numbers: bool) -> String {
    // let mut words = String::new();
    let mut words = String::with_capacity(symbol.len()*2);
    let mut prev_numeric = false;
    let mut prev_space = true;
    for c in symbol.chars() {
        if !c.is_alphanumeric() {
            // Character is not alphanumeric, assume that it is a separator character
            if !prev_space {
                words.push_str(" ");
                prev_space = true;
            }
            prev_numeric = false;
            continue;
        }

        if c.is_numeric() {
            if !prev_numeric && !prev_space && numbers{
                // transition from alphabetic to numeric
                words.push_str(" ");
                prev_space = true;
            }
            prev_numeric = true;
        } else if (prev_numeric || c.is_uppercase()) && !prev_space {
            // Transition from numeric to alphabetic, or
            // uppercase character, insert space if necessary
            words.push_str(" ");
            prev_numeric = false;
            prev_space = true;
        } else {
            prev_numeric = false;
        }

        if !(prev_numeric && !numbers) {
            // If we aren't including numbers and the character is a number,
            // then prev_numeric was set above and we don't include the character.
            words.push(c);
            prev_space = false;
        }
    }
    if prev_space {
        // Remove trailing space if present
        words.pop();
    }
    return words;
}

fn _split_list(symbols: Vec<&str>, numbers: bool) -> Vec<String> {
    return symbols.iter().map(|s| _split_symbol(s, numbers)).collect();
}

#[pyfunction(symbol, numbers = "true")]
fn split_symbol(symbol: &str, numbers: bool) -> PyResult<String> {
    Ok(_split_symbol(symbol, numbers))
}

#[pyfunction(symbols, numbers = "true")]
fn split_symbols(symbols: Vec<&str>, numbers: bool) -> PyResult<Vec<String>> {
    Ok(_split_list(symbols, numbers))
}

#[pymodule]
fn symbol_split(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(split_symbol))?;
    m.add_wrapped(wrap_pyfunction!(split_symbols))?;

    Ok(())
}


#[test]
fn test_basic() {
    assert_eq!(_split_symbol("a", true), "a");
    assert_eq!(_split_symbol("a_test", true), "a test");
    assert_eq!(_split_symbol("aTest", true), "a Test");
}

#[test]
fn test_caps() {
    assert_eq!(_split_symbol("A", true), "A");
    assert_eq!(_split_symbol("ATestCase", true), "A Test Case");
    assert_eq!(_split_symbol("ABCDEF", true), "A B C D E F");
}

#[test]
fn test_separators() {
    assert_eq!(_split_symbol("A.word_C", true), "A word C");
    assert_eq!(_split_symbol("module_aTestCase", true), "module a Test Case");
    assert_eq!(_split_symbol("_ABCDEF", true), "A B C D E F");
    assert_eq!(_split_symbol("__init__", true), "init");
}

#[test]
fn test_numbers() {
    assert_eq!(_split_symbol("999", true), "999");
    assert_eq!(_split_symbol("99module_aTestCase", true), "99 module a Test Case");
    assert_eq!(_split_symbol("0_A1B2C3DEF99", true), "0 A 1 B 2 C 3 D E F 99");
    assert_eq!(_split_symbol("_9a", true), "9 a");
    assert_eq!(_split_symbol("_a9", true), "a 9");
    assert_eq!(_split_symbol("9aa99", true), "9 aa 99");
}

#[test]
fn test_no_numbers() {
    assert_eq!(_split_symbol("999", false), "");
    assert_eq!(_split_symbol("99module_aTestCase", false), "module a Test Case");
    assert_eq!(_split_symbol("0_A1B2C3DEF99", false), "A B C D E F");
    assert_eq!(_split_symbol("_9a", false), "a");
    assert_eq!(_split_symbol("_a9", false), "a");
    assert_eq!(_split_symbol("9aa99", false), "aa");
}

#[test]
fn test_list() {
    assert_eq!(
        _split_list(vec![
            "999",
            "99module_aTestCase",
            "0_A1B2C3DEF99",
            "_9a",
            "_a9",
            "9aa99"
        ], true),
        vec![
            "999",
            "99 module a Test Case",
            "0 A 1 B 2 C 3 D E F 99",
            "9 a",
            "a 9",
            "9 aa 99"
        ]
    );
}
