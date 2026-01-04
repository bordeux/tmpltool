//! String manipulation functions for MiniJinja templates
//!
//! This module provides utility functions for working with strings:
//! - Regex operations: `regex_match`, `regex_find_all`
//! - String searching: `contains`, `index_of`, `count_occurrences`
//! - Text transformation: `sentence_case`, `to_constant_case`
//! - Pluralization: `pluralize`
//!
//! Note: regex_replace, substring, truncate, word_count, split_lines, wrap,
//! center, strip_html, strip_ansi, normalize_whitespace, slugify, indent, dedent,
//! quote, escape_quotes, and case conversion functions are now in
//! filter_functions/string.rs with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use regex::Regex;

/// Check if string matches regex pattern
pub struct RegexMatch;

impl Function for RegexMatch {
    const NAME: &'static str = "regex_match";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "regex_match",
        category: "string",
        description: "Check if string matches regex pattern",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The input string",
            },
            ArgumentMetadata {
                name: "pattern",
                arg_type: "string",
                required: true,
                default: None,
                description: "Regex pattern to match",
            },
        ],
        return_type: "boolean",
        examples: &["{{ regex_match(string=\"hello123\", pattern=\"[0-9]+\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let pattern: String = kwargs.get("pattern")?;

        let regex = Regex::new(&pattern).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid regex pattern '{}': {}", pattern, e),
            )
        })?;

        Ok(Value::from(regex.is_match(&string)))
    }
}

/// Find all regex matches in string
pub struct RegexFindAll;

impl Function for RegexFindAll {
    const NAME: &'static str = "regex_find_all";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "regex_find_all",
        category: "string",
        description: "Find all regex matches in string",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The input string",
            },
            ArgumentMetadata {
                name: "pattern",
                arg_type: "string",
                required: true,
                default: None,
                description: "Regex pattern to match",
            },
        ],
        return_type: "array",
        examples: &["{{ regex_find_all(string=\"a1b2c3\", pattern=\"[0-9]+\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let pattern: String = kwargs.get("pattern")?;

        let regex = Regex::new(&pattern).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid regex pattern '{}': {}", pattern, e),
            )
        })?;

        let matches: Vec<String> = regex
            .find_iter(&string)
            .map(|m| m.as_str().to_string())
            .collect();

        Ok(Value::from_serialize(&matches))
    }
}

/// Check if string contains substring
pub struct Contains;

impl Function for Contains {
    const NAME: &'static str = "contains";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "contains",
        category: "string",
        description: "Check if string contains substring",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The input string",
            },
            ArgumentMetadata {
                name: "substring",
                arg_type: "string",
                required: true,
                default: None,
                description: "Substring to search for",
            },
        ],
        return_type: "boolean",
        examples: &["{{ contains(string=\"hello world\", substring=\"world\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let substring: String = kwargs.get("substring")?;

        Ok(Value::from(string.contains(&substring)))
    }
}

/// Find position of substring
pub struct IndexOf;

impl Function for IndexOf {
    const NAME: &'static str = "index_of";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "index_of",
        category: "string",
        description: "Find position of substring (0-based, -1 if not found)",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The input string",
            },
            ArgumentMetadata {
                name: "substring",
                arg_type: "string",
                required: true,
                default: None,
                description: "Substring to search for",
            },
        ],
        return_type: "integer",
        examples: &["{{ index_of(string=\"hello world\", substring=\"world\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let substring: String = kwargs.get("substring")?;

        let result = match string.find(&substring) {
            Some(pos) => pos as i64,
            None => -1,
        };

        Ok(Value::from(result))
    }
}

/// Count occurrences of substring
pub struct CountOccurrences;

impl Function for CountOccurrences {
    const NAME: &'static str = "count_occurrences";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "count_occurrences",
        category: "string",
        description: "Count non-overlapping occurrences of substring",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The input string",
            },
            ArgumentMetadata {
                name: "substring",
                arg_type: "string",
                required: true,
                default: None,
                description: "Substring to count",
            },
        ],
        return_type: "integer",
        examples: &["{{ count_occurrences(string=\"hello hello hello\", substring=\"hello\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let substring: String = kwargs.get("substring")?;

        if substring.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "substring cannot be empty",
            ));
        }

        let count = string.matches(&substring).count();
        Ok(Value::from(count))
    }
}

/// Convert to Sentence case
pub struct SentenceCase;

impl Function for SentenceCase {
    const NAME: &'static str = "sentence_case";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "sentence_case",
        category: "string",
        description: "Convert to Sentence case (first letter uppercase, rest lowercase)",
        arguments: &[ArgumentMetadata {
            name: "string",
            arg_type: "string",
            required: true,
            default: None,
            description: "The input string",
        }],
        return_type: "string",
        examples: &["{{ sentence_case(string=\"hello world\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;

        if string.is_empty() {
            return Ok(Value::from(string));
        }

        let mut chars = string.chars();
        let first = chars.next().unwrap().to_uppercase().to_string();
        let rest: String = chars.collect::<String>().to_lowercase();

        Ok(Value::from(first + &rest))
    }
}

/// Convert to CONSTANT_CASE (uppercase snake case)
pub struct ToConstantCase;

impl Function for ToConstantCase {
    const NAME: &'static str = "to_constant_case";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "to_constant_case",
        category: "string",
        description: "Convert to CONSTANT_CASE (uppercase snake case)",
        arguments: &[ArgumentMetadata {
            name: "string",
            arg_type: "string",
            required: true,
            default: None,
            description: "The input string",
        }],
        return_type: "string",
        examples: &[
            "{{ to_constant_case(string=\"hello world\") }}",
            "{{ to_constant_case(string=\"helloWorld\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;

        if string.is_empty() {
            return Ok(Value::from(string));
        }

        let mut result = String::new();
        let mut prev_was_lowercase = false;
        let mut prev_was_uppercase = false;

        for ch in string.chars() {
            if ch.is_alphanumeric() {
                if ch.is_uppercase() && prev_was_lowercase {
                    result.push('_');
                }
                result.push(ch.to_ascii_uppercase());
                prev_was_lowercase = ch.is_lowercase();
                prev_was_uppercase = ch.is_uppercase();
            } else if ch == ' ' || ch == '-' || ch == '_' {
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
                prev_was_lowercase = false;
                prev_was_uppercase = false;
            }
            let _ = prev_was_uppercase;
        }

        if result.ends_with('_') {
            result.pop();
        }

        Ok(Value::from(result))
    }
}

/// Pluralize a word based on count
pub struct Pluralize;

impl Function for Pluralize {
    const NAME: &'static str = "pluralize";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "pluralize",
        category: "string",
        description: "Return singular or plural form based on count",
        arguments: &[
            ArgumentMetadata {
                name: "count",
                arg_type: "integer",
                required: true,
                default: None,
                description: "The count to check",
            },
            ArgumentMetadata {
                name: "singular",
                arg_type: "string",
                required: true,
                default: None,
                description: "The singular form",
            },
            ArgumentMetadata {
                name: "plural",
                arg_type: "string",
                required: false,
                default: None,
                description: "The plural form (default: singular + 's')",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ pluralize(count=1, singular=\"item\") }}",
            "{{ pluralize(count=5, singular=\"child\", plural=\"children\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let count: i64 = kwargs.get("count")?;
        let singular: String = kwargs.get("singular")?;
        let plural: Option<String> = kwargs.get("plural")?;

        let result = if count == 1 {
            singular
        } else {
            plural.unwrap_or_else(|| format!("{}s", singular))
        };

        Ok(Value::from(result))
    }
}
