# Removal Plan: Dead Code and Consolidation

This document analyzes the current state of functions in `tmpltool` and identifies dead code that can be removed.

## Current Architecture

### filter_functions/ (Trait-Based, Dual Syntax)
Functions implementing `FilterFunction` trait with:
- Function syntax: `{{ func(string="value") }}`
- Filter syntax: `{{ "value" | func }}`
- Required `METADATA` for IDE integration

**74 functions in 13 modules:**

| Module | Functions |
|--------|-----------|
| `hash.rs` | `Md5`, `Sha1`, `Sha256`, `Sha512` |
| `encoding.rs` | `Base64Encode`, `Base64Decode`, `HexEncode`, `HexDecode`, `EscapeHtml`, `EscapeXml`, `EscapeShell` |
| `string.rs` | `RegexReplace`, `Substring`, `Truncate`, `WordCount`, `SplitLines`, `Wrap`, `Center`, `StripHtml`, `StripAnsi`, `NormalizeWhitespace`, `Slugify`, `Indent`, `Dedent`, `Quote`, `EscapeQuotes`, `ToSnakeCase`, `ToCamelCase`, `ToPascalCase`, `ToKebabCase`, `PadLeft`, `PadRight`, `Repeat`, `Reverse` |
| `formatting.rs` | `Filesizeformat`, `Urlencode` |
| `path.rs` | `Basename`, `Dirname`, `FileExtension`, `JoinPath`, `NormalizePath` |
| `datetime.rs` | `FormatDate`, `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond` |
| `math.rs` | `Abs`, `Round`, `Ceil`, `Floor` |
| `array.rs` | `ArraySum`, `ArrayAvg`, `ArrayMedian`, `ArrayMin`, `ArrayMax`, `ArrayUnique`, `ArrayFlatten` |
| `object.rs` | `ObjectKeys`, `ObjectValues`, `ObjectFlatten` |
| `serialization.rs` | `ToJson`, `ToYaml`, `ToToml`, `ParseJson`, `ParseYaml`, `ParseToml` |
| `url.rs` | `UrlEncode`, `UrlDecode`, `ParseUrl` |
| `kubernetes.rs` | `K8sLabelSafe`, `K8sDnsLabelSafe`, `K8sAnnotationSafe` |

### is_functions/ (Trait-Based, Dual Syntax)
Functions implementing `IsFunction`/`ContextIsFunction` traits with:
- Function syntax: `{{ is_email(string="value") }}`
- Is-test syntax: `{% if "value" is email %}`
- Required `METADATA` for IDE integration

**9 functions in 4 modules:**

| Module | Functions |
|--------|-----------|
| `validation.rs` | `Email`, `Url`, `Ip`, `Uuid` |
| `datetime.rs` | `LeapYear` |
| `network.rs` | `PortAvailable` |
| `filesystem.rs` | `File`, `Dir`, `Symlink` |

### functions/ (Mixed: Active + Dead Code)
Standard functions registered via `env.add_function()`. Some are active, some are dead code (replaced by filter_functions).

---

## Dead Code Analysis

### src/functions/encoding.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `base64_encode_fn` | **DEAD** | `filter_functions::encoding::Base64Encode` |
| `base64_decode_fn` | **DEAD** | `filter_functions::encoding::Base64Decode` |
| `hex_encode_fn` | **DEAD** | `filter_functions::encoding::HexEncode` |
| `hex_decode_fn` | **DEAD** | `filter_functions::encoding::HexDecode` |
| `escape_html_fn` | **DEAD** | `filter_functions::encoding::EscapeHtml` |
| `escape_xml_fn` | **DEAD** | `filter_functions::encoding::EscapeXml` |
| `escape_shell_fn` | **DEAD** | `filter_functions::encoding::EscapeShell` |
| `bcrypt_fn` | **ACTIVE** | - |
| `generate_secret_fn` | **ACTIVE** | - |
| `hmac_sha256_fn` | **ACTIVE** | - |

