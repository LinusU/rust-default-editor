# Default editor

Get the default editor for the current environment

## Usage

```rust
extern crate default_editor;

fn main () {
    match default_editor::get() {
        Ok(editor) => println!("The default editor is: {}", editor),
        Err(error) => println!("Error getting default editor: {}", error),
    }
}

```
