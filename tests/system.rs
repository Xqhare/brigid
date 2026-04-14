use brigid::Brigid;
use std::path::Path;

#[test]
fn test_add_license() {
    let root = "test_root_license";
    let license = "TEST STRING MIT License Content lorem ipsum dolor sit amet";
    let license_target_dir = "test_license_target_dir";
    let license_target = Path::new(license_target_dir).join("copyright.txt");

    let brigid = Brigid::new(root)
        .add_license(license, &license_target)
        .establish()
        .expect("Failed to establish");

    assert!(license_target.exists());
    assert_eq!(std::fs::read_to_string(license_target).unwrap(), license);
    assert!(brigid.no_warnings());

    brigid.delete_all().expect("Failed to delete");
    std::fs::remove_dir_all(license_target_dir).unwrap();
}

#[test]
fn test_license_not_found_warning() {
    let root = "test_root_license_warning";
    let brigid = Brigid::new(root)
        .add_license("LICENSE TEXT", "/tmp/non_existent_target/")
        .establish()
        .expect("Failed to establish");

    assert!(brigid.has_warnings());
    let warnings = brigid.get_warnings();
    assert!(
        warnings
            .iter()
            .any(|w| w.to_string().to_lowercase().contains("is a directory"))
    );

    brigid.delete_all().expect("Failed to delete");
}

#[test]
fn test_priority_out_of_bounds() {
    let root = "test_root_priority_warning";

    // Test too high
    let brigid_high = Brigid::new(root)
        .with_priority(25)
        .establish()
        .expect("Failed to establish");
    assert!(brigid_high.has_warnings());
    assert!(
        brigid_high
            .get_warnings()
            .iter()
            .any(|w| w.to_string().contains("Priority too high"))
    );
    brigid_high.delete_all().unwrap();

    // Test too low
    let brigid_low = Brigid::new(root)
        .with_priority(-25)
        .establish()
        .expect("Failed to establish");
    assert!(brigid_low.has_warnings());
    assert!(
        brigid_low
            .get_warnings()
            .iter()
            .any(|w| w.to_string().contains("Priority too low"))
    );
    brigid_low.delete_all().unwrap();
}
