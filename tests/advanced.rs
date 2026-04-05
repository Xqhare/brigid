use athena::XffValue;
use brigid::Brigid;
use brigid::content::Content;
use std::path::Path;

#[test]
fn test_data_type_inference() {
    let root = "test_root_inference";
    let brigid = Brigid::new(root)
        .file("config.json", |_file| {
            // No default content, relying on inference
        })
        .establish()
        .expect("Failed to establish");

    // Manually create the file
    std::fs::write(Path::new(root).join("config.json"), "null").unwrap();

    let val = brigid.get_file("config.json").expect("Failed to get inferred JSON");
    assert_eq!(val, XffValue::Null);

    brigid.delete_all().expect("Failed to delete");
}

#[test]
fn test_fallback_path() {
    let root = "test_root_fallback_path";
    let fallback_dir = "test_fallback_dir";
    
    if !Path::new(fallback_dir).exists() {
        std::fs::create_dir_all(fallback_dir).unwrap();
    }
    
    let fallback_file = Path::new(fallback_dir).join("global_config.json");
    std::fs::write(&fallback_file, "\"fallback value\"").unwrap();

    let brigid = Brigid::new(root)
        .file("local_config.json", |file| {
            file.with_fallback_path(fallback_file);
        })
        .establish()
        .expect("Failed to establish");

    // local_config.json doesn't exist, should fallback to global_config.json
    let val = brigid.get_file("local_config.json").expect("Failed to get fallback file");
    assert_eq!(val, XffValue::from("fallback value"));

    brigid.delete_all().expect("Failed to delete");
    std::fs::remove_dir_all(fallback_dir).unwrap();
}

#[test]
fn test_complex_csv() {
    let root = "test_root_csv";
    let csv_data = XffValue::Array(vec![
        XffValue::Array(vec![XffValue::from("a"), XffValue::from("b")].into()),
        XffValue::Array(vec![XffValue::from("c"), XffValue::from("d")].into()),
    ].into());

    let brigid = Brigid::new(root)
        .file("data.csv", |file| {
            file.with_default_content(Content::CSV(csv_data.clone()));
        })
        .establish()
        .expect("Failed to establish");

    let val = brigid.get_file("data.csv").expect("Failed to get CSV");
    assert_eq!(val, csv_data);

    brigid.delete_all().expect("Failed to delete");
}

#[test]
fn test_lookup_ambiguity() {
    let root = "test_root_lookup";
    let brigid = Brigid::new(root)
        .file("config.json", |file| {
            file.with_default_content(Content::JSON(XffValue::from("root")));
        })
        .directory("subdir", |dir| {
            dir.file("config.json", |file| {
                file.with_default_content(Content::JSON(XffValue::from("subdir")));
            });
        })
        .establish()
        .expect("Failed to establish");

    // Exact path lookup
    assert_eq!(brigid.get_file("config.json").unwrap(), XffValue::from("root"));
    assert_eq!(brigid.get_file("subdir/config.json").unwrap(), XffValue::from("subdir"));

    // Recursive lookup (since "config.json" is in root, it should find that first in exact match)
    // If we only had it in subdir, recursive lookup would find it.
    
    brigid.delete_all().expect("Failed to delete");
}
