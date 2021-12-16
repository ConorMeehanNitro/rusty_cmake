#[cxx::bridge]
mod ffi {
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        fn rusty_cxxbridge_integer() -> i32;

        fn rusty_get_blob() -> BlobMetadata;
    }
}

pub fn rusty_cxxbridge_integer() -> i32 {
    42
}


pub fn rusty_get_blob() -> ffi::BlobMetadata {
    let blob = ffi::BlobMetadata{
        size: 3,
        tags: vec![
            String::from("Tag The First"),
            String::from("Tag The Second"),
            String::from("Tag The Third")
        ]
    };

    blob
}

#[no_mangle]
pub extern "C" fn rusty_extern_c_integer() -> i32 {
    322
}
