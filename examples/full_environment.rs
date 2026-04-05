use athena::XffValue;
use brigid::Brigid;
use brigid::SchedulerPolicy;
use brigid::content::Content;
use std::path::Path;

fn main() {
    // 1. Define a complex environment
    let root = "my_app_env";

    // Create a dummy global config for fallback demonstration
    let global_fallback = "global_fallback.json";
    std::fs::write(global_fallback, "\"global default\"").expect("Failed to write dummy fallback");

    let brigid = Brigid::new(root)
        // System settings
        .with_priority(10)
        .with_scheduler_policy(SchedulerPolicy::Fifo)
        // Root files
        .file("config.json", |f| {
            f.with_default_content(Content::JSON(XffValue::from("local default")))
                .with_fallback_path(global_fallback) // Try global file if local is missing
                .with_fallback(); // Finally fall back to "local default" in-memory
        })
        // Nested structure
        .directory("data", |dir| {
            dir.file("database.xff", |f| {
                f.with_default_content(Content::XFF(XffValue::Null));
            })
            .directory("backups", |subdir| {
                subdir.file("last_backup.json", |f| {
                    f.with_default_content(Content::JSON(XffValue::Null));
                });
            });
        })
        // Establish everything on disk
        .establish()
        .expect("Failed to establish environment");

    // 2. Verify and use the environment
    assert!(Path::new(root).join("config.json").exists());
    assert!(Path::new(root).join("data/database.xff").exists());
    assert!(
        Path::new(root)
            .join("data/backups/last_backup.json")
            .exists()
    );

    // Check fallback logic (deleting local config to trigger fallback to global_fallback.json)
    std::fs::remove_file(Path::new(root).join("config.json")).unwrap();
    let val = brigid.get_file("config.json").expect("Failed to get file");
    assert_eq!(val, XffValue::from("global default"));

    // 3. Cleanup
    brigid.delete_all().expect("Failed to cleanup root");
    std::fs::remove_file(global_fallback).expect("Failed to cleanup global fallback");

    println!("Full environment example completed successfully!");
}

#[test]
fn general_test() {
    main();
}
