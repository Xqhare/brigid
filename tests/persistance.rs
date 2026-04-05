use brigid::Brigid;
use nabu::{XffValue, xff};

#[test]
fn simple_structure() {
    let root = "test_root_simple";
    let brigid = Brigid::new(root)
        .file("test.xff", |file| {
            file.with_default_content(brigid::content::Content::XFF(xff!("Hello World")));
        })
        .establish()
        .expect("Failed to establish");
    let path = brigid
        .get_file_path("test.xff")
        .expect("Failed to get file path");
    assert!(path.exists());

    let val = brigid.get_file("test.xff").expect("Failed to get file");
    assert_eq!(val, XffValue::from("Hello World"));
    assert!(
        brigid
            .update_file(
                "test.xff",
                brigid::content::Content::XFF(xff!("Goodbye World"))
            )
            .is_ok()
    );

    let val = brigid.get_file("test.xff").expect("Failed to get file");
    assert_eq!(val, XffValue::from("Goodbye World"));

    // Run it again - Default content should NOT be found when file is read back
    let _brigid_the_same = Brigid::new(root)
        .file("test.xff", |file| {
            file.with_default_content(brigid::content::Content::XFF(xff!("Not saved to disk")));
        })
        .establish()
        .expect("Failed to establish");

    let path = brigid
        .get_file_path("test.xff")
        .expect("Failed to get file path");
    assert!(path.exists());
    let del = brigid.delete_file("test.xff");
    println!("{:?}", del);
    assert!(del.is_ok());
    assert!(brigid.delete_all().is_ok());
}

#[test]
fn updating() {
    let root = "test_root_update";
    let brigid = Brigid::new(root)
        .file("test.xff", |file| {
            file.with_default_content(brigid::content::Content::XFF(xff!("Hello World")));
        })
        .establish()
        .expect("Failed to establish");

    let new_val = XffValue::from("Goodbye World");
    assert!(
        brigid
            .update_file("test.xff", brigid::content::Content::XFF(new_val.clone()))
            .is_ok()
    );
    let val = brigid.get_file("test.xff").expect("Failed to get file");
    assert_eq!(val, new_val);

    assert!(brigid.delete_all().is_ok());
}
