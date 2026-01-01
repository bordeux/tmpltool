use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::{TemplateContext, functions::register_all};

fn render_template(template: &str) -> String {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env.template_from_str(template).unwrap();
    tmpl.render(()).unwrap()
}

// ==================== array_take Tests ====================

#[test]
fn test_array_take_basic() {
    let result = render_template(r#"{{ array_take(array=[1,2,3,4,5], n=3) | tojson }}"#);
    assert_eq!(result, "[1,2,3]");
}

#[test]
fn test_array_take_more_than_available() {
    let result = render_template(r#"{{ array_take(array=[1,2], n=5) | tojson }}"#);
    assert_eq!(result, "[1,2]");
}

#[test]
fn test_array_take_zero() {
    let result = render_template(r#"{{ array_take(array=[1,2,3], n=0) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_take_strings() {
    let result = render_template(r#"{{ array_take(array=["a", "b", "c", "d"], n=2) | tojson }}"#);
    assert_eq!(result, r#"["a","b"]"#);
}

// ==================== array_drop Tests ====================

#[test]
fn test_array_drop_basic() {
    let result = render_template(r#"{{ array_drop(array=[1,2,3,4,5], n=2) | tojson }}"#);
    assert_eq!(result, "[3,4,5]");
}

#[test]
fn test_array_drop_more_than_available() {
    let result = render_template(r#"{{ array_drop(array=[1,2], n=5) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_drop_zero() {
    let result = render_template(r#"{{ array_drop(array=[1,2,3], n=0) | tojson }}"#);
    assert_eq!(result, "[1,2,3]");
}

#[test]
fn test_array_drop_strings() {
    let result = render_template(r#"{{ array_drop(array=["a", "b", "c", "d"], n=2) | tojson }}"#);
    assert_eq!(result, r#"["c","d"]"#);
}

// ==================== array_index_of Tests ====================

#[test]
fn test_array_index_of_found() {
    let result = render_template(r#"{{ array_index_of(array=["a","b","c"], value="b") }}"#);
    assert_eq!(result, "1");
}

#[test]
fn test_array_index_of_not_found() {
    let result = render_template(r#"{{ array_index_of(array=[1,2,3], value=5) }}"#);
    assert_eq!(result, "-1");
}

#[test]
fn test_array_index_of_first_element() {
    let result = render_template(r#"{{ array_index_of(array=["a","b","c"], value="a") }}"#);
    assert_eq!(result, "0");
}

#[test]
fn test_array_index_of_last_element() {
    let result = render_template(r#"{{ array_index_of(array=[1,2,3], value=3) }}"#);
    assert_eq!(result, "2");
}

#[test]
fn test_array_index_of_empty_array() {
    let result = render_template(r#"{{ array_index_of(array=[], value="x") }}"#);
    assert_eq!(result, "-1");
}

// ==================== array_find Tests ====================

#[test]
fn test_array_find_found() {
    let result = render_template(
        r#"{% set users = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}] %}{{ array_find(array=users, key="id", value=2).name }}"#,
    );
    assert_eq!(result, "Bob");
}

#[test]
fn test_array_find_not_found() {
    let result = render_template(
        r#"{% set users = [{"id": 1, "name": "Alice"}] %}{{ array_find(array=users, key="id", value=99) }}"#,
    );
    assert_eq!(result, "none");
}

#[test]
fn test_array_find_string_match() {
    let result = render_template(
        r#"{% set items = [{"type": "a"}, {"type": "b"}] %}{{ array_find(array=items, key="type", value="b").type }}"#,
    );
    assert_eq!(result, "b");
}

// ==================== array_filter_by Tests ====================

#[test]
fn test_array_filter_by_eq() {
    let result = render_template(
        r#"{% set items = [{"status": "active"}, {"status": "inactive"}, {"status": "active"}] %}{{ array_filter_by(array=items, key="status", op="eq", value="active") | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_gt() {
    let result = render_template(
        r#"{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}{{ array_filter_by(array=items, key="price", op="gt", value=15) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_lt() {
    let result = render_template(
        r#"{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}{{ array_filter_by(array=items, key="price", op="lt", value=25) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_gte() {
    let result = render_template(
        r#"{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}{{ array_filter_by(array=items, key="price", op="gte", value=20) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_lte() {
    let result = render_template(
        r#"{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}{{ array_filter_by(array=items, key="price", op="lte", value=20) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_ne() {
    let result = render_template(
        r#"{% set items = [{"status": "a"}, {"status": "b"}, {"status": "a"}] %}{{ array_filter_by(array=items, key="status", op="ne", value="a") | length }}"#,
    );
    assert_eq!(result, "1");
}

#[test]
fn test_array_filter_by_contains() {
    let result = render_template(
        r#"{% set items = [{"name": "Alice"}, {"name": "Bob"}, {"name": "Charlie"}] %}{{ array_filter_by(array=items, key="name", op="contains", value="li") | length }}"#,
    );
    assert_eq!(result, "2");
}

// ==================== array_pluck Tests ====================

#[test]
fn test_array_pluck_simple() {
    let result = render_template(
        r#"{% set users = [{"name": "Alice"}, {"name": "Bob"}] %}{{ array_pluck(array=users, key="name") | tojson }}"#,
    );
    assert_eq!(result, r#"["Alice","Bob"]"#);
}

#[test]
fn test_array_pluck_nested() {
    let result = render_template(
        r#"{% set data = [{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}] %}{{ array_pluck(array=data, key="user.name") | tojson }}"#,
    );
    assert_eq!(result, r#"["Alice","Bob"]"#);
}

#[test]
fn test_array_pluck_missing_key() {
    let result = render_template(
        r#"{% set data = [{"a": 1}, {"b": 2}] %}{{ array_pluck(array=data, key="a") | tojson }}"#,
    );
    assert_eq!(result, r#"[1,null]"#);
}

#[test]
fn test_array_pluck_numbers() {
    let result = render_template(
        r#"{% set data = [{"val": 1}, {"val": 2}, {"val": 3}] %}{{ array_pluck(array=data, key="val") | tojson }}"#,
    );
    assert_eq!(result, "[1,2,3]");
}

// ==================== array_intersection Tests ====================

#[test]
fn test_array_intersection_numbers() {
    let result = render_template(
        r#"{{ array_intersection(array1=[1,2,3,4], array2=[3, 4, 5, 6]) | tojson }}"#,
    );
    assert_eq!(result, "[3,4]");
}

#[test]
fn test_array_intersection_strings() {
    let result = render_template(
        r#"{{ array_intersection(array1=["a","b","c"], array2=["b", "c", "d"]) | tojson }}"#,
    );
    assert_eq!(result, r#"["b","c"]"#);
}

#[test]
fn test_array_intersection_no_common() {
    let result =
        render_template(r#"{{ array_intersection(array1=[1,2], array2=[3,4]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_intersection_empty_array() {
    let result = render_template(r#"{{ array_intersection(array1=[1,2], array2=[]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_intersection_with_duplicates() {
    let result = render_template(
        r#"{{ array_intersection(array1=[1, 1, 2, 2], array2=[1,2,3]) | tojson }}"#,
    );
    assert_eq!(result, "[1,2]");
}

// ==================== array_difference Tests ====================

#[test]
fn test_array_difference_numbers() {
    let result = render_template(
        r#"{{ array_difference(array1=[1,2,3,4], array2=[3, 4, 5, 6]) | tojson }}"#,
    );
    assert_eq!(result, "[1,2]");
}

#[test]
fn test_array_difference_strings() {
    let result =
        render_template(r#"{{ array_difference(array1=["a","b","c"], array2=["b"]) | tojson }}"#);
    assert_eq!(result, r#"["a","c"]"#);
}

#[test]
fn test_array_difference_no_difference() {
    let result =
        render_template(r#"{{ array_difference(array1=[1,2], array2=[1,2,3]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_difference_empty_second() {
    let result = render_template(r#"{{ array_difference(array1=[1,2,3], array2=[]) | tojson }}"#);
    assert_eq!(result, "[1,2,3]");
}

// ==================== array_union Tests ====================

#[test]
fn test_array_union_numbers() {
    let result = render_template(r#"{{ array_union(array1=[1,2,3], array2=[3,4,5]) | tojson }}"#);
    assert_eq!(result, "[1,2,3,4,5]");
}

#[test]
fn test_array_union_strings() {
    let result =
        render_template(r#"{{ array_union(array1=["a","b"], array2=["b","c"]) | tojson }}"#);
    assert_eq!(result, r#"["a","b","c"]"#);
}

#[test]
fn test_array_union_no_overlap() {
    let result = render_template(r#"{{ array_union(array1=[1,2], array2=[3,4]) | tojson }}"#);
    assert_eq!(result, "[1,2,3,4]");
}

#[test]
fn test_array_union_empty_first() {
    let result = render_template(r#"{{ array_union(array1=[], array2=[1,2]) | tojson }}"#);
    assert_eq!(result, "[1,2]");
}

// ==================== array_symmetric_difference Tests ====================

#[test]
fn test_array_symmetric_difference_numbers() {
    let result = render_template(
        r#"{{ array_symmetric_difference(array1=[1,2,3,4], array2=[3, 4, 5, 6]) | tojson }}"#,
    );
    assert_eq!(result, "[1,2,5,6]");
}

#[test]
fn test_array_symmetric_difference_strings() {
    let result = render_template(
        r#"{{ array_symmetric_difference(array1=["a","b","c"], array2=["b", "c", "d"]) | tojson }}"#,
    );
    assert_eq!(result, r#"["a","d"]"#);
}

#[test]
fn test_array_symmetric_difference_no_common() {
    let result =
        render_template(r#"{{ array_symmetric_difference(array1=[1,2], array2=[3,4]) | tojson }}"#);
    assert_eq!(result, "[1,2,3,4]");
}

#[test]
fn test_array_symmetric_difference_all_common() {
    let result =
        render_template(r#"{{ array_symmetric_difference(array1=[1,2], array2=[1,2]) | tojson }}"#);
    assert_eq!(result, "[]");
}

// ==================== Error Cases ====================

#[test]
fn test_array_filter_by_invalid_operator() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(
            r#"{% set items = [{"a": 1}] %}{{ array_filter_by(array=items, key="a", op="invalid", value=1) }}"#,
        )
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid operator"));
}

#[test]
fn test_array_take_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_take(array="not an array", n=1) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

// ==================== Additional array_take Tests ====================

#[test]
fn test_array_take_with_objects() {
    let result = render_template(
        r#"{% set data = [{"a": 1}, {"b": 2}, {"c": 3}] %}{{ array_take(array=data, n=2) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_take_empty_array() {
    let result = render_template(r#"{{ array_take(array=[], n=5) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_take_with_nulls() {
    let result = render_template(
        r#"{% set data = [1, null, 3, null, 5] %}{{ array_take(array=data, n=3) | tojson }}"#,
    );
    assert_eq!(result, "[1,null,3]");
}

// ==================== Additional array_drop Tests ====================

#[test]
fn test_array_drop_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_drop(array="not an array", n=1) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_drop_with_objects() {
    let result = render_template(
        r#"{% set data = [{"a": 1}, {"b": 2}, {"c": 3}] %}{{ array_drop(array=data, n=1) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_drop_empty_array() {
    let result = render_template(r#"{{ array_drop(array=[], n=5) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_drop_with_nulls() {
    let result = render_template(
        r#"{% set data = [1, null, 3, null, 5] %}{{ array_drop(array=data, n=2) | tojson }}"#,
    );
    assert_eq!(result, "[3,null,5]");
}

// ==================== Additional array_index_of Tests ====================

#[test]
fn test_array_index_of_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_index_of(array="not array", value=1) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_index_of_duplicate_values() {
    let result = render_template(r#"{{ array_index_of(array=[1, 2, 1, 2, 1], value=1) }}"#);
    assert_eq!(result, "0"); // Returns first occurrence
}

#[test]
fn test_array_index_of_null_value() {
    let result = render_template(
        r#"{% set data = [1, null, 3] %}{{ array_index_of(array=data, value=null) }}"#,
    );
    // Searching for null
    assert!(result == "1" || result == "-1");
}

#[test]
fn test_array_index_of_object() {
    let result = render_template(
        r#"{% set data = [{"id": 1}, {"id": 2}] %}{% set search = {"id": 2} %}{{ array_index_of(array=data, value=search) }}"#,
    );
    assert_eq!(result, "1");
}

#[test]
fn test_array_index_of_boolean() {
    let result = render_template(r#"{{ array_index_of(array=[false, true, false], value=true) }}"#);
    assert_eq!(result, "1");
}

// ==================== Additional array_find Tests ====================

#[test]
fn test_array_find_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_find(array="not array", key="id", value=1) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_find_empty_array() {
    let result = render_template(r#"{{ array_find(array=[], key="id", value=1) }}"#);
    assert_eq!(result, "none");
}

#[test]
fn test_array_find_multiple_matches() {
    let result = render_template(
        r#"{% set data = [{"type": "a", "val": 1}, {"type": "a", "val": 2}] %}{{ array_find(array=data, key="type", value="a").val }}"#,
    );
    assert_eq!(result, "1"); // Returns first match
}

#[test]
fn test_array_find_numeric_value() {
    let result = render_template(
        r#"{% set data = [{"count": 10}, {"count": 20}, {"count": 30}] %}{{ array_find(array=data, key="count", value=20).count }}"#,
    );
    assert_eq!(result, "20");
}

#[test]
fn test_array_find_boolean_value() {
    let result = render_template(
        r#"{% set data = [{"active": false}, {"active": true}] %}{{ array_find(array=data, key="active", value=true).active }}"#,
    );
    assert_eq!(result, "true");
}

// ==================== Additional array_filter_by Tests ====================

#[test]
fn test_array_filter_by_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_filter_by(array="not array", key="a", op="eq", value=1) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_filter_by_empty_array() {
    let result = render_template(
        r#"{{ array_filter_by(array=[], key="price", op="gt", value=10) | length }}"#,
    );
    assert_eq!(result, "0");
}

#[test]
fn test_array_filter_by_no_matches() {
    let result = render_template(
        r#"{% set data = [{"price": 5}, {"price": 8}] %}{{ array_filter_by(array=data, key="price", op="gt", value=100) | length }}"#,
    );
    assert_eq!(result, "0");
}

#[test]
fn test_array_filter_by_all_match() {
    let result = render_template(
        r#"{% set data = [{"price": 50}, {"price": 80}] %}{{ array_filter_by(array=data, key="price", op="gt", value=10) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_missing_key_in_items() {
    let result = render_template(
        r#"{% set data = [{"a": 1}, {"b": 2}, {"a": 3}] %}{{ array_filter_by(array=data, key="a", op="gt", value=0) | length }}"#,
    );
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter_by_contains_case_sensitive() {
    let result = render_template(
        r#"{% set data = [{"name": "Alice"}, {"name": "BOB"}] %}{{ array_filter_by(array=data, key="name", op="contains", value="li") | length }}"#,
    );
    assert_eq!(result, "1");
}

#[test]
fn test_array_filter_by_contains_not_string() {
    // When values aren't strings, contains should return false
    let result = render_template(
        r#"{% set data = [{"val": 123}, {"val": 456}] %}{{ array_filter_by(array=data, key="val", op="contains", value="2") | length }}"#,
    );
    assert_eq!(result, "0");
}

// ==================== Additional array_pluck Tests ====================

#[test]
fn test_array_pluck_not_array() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_pluck(array="not array", key="name") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_pluck_empty_array() {
    let result = render_template(r#"{{ array_pluck(array=[], key="name") | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_pluck_deep_nested_key() {
    let result = render_template(
        r#"{% set data = [{"a": {"b": {"c": 1}}}, {"a": {"b": {"c": 2}}}] %}{{ array_pluck(array=data, key="a.b.c") | tojson }}"#,
    );
    assert_eq!(result, "[1,2]");
}

#[test]
fn test_array_pluck_all_missing_key() {
    let result = render_template(
        r#"{% set data = [{"a": 1}, {"b": 2}] %}{{ array_pluck(array=data, key="missing") | tojson }}"#,
    );
    assert_eq!(result, "[null,null]");
}

#[test]
fn test_array_pluck_mixed_types() {
    let result = render_template(
        r#"{% set data = [{"val": "string"}, {"val": 123}, {"val": true}] %}{{ array_pluck(array=data, key="val") | tojson }}"#,
    );
    assert_eq!(result, r#"["string",123,true]"#);
}

// ==================== Additional Set Operations Tests ====================

// --- array_intersection ---

#[test]
fn test_array_intersection_not_array1() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_intersection(array1="not array", array2=[1,2]) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_intersection_not_array2() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_intersection(array1=[1,2], array2="not array") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_intersection_both_empty() {
    let result = render_template(r#"{{ array_intersection(array1=[], array2=[]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_intersection_objects() {
    let result = render_template(
        r#"{% set a = [{"id": 1}, {"id": 2}] %}{% set b = [{"id": 2}, {"id": 3}] %}{{ array_intersection(array1=a, array2=b) | length }}"#,
    );
    assert_eq!(result, "1");
}

// --- array_difference ---

#[test]
fn test_array_difference_not_array1() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_difference(array1="not array", array2=[1,2]) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_difference_not_array2() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_difference(array1=[1,2], array2="not array") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_difference_both_empty() {
    let result = render_template(r#"{{ array_difference(array1=[], array2=[]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_difference_with_duplicates() {
    let result =
        render_template(r#"{{ array_difference(array1=[1,1,2,2,3], array2=[2]) | tojson }}"#);
    assert_eq!(result, "[1,3]");
}

// --- array_union ---

#[test]
fn test_array_union_not_array1() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_union(array1="not array", array2=[1,2]) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_union_not_array2() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_union(array1=[1,2], array2="not array") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_union_both_empty() {
    let result = render_template(r#"{{ array_union(array1=[], array2=[]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_union_with_duplicates_in_both() {
    let result = render_template(r#"{{ array_union(array1=[1,1,2], array2=[2,2,3]) | tojson }}"#);
    assert_eq!(result, "[1,2,3]");
}

#[test]
fn test_array_union_empty_second() {
    let result = render_template(r#"{{ array_union(array1=[1,2,3], array2=[]) | tojson }}"#);
    assert_eq!(result, "[1,2,3]");
}

// --- array_symmetric_difference ---

#[test]
fn test_array_symmetric_difference_not_array1() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_symmetric_difference(array1="not array", array2=[1,2]) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_symmetric_difference_not_array2() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ array_symmetric_difference(array1=[1,2], array2="not array") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

#[test]
fn test_array_symmetric_difference_both_empty() {
    let result =
        render_template(r#"{{ array_symmetric_difference(array1=[], array2=[]) | tojson }}"#);
    assert_eq!(result, "[]");
}

#[test]
fn test_array_symmetric_difference_with_duplicates() {
    let result = render_template(
        r#"{{ array_symmetric_difference(array1=[1,1,2,2], array2=[2,2,3,3]) | tojson }}"#,
    );
    assert_eq!(result, "[1,3]");
}

#[test]
fn test_array_symmetric_difference_empty_first() {
    let result =
        render_template(r#"{{ array_symmetric_difference(array1=[], array2=[1,2]) | tojson }}"#);
    assert_eq!(result, "[1,2]");
}

#[test]
fn test_array_symmetric_difference_empty_second() {
    let result =
        render_template(r#"{{ array_symmetric_difference(array1=[1,2], array2=[]) | tojson }}"#);
    assert_eq!(result, "[1,2]");
}
