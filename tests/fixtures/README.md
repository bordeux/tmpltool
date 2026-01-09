# Test Fixtures

This directory contains test fixtures for tmpltool tests. Fixtures make tests easier to maintain by separating test data from test logic.

## Directory Structure

```
fixtures/
├── templates/           # Input template files
│   ├── simple.tmpltool
│   ├── with_env.tmpltool
│   ├── with_default.tmpltool
│   ├── multiline.tmpltool
│   ├── conditionals.tmpltool
│   ├── invalid.tmpltool
│   ├── direct_var.tmpltool
│   └── docker-compose.tmpltool
└── expected/            # Expected output files
    ├── simple.txt
    ├── with_env.txt
    ├── with_default.txt
    ├── multiline.txt
    ├── conditionals_true.txt
    ├── conditionals_false.txt
    └── docker-compose.txt
```

## How It Works

1. **Templates** (`templates/`) - Input files containing Tera templates
2. **Expected Outputs** (`expected/`) - The expected rendered output for each template

Tests read these files and compare the actual output with the expected output.

## Available Fixtures

### simple.tmpltool
Basic static template with no variables.
- **Expected:** `simple.txt`

### with_env.tmpltool
Template using `get_env()` function.
- **Environment:** `TEST_VAR=test_value`
- **Expected:** `with_env.txt`

### with_default.tmpltool
Template using `get_env()` with default value.
- **Environment:** None (uses default)
- **Expected:** `with_default.txt`

### multiline.tmpltool
Multi-line template with environment variables.
- **Environment:** `LINE1=First`, `LINE2=Second`
- **Expected:** `multiline.txt`

### conditionals.tmpltool
Template with conditional logic.
- **Environment:** `CONDITION=yes` (for true case)
- **Expected (true):** `conditionals_true.txt`
- **Expected (false):** `conditionals_false.txt`

### invalid.tmpltool
Template with invalid syntax (for error testing).
- **Expected:** Should fail to render

### direct_var.tmpltool
Template trying to access variable directly without `get_env()`.
- **Environment:** `DIRECT_VAR=value`
- **Expected:** Should fail to render

### docker-compose.tmpltool
Real-world example: Docker Compose template.
- **Environment:** Uses defaults
- **Expected:** `docker-compose.txt`

## Adding New Fixtures

To add a new fixture for testing:

### 1. Create Template File

Create a new file in `templates/`:

```bash
# Example: tests/fixtures/templates/my_feature.tmpltool
My feature: {{ get_env(name="FEATURE_VAR", default="default_value") }}
```

### 2. Create Expected Output File

Create corresponding expected output in `expected/`:

```bash
# Example: tests/fixtures/expected/my_feature.txt
My feature: default_value
```

### 3. Use in Tests

Use the fixture helper functions in your tests:

```rust
mod common;

use common::{
    cleanup_test_file, get_test_file_path,
    read_fixture_expected, read_fixture_template
};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let output_path = get_test_file_path("output_my_feature.txt");

    // Read template from fixtures
    let template_content = read_fixture_template("my_feature.tmpltool");
    let template_path = get_test_file_path("template_my_feature.txt");
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify output matches expected
    assert!(result.is_ok());
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("my_feature.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
```

## Benefits

✅ **Maintainability** - Test data separated from test logic
✅ **Readability** - Easier to see what templates look like
✅ **Reusability** - Same fixture can be used in multiple tests
✅ **Version Control** - Templates are tracked as files, easy to review changes
✅ **Real-world Examples** - Can use actual template files from projects

## Helper Functions

From `tests/common.rs`:

- `read_fixture_template(name)` - Read a template fixture
- `read_fixture_expected(name)` - Read an expected output fixture
- `get_fixture_template(name)` - Get path to template fixture
- `get_fixture_expected(name)` - Get path to expected fixture
