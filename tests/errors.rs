use brigid::Brigid;
use brigid::error::BrigidError;

#[test]
fn test_file_not_found() {
    let root = "test_root_not_found";
    let brigid = Brigid::new(root)
        .establish()
        .expect("Failed to establish");

    let res = brigid.get_file("non_existent.json");
    match res {
        Err(BrigidError::FileNotFound(name)) => assert!(name.contains("non_existent.json")),
        _ => panic!("Expected FileNotFound error, got {:?}", res),
    }

    brigid.delete_all().unwrap();
}

#[test]
fn test_nested_file_not_found() {
    let root = "test_root_nested_not_found";
    let brigid = Brigid::new(root)
        .directory("subdir", |_| {})
        .establish()
        .expect("Failed to establish");

    let res = brigid.get_file("subdir/missing.json");
    match res {
        Err(BrigidError::FileNotFound(name)) => assert!(name.contains("missing.json")),
        _ => panic!("Expected FileNotFound error, got {:?}", res),
    }

    brigid.delete_all().unwrap();
}
