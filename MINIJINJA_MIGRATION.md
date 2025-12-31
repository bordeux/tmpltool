# MiniJinja Migration Plan

## Executive Summary

This document outlines the plan to migrate tmpltool from [Tera](https://github.com/Keats/tera) to [MiniJinja](https://github.com/mitsuhiko/minijinja) template engine.

**Migration Difficulty:** MODERATE (3-5 days)
**Risk Level:** MEDIUM
**Recommended Approach:** Feature branch with comprehensive testing
**Breaking Changes:** Potentially minor template syntax differences

## Table of Contents

- [Motivation](#motivation)
- [Comparison: Tera vs MiniJinja](#comparison-tera-vs-minijinja)
- [Impact Analysis](#impact-analysis)
- [Migration Strategy](#migration-strategy)
- [Detailed Migration Steps](#detailed-migration-steps)
- [Code Changes Required](#code-changes-required)
- [Testing Strategy](#testing-strategy)
- [Risk Mitigation](#risk-mitigation)
- [Rollback Plan](#rollback-plan)
- [Timeline](#timeline)
- [Success Criteria](#success-criteria)

## Motivation

### Why Migrate?

**Advantages of MiniJinja:**

1. **Smaller Binary Size**
   - Minimal dependencies (only `serde` required)
   - Current tmpltool binary size: ~4-5 MB
   - Expected after migration: ~2-3 MB (40-50% reduction)

2. **Better Jinja2 Compatibility**
   - MiniJinja stays closer to Python's Jinja2 syntax
   - Easier for users familiar with Jinja2/Flask
   - More predictable behavior for cross-platform templates

3. **Faster Compilation**
   - Fewer dependencies = faster build times
   - Better for CI/CD pipelines
   - Reduced dependency maintenance burden

4. **Better Maintained**
   - Actively maintained by Armin Ronacher (creator of Flask, Click, Jinja2)
   - Modern codebase with excellent documentation
   - Strong community support

5. **Additional Features**
   - WASM support (can run in browser if needed)
   - Better error messages
   - Expression evaluation outside templates
   - Multi-language bindings (Python, JavaScript, C)

### Why NOT to Migrate?

**Potential Drawbacks:**

1. **Built-in Functions Missing**
   - Tera's `get_env()` is built-in, MiniJinja requires custom implementation
   - `now()` and `get_random()` also need custom implementation
   - More code to maintain

2. **Breaking Changes**
   - Minor template syntax differences possible
   - Error messages will change (tests may need updates)
   - Users may need to update templates

3. **Migration Effort**
   - 3-5 days of focused work
   - All 180 tests must pass
   - Documentation updates required

4. **Risk**
   - Potential for bugs during transition
   - Need thorough testing before release

## Comparison: Tera vs MiniJinja

### Feature Comparison

| Feature | Tera | MiniJinja | Winner |
|---------|------|-----------|--------|
| **Dependencies** | Many (pest, regex, etc.) | Minimal (serde only) | MiniJinja |
| **Binary Size** | Larger (~4-5 MB) | Smaller (~2-3 MB) | MiniJinja |
| **Jinja2 Compatibility** | Inspired, diverges | Very close | MiniJinja |
| **Built-in Functions** | Many (`get_env`, `now`, etc.) | Fewer | Tera |
| **Documentation** | Good | Excellent | MiniJinja |
| **Maintenance** | Active | Very Active | MiniJinja |
| **Error Messages** | Good | Better | MiniJinja |
| **Compile Time** | Slower | Faster | MiniJinja |
| **WASM Support** | No | Yes | MiniJinja |
| **Maturity** | Mature | Mature | Tie |

### API Comparison

**Creating Environment:**
```rust
// Tera
let mut tera = Tera::default();

// MiniJinja
let mut env = Environment::new();
```

**Adding Templates:**
```rust
// Tera
tera.add_raw_template("name", content)?;

// MiniJinja
env.add_template("name", content)?;
```

**Rendering:**
```rust
// Tera
let rendered = tera.render("name", &context)?;

// MiniJinja
let tmpl = env.get_template("name")?;
let rendered = tmpl.render(&context)?;
```

**Custom Functions:**
```rust
// Tera - Struct implementing trait
pub struct MyFunc;
impl Function for MyFunc {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        // ...
    }
}
tera.register_function("my_func", MyFunc);

// MiniJinja - Simple closure
env.add_function("my_func", |args: &HashMap<String, Value>| -> Result<Value, Error> {
    // ...
});
```

## Impact Analysis

### Files Requiring Changes

**Core Files (Must Change):**
1. `Cargo.toml` - Dependency change
2. `src/renderer.rs` - Main rendering logic
3. `src/functions/mod.rs` - Function registration
4. `src/functions/*.rs` - All 20 custom function files

**Test Files (Must Verify):**
5. All test files in `tests/` - Ensure they still pass

**Documentation (Must Update):**
6. `README.md` - Update all Tera references
7. `CONTRIBUTING.md` - Update development instructions
8. Code comments mentioning Tera

### Custom Functions to Migrate

**Total: 20 functions + 3 new built-ins**

**Existing Functions:**
1. `filter_env` - Environment variable filtering
2. `md5` - MD5 hash
3. `sha1` - SHA1 hash
4. `sha256` - SHA256 hash
5. `sha512` - SHA512 hash
6. `uuid` - UUID generation
7. `random_string` - Random string generation
8. `read_file` - Read file content
9. `file_exists` - Check file existence
10. `list_dir` - List directory
11. `glob` - Glob pattern matching
12. `file_size` - Get file size
13. `file_modified` - Get file modification time
14. `is_email` - Email validation
15. `is_url` - URL validation
16. `is_ip` - IP address validation
17. `is_uuid` - UUID validation
18. `matches_regex` - Regex matching
19. `parse_json` - Parse JSON string
20. `parse_yaml` - Parse YAML string
21. `parse_toml` - Parse TOML string
22. `read_json_file` - Read JSON file
23. `read_yaml_file` - Read YAML file
24. `read_toml_file` - Read TOML file

**New Functions (Replace Tera built-ins):**
25. `get_env` - Get environment variable (was Tera built-in)
26. `now` - Current timestamp (was Tera built-in)
27. `get_random` - Random number (was Tera built-in)

### Breaking Changes for Users

**Potential Template Syntax Differences:**

1. **Error Messages**
   - Error format will change
   - Line numbers and error descriptions different
   - Tests checking error messages must update

2. **Edge Cases**
   - Some edge case behaviors may differ
   - Whitespace handling might differ
   - Filter/function call syntax edge cases

3. **Built-in Filters**
   - MiniJinja has different built-in filters
   - Need to verify compatibility or add missing ones

## Migration Strategy

### Approach: Feature Branch with Parallel Implementation

1. **Create feature branch** `feat/migrate-to-minijinja`
2. **Implement changes** incrementally with commits
3. **Test thoroughly** after each major change
4. **Document changes** as we go
5. **Review and merge** when all tests pass

### Migration Phases

**Phase 1: Setup & Dependencies** (Day 1)
- Update `Cargo.toml`
- Add MiniJinja dependency
- Remove Tera dependency
- Verify project compiles

**Phase 2: Core Renderer** (Day 1)
- Update `src/renderer.rs`
- Change from Tera to Environment API
- Update error handling
- Maintain same public API

**Phase 3: Custom Functions** (Day 2-3)
- Migrate all 20+ functions from structs to closures
- Implement missing built-in functions
- Update `src/functions/mod.rs`
- Ensure all functions work

**Phase 4: Testing** (Day 3-4)
- Run all 180 tests
- Fix failing tests
- Update test expectations
- Test example templates

**Phase 5: Documentation** (Day 4-5)
- Update README.md
- Update code comments
- Update CONTRIBUTING.md
- Add migration notes for users

**Phase 6: Validation** (Day 5)
- Run `cargo make qa`
- Verify all CI checks
- Compare binary sizes
- Performance benchmarks

## Detailed Migration Steps

### Step 1: Update Dependencies

**File:** `Cargo.toml`

```diff
 [dependencies]
-tera = { version = "1", features = ["builtins"] }
+minijinja = { version = "2", features = ["builtins"] }
 clap = { version = "4", features = ["derive"] }
 regex = "1"
 # ... rest unchanged
```

**Verify:**
```bash
cargo build
# Should fail with compile errors - expected
```

### Step 2: Update Renderer Module

**File:** `src/renderer.rs`

**Before:**
```rust
use tera::{Context, Tera};

fn render(
    template_source: Option<&str>,
    template_content: &str,
    context: &Context,
    template_context: TemplateContext,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut tera = Tera::default();

    // Register all custom functions
    functions::register_all(&mut tera, template_context);

    let template_name = template_source.unwrap_or("template");

    tera.add_raw_template(template_name, template_content)
        .map_err(|e| format_tera_error("Failed to parse template", &e))?;

    tera.render(template_name, context)
        .map_err(|e| format_tera_error("Failed to render template", &e).into())
}
```

**After:**
```rust
use minijinja::Environment;
use serde::Serialize;

fn render(
    template_source: Option<&str>,
    template_content: &str,
    context: &impl Serialize,
    template_context: TemplateContext,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut env = Environment::new();

    // Register all custom functions
    functions::register_all(&mut env, template_context);

    let template_name = template_source.unwrap_or("template");

    env.add_template(template_name, template_content)
        .map_err(|e| format_minijinja_error("Failed to parse template", &e))?;

    let tmpl = env.get_template(template_name)?;
    tmpl.render(context)
        .map_err(|e| format_minijinja_error("Failed to render template", &e).into())
}
```

**Error Formatting:**
```rust
fn format_minijinja_error(prefix: &str, error: &minijinja::Error) -> String {
    use std::fmt::Write;

    let mut msg = String::new();
    writeln!(&mut msg, "{}", prefix).ok();
    writeln!(&mut msg).ok();
    writeln!(&mut msg, "Error: {}", error).ok();

    // MiniJinja has excellent error messages built-in
    if let Some(detail) = error.detail() {
        writeln!(&mut msg).ok();
        writeln!(&mut msg, "{}", detail).ok();
    }

    msg
}
```

### Step 3: Update Function Registration

**File:** `src/functions/mod.rs`

**Before:**
```rust
use tera::Tera;

pub fn register_all(tera: &mut Tera, context: TemplateContext) {
    tera.register_function("filter_env", filter_env::FilterEnv);
    tera.register_function("md5", hash::Md5);
    // ... etc
}
```

**After:**
```rust
use minijinja::Environment;

pub fn register_all(env: &mut Environment, context: TemplateContext) {
    // Add custom functions
    env.add_function("filter_env", filter_env::filter_env_fn);
    env.add_function("md5", hash::md5_fn);

    // Add built-in replacements (were built-in in Tera)
    env.add_function("get_env", builtins::get_env_fn);
    env.add_function("now", builtins::now_fn);
    env.add_function("get_random", builtins::get_random_fn);

    // ... rest of functions
}
```

### Step 4: Migrate Custom Functions

**Example: filter_env function**

**File:** `src/functions/filter_env.rs`

**Before (Tera):**
```rust
use tera::{Function, Result, Value, to_value};

pub struct FilterEnv;

impl Function for FilterEnv {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg(
                    "filter_env requires a 'pattern' argument (e.g., pattern=\"SERVER_*\")",
                )
            })?;

        let regex_pattern = glob_to_regex(pattern);
        let re = regex::Regex::new(&regex_pattern)
            .map_err(|e| tera::Error::msg(format!("Invalid pattern: {}", e)))?;

        let mut results: Vec<HashMap<String, String>> = env::vars()
            .filter(|(key, _)| re.is_match(key))
            .map(|(key, value)| {
                let mut map = HashMap::new();
                map.insert("key".to_string(), key);
                map.insert("value".to_string(), value);
                map
            })
            .collect();

        results.sort_by(|a, b| a.get("key").cmp(&b.get("key")));

        to_value(&results)
            .map_err(|e| tera::Error::msg(format!("Failed to convert results: {}", e)))
    }
}
```

**After (MiniJinja):**
```rust
use minijinja::{Error, ErrorKind, Value};

pub fn filter_env_fn(pattern: String) -> Result<Value, Error> {
    let regex_pattern = glob_to_regex(&pattern);
    let re = regex::Regex::new(&regex_pattern)
        .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("Invalid pattern: {}", e)))?;

    let mut results: Vec<HashMap<String, String>> = env::vars()
        .filter(|(key, _)| re.is_match(key))
        .map(|(key, value)| {
            let mut map = HashMap::new();
            map.insert("key".to_string(), key);
            map.insert("value".to_string(), value);
            map
        })
        .collect();

    results.sort_by(|a, b| a.get("key").cmp(&b.get("key")));

    Ok(Value::from_serialize(&results))
}

// glob_to_regex remains the same
```

**Key Changes:**
1. Struct → Function
2. `&HashMap<String, Value>` args → Direct parameters
3. `tera::Error::msg()` → `minijinja::Error::new()`
4. `to_value()` → `Value::from_serialize()`
5. `Result<Value>` → `Result<Value, Error>`

### Step 5: Implement Built-in Functions

**File:** `src/functions/builtins.rs` (NEW)

```rust
use minijinja::{Error, ErrorKind, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

/// Get environment variable with optional default
/// Replacement for Tera's built-in get_env()
pub fn get_env_fn(name: String, default: Option<String>) -> Result<Value, Error> {
    match std::env::var(&name) {
        Ok(value) => Ok(Value::from(value)),
        Err(_) => {
            if let Some(def) = default {
                Ok(Value::from(def))
            } else {
                Err(Error::new(
                    ErrorKind::UndefinedError,
                    format!("Environment variable '{}' is not set", name)
                ))
            }
        }
    }
}

/// Get current Unix timestamp
/// Replacement for Tera's built-in now()
pub fn now_fn() -> Result<Value, Error> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("Failed to get timestamp: {}", e)))?
        .as_secs();

    Ok(Value::from(timestamp))
}

/// Generate random number in range
/// Replacement for Tera's built-in get_random()
pub fn get_random_fn(start: Option<i64>, end: Option<i64>) -> Result<Value, Error> {
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(100);

    if start >= end {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("start ({}) must be less than end ({})", start, end)
        ));
    }

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(start..end);

    Ok(Value::from(random))
}
```

### Step 6: Update All Other Functions

**Pattern for each function:**

1. Remove struct definition
2. Create function with appropriate parameters
3. Change `tera::Error` → `minijinja::Error`
4. Change `to_value()` → `Value::from_serialize()`
5. Update error creation: `Error::new(ErrorKind::..., message)`

**Functions to update (20 files):**
- `src/functions/hash.rs` - 4 hash functions
- `src/functions/uuid_gen.rs` - 1 UUID function
- `src/functions/random_string.rs` - 1 random function
- `src/functions/filesystem.rs` - 6 filesystem functions
- `src/functions/validation.rs` - 5 validation functions
- `src/functions/data_parsing.rs` - 6 parsing functions

### Step 7: Update Tests

**Changes needed:**

1. **Error message tests** - Update expected error strings
2. **Value comparisons** - May need adjustments
3. **Test fixtures** - Verify template syntax compatibility

**Example test update:**

**Before:**
```rust
let result = render_template(Some("template.tmpl"), None, false);
assert!(result.is_err());
assert!(result.unwrap_err().to_string().contains("Failed to parse template"));
```

**After:**
```rust
let result = render_template(Some("template.tmpl"), None, false);
assert!(result.is_err());
// Error message might be different in MiniJinja
assert!(result.unwrap_err().to_string().contains("template"));
```

## Testing Strategy

### Testing Phases

**Phase 1: Unit Tests**
```bash
# Run all unit tests
cargo test

# Expected: Some tests may fail due to error message changes
# Action: Update test expectations
```

**Phase 2: Integration Tests**
```bash
# Test each example template
tmpltool examples/basic.tmpl
tmpltool examples/greeting.tmpl
tmpltool examples/docker-compose.tmpl
tmpltool examples/comprehensive-app-config.tmpl

# Expected: All should render correctly
```

**Phase 3: QA Suite**
```bash
# Run full QA
cargo make qa

# Expected: All checks should pass
```

**Phase 4: Binary Size Comparison**
```bash
# Build release binary
cargo build --release

# Check size
ls -lh target/release/tmpltool

# Expected: ~40-50% smaller than Tera version
```

**Phase 5: Performance Benchmark**
```bash
# Benchmark rendering time
time ./target/release/tmpltool examples/comprehensive-app-config.tmpl > /dev/null

# Expected: Similar or faster performance
```

### Test Checklist

- [ ] All 180 unit tests pass
- [ ] All integration tests pass
- [ ] All example templates render correctly
- [ ] QA suite passes (`cargo make qa`)
- [ ] Binary size reduced by 40-50%
- [ ] Performance maintained or improved
- [ ] Error messages are helpful
- [ ] Documentation tests pass
- [ ] Cross-platform tests pass (Linux, macOS, Windows)

## Risk Mitigation

### Identified Risks

**Risk 1: Breaking Template Syntax**
- **Impact:** HIGH - Users' templates may break
- **Probability:** MEDIUM
- **Mitigation:**
  - Thorough testing with all example templates
  - Document any syntax differences
  - Provide migration guide for users
  - Consider maintaining backward compatibility

**Risk 2: Missing Features**
- **Impact:** HIGH - Some Tera features may not exist in MiniJinja
- **Probability:** LOW
- **Mitigation:**
  - Complete feature audit before starting
  - Implement missing features as custom functions
  - Document any removed features

**Risk 3: Performance Regression**
- **Impact:** MEDIUM - Slower rendering
- **Probability:** LOW
- **Mitigation:**
  - Benchmark before and after
  - Profile any slow operations
  - Optimize if needed

**Risk 4: Bug Introduction**
- **Impact:** HIGH - New bugs in production
- **Probability:** MEDIUM
- **Mitigation:**
  - Comprehensive testing (180 tests)
  - Manual testing of all examples
  - Beta release for early adopters
  - Quick rollback plan

**Risk 5: Incomplete Migration**
- **Impact:** HIGH - Project in broken state
- **Probability:** LOW
- **Mitigation:**
  - Work in feature branch
  - Don't merge until 100% complete
  - All tests must pass
  - Documentation complete

## Rollback Plan

### If Migration Fails

**Option 1: Revert Feature Branch**
```bash
git checkout master
git branch -D feat/migrate-to-minijinja
# Migration abandoned, continue with Tera
```

**Option 2: Keep Both (Feature Flag)**
```toml
[features]
default = ["tera-engine"]
tera-engine = ["tera"]
minijinja-engine = ["minijinja"]
```

This allows users to choose which engine to use.

**Option 3: Partial Migration**
- Identify blocking issues
- Fix critical issues first
- Postpone migration until resolved

## Timeline

### Estimated Timeline: 3-5 Days

**Day 1: Setup & Core Changes**
- [ ] Create feature branch
- [ ] Update dependencies (Cargo.toml)
- [ ] Update renderer.rs
- [ ] Implement built-in functions
- [ ] Initial compilation test

**Day 2: Function Migration (Part 1)**
- [ ] Migrate hash functions (4)
- [ ] Migrate UUID & random functions (2)
- [ ] Migrate filesystem functions (6)
- [ ] Test migrated functions

**Day 3: Function Migration (Part 2)**
- [ ] Migrate validation functions (5)
- [ ] Migrate data parsing functions (6)
- [ ] Update function registration
- [ ] Run initial test suite

**Day 4: Testing & Fixes**
- [ ] Fix failing tests
- [ ] Update test expectations
- [ ] Test all examples
- [ ] Run QA suite
- [ ] Performance benchmarks

**Day 5: Documentation & Polish**
- [ ] Update README.md
- [ ] Update CONTRIBUTING.md
- [ ] Update code comments
- [ ] Create migration guide for users
- [ ] Final testing
- [ ] Create PR

### Dependencies & Blockers

**No External Blockers**
- All dependencies are under our control
- No upstream changes needed
- Can proceed immediately

## Success Criteria

### Migration is Successful When:

✅ **All Tests Pass**
- [ ] 180 unit tests pass
- [ ] All integration tests pass
- [ ] All example templates render
- [ ] QA suite passes

✅ **Binary Size Reduced**
- [ ] Binary is 40-50% smaller
- [ ] Dependencies reduced to minimal

✅ **Performance Maintained**
- [ ] Rendering speed same or faster
- [ ] Compilation time faster
- [ ] Memory usage same or lower

✅ **Documentation Complete**
- [ ] README.md updated
- [ ] All Tera references removed
- [ ] Migration guide created
- [ ] Code comments updated

✅ **No Regressions**
- [ ] All existing features work
- [ ] No new bugs introduced
- [ ] Error messages are helpful

✅ **User Impact Minimal**
- [ ] Template syntax unchanged (or documented)
- [ ] Migration guide provided
- [ ] Breaking changes documented

## Next Steps

### To Start Migration:

1. **Review this plan** - Ensure team agreement
2. **Create feature branch** - `feat/migrate-to-minijinja`
3. **Start with Day 1 tasks** - Follow timeline above
4. **Commit frequently** - Small, atomic commits
5. **Test continuously** - Don't wait until end
6. **Document as you go** - Update docs immediately
7. **Create PR when complete** - Request thorough review

### Questions to Answer Before Starting:

- [ ] Do we want to maintain Tera compatibility mode?
- [ ] Should we create a major version bump (2.0.0)?
- [ ] Do we need a beta release first?
- [ ] How will we handle user templates that break?
- [ ] What's the deprecation timeline (if any)?

## References

- [MiniJinja Documentation](https://docs.rs/minijinja/)
- [MiniJinja GitHub](https://github.com/mitsuhiko/minijinja)
- [Tera Documentation](https://keats.github.io/tera/docs/)
- [Jinja2 Template Designer Documentation](https://jinja.palletsprojects.com/templates/)

## Appendix A: Full Function Migration Checklist

### Custom Functions (20 functions)

**Hash & Crypto (6):**
- [ ] `filter_env` → `filter_env_fn`
- [ ] `md5` → `md5_fn`
- [ ] `sha1` → `sha1_fn`
- [ ] `sha256` → `sha256_fn`
- [ ] `sha512` → `sha512_fn`
- [ ] `uuid` → `uuid_fn`
- [ ] `random_string` → `random_string_fn`

**Filesystem (6):**
- [ ] `read_file` → `read_file_fn`
- [ ] `file_exists` → `file_exists_fn`
- [ ] `list_dir` → `list_dir_fn`
- [ ] `glob` → `glob_fn`
- [ ] `file_size` → `file_size_fn`
- [ ] `file_modified` → `file_modified_fn`

**Validation (5):**
- [ ] `is_email` → `is_email_fn`
- [ ] `is_url` → `is_url_fn`
- [ ] `is_ip` → `is_ip_fn`
- [ ] `is_uuid` → `is_uuid_fn`
- [ ] `matches_regex` → `matches_regex_fn`

**Data Parsing (6):**
- [ ] `parse_json` → `parse_json_fn`
- [ ] `parse_yaml` → `parse_yaml_fn`
- [ ] `parse_toml` → `parse_toml_fn`
- [ ] `read_json_file` → `read_json_file_fn`
- [ ] `read_yaml_file` → `read_yaml_file_fn`
- [ ] `read_toml_file` → `read_toml_file_fn`

**Built-in Replacements (3):**
- [ ] `get_env` → `get_env_fn` (NEW - was Tera built-in)
- [ ] `now` → `now_fn` (NEW - was Tera built-in)
- [ ] `get_random` → `get_random_fn` (NEW - was Tera built-in)

**Total: 23 functions**

## Appendix B: Code Patterns

### Pattern 1: Simple Function with No Context

**Before (Tera):**
```rust
pub struct Md5;

impl Function for Md5 {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("md5 requires a 'string' argument")
        })?;

        let hash = format!("{:x}", md5::compute(string));
        to_value(&hash).map_err(|e| tera::Error::msg(format!("Error: {}", e)))
    }
}
```

**After (MiniJinja):**
```rust
pub fn md5_fn(string: String) -> Result<Value, Error> {
    let hash = format!("{:x}", md5::compute(&string));
    Ok(Value::from(hash))
}
```

### Pattern 2: Function with Context

**Before (Tera):**
```rust
pub struct ReadFile {
    context: TemplateContext,
}

impl ReadFile {
    pub fn new(context: TemplateContext) -> Self {
        Self { context }
    }
}

impl Function for ReadFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("read_file requires a 'path' argument")
        })?;

        if !self.context.is_trust_mode() {
            validate_path_security(path)?;
        }

        let resolved_path = self.context.resolve_path(path);
        let content = fs::read_to_string(&resolved_path)
            .map_err(|e| tera::Error::msg(format!("Failed to read: {}", e)))?;

        to_value(&content).map_err(|e| tera::Error::msg(format!("Error: {}", e)))
    }
}
```

**After (MiniJinja):**
```rust
use std::sync::Arc;

pub fn create_read_file_fn(context: Arc<TemplateContext>) -> impl Fn(String) -> Result<Value, Error> {
    move |path: String| {
        if !context.is_trust_mode() {
            validate_path_security(&path)
                .map_err(|e| Error::new(ErrorKind::InvalidOperation, e.to_string()))?;
        }

        let resolved_path = context.resolve_path(&path);
        let content = fs::read_to_string(&resolved_path)
            .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("Failed to read: {}", e)))?;

        Ok(Value::from(content))
    }
}

// Register in mod.rs:
let context_arc = Arc::new(context);
env.add_function("read_file", create_read_file_fn(context_arc.clone()));
```

### Pattern 3: Error Handling

**Before (Tera):**
```rust
tera::Error::msg("error message")
```

**After (MiniJinja):**
```rust
Error::new(ErrorKind::InvalidOperation, "error message")

// Available ErrorKinds:
// - InvalidOperation
// - UndefinedError
// - BadEscape
// - CannotUnpack
// - BadSerialization
```

---

**Document Version:** 1.0
**Last Updated:** 2024-12-31
**Status:** DRAFT - Pending Review
