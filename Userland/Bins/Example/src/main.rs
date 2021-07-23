#![no_std]

extern crate mx_std;

use mx_std::{
    api_bits::{
        error::OsError,
        path::PathExistsState
    },
    object::{
        impls::file::File,
        TObject,
        TUserCreatableObject
    },
    option::Option::None,
    path::Path,
    result::{
        Result,
        Result::Ok
    }
};

fn main() -> Result<usize, OsError> {
    let file_path = Path::from("/Users/Marco/Docs/example.txt");
    let file = match file_path.exists()? {
        PathExistsState::Exists(_) => File::open().for_read()
                                                  .for_write()
                                                  .apply_for(&file_path)
                                                  .expect("Failed to open"),
        PathExistsState::NotExists => File::creat().for_read()
                                                   .for_write()
                                                   .apply_for(&file_path)
                                                   .expect("Failed to create"),
        PathExistsState::ExistsUntil(_) | PathExistsState::EmptyPath => {
            panic!("Cannot create {}", file_path)
        }
    };

    let mmap = file.map_to_memory(None, 0, file.info()?.data_bytes_used(), true)?;

    let mut ptr_box = mmap.ptr_mut::<u8>()?;
    for byte in ptr_box.iter_mut() {
        *byte = 0;
    }

    Ok(0)
}
