use athena::XffValue;
use brigid::Brigid;

fn main() {
    let brigid = Brigid::new("general_testing")
        .file("test", |file| {
            file.with_default_content(brigid::content::Content::XFF(XffValue::Null))
                .with_fallback();
        })
        .directory("test2", |dir| {
            dir.file("testing2", |file| {
                file.with_default_content(brigid::content::Content::XFF(XffValue::Null));
            });
        })
        .establish();
    assert!(brigid.is_ok());
    assert!(brigid.unwrap().delete_all().is_ok());
}
