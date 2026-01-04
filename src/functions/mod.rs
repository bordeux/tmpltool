//! Custom MiniJinja functions for tmpltool
//!
//! This module contains all custom functions that can be used in MiniJinja templates.
//! Each function is defined in its own file for better organization and maintainability.
//!
//! # Built-in Functions
//!
//! MiniJinja provides these functions (implemented in the builtins module):
//! - `env(name, default)` - Get environment variables with optional default values
//! - `now()` - Get current timestamp
//! - `get_random(start, end)` - Generate random integers
//!
//! # Custom Functions
//!
//! tmpltool provides additional custom functions:
//! - `filter_env(pattern)` - Filter environment variables by glob pattern (e.g., "SERVER_*")
//! - `uuid(version)` - Generate a UUID (v4 default, or v7 for time-ordered)
//! - `random_string(length, charset)` - Generate a random string with custom length and character set
//! - `read_file(path)` - Read content from a file
//! - `file_exists(path)` - Check if a file exists
//! - `list_dir(path)` - List files in a directory
//! - `glob(pattern)` - List files matching a glob pattern
//! - `file_size(path)` - Get file size in bytes
//! - `file_modified(path)` - Get file modification timestamp
//! - `is_email(string)` - Validate email address format
//! - `is_url(string)` - Validate URL format
//! - `is_ip(string)` - Validate IP address (IPv4 or IPv6)
//! - `is_uuid(string)` - Validate UUID format
//! - `matches_regex(pattern, string)` - Check if string matches regex pattern
//! - `parse_json(string)` - Parse JSON string into object
//! - `parse_yaml(string)` - Parse YAML string into object
//! - `parse_toml(string)` - Parse TOML string into object
//! - `read_json_file(path)` - Read and parse JSON file
//! - `read_yaml_file(path)` - Read and parse YAML file
//! - `read_toml_file(path)` - Read and parse TOML file
//! - `get_hostname()` - Get system hostname
//! - `get_username()` - Get current system username
//! - `get_home_dir()` - Get user's home directory
//! - `get_temp_dir()` - Get system temporary directory
//! - `get_ip_address(interface)` - Get IP address of network interface
//! - `get_interfaces()` - List all network interfaces with IPs
//! - `resolve_dns(hostname)` - Resolve hostname to IP address
//! - `is_port_available(port)` - Check if port is available
//!
//! # Adding Custom Functions
//!
//! To add a new custom function:
//!
//! 1. Create a new file in `src/functions/` (e.g., `my_function.rs`)
//! 2. Implement your function using MiniJinja's Kwargs pattern
//! 3. Add `pub mod my_function;` to this file
//! 4. Add your function to the `register_all()` function below
//!
//! # Example
//!
//! ```rust
//! // In src/functions/my_function.rs
//! use minijinja::value::Kwargs;
//! use minijinja::{Error, Value};
//!
//! pub fn my_function(kwargs: Kwargs) -> Result<Value, Error> {
//!     let input: String = kwargs.get("input")?;
//!     // Your implementation here
//!     Ok(Value::from("result"))
//! }
//! ```

pub mod array;
pub mod data_parsing;
pub mod datetime;
pub mod debug;
pub mod encoding;
pub mod environment;
pub mod exec;
pub mod filesystem;
pub mod kubernetes;
pub mod logic;
pub mod math;
pub mod metadata;
pub mod network;
pub mod object;
pub mod predicates;
pub mod random;
pub mod string;
pub mod system;
pub mod traits;
pub mod url;
pub mod uuid_gen;
pub mod validation;

use crate::TemplateContext;
use minijinja::Environment;

// Re-export metadata types for external use
pub use metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
pub use traits::{ContextFunction, Function};

