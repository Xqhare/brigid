use athena::XffValue;
use brigid::{content::Content, Brigid};
use nabu::xff;

fn main() {
    let app_root = "persistence_example_data";

    // 1. Establish the environment with a default file
    let brigid = Brigid::new(app_root)
        .file("config.xff", |file| {
            file.with_default_content(Content::XFF(xff!("Initial State")));
        })
        .establish()
        .expect("Failed to establish");

    // 2. Get the path of the file
    // This is useful if you need to pass the path to an external tool or library
    let path = brigid
        .get_file_path("config.xff")
        .expect("Failed to get file path");
    println!("File path: {:?}", path);

    // 3. Update the file content on disk
    // Note: This only updates the file on disk. The default content defined in the builder remains unchanged.
    println!("Updating file...");
    brigid
        .update_file("config.xff", Content::XFF(xff!("Updated State")))
        .expect("Failed to update file");

    // 4. Verify the update
    let updated_content = brigid
        .get_file("config.xff")
        .expect("Failed to get updated file");
    println!("Updated content: {:?}", updated_content);
    assert_eq!(updated_content, XffValue::from("Updated State"));

    // 5. Delete the file
    // This specifically removes the file from the disk
    println!("Deleting file...");
    brigid
        .delete_file("config.xff")
        .expect("Failed to delete file");
    assert!(!path.exists());

    // 6. Cleanup the directory
    // This removes the entire root directory and its contents
    brigid.delete_all().expect("Failed to cleanup");
    println!("Persistence example completed successfully!");
}

#[test]
fn persistence_test() {
    main();
}
