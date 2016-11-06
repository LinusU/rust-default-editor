extern crate default_editor;

fn main () {
    match default_editor::get() {
        Ok(editor) => println!("The default editor is: {}", editor),
        Err(error) => println!("Error getting default editor: {}", error),
    }
}
