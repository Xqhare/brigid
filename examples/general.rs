use athena::XffValue;
use brigid::Brigid;

fn main() {
    let brigid = Brigid::new("general/app_name")
        .file("mainfile.xff", |file| {
            file.with_default_content(brigid::content::Content::XFF(XffValue::Null))
                .with_fallback();
        })
        .directory("internal", |dir| {
            dir.file("subfile.xff", |file| {
                file.with_default_content(brigid::content::Content::XFF(XffValue::Null));
            });
        })
        .establish();
    assert!(brigid.is_ok());
    let brigid = brigid.unwrap();
    let main = brigid.get_file("mainfile.xff");
    let sub = brigid.get_file("internal/subfile.xff");
    let sub_alt = brigid.get_file("subfile.xff");
    assert!(main.is_ok());
    assert!(sub.is_ok());
    assert!(sub_alt.is_ok());
    assert!(brigid.delete_all().is_ok());
}

#[test]
fn general_test() {
    main();
}
