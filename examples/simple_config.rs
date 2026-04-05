use athena::XffValue;
use brigid::Brigid;
use brigid::content::Content;

fn main() {
    let app_root = "simple_app_data";

    // Quick and clean setup for a single config file
    let brigid = Brigid::new(app_root)
        .file("config.json", |f| {
            f.with_default_content(Content::JSON(XffValue::from("default value")))
                .with_fallback();
        })
        .establish()
        .expect("Failed to setup config");

    let config = brigid
        .get_file("config.json")
        .expect("Failed to get config");
    assert_eq!(config, XffValue::from("default value"));

    // Deleting the file on disk, brigid still returns the default value because of .with_fallback()
    std::fs::remove_file("simple_app_data/config.json").unwrap();
    let val = brigid
        .get_file("config.json")
        .expect("Failed to get fallback");
    assert_eq!(val, XffValue::from("default value"));

    brigid.delete_all().unwrap();
    println!("Simple config example completed successfully!");
}

#[test]
fn general_test() {
    main();
}
