use std::{env, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let cache_dir = Path::new(&out_dir).join(".include_compress_bytes_cache");
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    }
    cargo_emit::rustc_env!(
        "INCLUDE_COMPRESS_BYTES_CACHE_DIR",
        "{}",
        cache_dir.display().to_string()
    );
}
