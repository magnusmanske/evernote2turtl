This implements a converter from Evernote export (zipped or unzipped HTML files) into [turtl](https://github.com/turtl) backup format, which can be imported into turtl.

# Setup
* [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
* Checkout this repo
* Run `cargo build --release` to build the release binary

# Run
To convert a zipped Evernote file into a Turtl backup file:
```
./target/release/evernote2turtl EVERNOTE.zip USER_ID
```
You can get the `USER_ID` by generating a backup file for your turtl setup, and look at the backup file.

To convert a Keep directory structure into a Turtl backup file:
```
./target/release/evernote2turtl KEEP_PATH USER_ID keep
```

# TODO
- Checkboxes `<div style="-moz-user-select: none;"><input checked="true" type="checkbox"/>TEXT</div>`
- Import files
