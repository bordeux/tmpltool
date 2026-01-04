//! Unified filter-functions that support both function and filter syntax.
//!
//! This module provides implementations that can be used as either:
//! - Functions: `{{ sha256(string="hello") }}`
//! - Filters: `{{ "hello" | sha256 }}`
//!
//! Both syntaxes are equivalent and share the same underlying implementation.
//!
//! # Benefits
//!
//! - **User choice**: Users can pick their preferred syntax
//! - **Chainability**: Filter syntax enables `{{ x | a | b | c }}`
//! - **DRY**: Single implementation for both syntaxes
//! - **Backwards compatible**: Existing function calls continue to work
//!
//! # Example
//!
//! ```jinja
//! {# Function syntax #}
//! {{ sha256(string="hello") }}
//!
//! {# Filter syntax #}
//! {{ "hello" | sha256 }}
//!
//! {# Chaining filters #}
//! {{ "hello" | sha256 | base64_encode }}
//! ```

pub mod array;
pub mod datetime;
pub mod encoding;
pub mod formatting;
pub mod hash;
pub mod kubernetes;
pub mod math;
pub mod object;
pub mod path;
pub mod serialization;
pub mod string;
pub mod traits;
pub mod url;

pub use traits::FilterFunction;

use crate::functions::metadata::FunctionMetadata;
use minijinja::Environment;

/// Get all metadata from filter-functions
pub fn get_all_metadata() -> Vec<&'static FunctionMetadata> {
    vec![
        // Hash functions
        &hash::Md5::METADATA,
        &hash::Sha1::METADATA,
        &hash::Sha256::METADATA,
        &hash::Sha512::METADATA,
        // Encoding functions
        &encoding::Base64Encode::METADATA,
        &encoding::Base64Decode::METADATA,
        &encoding::HexEncode::METADATA,
        &encoding::HexDecode::METADATA,
        &encoding::EscapeHtml::METADATA,
        &encoding::EscapeXml::METADATA,
        &encoding::EscapeShell::METADATA,
        // Serialization functions
        &serialization::ToJson::METADATA,
        &serialization::ToYaml::METADATA,
        &serialization::ToToml::METADATA,
        &serialization::ParseJson::METADATA,
        &serialization::ParseYaml::METADATA,
        &serialization::ParseToml::METADATA,
        // Math functions
        &math::Abs::METADATA,
        &math::Round::METADATA,
        &math::Ceil::METADATA,
        &math::Floor::METADATA,
        // String functions
        &string::RegexReplace::METADATA,
        &string::Substring::METADATA,
        &string::Truncate::METADATA,
        &string::WordCount::METADATA,
        &string::SplitLines::METADATA,
        &string::Wrap::METADATA,
        &string::Center::METADATA,
        &string::StripHtml::METADATA,
        &string::StripAnsi::METADATA,
        &string::NormalizeWhitespace::METADATA,
        &string::Slugify::METADATA,
        &string::Indent::METADATA,
        &string::Dedent::METADATA,
        &string::Quote::METADATA,
        &string::EscapeQuotes::METADATA,
        &string::ToSnakeCase::METADATA,
        &string::ToCamelCase::METADATA,
        &string::ToPascalCase::METADATA,
        &string::ToKebabCase::METADATA,
        &string::PadLeft::METADATA,
        &string::PadRight::METADATA,
        &string::Repeat::METADATA,
        &string::Reverse::METADATA,
        // Array functions
        &array::ArraySum::METADATA,
        &array::ArrayAvg::METADATA,
        &array::ArrayMedian::METADATA,
        &array::ArrayMin::METADATA,
        &array::ArrayMax::METADATA,
        &array::ArrayUnique::METADATA,
        &array::ArrayFlatten::METADATA,
        // DateTime functions
        &datetime::FormatDate::METADATA,
        &datetime::GetYear::METADATA,
        &datetime::GetMonth::METADATA,
        &datetime::GetDay::METADATA,
        &datetime::GetHour::METADATA,
        &datetime::GetMinute::METADATA,
        &datetime::GetSecond::METADATA,
        // Path functions
        &path::Basename::METADATA,
        &path::Dirname::METADATA,
        &path::FileExtension::METADATA,
        &path::JoinPath::METADATA,
        &path::NormalizePath::METADATA,
        // URL functions
        &url::UrlEncode::METADATA,
        &url::UrlDecode::METADATA,
        &url::ParseUrl::METADATA,
        // Object functions
        &object::ObjectKeys::METADATA,
        &object::ObjectValues::METADATA,
        &object::ObjectFlatten::METADATA,
        // Kubernetes functions
        &kubernetes::K8sLabelSafe::METADATA,
        &kubernetes::K8sDnsLabelSafe::METADATA,
        &kubernetes::K8sAnnotationSafe::METADATA,
        // Formatting functions
        &formatting::Filesizeformat::METADATA,
        &formatting::Urlencode::METADATA,
    ]
}

