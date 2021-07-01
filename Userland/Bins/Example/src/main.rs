use std::{
    bits::{
        error::OsError,
        path::PathExistsState
    },
    obj::{
        impls::file::File,
        Object,
        UserCreatableObject
    },
    option::Option::None,
    result::{
        Result,
        Result::Ok
    }
};

fn main() -> Result<usize, OsError> {
    let file_path = Path::from("/Users/Marco/Docs/example.txt");
    let file = match file_path.exists() {
        PathExistsState::Exists(_) => File::open().for_read()
                                                  .for_write()
                                                  .apply_for(file_path)
                                                  .expect("Failed to open"),
        PathExistsState::NotExists => File::creat().for_read()
                                                   .for_write()
                                                   .apply_for(file_path)
                                                   .expect("Failed to create"),
        PathExistsState::ExistsUntil(_) | PathExistsState::EmptyPath => {
            panic!("Cannot create {}", file_path)
        }
    };

    let file_size =
        file.info().expect("Failed to retrieve file size").mem_info().used_size();

    let mmap = file.map_to_memory(None, 0, file_size, true)
                   .expect("Failed to map file to memory");

    let mut ptr_box = mmap.get_ptr_mut::<u8>().expect("Failed to obtain MMap pointer");
    for byte in ptr_box.iter_mut() {
        *byte = 0;
    }

    Ok(0)
}
