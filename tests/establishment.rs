use brigid::Brigid;
use brigid::content::Content;
use athena::XffValue;
use std::path::Path;

#[test]
fn test_establishment() {
    let root = "test_root_establishment";
    let brigid = Brigid::new(root)
        .file("config.json", |file| {
            file.with_default_content(Content::JSON(XffValue::Null));
        })
        .directory("data", |dir| {
            dir.file("db.xff", |file| {
                file.with_default_content(Content::XFF(XffValue::Null));
            });
        })
        .establish()
        .expect("Failed to establish Brigid");

    assert!(Path::new(root).exists());
    assert!(Path::new(root).join("config.json").exists());
    assert!(Path::new(root).join("data").exists());
    assert!(Path::new(root).join("data").join("db.xff").exists());

    let val = brigid.get_file("data/db.xff").expect("Failed to get file");
    assert_eq!(val, XffValue::Null);

    brigid.delete_all().expect("Failed to delete all");
    assert!(!Path::new(root).exists());
}

#[test]
fn test_fallback() {
    let root = "test_root_fallback";
    let brigid = Brigid::new(root)
        .file("missing.json", |file| {
            file.with_default_content(Content::JSON(XffValue::Null))
                .with_fallback();
        })
        .establish()
        .expect("Failed to establish Brigid");

    // Manually delete the file to force fallback
    std::fs::remove_file(Path::new(root).join("missing.json")).unwrap();

    let val = brigid.get_file("missing.json").expect("Failed to get file with fallback");
    assert_eq!(val, XffValue::Null);

    brigid.delete_all().expect("Failed to delete all");
}
