use std::{time::Instant};

use serde::Deserialize;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
#[derive(Deserialize)]
struct FileMetadata {
    id: String,
    name: String,
    thumbnail: Option<String>,
    sourceFileType: String,
    providedFileType: String,

    // deprecated in v2
    fileType: String,
}

#[inline(never)]
fn decode(data: &[u8]) -> FileMetadata {
    serde_json::from_slice(data).expect("Deserialization error")
}

fn benchmark(name: &str, data: &[u8], n: u32) {
    let mut filemeta: FileMetadata;
    
    let st = Instant::now();

    for _ in 0..n {
        filemeta = decode(data);
        black_box(filemeta);
    }
    
    let dt = st.elapsed();
    
    println!("{name:24} :: {:>12?} ({n} iterations)", dt);
}

fn main() {
    let metadata_bytes: &[u8] = include_bytes!("../metadata.json");
    let metadata_w_thumbnail_bytes: &[u8] = include_bytes!("../metadata_w_thumbnail.json");

    let n = 128;

    benchmark("metadata", metadata_bytes, n);
    benchmark("metadata /w thumbnail", metadata_w_thumbnail_bytes, n);
}
