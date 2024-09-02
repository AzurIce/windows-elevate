# windows-elevate

 **windows-elevate** is a Windows-only crate, it provides two utility functions:
 - `check_elevated()` is used to determine whether the current process is running as elevated.
 - `elevate()` is used to elevate the current process permissions.

 ## Example
 ```rust
 use windows_elevate::{check_elevated, elevate};

fn test_elevate() {
    let is_elevated = check_elevated().expect("Failed to call check_elevated");

    if !is_elevated {
        elevate().expect("Failed to elevate");
        return;
    }
    // From here, it's the new elevated process
}
 ```

