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
//! - `md5(string)` - Calculate MD5 hash of a string
//! - `sha1(string)` - Calculate SHA1 hash of a string
//! - `sha256(string)` - Calculate SHA256 hash of a string
//! - `sha512(string)` - Calculate SHA512 hash of a string
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
pub mod hash;
pub mod kubernetes;
pub mod logic;
pub mod math;
pub mod network;
pub mod object;
pub mod predicates;
pub mod random;
pub mod serialization;
pub mod statistics;
pub mod string;
pub mod system;
pub mod url;
pub mod uuid_gen;
pub mod validation;

use crate::TemplateContext;
use minijinja::Environment;

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

    // Register built-in function replacements (were built-in in Tera)
    env.add_function("get_env", environment::env_fn);
    env.add_function("now", datetime::now_fn);
    env.add_function("get_random", random::get_random_fn);

    // Date/Time functions
    env.add_function("format_date", datetime::format_date_fn);
    env.add_function("parse_date", datetime::parse_date_fn);
    env.add_function("date_add", datetime::date_add_fn);
    env.add_function("date_diff", datetime::date_diff_fn);
    env.add_function("get_year", datetime::get_year_fn);
    env.add_function("get_month", datetime::get_month_fn);
    env.add_function("get_day", datetime::get_day_fn);
    env.add_function("get_hour", datetime::get_hour_fn);
    env.add_function("get_minute", datetime::get_minute_fn);
    env.add_function("timezone_convert", datetime::timezone_convert_fn);
    env.add_function("is_leap_year", datetime::is_leap_year_fn);

    // Register custom functions (simple, no context needed)
    env.add_function("filter_env", environment::filter_env_fn);
    env.add_function("md5", hash::md5_fn);
    env.add_function("sha1", hash::sha1_fn);
    env.add_function("sha256", hash::sha256_fn);
    env.add_function("sha512", hash::sha512_fn);
    env.add_function("uuid", uuid_gen::uuid_fn);
    env.add_function("random_string", random::random_string_fn);

    // Validation functions
    env.add_function("is_email", validation::is_email_fn);
    env.add_function("is_url", validation::is_url_fn);
    env.add_function("is_ip", validation::is_ip_fn);
    env.add_function("is_uuid", validation::is_uuid_fn);
    env.add_function("matches_regex", validation::matches_regex_fn);

    // System information functions
    env.add_function("get_hostname", system::get_hostname_fn);
    env.add_function("get_username", system::get_username_fn);
    env.add_function("get_home_dir", system::get_home_dir_fn);
    env.add_function("get_temp_dir", system::get_temp_dir_fn);

    // Network functions
    env.add_function("get_ip_address", network::get_ip_address_fn);
    env.add_function("resolve_dns", network::resolve_dns_fn);
    env.add_function("is_port_available", network::is_port_available_fn);

    // Data parsing functions (simple, no context)
    env.add_function("parse_json", data_parsing::parse_json_fn);
    env.add_function("parse_yaml", data_parsing::parse_yaml_fn);
    env.add_function("parse_toml", data_parsing::parse_toml_fn);

    // File system functions (need context)
    let context_arc = Arc::new(context);
    env.add_function(
        "read_file",
        filesystem::create_read_file_fn(context_arc.clone()),
    );
    env.add_function(
        "file_exists",
        filesystem::create_file_exists_fn(context_arc.clone()),
    );
    env.add_function(
        "list_dir",
        filesystem::create_list_dir_fn(context_arc.clone()),
    );
    env.add_function("glob", filesystem::create_glob_fn(context_arc.clone()));
    env.add_function(
        "file_size",
        filesystem::create_file_size_fn(context_arc.clone()),
    );
    env.add_function(
        "file_modified",
        filesystem::create_file_modified_fn(context_arc.clone()),
    );
    env.add_function(
        "is_file",
        filesystem::create_is_file_fn(context_arc.clone()),
    );
    env.add_function("is_dir", filesystem::create_is_dir_fn(context_arc.clone()));
    env.add_function(
        "is_symlink",
        filesystem::create_is_symlink_fn(context_arc.clone()),
    );
    env.add_function(
        "read_lines",
        filesystem::create_read_lines_fn(context_arc.clone()),
    );

    // Path utility functions (simple, no context)
    env.add_function("basename", filesystem::basename_fn);
    env.add_function("dirname", filesystem::dirname_fn);
    env.add_function("file_extension", filesystem::file_extension_fn);
    env.add_function("join_path", filesystem::join_path_fn);
    env.add_function("normalize_path", filesystem::normalize_path_fn);

    // Data parsing file functions (need context)
    env.add_function(
        "read_json_file",
        data_parsing::create_read_json_file_fn(context_arc.clone()),
    );
    env.add_function(
        "read_yaml_file",
        data_parsing::create_read_yaml_file_fn(context_arc.clone()),
    );
    env.add_function(
        "read_toml_file",
        data_parsing::create_read_toml_file_fn(context_arc.clone()),
    );

    // Execution functions (need context)
    env.add_function("exec", exec::create_exec_fn(context_arc.clone()));
    env.add_function("exec_raw", exec::create_exec_raw_fn(context_arc));

    // Encoding and security functions
    env.add_function("base64_encode", encoding::base64_encode_fn);
    env.add_function("base64_decode", encoding::base64_decode_fn);
    env.add_function("hex_encode", encoding::hex_encode_fn);
    env.add_function("hex_decode", encoding::hex_decode_fn);
    env.add_function("bcrypt", encoding::bcrypt_fn);
    env.add_function("generate_secret", encoding::generate_secret_fn);
    env.add_function("hmac_sha256", encoding::hmac_sha256_fn);
    env.add_function("escape_html", encoding::escape_html_fn);
    env.add_function("escape_xml", encoding::escape_xml_fn);
    env.add_function("escape_shell", encoding::escape_shell_fn);

    // Debug and development functions
    env.add_function("debug", debug::debug_fn);
    env.add_function("type_of", debug::type_of_fn);
    env.add_function("inspect", debug::inspect_fn);
    env.add_function("assert", debug::assert_fn);
    env.add_function("warn", debug::warn_fn);
    env.add_function("abort", debug::abort_fn);

    // Serialization functions
    env.add_function("to_json", serialization::to_json_fn);
    env.add_function("to_yaml", serialization::to_yaml_fn);
    env.add_function("to_toml", serialization::to_toml_fn);

    // Object manipulation functions
    env.add_function("object_merge", object::object_merge_fn);
    env.add_function("object_get", object::object_get_fn);
    env.add_function("object_set", object::object_set_fn);
    env.add_function("object_keys", object::object_keys_fn);
    env.add_function("object_values", object::object_values_fn);
    env.add_function("object_has_key", object::object_has_key_fn);

    // Predicate functions
    env.add_function("array_any", predicates::array_any_fn);
    env.add_function("array_all", predicates::array_all_fn);
    env.add_function("array_contains", predicates::array_contains_fn);
    env.add_function("starts_with", predicates::starts_with_fn);
    env.add_function("ends_with", predicates::ends_with_fn);

    // Statistical functions
    env.add_function("array_sum", statistics::array_sum_fn);
    env.add_function("array_avg", statistics::array_avg_fn);
    env.add_function("array_median", statistics::array_median_fn);
    env.add_function("array_min", statistics::array_min_fn);
    env.add_function("array_max", statistics::array_max_fn);

    // Array manipulation functions
    env.add_function("array_count", array::array_count_fn);
    env.add_function("array_chunk", array::array_chunk_fn);
    env.add_function("array_zip", array::array_zip_fn);
    env.add_function("array_sort_by", array::array_sort_by_fn);
    env.add_function("array_group_by", array::array_group_by_fn);
    env.add_function("array_unique", array::array_unique_fn);
    env.add_function("array_flatten", array::array_flatten_fn);
    env.add_function("array_take", array::array_take_fn);
    env.add_function("array_drop", array::array_drop_fn);
    env.add_function("array_index_of", array::array_index_of_fn);
    env.add_function("array_find", array::array_find_fn);
    env.add_function("array_filter_by", array::array_filter_by_fn);
    env.add_function("array_pluck", array::array_pluck_fn);

    // Set operations
    env.add_function("array_intersection", array::array_intersection_fn);
    env.add_function("array_difference", array::array_difference_fn);
    env.add_function("array_union", array::array_union_fn);
    env.add_function(
        "array_symmetric_difference",
        array::array_symmetric_difference_fn,
    );

    // String manipulation functions
    env.add_function("regex_replace", string::regex_replace_fn);
    env.add_function("regex_match", string::regex_match_fn);
    env.add_function("regex_find_all", string::regex_find_all_fn);
    env.add_function("substring", string::substring_fn);
    env.add_function("contains", string::contains_fn);
    env.add_function("index_of", string::index_of_fn);
    env.add_function("count_occurrences", string::count_occurrences_fn);
    env.add_function("truncate", string::truncate_fn);
    env.add_function("word_count", string::word_count_fn);
    env.add_function("split_lines", string::split_lines_fn);

    // Math functions
    env.add_function("min", math::min_fn);
    env.add_function("max", math::max_fn);
    env.add_function("abs", math::abs_fn);
    env.add_function("round", math::round_fn);
    env.add_function("ceil", math::ceil_fn);
    env.add_function("floor", math::floor_fn);
    env.add_function("percentage", math::percentage_fn);

    // Logic functions
    env.add_function("default", logic::default_fn);
    env.add_function("coalesce", logic::coalesce_fn);
    env.add_function("ternary", logic::ternary_fn);
    env.add_function("in_range", logic::in_range_fn);

    // Kubernetes functions
    env.add_function("k8s_resource_request", kubernetes::k8s_resource_request_fn);
    env.add_function("k8s_label_safe", kubernetes::k8s_label_safe_fn);
    env.add_function("k8s_dns_label_safe", kubernetes::k8s_dns_label_safe_fn);
    env.add_function("k8s_env_var_ref", kubernetes::k8s_env_var_ref_fn);
    env.add_function("k8s_secret_ref", kubernetes::k8s_secret_ref_fn);
    env.add_function("k8s_configmap_ref", kubernetes::k8s_configmap_ref_fn);

    // URL and HTTP utility functions
    env.add_function("basic_auth", url::basic_auth_fn);
    env.add_function("parse_url", url::parse_url_fn);
    env.add_function("build_url", url::build_url_fn);
    env.add_function("query_string", url::query_string_fn);

    // Register custom filters from the filters module
    crate::filters::register_all(env);
}
