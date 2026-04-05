use athena::XffValue;
use brigid::Brigid;

fn main() {
    let app_root = "app_name_data";
    
    let brigid = Brigid::new(app_root)
        // Root file with fallback
        .file("mainfile.xff", |file| {
            file.with_default_content(brigid::content::Content::XFF(XffValue::Null))
                .with_fallback();
        })
        // Nested directory and file
        .directory("internal", |dir| {
            dir.file("subfile.xff", |file| {
                file.with_default_content(brigid::content::Content::XFF(XffValue::Null));
            });
        })
        .establish()
        .expect("Failed to establish");

    // Retrieve files using relative paths
    let main = brigid.get_file("mainfile.xff").expect("Failed to get main");
    let sub = brigid.get_file("internal/subfile.xff").expect("Failed to get subfile");
    
    // Demonstrate recursive lookup (works if name is unique and not a path)
    let sub_alt = brigid.get_file("subfile.xff").expect("Failed to get subfile via recursive lookup");

    assert_eq!(main, XffValue::Null);
    assert_eq!(sub, XffValue::Null);
    assert_eq!(sub_alt, XffValue::Null);

    // Demonstration complete, cleaning up
    brigid.delete_all().expect("Failed to cleanup");
    println!("General example completed successfully!");
}

#[test]
fn general_test() {
    main();
}
