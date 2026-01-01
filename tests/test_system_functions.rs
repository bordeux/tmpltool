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
