use athena::XffValue;
use brigid::Brigid;
use brigid::content::Content;

fn main() {
    let root = "csv_demo_root";

    // Create multi-row, multi-column data
    let row1 = XffValue::Array(vec![XffValue::from("ID"), XffValue::from("Name")].into());
    let row2 = XffValue::Array(vec![XffValue::from("A"), XffValue::from("Alice")].into());
    let row3 = XffValue::Array(vec![XffValue::from("B"), XffValue::from("Bob")].into());
    let data = XffValue::Array(vec![row1, row2, row3].into());

    let brigid = Brigid::new(root)
        .file("users.csv", |f| {
            f.with_default_content(Content::CSV(data.clone()));
        })
        .establish()
        .expect("Failed to create CSV");

    // Load the whole CSV back as an XffValue::Array
    let loaded = brigid.get_file("users.csv").expect("Failed to read CSV");

    // Verify results
    assert_eq!(loaded, data);

    // Display the first row
    if let XffValue::Array(rows) = loaded {
        let first_row = &rows[0];
        println!("Header row: {:?}", first_row);
    }

    brigid.delete_all().unwrap();
    println!("Advanced CSV example completed successfully!");
}

#[test]
fn general_test() {
    main();
}
