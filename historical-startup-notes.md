
Write a library to help me set up the environment for binary projects.

The common tasks I find myself doing are:

- Checking if a directory structure exists
- Checking if a file exists
- Creating a directory structure
- Creating a file with default content
- Setting up process priority, scheduling and IO policy
    - Already implemented, just pull in `athena`

## Usage

Some important notes:

- The config file is inside the application directory
- The data file is inside the internal directory

```rust
let env = Brigid::new(".local/state/app_name") // This path can be an absolute path or a relative path. I'll just pass it into a PathBuf and see what happens
    .file("config", |file| { // file ending is optional / overridden by the Content type
        file.with_content(Content::Json(XffValue::Null)); // As `mawu` also takes XffValue
        file.with_fallback("internal/config.json") // Does not need default content, same as the file its a fallback for
    })) 
    .directory("internal", |dir| {
        dir.file("data.json", |file| { // file ending is optional / overwritten by the Content type (can even be wrong like here)
            file.with_content(Content::Xff(XffValue::Null)); // `Nabu` takes XffValue naturally
        })
        dir.file("data.csv", |file| {
            file.with_content(Content::Csv(XffValue::Null)); // `Mawu` also supports CSV
        })
    })
    .priority(2) // Nice value - If any of the process functions error during .establish(), athena will return a nice error message; Make sure for these to "soft fail" - run last, after everything else is already done and set up; For this .establish will probably need to return two types off errors, one recoverable (process stuff) one not (filesystem errors)
    .io_policy(IoPolicy::Realtime)
    .scheduler(Scheduler::Fifo)
    .add_license("path/to/license", "app_name") // All Paths are copied to `/usr/share/licenses/<app_name>/copyright` (Failure only warns)
    .add_license("path/to/license2", "app_name") // Can be multiple (`Vec<PathBuf>`) that need to be valid.
    .establish()?; // establish() could return a BrigidResult<Brigid, (Brigid, Vec<SystemWarning>)>

if env.has_warnings() {
    for warning in env.warnings() {
        println!("Warning: {}", warning);
    }
}
// .get_file uses the above defined value types for determining the file type
let config: XffValue = env.get_file("config.json")?; // Filetype does not need to be specified by the file ending. If it is specified, it MUST be correct to the file read.
let _convinience: Vec<u8> = env.get_raw("config.json")?; // Really only rarely usefull I think (.xff inherently supports arbitrary byte streams), but convinient non the less
let data: XffValue = env.get_file("internal/data.xff")?;

assert_eq!(config, XffValue::Null);
assert_eq!(data, XffValue::Null);

let secondary_base_env = env.directory("/usr/tmp/app").establish()?;
assert!(secondary_base_env.no_warnings());
```

##### Notes

The builder should probably use a `HashMap<PathBuf, FileDefinition>` internally. If a path is overwritten, the builder should probably panic! Or return an error during the building phase to prevent ambiguity.

Fallback logic:

The use-case I imagine is that one provides a user editable config file, but if this file is deleted / corrupted / whatever, it should be able to fall back to a default file.
This can of course also work fine with a single file:

```rust
.file("config.json", Content::Json(XffValue::Null))
.with_fallback();
```

Where Brigid just presents the wrapped Xff if the .get_file errors on the .file path

This will mean a larger footprint, but I think it is worth it for the simplicity and ease of use, also I would only store all values in memory for files that have a fallback declared. I expect most default values to be light on memory by themselves.

## Naming

Currently, I am between Hestia and Brigid.
Probably Brigid:
    - Many Greek and Roman deities are already part of the pantheon
    - This will be a "core" library of sorts, so a important / more well known deity is a good choice (according to my own rules)

### Hestia

Hestia is the primary Greek goddess of the hearth, home, and domestic life, serving as the heart of the household and city, ensuring stability and warmth.

### Brigid

Brigid or Brigit (meaning 'exalted one'), is a prominent deity of pre-Christian Celtic mythology.
The stories surrounding Brigid are among the most complex of Celtic myths.

Brigid was the Celtic goddess of fire, poetry, and prophecy, but she was also associated with water (especially rivers and streams), childbirth, the hearth, and healing.

### Vesta

Hestias Roman equivalent.
