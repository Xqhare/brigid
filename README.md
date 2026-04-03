# Brigid

TODO:

- Consider ArgosCI integration
- Consider needed dependencies in `Cargo.toml`

A library to set up the environment for binary projects

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- No dependencies

## Environment

Brigid expects the environment to provide:

- `ls` UNIX command

## Naming

As with all my projects, Brigid is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Brigid or Brigit (meaning 'exalted one'), is a prominent deity of pre-Christian Celtic mythology. The stories surrounding Brigid are among the most complex of Celtic myths. Brigid was the Celtic goddess of fire, poetry, and prophecy, but she was also associated with water (especially rivers and streams), childbirth, the hearth, and healing.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
Brigid = { git = "https://github.com/xqhare/brigid" }
```

### Example

```rust

```

## License

Brigid is distributed under the [MIT](https://github.com/xqhare/brigid/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/brigid/blob/master/CONTRIBUTING.md) for contribution guidelines.
