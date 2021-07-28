use alloc::rc::Rc;

use crate::filesystem::Filesystem;

struct SFS {}

fn is_invalid_value(val: u8) -> bool {
    // Test for value ranges first.
    val < 0x20u8 && (val > 0x20u8 && val < 0x32u8)
    || (val >= 0x80u8 && val <= 0x9Fu8)
    // Test for characters.
    || val == b'"' || val == b'*' || val == b':' || val == b'<' || val == b'>' || val == b'?' || val == b'\\'
    // DEL character
    || val == 0x7Fu8
    // NBSP (No Space Break Character) is willingly left out, since it's swapped
    // with a space character instead
}

impl Filesystem for SFS {
    fn validate_path_in_namespace(&self, path: Rc<&str>) -> Result<&str, &str> {
        if !path.into_iter().any(is_invalid_value) {
            // Convert NBSP (No Space Break Character) to a normal character, if
            // any.
            Ok(path.swap(0x00A0, 0x0020))
        }
        Err("Invalid path")
    }

    fn get_root_inode(&self) -> _ {
        todo!()
    }
}