/// Register all filter-functions with the MiniJinja environment.
///
/// This registers each implementation as both a function and a filter,
/// allowing users to choose their preferred syntax.
///
/// # Example
///
/// ```rust,ignore
/// use minijinja::Environment;
/// use tmpltool::filter_functions;
///
/// let mut env = Environment::new();
/// filter_functions::register_all(&mut env);
/// ```
pub fn register_all(env: &mut Environment) {
    // Phase 2: Hash functions
    hash::Md5::register(env);
    hash::Sha1::register(env);
    hash::Sha256::register(env);
    hash::Sha512::register(env);

    // Phase 3: Encoding functions
    encoding::Base64Encode::register(env);
    encoding::Base64Decode::register(env);
    encoding::HexEncode::register(env);
    encoding::HexDecode::register(env);
    encoding::EscapeHtml::register(env);
    encoding::EscapeXml::register(env);
    encoding::EscapeShell::register(env);

    // Phase 4: Serialization functions
    serialization::ToJson::register(env);
    serialization::ToYaml::register(env);
    serialization::ToToml::register(env);
    serialization::ParseJson::register(env);
    serialization::ParseYaml::register(env);
    serialization::ParseToml::register(env);

    // Phase 5: Math functions
    math::Abs::register(env);
    math::Round::register(env);
    math::Ceil::register(env);
    math::Floor::register(env);

    // Phase 6: String functions
    string::RegexReplace::register(env);
    string::Substring::register(env);
    string::Truncate::register(env);
    string::WordCount::register(env);
    string::SplitLines::register(env);
    string::Wrap::register(env);
    string::Center::register(env);
    string::StripHtml::register(env);
    string::StripAnsi::register(env);
    string::NormalizeWhitespace::register(env);

    // Phase 7: Array functions
    array::ArraySum::register(env);
    array::ArrayAvg::register(env);
    array::ArrayMedian::register(env);
    array::ArrayMin::register(env);
    array::ArrayMax::register(env);
    array::ArrayUnique::register(env);
    array::ArrayFlatten::register(env);

    // Phase 8: DateTime functions
    datetime::FormatDate::register(env);
    datetime::GetYear::register(env);
    datetime::GetMonth::register(env);
    datetime::GetDay::register(env);
    datetime::GetHour::register(env);
    datetime::GetMinute::register(env);
    datetime::GetSecond::register(env);

    // Phase 9: Path functions
    path::Basename::register(env);
    path::Dirname::register(env);
    path::FileExtension::register(env);
    path::JoinPath::register(env);
    path::NormalizePath::register(env);

    // Phase 10: URL functions
    url::UrlEncode::register(env);
    url::UrlDecode::register(env);
    url::ParseUrl::register(env);

    // Phase 11: Object functions
    object::ObjectKeys::register(env);
    object::ObjectValues::register(env);
    object::ObjectFlatten::register(env);

    // Phase 12: Kubernetes functions
    kubernetes::K8sLabelSafe::register(env);
    kubernetes::K8sDnsLabelSafe::register(env);
    kubernetes::K8sAnnotationSafe::register(env);

    // Formatting functions (migrated from src/filters)
    formatting::Filesizeformat::register(env);
    formatting::Urlencode::register(env);

    // String filters (migrated from src/filters)
    string::Slugify::register(env);
    string::Indent::register(env);
    string::Dedent::register(env);
    string::Quote::register(env);
    string::EscapeQuotes::register(env);
    string::ToSnakeCase::register(env);
    string::ToCamelCase::register(env);
    string::ToPascalCase::register(env);
    string::ToKebabCase::register(env);
    string::PadLeft::register(env);
    string::PadRight::register(env);
    string::Repeat::register(env);
    string::Reverse::register(env);
}
