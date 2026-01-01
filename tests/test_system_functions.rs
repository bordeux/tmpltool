use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::system;

#[test]
fn test_get_hostname() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_hostname_fn(kwargs);
    assert!(result.is_ok());
    let hostname = result.unwrap();
    assert!(!hostname.as_str().unwrap().is_empty());
}

#[test]
fn test_get_username() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_username_fn(kwargs);
    assert!(result.is_ok());
    let username = result.unwrap();
    assert!(!username.as_str().unwrap().is_empty());
}

#[test]
fn test_get_home_dir() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_home_dir_fn(kwargs);
    assert!(result.is_ok());
    let home_dir = result.unwrap();
    assert!(!home_dir.as_str().unwrap().is_empty());
}

#[test]
fn test_get_temp_dir() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_temp_dir_fn(kwargs);
    assert!(result.is_ok());
    let temp_dir = result.unwrap();
    assert!(!temp_dir.as_str().unwrap().is_empty());
}

#[test]
fn test_get_os() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_os_fn(kwargs);
    assert!(result.is_ok());
    let os = result.unwrap();
    let os_str = os.as_str().unwrap();

    // Should be one of the known OS values
    let valid_os = ["linux", "macos", "windows", "freebsd", "openbsd", "netbsd"];
    assert!(
        valid_os.contains(&os_str),
        "Expected one of {:?}, got '{}'",
        valid_os,
        os_str
    );
}

#[test]
fn test_get_arch() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_arch_fn(kwargs);
    assert!(result.is_ok());
    let arch = result.unwrap();
    let arch_str = arch.as_str().unwrap();

    // Should be one of the known architecture values
    let valid_arch = ["x86_64", "x86", "aarch64", "arm", "riscv64", "powerpc64", "s390x"];
    assert!(
        valid_arch.contains(&arch_str),
        "Expected one of {:?}, got '{}'",
        valid_arch,
        arch_str
    );
}

#[test]
fn test_get_cwd() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_cwd_fn(kwargs);
    assert!(result.is_ok());
    let cwd = result.unwrap();
    let cwd_str = cwd.as_str().unwrap();

    // CWD should not be empty
    assert!(!cwd_str.is_empty());

    // CWD should be an absolute path
    assert!(
        cwd_str.starts_with('/') || cwd_str.chars().nth(1) == Some(':'),
        "Expected absolute path, got '{}'",
        cwd_str
    );
}

#[test]
fn test_get_cwd_exists() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = system::get_cwd_fn(kwargs);
    assert!(result.is_ok());
    let cwd = result.unwrap();
    let cwd_str = cwd.as_str().unwrap();

    // The CWD path should exist
    assert!(
        std::path::Path::new(cwd_str).exists(),
        "CWD path '{}' should exist",
        cwd_str
    );
}