### src/functions/datetime.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `now_fn` | **ACTIVE** | - |
| `format_date_fn` | **DEAD** | `filter_functions::datetime::FormatDate` |
| `parse_date_fn` | **ACTIVE** | - |
| `date_add_fn` | **ACTIVE** | - |
| `date_diff_fn` | **ACTIVE** | - |
| `get_year_fn` | **DEAD** | `filter_functions::datetime::GetYear` |
| `get_month_fn` | **DEAD** | `filter_functions::datetime::GetMonth` |
| `get_day_fn` | **DEAD** | `filter_functions::datetime::GetDay` |
| `get_hour_fn` | **DEAD** | `filter_functions::datetime::GetHour` |
| `get_minute_fn` | **DEAD** | `filter_functions::datetime::GetMinute` |
| `timezone_convert_fn` | **ACTIVE** | - |

**Note:** `get_second_fn` doesn't exist in functions/ but exists in filter_functions.

### src/functions/filesystem.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `create_read_file_fn` | **ACTIVE** | - |
| `create_file_exists_fn` | **ACTIVE** | - |
| `create_list_dir_fn` | **ACTIVE** | - |
| `create_glob_fn` | **ACTIVE** | - |
| `create_file_size_fn` | **ACTIVE** | - |
| `create_file_modified_fn` | **ACTIVE** | - |
| `create_read_lines_fn` | **ACTIVE** | - |
| `basename_fn` | **DEAD** | `filter_functions::path::Basename` |
| `dirname_fn` | **DEAD** | `filter_functions::path::Dirname` |
| `file_extension_fn` | **DEAD** | `filter_functions::path::FileExtension` |
| `join_path_fn` | **DEAD** | `filter_functions::path::JoinPath` |
| `normalize_path_fn` | **DEAD** | `filter_functions::path::NormalizePath` |
| `validate_path_security` | **ACTIVE** (helper) | - |

### src/functions/data_parsing.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `parse_json_fn` | **DEAD** | `filter_functions::serialization::ParseJson` |
| `parse_yaml_fn` | **DEAD** | `filter_functions::serialization::ParseYaml` |
| `parse_toml_fn` | **DEAD** | `filter_functions::serialization::ParseToml` |
| `create_read_json_file_fn` | **ACTIVE** | - |
| `create_read_yaml_file_fn` | **ACTIVE** | - |
| `create_read_toml_file_fn` | **ACTIVE** | - |

### src/functions/string.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `regex_replace_fn` | **DEAD** | `filter_functions::string::RegexReplace` |
| `regex_match_fn` | **ACTIVE** | - |
| `regex_find_all_fn` | **ACTIVE** | - |
| `substring_fn` | **DEAD** | `filter_functions::string::Substring` |
| `contains_fn` | **ACTIVE** | - |
| `index_of_fn` | **ACTIVE** | - |
| `count_occurrences_fn` | **ACTIVE** | - |
| `truncate_fn` | **DEAD** | `filter_functions::string::Truncate` |
| `word_count_fn` | **DEAD** | `filter_functions::string::WordCount` |
| `split_lines_fn` | **DEAD** | `filter_functions::string::SplitLines` |
| `wrap_fn` | **DEAD** | `filter_functions::string::Wrap` |
| `center_fn` | **DEAD** | `filter_functions::string::Center` |
| `sentence_case_fn` | **ACTIVE** | - |
| `strip_html_fn` | **DEAD** | `filter_functions::string::StripHtml` |
| `strip_ansi_fn` | **DEAD** | `filter_functions::string::StripAnsi` |
| `normalize_whitespace_fn` | **DEAD** | `filter_functions::string::NormalizeWhitespace` |
| `to_constant_case_fn` | **ACTIVE** | - |
| `pluralize_fn` | **ACTIVE** | - |

### src/functions/math.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `min_fn` | **ACTIVE** | - (multi-arg) |
| `max_fn` | **ACTIVE** | - (multi-arg) |
| `abs_fn` | **DEAD** | `filter_functions::math::Abs` |
| `round_fn` | **DEAD** | `filter_functions::math::Round` |
| `ceil_fn` | **DEAD** | `filter_functions::math::Ceil` |
| `floor_fn` | **DEAD** | `filter_functions::math::Floor` |
| `percentage_fn` | **ACTIVE** | - (multi-arg) |

