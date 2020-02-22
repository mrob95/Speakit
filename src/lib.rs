use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const SPOKEN_NUMBERS: [&'static str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn _split_symbol(symbol: &str, numbers: bool, max_len: usize) -> String {
    // let mut words = String::new();
    let mut words = String::with_capacity(symbol.len() * 2);
    let mut prev_numeric = false;
    let mut prev_space = true;
    let mut num_spaces = 1;
    // If the last character wasn't a space, add one
    // if we have reached the target number of words, break out of the
    macro_rules! add_space {
        () => {
            if !prev_space {
                if num_spaces == max_len { break; }
                words.push_str(" ");
                num_spaces += 1;
                prev_space = true;
            }
        };
    }
    for c in symbol.chars() {
        if !c.is_alphanumeric() {
            // Character is not alphanumeric, assume that it is a separator character
            add_space!();
            prev_numeric = false;
            continue;
        }

        if c.is_numeric() {
            if numbers {
                add_space!();
                let i = c.to_digit(10).unwrap();
                words.push_str(SPOKEN_NUMBERS[i as usize]);
                prev_space = false;
            }
            prev_numeric = true;
        } else {
            if prev_numeric || c.is_uppercase() {
                // Transition from numeric to alphabetic, or
                // uppercase character, insert space if necessary
                add_space!();
            }
            words.push(c);
            prev_space = false;
            prev_numeric = false;
        }
    }

    if prev_space {
        // Remove trailing space if present
        words.pop();
    }
    return words;
}

fn _split_list(symbols: Vec<&str>, numbers: bool, max_len: usize) -> Vec<String> {
    return symbols
        .iter()
        .map(|s| _split_symbol(s, numbers, max_len))
        .collect();
}

#[pyfunction(symbol, numbers = "true", max_len = "0")]
fn split_symbol(symbol: &str, numbers: bool, max_len: usize) -> PyResult<String> {
    Ok(_split_symbol(symbol, numbers, max_len))
}

#[pyfunction(symbols, numbers = "true", max_len = "0")]
fn split_symbols(symbols: Vec<&str>, numbers: bool, max_len: usize) -> PyResult<Vec<String>> {
    Ok(_split_list(symbols, numbers, max_len))
}

#[pymodule]
fn speakit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(split_symbol))?;
    m.add_wrapped(wrap_pyfunction!(split_symbols))?;

    Ok(())
}

#[test]
fn test_basic() {
    assert_eq!(_split_symbol("a", true, 0), "a");
    assert_eq!(_split_symbol("a_test", true, 0), "a test");
    assert_eq!(_split_symbol("aTest", true, 0), "a Test");
}

#[test]
fn test_caps() {
    assert_eq!(_split_symbol("A", true, 0), "A");
    assert_eq!(_split_symbol("ATestCase", true, 0), "A Test Case");
    assert_eq!(_split_symbol("ABCDEF", true, 0), "A B C D E F");
}

#[test]
fn test_separators() {
    assert_eq!(_split_symbol("A.word_C", true, 0), "A word C");
    assert_eq!(
        _split_symbol("module_aTestCase", true, 0),
        "module a Test Case"
    );
    assert_eq!(_split_symbol("_ABCDEF", true, 0), "A B C D E F");
    assert_eq!(_split_symbol("__init__", true, 0), "init");
}

#[test]
fn test_numbers() {
    assert_eq!(_split_symbol("999", true, 0), "nine nine nine");
    assert_eq!(
        _split_symbol("99module_aTestCase", true, 0),
        "nine nine module a Test Case"
    );
    assert_eq!(
        _split_symbol("0_A1B2C3DEF99", true, 0),
        "zero A one B two C three D E F nine nine"
    );
    assert_eq!(_split_symbol("_9a", true, 0), "nine a");
    assert_eq!(_split_symbol("_a9", true, 0), "a nine");
    assert_eq!(_split_symbol("9aa99", true, 0), "nine aa nine nine");
}

#[test]
fn test_no_numbers() {
    assert_eq!(_split_symbol("999", false, 0), "");
    assert_eq!(
        _split_symbol("99module_aTestCase", false, 0),
        "module a Test Case"
    );
    assert_eq!(_split_symbol("0_A1B2C3DEF99", false, 0), "A B C D E F");
    assert_eq!(_split_symbol("_9a", false, 0), "a");
    assert_eq!(_split_symbol("_a9", false, 0), "a");
    assert_eq!(_split_symbol("9aa99", false, 0), "aa");
}

#[test]
fn test_max_len() {
    assert_eq!(_split_symbol("99module_aTestCase", true, 3), "nine nine module");
    assert_eq!(
        _split_symbol("99module_aTestCase", false, 3),
        "module a Test"
    );
    assert_eq!(_split_symbol("0_A1B2C3DEF99", false, 3), "A B C");
    assert_eq!(_split_symbol("0_A1B2C3DEF99", true, 3), "zero A one");
}

#[test]
fn test_list() {
    assert_eq!(
        _split_list(
            vec![
                "999",
                "99module_aTestCase",
                "0_A1B2C3DEF99",
                "_9a",
                "_a9",
                "9aa99"
            ],
            true,
            0
        ),
        vec![
            "nine nine nine",
            "nine nine module a Test Case",
            "zero A one B two C three D E F nine nine",
            "nine a",
            "a nine",
            "nine aa nine nine"
        ]
    );
}
