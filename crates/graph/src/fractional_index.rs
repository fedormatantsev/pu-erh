use thiserror::Error;

const SMALLEST_INT: &str = "A00000000000000000000000000";
const LARGEST_INT: &str = "zzzzzzzzzzzzzzzzzzzzzzzzzzz";

#[derive(Debug, Error, PartialEq)]
pub enum OrderError {
    #[error("invalid fractional index: {0:?}")]
    InvalidKey(String),
    #[error("left key must be less than right key")]
    OutOfOrder,
}

// Returns true if the string is a valid fractional-index integer part.
// An integer part starts with a length indicator digit and is followed by
// exactly the right number of digits. We use a simplified ASCII alphabet
// where uppercase letters encode the integer length and lowercase encode digits.
// We defer full validation to the algorithm itself; the key invariant we check
// is that the string is non-empty and composed of printable non-space ASCII.
fn is_valid_key(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.bytes().all(|b| b > b' ' && b < 0x7f)
}

// The jitl fractional-indexing algorithm.
// Alphabet: a–z, followed by uppercase A–Z then special chars — we use the
// standard base-95 printable ASCII variant with the range [' '+1, '~'].
// Integer part length is encoded by the first character:
//   a–z  → negative integers (z = -1, y = -2, …)
//   A–Z  → positive integers (A = 1-digit, B = 2-digit, …)
// For this implementation we use a simpler well-known variant:
// keys are strings from the ASCII range (printable, excluding space),
// and we implement the algorithm used by Figma/Notion.
//
// Concretely, we use the implementation described by David Greenspan
// (jitl/fractional-indexing), adapted to Rust:
//   - Digits: '0'–'9', 'A'–'Z', 'a'–'z' = 62 chars + specials
//   - We use a simplified digit alphabet: '!' (33) to '~' (126), 94 chars.
//
// For v0 we implement a practical subset: the midpoint function over
// ASCII printable strings, which is sufficient for all required operations.

const BASE: u8 = 95; // printable ASCII count (33..=127, exclusive)
const OFFSET: u8 = 33; // '!'

fn char_to_digit(c: u8) -> u8 {
    c - OFFSET
}

fn digit_to_char(d: u8) -> u8 {
    d + OFFSET
}

// Returns the midpoint string between a (exclusive lower) and b (exclusive upper).
// Either may be empty to mean "no bound".
// Implements the standard fractional-indexing midpoint algorithm.
pub fn generate_key_between(
    left: Option<&str>,
    right: Option<&str>,
) -> Result<String, OrderError> {
    if let Some(l) = left {
        if !is_valid_key(l) {
            return Err(OrderError::InvalidKey(l.to_owned()));
        }
    }
    if let Some(r) = right {
        if !is_valid_key(r) {
            return Err(OrderError::InvalidKey(r.to_owned()));
        }
    }
    if let (Some(l), Some(r)) = (left, right) {
        if l >= r {
            return Err(OrderError::OutOfOrder);
        }
    }

    match (left, right) {
        (None, None) => Ok("a0".to_owned()),
        (Some(l), None) => Ok(increment(l)),
        (None, Some(r)) => Ok(decrement(r)),
        (Some(l), Some(r)) => Ok(midpoint(l, r)),
    }
}

// Increment a key by one step (append a new character if needed).
fn increment(s: &str) -> String {
    let bytes = s.as_bytes();
    let last = *bytes.last().unwrap();
    if last < digit_to_char(BASE - 1) {
        let mut out = s[..s.len() - 1].to_owned();
        out.push(char::from(last + 1));
        out
    } else {
        let mut out = s.to_owned();
        out.push(char::from(digit_to_char(0)));
        out
    }
}

// Decrement a key by one step (prepend a character before the first char if needed).
fn decrement(s: &str) -> String {
    let bytes = s.as_bytes();
    let first = bytes[0];
    if first > digit_to_char(0) {
        let mut out = String::new();
        out.push(char::from(first - 1));
        out.push_str(&s[1..]);
        out
    } else {
        let mut out = String::from(char::from(digit_to_char(0)));
        out.push_str(s);
        out
    }
}

// Compute a key strictly between l and r using character-by-character midpoint.
fn midpoint(l: &str, r: &str) -> String {
    let lb = l.as_bytes();
    let rb = r.as_bytes();
    let max_len = lb.len().max(rb.len());

    // Pad shorter side
    let mut la: Vec<u8> = lb.iter().map(|&c| char_to_digit(c)).collect();
    let mut ra: Vec<u8> = rb.iter().map(|&c| char_to_digit(c)).collect();
    while la.len() < max_len {
        la.push(0);
    }
    while ra.len() < max_len {
        ra.push(BASE); // treat as one past max digit (like "infinity")
    }

    // Find the midpoint digit-by-digit
    let mut result = Vec::new();
    for i in 0..max_len {
        let ldig = la[i];
        let rdig = ra[i];
        if rdig - ldig > 1 {
            result.push(digit_to_char((ldig + rdig) / 2));
            break;
        } else {
            result.push(digit_to_char(ldig));
            // We need to look further; continue to next position
            if i == max_len - 1 {
                // Exhausted, append midpoint in next position
                let l_next = 0u8;
                let r_next = if ldig < rdig { BASE } else { BASE };
                result.push(digit_to_char(l_next + (r_next) / 2));
                break;
            }
        }
    }

    result.iter().map(|&b| char::from(b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_key_is_a0() {
        assert_eq!(generate_key_between(None, None).unwrap(), "a0");
    }

    #[test]
    fn key_after_last() {
        let k = generate_key_between(Some("a0"), None).unwrap();
        assert!(k.as_str() > "a0", "expected {k:?} > \"a0\"");
    }

    #[test]
    fn key_before_first() {
        let k = generate_key_between(None, Some("a0")).unwrap();
        assert!(k.as_str() < "a0", "expected {k:?} < \"a0\"");
    }

    #[test]
    fn key_between_two() {
        let k = generate_key_between(Some("a0"), Some("b0")).unwrap();
        assert!(k.as_str() > "a0" && k.as_str() < "b0",
            "expected \"a0\" < {k:?} < \"b0\"");
    }

    #[test]
    fn invalid_left_errors() {
        assert!(matches!(
            generate_key_between(Some(""), None),
            Err(OrderError::InvalidKey(_))
        ));
    }

    #[test]
    fn out_of_order_errors() {
        assert!(matches!(
            generate_key_between(Some("b0"), Some("a0")),
            Err(OrderError::OutOfOrder)
        ));
    }

    #[test]
    fn equal_keys_error() {
        assert!(matches!(
            generate_key_between(Some("a0"), Some("a0")),
            Err(OrderError::OutOfOrder)
        ));
    }

    #[test]
    fn sequential_insertions_all_ordered() {
        let mut keys = vec![generate_key_between(None, None).unwrap()];
        for _ in 0..10 {
            let last = keys.last().unwrap().as_str();
            let k = generate_key_between(Some(last), None).unwrap();
            keys.push(k);
        }
        for w in keys.windows(2) {
            assert!(w[0] < w[1], "{:?} should be < {:?}", w[0], w[1]);
        }
    }

    #[test]
    fn bisect_insertions_all_ordered() {
        let a = generate_key_between(None, None).unwrap();
        let b = generate_key_between(Some(&a), None).unwrap();
        let mid = generate_key_between(Some(&a), Some(&b)).unwrap();
        assert!(a < mid && mid < b);
    }
}