### src/functions/array.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `array_count_fn` | **ACTIVE** | - |
| `array_chunk_fn` | **ACTIVE** | - |
| `array_zip_fn` | **ACTIVE** | - |
| `array_sort_by_fn` | **ACTIVE** | - |
| `array_group_by_fn` | **ACTIVE** | - |
| `array_unique_fn` | **DEAD** | `filter_functions::array::ArrayUnique` |
| `array_flatten_fn` | **DEAD** | `filter_functions::array::ArrayFlatten` |
| `array_take_fn` | **ACTIVE** | - |
| `array_drop_fn` | **ACTIVE** | - |
| `array_index_of_fn` | **ACTIVE** | - |
| `array_find_fn` | **ACTIVE** | - |
| `array_filter_by_fn` | **ACTIVE** | - |
| `array_pluck_fn` | **ACTIVE** | - |
| `array_intersection_fn` | **ACTIVE** | - |
| `array_difference_fn` | **ACTIVE** | - |
| `array_union_fn` | **ACTIVE** | - |
| `array_symmetric_difference_fn` | **ACTIVE** | - |

### src/functions/object.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `object_merge_fn` | **ACTIVE** | - |
| `object_get_fn` | **ACTIVE** | - |
| `object_set_fn` | **ACTIVE** | - |
| `object_keys_fn` | **DEAD** | `filter_functions::object::ObjectKeys` |
| `object_values_fn` | **DEAD** | `filter_functions::object::ObjectValues` |
| `object_has_key_fn` | **ACTIVE** | - |
| `json_path_fn` | **ACTIVE** | - |
| `object_pick_fn` | **ACTIVE** | - |
| `object_omit_fn` | **ACTIVE** | - |
| `object_rename_keys_fn` | **ACTIVE** | - |
| `object_flatten_fn` | **DEAD** | `filter_functions::object::ObjectFlatten` |
| `object_unflatten_fn` | **ACTIVE** | - |

### src/functions/kubernetes.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `k8s_resource_request_fn` | **ACTIVE** | - |
| `k8s_label_safe_fn` | **DEAD** | `filter_functions::kubernetes::K8sLabelSafe` |
| `k8s_dns_label_safe_fn` | **DEAD** | `filter_functions::kubernetes::K8sDnsLabelSafe` |
| `k8s_env_var_ref_fn` | **ACTIVE** | - |
| `k8s_secret_ref_fn` | **ACTIVE** | - |
| `k8s_configmap_ref_fn` | **ACTIVE** | - |
| `helm_tpl_fn` | **ACTIVE** | - |
| `k8s_annotation_safe_fn` | **DEAD** | `filter_functions::kubernetes::K8sAnnotationSafe` |
| `k8s_quantity_to_bytes_fn` | **ACTIVE** | - |
| `k8s_bytes_to_quantity_fn` | **ACTIVE** | - |
| `k8s_selector_fn` | **ACTIVE** | - |
| `k8s_pod_affinity_fn` | **ACTIVE** | - |
| `k8s_toleration_fn` | **ACTIVE** | - |
| `k8s_probe_fn` | **ACTIVE** | - |

### src/functions/url.rs

| Function | Status | Replaced By |
|----------|--------|-------------|
| `basic_auth_fn` | **ACTIVE** | - |
| `parse_url_fn` | **DEAD** | `filter_functions::url::ParseUrl` |
| `build_url_fn` | **ACTIVE** | - |
| `query_string_fn` | **ACTIVE** | - |

### Fully Active Modules (No Dead Code)

These modules have no duplicates in filter_functions:

- `src/functions/environment.rs` - `env_fn`, `filter_env_fn`
- `src/functions/system.rs` - All 7 functions active
- `src/functions/network.rs` - All 9 functions active
- `src/functions/debug.rs` - All 6 functions active
- `src/functions/predicates.rs` - All 5 functions active
- `src/functions/logic.rs` - All 4 functions active
- `src/functions/random.rs` - All 2 functions active
- `src/functions/uuid_gen.rs` - `uuid_fn` active
- `src/functions/validation.rs` - `matches_regex_fn` active
- `src/functions/exec.rs` - Both functions active

---

## Summary: Dead Code to Remove

### Total Dead Functions: 43

