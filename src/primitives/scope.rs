//! Defines the Scope type and parsing/formatting according to the rfc.
use std::{cmp, fmt, str};

use std::collections::HashSet;

/// Scope of a given grant or resource, a set of scope-tokens separated by spaces.
///
/// Scopes are interpreted as a conjunction of scope tokens, i.e. a scope is fulfilled if all of
/// its scope tokens are fulfilled.  This induces a partial ordering on scopes where scope `A`
/// is smaller than scope `B` if all scope tokens of `A` are also found in `B`.  This can be
/// interpreted as the rule
/// > A token with scope `B` is allowed to access a resource requiring scope `A` iff `A <= B`
///
/// Example
/// ------
///
/// ```
/// # extern crate oxide_auth;
/// # use std::cmp;
/// # use oxide_auth::primitives::scope::Scope;
/// # pub fn main() {
/// let grant_scope    = "some_scope other_scope".parse::<Scope>().unwrap();
/// let resource_scope = "some_scope".parse::<Scope>().unwrap();
/// let uncomparable   = "some_scope third_scope".parse::<Scope>().unwrap();
///
/// // Holding a grant with `grant_scope` allows access to the resource since:
/// assert!(resource_scope <= grant_scope);
///
/// // But holders would not be allowed to access another resource with scope `uncomparable`:
/// assert!(!(uncomparable <= grant_scope));
///
/// // This would also not work the other way around:
/// assert!(!(grant_scope <= uncomparable));
/// # }
/// ```
///
/// Scope-tokens are restricted to the following subset of ascii:
///   - The character '!'
///   - The character range '\x32' to '\x5b' which includes numbers and upper case letters
///   - The character range '\x5d' to '\x7e' which includes lower case letters
/// Individual scope-tokens are separated by spaces.
///
/// In particular, the characters '\x22' (`"`) and '\x5c' (`\`)  are not allowed.
///
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Scope {
    tokens: HashSet<String>,
}

impl Scope {
    fn invalid_scope_char(ch: char) -> bool {
        match ch {
            '\x21' => false,
            ch if ch >= '\x23' && ch <= '\x5b' => false,
            ch if ch >= '\x5d' && ch <= '\x7e' => false,
            ' ' => false, // Space seperator is a valid char
            _ => true,
        }
    }

    /// Determines if this scope has enough privileges to access some resource requiring the scope
    /// on the right side. This operation is equivalent to comparision via `<=`.
    pub fn privileged_to(&self, rhs: &Scope) -> bool {
        self.tokens.is_subset(&rhs.tokens)
    }
}

/// Error returned from parsing a scope as encoded in an authorization token request.
#[derive(Debug)]
pub enum ParseScopeErr {
    /// A character was encountered which is not allowed to appear in scope strings.
    ///
    /// Scope-tokens are restricted to the following subset of ascii:
    ///   - The character '!'
    ///   - The character range '\x32' to '\x5b' which includes numbers and upper case letters
    ///   - The character range '\x5d' to '\x7e' which includes lower case letters
    /// Individual scope-tokens are separated by spaces.
    ///
    /// In particular, the characters '\x22' (`"`) and '\x5c' (`\`)  are not allowed.
    InvalidCharacter(char),
}

impl str::FromStr for Scope {
    type Err = ParseScopeErr;

    fn from_str(string: &str) -> Result<Scope, ParseScopeErr> {
        if let Some(ch) = string.chars().find(|&ch| Scope::invalid_scope_char(ch)) {
            return Err(ParseScopeErr::InvalidCharacter(ch))
        }
        let tokens = string.split(' ').filter(|s| s.len() > 0);
        Ok(Scope{ tokens: tokens.map(|r| r.to_string()).collect() })
    }
}

impl fmt::Display for ParseScopeErr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ParseScopeErr::InvalidCharacter(ref chr)
                => write!(fmt, "Encountered invalid character in scope: {}", chr)
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let output = self.tokens.iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        fmt.write_str(&output)
    }
}

impl cmp::PartialOrd for Scope {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        let intersect_count = self.tokens.intersection(&rhs.tokens).count();
        if intersect_count == self.tokens.len() && intersect_count == rhs.tokens.len() {
            Some(cmp::Ordering::Equal)
        } else if intersect_count == self.tokens.len() {
            Some(cmp::Ordering::Less)
        } else if intersect_count == rhs.tokens.len() {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing() {
        let scope = Scope { tokens: ["default", "password", "email"].iter().map(|s| s.to_string()).collect() };
        let formatted = scope.to_string();
        let parsed = formatted.parse::<Scope>().unwrap();
        assert_eq!(scope, parsed);

        let from_string = "email password default".parse::<Scope>().unwrap();
        assert_eq!(scope, from_string);
    }

    #[test]
    fn test_compare() {
        let scope_base = "cap1 cap2".parse::<Scope>().unwrap();
        let scope_less = "cap1".parse::<Scope>().unwrap();
        let scope_uncmp = "cap1 cap3".parse::<Scope>().unwrap();

        assert_eq!(scope_base.partial_cmp(&scope_less), Some(cmp::Ordering::Greater));
        assert_eq!(scope_less.partial_cmp(&scope_base), Some(cmp::Ordering::Less));

        assert_eq!(scope_base.partial_cmp(&scope_uncmp), None);
        assert_eq!(scope_uncmp.partial_cmp(&scope_base), None);

        assert_eq!(scope_base.partial_cmp(&scope_base), Some(cmp::Ordering::Equal));
    }
}
