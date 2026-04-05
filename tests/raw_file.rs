use brigid::Brigid;
use brigid::content::Content;
use athena::XffValue;

#[test]
fn test_get_raw_file() {
    let root = "test_root_raw";
    let brigid = Brigid::new(root)
        .file("test.bin", |file| {
            file.with_default_content(Content::JSON(XffValue::from("Hello World")));
        })
        .establish()
        .expect("Failed to establish");

    let raw = brigid.get_raw_file("test.bin").expect("Failed to get raw file");
    let content = String::from_utf8(raw).unwrap();
    // JSON "Hello World" will be saved with pretty formatting (2 spaces indent)
    // but the value itself is on its own line
    assert!(content.contains("Hello World"));

    brigid.delete_all().unwrap();
}

#[test]
fn test_deep_nesting() {
    let root = "test_root_deep";
    let brigid = Brigid::new(root)
        .directory("a", |dir| {
            dir.directory("b", |dir| {
                dir.directory("c", |dir| {
                    dir.file("deep.json", |file| {
                        file.with_default_content(Content::JSON(XffValue::from("deep")));
                    });
                });
            });
        })
        .establish()
        .expect("Failed to establish");

    assert_eq!(
        brigid.get_file("a/b/c/deep.json").unwrap(),
        XffValue::from("deep")
    );

    brigid.delete_all().unwrap();
}