| Module | Dead Functions | Lines to Remove (approx) |
|--------|----------------|--------------------------|
| `encoding.rs` | 7 | ~200 |
| `datetime.rs` | 6 | ~120 |
| `filesystem.rs` | 5 | ~120 |
| `data_parsing.rs` | 3 | ~60 |
| `string.rs` | 10 | ~300 |
| `math.rs` | 4 | ~150 |
| `array.rs` | 2 | ~80 |
| `object.rs` | 3 | ~100 |
| `kubernetes.rs` | 3 | ~150 |
| `url.rs` | 1 | ~50 |

**Estimated total: ~1,330 lines of dead code**

---

## Recommended Cleanup Steps

### Phase 1: Remove Dead Functions ✅ COMPLETE

Dead code has been removed (commit `0999b97`):
- Removed 43 dead functions (~3,500 lines)
- Updated 10 test files to remove tests for removed functions
- All 1,100+ tests pass

### Phase 2: Consider Further Consolidation
Some active functions could be migrated to `filter_functions/` if dual syntax would be useful:

**Good candidates for migration (benefit from filter syntax):**
- `sentence_case` - `{{ "hello world" | sentence_case }}`
- `to_constant_case` - `{{ "hello world" | to_constant_case }}`
- `pluralize` - `{{ "apple" | pluralize(count=5) }}`

**Poor candidates (multi-arg or context-aware):**
- `min`, `max`, `percentage` - require multiple arguments
- `read_file`, `file_exists`, etc. - require context
- `object_merge`, `object_set` - complex multi-arg

### Phase 3: Add Missing Metadata ✅ PARTIALLY COMPLETE

The active functions in `functions/` directory now have metadata via `Function` trait.

**Completed migrations (now have METADATA):**
- `environment.rs`: `GetEnv`, `FilterEnv`
- `random.rs`: `GetRandom`, `RandomString`
- `uuid_gen.rs`: `UuidGen`
- `validation.rs`: `MatchesRegex`
- `system.rs`: `GetHostname`, `GetUsername`, `GetHomeDir`, `GetTempDir`, `GetOs`, `GetArch`, `GetCwd`
- `network.rs`: `GetIpAddress`, `GetInterfaces`, `ResolveDns`, `CidrContains`, `CidrNetwork`, `CidrBroadcast`, `CidrNetmask`, `IpToInt`, `IntToIp`
- `debug.rs`: `Debug`, `TypeOf`, `Inspect`, `Assert`, `Warn`, `Abort`
- `predicates.rs`: `ArrayAny`, `ArrayAll`, `ArrayContains`, `StartsWith`, `EndsWith`
- `logic.rs`: `Default`, `Coalesce`, `Ternary`, `InRange`
- `datetime.rs`: `Now`, `ParseDate`, `DateAdd`, `DateDiff`, `TimezoneConvert`
- `encoding.rs`: `Bcrypt`, `GenerateSecret`, `HmacSha256`
- `math.rs`: `Min`, `Max`, `Percentage`

**Total: 45 functions now have trait-based metadata**

**Added infrastructure:**
- `get_all_metadata()` function in `src/functions/mod.rs` collects all metadata
- Metadata accessible via `tmpltool::functions::get_all_metadata()`

**Remaining modules to migrate (still use legacy function style):**
- `string.rs` - 7 active functions
- `array.rs` - 15 active functions
- `object.rs` - 9 active functions
- `kubernetes.rs` - 11 active functions
- `url.rs` - 3 active functions
- `filesystem.rs` - 7 context-aware functions (need `ContextFunction` trait)
- `data_parsing.rs` - 3 context-aware functions
- `exec.rs` - 2 context-aware functions

---

## Files to Modify

When implementing removal:

1. `src/functions/encoding.rs` - Remove 7 functions
2. `src/functions/datetime.rs` - Remove 6 functions
3. `src/functions/filesystem.rs` - Remove 5 functions
4. `src/functions/data_parsing.rs` - Remove 3 functions
5. `src/functions/string.rs` - Remove 10 functions
6. `src/functions/math.rs` - Remove 4 functions
7. `src/functions/array.rs` - Remove 2 functions
8. `src/functions/object.rs` - Remove 3 functions
9. `src/functions/kubernetes.rs` - Remove 3 functions
10. `src/functions/url.rs` - Remove 1 function
