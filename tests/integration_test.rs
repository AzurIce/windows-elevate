use windows_elevate::{check_elevated, elevate};

#[test]
fn test_check_elevated() {
    assert_eq!(Ok(false), check_elevated());
}

#[test]
fn test_elevate() {
    let is_elevated = check_elevated().expect("Failed to call check_elevated");

    if !is_elevated {
        elevate().expect("Failed to elevate");
        return;
    }
    // From here, it's the new elevated process
}
