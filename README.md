# Brigid

A library to set up and manage local file/directory environments for binary projects within the **Xqhare** ecosystem.

Brigid provides a builder-based API to define directory structures, establish them on disk with default content (CSV, JSON, XFF), and configure system-level process settings like CPU/IO priority and licenses.

It follows the "The Pantheon" philosophy: minimal dependencies, relying only on the Rust standard library and other internal ecosystem crates (`athena`, `nabu`, `mawu`, `aequa`).

## Features

- **Dependency-Free**: Only uses internal ecosystem crates and std.
- **Environment Establishment**: Define and create complex directory structures recursively.
- **Default Content**: Save files with initial content in CSV, JSON, or XFF formats.
- **Data Type Inference**: Automatically infer file formats from extensions (.json, .csv, .xff).
- **Robust Access**: Optional fallback to default content or other on-disk files if primary files are missing or corrupted.
- **System Configuration**: Configure process priority (nice value -20 to 19), CPU scheduler, and I/O scheduling policy.
- **License Persistence**: Automatically copy license files to system locations during establishment.
- **Non-Fatal Warnings**: Collects system warnings (e.g., if priority cannot be set) instead of panicking.

## Naming

As with all my projects, Brigid is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Brigid or Brigit (meaning 'exalted one'), is a prominent deity of pre-Christian Celtic mythology. The stories surrounding Brigid are among the most complex of Celtic myths. Brigid was the Celtic goddess of fire, poetry, and prophecy, but she was also associated with water (especially rivers and streams), childbirth, the hearth, and healing.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
brigid = { git = "https://github.com/xqhare/brigid" }
```

### Example

```rust
use brigid::Brigid;
use brigid::content::Content;
use athena::XffValue;

fn main() {
    let brigid = Brigid::new("my_app_data")
        .file("config.json", |file| {
            file.with_default_content(Content::JSON(XffValue::Null))
                .with_fallback();
        })
        .directory("data", |dir| {
            dir.file("db.xff", |file| {
                file.with_default_content(Content::XFF(XffValue::Null));
            });
        })
        .add_license("LICENSE", "my_app_data/copyright")
        .with_priority(19)
        .establish()
        .expect("Failed to establish environment");

    if brigid.has_warnings() {
        for warning in brigid.get_warnings() {
            println!("Warning: {:?}", warning);
        }
    }

    let config = brigid.get_file("config.json").expect("Failed to get config");
    println!("Config: {:?}", config);
    assert_eq!(config, XffValue::Null);
    assert!(brigid.no_warnings());
    assert!(brigid.delete_all().is_ok());
}
```

## License

Brigid is distributed under the [MIT](https://github.com/xqhare/brigid/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/brigid/blob/master/CONTRIBUTING.md) for contribution guidelines.