/// Collect all function metadata for IDE integration
///
/// This function gathers metadata from all functions that have been migrated
/// to use the `Function` trait with metadata. This enables IDE features like
/// autocomplete and documentation.
///
/// # Returns
///
/// Returns a vector of references to FunctionMetadata for all registered functions.
pub fn get_all_metadata() -> Vec<&'static FunctionMetadata> {
    vec![
        // Environment functions
        &environment::GetEnv::METADATA,
        &environment::FilterEnv::METADATA,
        // Random/UUID functions
        &random::GetRandom::METADATA,
        &random::RandomString::METADATA,
        &uuid_gen::UuidGen::METADATA,
        // Validation functions
        &validation::MatchesRegex::METADATA,
        // System functions
        &system::GetHostname::METADATA,
        &system::GetUsername::METADATA,
        &system::GetHomeDir::METADATA,
        &system::GetTempDir::METADATA,
        &system::GetOs::METADATA,
        &system::GetArch::METADATA,
        &system::GetCwd::METADATA,
        // Network functions
        &network::GetIpAddress::METADATA,
        &network::GetInterfaces::METADATA,
        &network::ResolveDns::METADATA,
        &network::CidrContains::METADATA,
        &network::CidrNetwork::METADATA,
        &network::CidrBroadcast::METADATA,
        &network::CidrNetmask::METADATA,
        &network::IpToInt::METADATA,
        &network::IntToIp::METADATA,
        // Debug functions
        &debug::Debug::METADATA,
        &debug::TypeOf::METADATA,
        &debug::Inspect::METADATA,
        &debug::Assert::METADATA,
        &debug::Warn::METADATA,
        &debug::Abort::METADATA,
        // Predicate functions
        &predicates::ArrayAny::METADATA,
        &predicates::ArrayAll::METADATA,
        &predicates::ArrayContains::METADATA,
        &predicates::StartsWith::METADATA,
        &predicates::EndsWith::METADATA,
        // Logic functions
        &logic::Default::METADATA,
        &logic::Coalesce::METADATA,
        &logic::Ternary::METADATA,
        &logic::InRange::METADATA,
        // DateTime functions
        &datetime::Now::METADATA,
        &datetime::ParseDate::METADATA,
        &datetime::DateAdd::METADATA,
        &datetime::DateDiff::METADATA,
        &datetime::TimezoneConvert::METADATA,
        // Encoding functions
        &encoding::Bcrypt::METADATA,
        &encoding::GenerateSecret::METADATA,
        &encoding::HmacSha256::METADATA,
        // Math functions
        &math::Min::METADATA,
        &math::Max::METADATA,
        &math::Percentage::METADATA,
        // String functions
        &string::RegexMatch::METADATA,
        &string::RegexFindAll::METADATA,
        &string::Contains::METADATA,
        &string::IndexOf::METADATA,
        &string::CountOccurrences::METADATA,
        &string::SentenceCase::METADATA,
        &string::ToConstantCase::METADATA,
        &string::Pluralize::METADATA,
        // Array functions
        &array::ArrayCount::METADATA,
        &array::ArrayChunk::METADATA,
        &array::ArrayZip::METADATA,
        &array::ArraySortBy::METADATA,
        &array::ArrayGroupBy::METADATA,
        &array::ArrayTake::METADATA,
        &array::ArrayDrop::METADATA,
        &array::ArrayIndexOf::METADATA,
        &array::ArrayFind::METADATA,
        &array::ArrayFilterBy::METADATA,
        &array::ArrayPluck::METADATA,
        &array::ArrayIntersection::METADATA,
        &array::ArrayDifference::METADATA,
        &array::ArrayUnion::METADATA,
        &array::ArraySymmetricDifference::METADATA,
        // Object functions
        &object::ObjectMerge::METADATA,
        &object::ObjectGet::METADATA,
        &object::ObjectSet::METADATA,
        &object::ObjectHasKey::METADATA,
        &object::JsonPath::METADATA,
        &object::ObjectPick::METADATA,
        &object::ObjectOmit::METADATA,
        &object::ObjectRenameKeys::METADATA,
        &object::ObjectUnflatten::METADATA,
        // Kubernetes functions
        &kubernetes::K8sResourceRequest::METADATA,
        &kubernetes::K8sEnvVarRef::METADATA,
        &kubernetes::K8sSecretRef::METADATA,
        &kubernetes::K8sConfigmapRef::METADATA,
        &kubernetes::HelmTpl::METADATA,
        &kubernetes::K8sQuantityToBytes::METADATA,
        &kubernetes::K8sBytesToQuantity::METADATA,
        &kubernetes::K8sSelector::METADATA,
        &kubernetes::K8sPodAffinity::METADATA,
        &kubernetes::K8sToleration::METADATA,
        &kubernetes::K8sProbe::METADATA,
        // URL functions
        &url::BasicAuth::METADATA,
        &url::BuildUrl::METADATA,
        &url::QueryString::METADATA,
        // Filesystem functions (context-aware)
        &filesystem::ReadFile::METADATA,
        &filesystem::FileExists::METADATA,
        &filesystem::ListDir::METADATA,
        &filesystem::Glob::METADATA,
        &filesystem::FileSize::METADATA,
        &filesystem::FileModified::METADATA,
        &filesystem::ReadLines::METADATA,
        // Data parsing functions (context-aware)
        &data_parsing::ReadJsonFile::METADATA,
        &data_parsing::ReadYamlFile::METADATA,
        &data_parsing::ReadTomlFile::METADATA,
        // Exec functions (context-aware)
        &exec::Exec::METADATA,
        &exec::ExecRaw::METADATA,
    ]
}

