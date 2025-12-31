use tmpltool::functions::uuid_gen::uuid_fn;

#[test]
fn test_uuid_v4_format() {
    let result = uuid_fn();
    let uuid_str = result.as_str().unwrap();

    // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    // where y is one of [8, 9, a, b]
    assert_eq!(uuid_str.len(), 36);
    assert_eq!(uuid_str.chars().nth(8).unwrap(), '-');
    assert_eq!(uuid_str.chars().nth(13).unwrap(), '-');
    assert_eq!(uuid_str.chars().nth(18).unwrap(), '-');
    assert_eq!(uuid_str.chars().nth(23).unwrap(), '-');

    // Version should be 4
    assert_eq!(uuid_str.chars().nth(14).unwrap(), '4');
}

#[test]
fn test_uuid_v4_uniqueness() {
    let result1 = uuid_fn();
    let result2 = uuid_fn();

    // Two UUIDs should be different
    assert_ne!(result1.as_str().unwrap(), result2.as_str().unwrap());
}

#[test]
fn test_uuid_v4_valid_hex() {
    let result = uuid_fn();
    let uuid_str = result.as_str().unwrap();

    // Remove dashes and check if all characters are valid hex
    let hex_part: String = uuid_str.chars().filter(|c| *c != '-').collect();
    assert_eq!(hex_part.len(), 32);

    for ch in hex_part.chars() {
        assert!(ch.is_ascii_hexdigit());
    }
}
