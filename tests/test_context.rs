use std::path::PathBuf;
use tmpltool::TemplateContext;

#[test]
fn test_resolve_relative_path() {
    let ctx = TemplateContext::new(PathBuf::from("/tmp/templates"), false);
    let resolved = ctx.resolve_path("data.txt");
    assert_eq!(resolved, PathBuf::from("/tmp/templates/data.txt"));
}

#[test]
fn test_resolve_absolute_path() {
    let ctx = TemplateContext::new(PathBuf::from("/tmp/templates"), false);
    let resolved = ctx.resolve_path("/etc/hosts");
    assert_eq!(resolved, PathBuf::from("/etc/hosts"));
}

#[test]
fn test_resolve_parent_relative_path() {
    let ctx = TemplateContext::new(PathBuf::from("/tmp/templates"), false);
    let resolved = ctx.resolve_path("../config.txt");
    assert_eq!(resolved, PathBuf::from("/tmp/templates/../config.txt"));
}

#[test]
fn test_trust_mode() {
    let ctx_no_trust = TemplateContext::new(PathBuf::from("/tmp"), false);
    assert!(!ctx_no_trust.is_trust_mode());

    let ctx_trust = TemplateContext::new(PathBuf::from("/tmp"), true);
    assert!(ctx_trust.is_trust_mode());
}

#[test]
fn test_base_dir() {
    let ctx = TemplateContext::new(PathBuf::from("/tmp/templates"), false);
    assert_eq!(ctx.base_dir(), PathBuf::from("/tmp/templates").as_path());
}