/// Register all custom functions with the MiniJinja environment
///
/// This function is called when setting up a MiniJinja environment to register
/// all custom functions, including built-in function replacements.
///
/// # Arguments
///
/// * `env` - Mutable reference to a MiniJinja Environment
/// * `context` - Template context with base directory and trust mode settings
///
/// # Example
///
/// ```
/// use minijinja::Environment;
/// use tmpltool::{TemplateContext, functions::register_all};
/// use std::path::PathBuf;
///
/// let mut env = Environment::new();
/// let ctx = TemplateContext::new(PathBuf::from("."), false);
/// register_all(&mut env, ctx);
/// ```
pub fn register_all(env: &mut Environment, context: TemplateContext) {
    use std::sync::Arc;
    use traits::{ContextFunction, Function};

    // Register filter-functions (functions that also work as filters)
    crate::filter_functions::register_all(env);

    // Create Arc for context-aware functions
    let context_arc = Arc::new(context);

    // Register is-functions (functions that also work as "is" tests)
    crate::is_functions::register_all(env, context_arc.clone());

    // ===== Simple Functions (no context needed) =====

    // Environment functions
    environment::GetEnv::register(env);
    environment::FilterEnv::register(env);

    // Random/UUID functions
    random::GetRandom::register(env);
    random::RandomString::register(env);
    uuid_gen::UuidGen::register(env);

    // DateTime functions
    datetime::Now::register(env);
    datetime::ParseDate::register(env);
    datetime::DateAdd::register(env);
    datetime::DateDiff::register(env);
    datetime::TimezoneConvert::register(env);

    // Validation functions
    validation::MatchesRegex::register(env);

    // System functions
    system::GetHostname::register(env);
    system::GetUsername::register(env);
    system::GetHomeDir::register(env);
    system::GetTempDir::register(env);
    system::GetOs::register(env);
    system::GetArch::register(env);
    system::GetCwd::register(env);

    // Network functions
    network::GetIpAddress::register(env);
    network::GetInterfaces::register(env);
    network::ResolveDns::register(env);
    network::CidrContains::register(env);
    network::CidrNetwork::register(env);
    network::CidrBroadcast::register(env);
    network::CidrNetmask::register(env);
    network::IpToInt::register(env);
    network::IntToIp::register(env);

    // Encoding functions
    encoding::Bcrypt::register(env);
    encoding::GenerateSecret::register(env);
    encoding::HmacSha256::register(env);

    // Debug functions
    debug::Debug::register(env);
    debug::TypeOf::register(env);
    debug::Inspect::register(env);
    debug::Assert::register(env);
    debug::Warn::register(env);
    debug::Abort::register(env);

    // Predicate functions
    predicates::ArrayAny::register(env);
    predicates::ArrayAll::register(env);
    predicates::ArrayContains::register(env);
    predicates::StartsWith::register(env);
    predicates::EndsWith::register(env);

    // Logic functions
    logic::Default::register(env);
    logic::Coalesce::register(env);
    logic::Ternary::register(env);
    logic::InRange::register(env);

    // Math functions
    math::Min::register(env);
    math::Max::register(env);
    math::Percentage::register(env);

    // String functions
    string::RegexMatch::register(env);
    string::RegexFindAll::register(env);
    string::Contains::register(env);
    string::IndexOf::register(env);
    string::CountOccurrences::register(env);
    string::SentenceCase::register(env);
    string::ToConstantCase::register(env);
    string::Pluralize::register(env);

    // Array functions
    array::ArrayCount::register(env);
    array::ArrayChunk::register(env);
    array::ArrayZip::register(env);
    array::ArraySortBy::register(env);
    array::ArrayGroupBy::register(env);
    array::ArrayTake::register(env);
    array::ArrayDrop::register(env);
    array::ArrayIndexOf::register(env);
    array::ArrayFind::register(env);
    array::ArrayFilterBy::register(env);
    array::ArrayPluck::register(env);
    array::ArrayIntersection::register(env);
    array::ArrayDifference::register(env);
    array::ArrayUnion::register(env);
    array::ArraySymmetricDifference::register(env);

    // Object functions
    object::ObjectMerge::register(env);
    object::ObjectGet::register(env);
    object::ObjectSet::register(env);
    object::ObjectHasKey::register(env);
    object::JsonPath::register(env);
    object::ObjectPick::register(env);
    object::ObjectOmit::register(env);
    object::ObjectRenameKeys::register(env);
    object::ObjectUnflatten::register(env);

    // Kubernetes functions
    kubernetes::K8sResourceRequest::register(env);
    kubernetes::K8sEnvVarRef::register(env);
    kubernetes::K8sSecretRef::register(env);
    kubernetes::K8sConfigmapRef::register(env);
    kubernetes::HelmTpl::register(env);
    kubernetes::K8sQuantityToBytes::register(env);
    kubernetes::K8sBytesToQuantity::register(env);
    kubernetes::K8sSelector::register(env);
    kubernetes::K8sPodAffinity::register(env);
    kubernetes::K8sToleration::register(env);
    kubernetes::K8sProbe::register(env);

    // URL functions
    url::BasicAuth::register(env);
    url::BuildUrl::register(env);
    url::QueryString::register(env);

    // ===== Context-Aware Functions (need filesystem/trust mode access) =====

    // Filesystem functions
    filesystem::ReadFile::register(env, context_arc.clone());
    filesystem::FileExists::register(env, context_arc.clone());
    filesystem::ListDir::register(env, context_arc.clone());
    filesystem::Glob::register(env, context_arc.clone());
    filesystem::FileSize::register(env, context_arc.clone());
    filesystem::FileModified::register(env, context_arc.clone());
    filesystem::ReadLines::register(env, context_arc.clone());

    // Data parsing file functions
    data_parsing::ReadJsonFile::register(env, context_arc.clone());
    data_parsing::ReadYamlFile::register(env, context_arc.clone());
    data_parsing::ReadTomlFile::register(env, context_arc.clone());

    // Execution functions
    exec::Exec::register(env, context_arc.clone());
    exec::ExecRaw::register(env, context_arc);
}
