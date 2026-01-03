//! Tests for datetime is-functions
//!
//! Tests both function syntax and "is" test syntax for:
//! - is_leap_year / leap_year

use minijinja::Environment;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::is_functions::datetime::LeapYear;

/// Helper to create a test environment with is-functions registered
fn create_test_env() -> Environment<'static> {
    let mut env = Environment::new();
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    tmpltool::is_functions::register_all(&mut env, context);
    env
}

/// Helper to render a template and check the result
fn render(env: &Environment, template: &str) -> String {
    env.render_str(template, ()).unwrap()
}

// ========== LeapYear Unit Tests ==========

#[test]
fn test_leap_year_validate_leap_years() {
    // Divisible by 4 but not by 100
    assert!(LeapYear::is_leap(2024));
    assert!(LeapYear::is_leap(2020));
    assert!(LeapYear::is_leap(2016));
    assert!(LeapYear::is_leap(2012));

    // Divisible by 400
    assert!(LeapYear::is_leap(2000));
    assert!(LeapYear::is_leap(1600));
    assert!(LeapYear::is_leap(2400));
}

#[test]
fn test_leap_year_validate_non_leap_years() {
    // Not divisible by 4
    assert!(!LeapYear::is_leap(2023));
    assert!(!LeapYear::is_leap(2022));
    assert!(!LeapYear::is_leap(2021));
    assert!(!LeapYear::is_leap(2019));

    // Divisible by 100 but not by 400 (century years)
    assert!(!LeapYear::is_leap(1900));
    assert!(!LeapYear::is_leap(2100));
    assert!(!LeapYear::is_leap(2200));
    assert!(!LeapYear::is_leap(2300));
}

#[test]
fn test_leap_year_edge_cases() {
    // Year 0 is a leap year (divisible by 400)
    assert!(LeapYear::is_leap(0));

    // Negative years
    assert!(LeapYear::is_leap(-4)); // divisible by 4
    assert!(!LeapYear::is_leap(-1)); // not divisible by 4
    assert!(LeapYear::is_leap(-400)); // divisible by 400
}

// ========== Function Syntax Tests ==========

#[test]
fn test_is_leap_year_function_syntax_leap_years() {
    let env = create_test_env();

    assert_eq!(render(&env, r#"{{ is_leap_year(year=2024) }}"#), "true");
    assert_eq!(render(&env, r#"{{ is_leap_year(year=2020) }}"#), "true");
    assert_eq!(render(&env, r#"{{ is_leap_year(year=2000) }}"#), "true");
}

#[test]
fn test_is_leap_year_function_syntax_non_leap_years() {
    let env = create_test_env();

    assert_eq!(render(&env, r#"{{ is_leap_year(year=2023) }}"#), "false");
    assert_eq!(render(&env, r#"{{ is_leap_year(year=2022) }}"#), "false");
    assert_eq!(render(&env, r#"{{ is_leap_year(year=1900) }}"#), "false");
    assert_eq!(render(&env, r#"{{ is_leap_year(year=2100) }}"#), "false");
}

#[test]
fn test_is_leap_year_function_syntax_in_conditional() {
    let env = create_test_env();

    assert_eq!(
        render(
            &env,
            r#"{% if is_leap_year(year=2024) %}leap{% else %}regular{% endif %}"#
        ),
        "leap"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if is_leap_year(year=2023) %}leap{% else %}regular{% endif %}"#
        ),
        "regular"
    );
}

// ========== Is-Test Syntax Tests ==========

#[test]
fn test_is_leap_year_is_syntax_leap_years() {
    let env = create_test_env();

    assert_eq!(
        render(
            &env,
            r#"{% if 2024 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 2020 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 2000 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
}

#[test]
fn test_is_leap_year_is_syntax_non_leap_years() {
    let env = create_test_env();

    assert_eq!(
        render(
            &env,
            r#"{% if 2023 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 2022 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 1900 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 2100 is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_leap_year_is_syntax_with_variable() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% if year is leap_year %}leap{% else %}regular{% endif %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();

    // Leap year
    let result = tmpl.render(minijinja::context! { year => 2024 }).unwrap();
    assert_eq!(result, "leap");

    // Non-leap year
    let result = tmpl.render(minijinja::context! { year => 2023 }).unwrap();
    assert_eq!(result, "regular");

    // Century year (not leap)
    let result = tmpl.render(minijinja::context! { year => 1900 }).unwrap();
    assert_eq!(result, "regular");

    // Century year (leap - divisible by 400)
    let result = tmpl.render(minijinja::context! { year => 2000 }).unwrap();
    assert_eq!(result, "leap");
}

#[test]
fn test_is_leap_year_is_syntax_with_string() {
    let env = create_test_env();

    // String representation of year should also work
    assert_eq!(
        render(
            &env,
            r#"{% if "2024" is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "2023" is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_leap_year_is_syntax_non_numeric() {
    let env = create_test_env();

    // Non-numeric strings should return false
    assert_eq!(
        render(
            &env,
            r#"{% if "not-a-year" is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "" is leap_year %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_leap_year_is_syntax_negation() {
    let env = create_test_env();

    // Using "is not" syntax
    assert_eq!(
        render(
            &env,
            r#"{% if 2023 is not leap_year %}regular{% else %}leap{% endif %}"#
        ),
        "regular"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if 2024 is not leap_year %}regular{% else %}leap{% endif %}"#
        ),
        "leap"
    );
}

// ========== Loop Tests ==========

#[test]
fn test_is_leap_year_in_for_loop() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% for year in years %}{% if year is leap_year %}{{ year }} {% endif %}{% endfor %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            years => vec![2019, 2020, 2021, 2022, 2023, 2024]
        })
        .unwrap();

    assert_eq!(result.trim(), "2020 2024");
}

#[test]
fn test_is_leap_year_with_century_years() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% for year in years %}{{ year }}:{% if year is leap_year %}L{% else %}R{% endif %} {% endfor %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            years => vec![1800, 1900, 2000, 2100, 2200, 2300, 2400]
        })
        .unwrap();

    // 1800, 1900, 2100, 2200, 2300 are not leap years (divisible by 100 but not 400)
    // 2000, 2400 are leap years (divisible by 400)
    assert_eq!(result, "1800:R 1900:R 2000:L 2100:R 2200:R 2300:R 2400:L ");
}

// ========== Edge Case Tests ==========

#[test]
fn test_is_leap_year_zero() {
    let env = create_test_env();

    // Year 0 is technically a leap year (divisible by 400)
    assert_eq!(
        render(&env, r#"{% if 0 is leap_year %}yes{% else %}no{% endif %}"#),
        "yes"
    );
}

#[test]
fn test_is_leap_year_negative_years() {
    let env = create_test_env();

    // Negative leap year
    assert_eq!(render(&env, r#"{{ is_leap_year(year=-4) }}"#), "true");

    // Negative non-leap year
    assert_eq!(render(&env, r#"{{ is_leap_year(year=-1) }}"#), "false");
}
