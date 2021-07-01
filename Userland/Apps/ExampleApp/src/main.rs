use std::obj::{
    impls::file::File,
    UserCreatableObject
};

fn main() {
    let f = File::creat().for_read()
                         .for_write()
                         .apply_for("/Users/Marco/Docs/example.txt")
                         .unwrap();

    let mut _read_buf = [0u8; 512];
    f.read(&mut _read_buf).unwrap();
}
