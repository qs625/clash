use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum CompressKind {
    #[default]
    None,
    #[cfg(feature = "brotli")]
    Brotli,
}

pub(crate) fn cached_path_content(
    path: &std::path::Path,
    compress_kind: CompressKind,
) -> Result<std::path::PathBuf, String> {
    let out_dir = std::path::Path::new(env!("INCLUDE_COMPRESS_BYTES_CACHE_DIR"));
    if !out_dir.exists() {
        std::fs::create_dir_all(out_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }
    let crate_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".into());
    let mut hasher = Sha256::new();
    hasher.update(crate_name.as_bytes());
    hasher.update(b"\0");
    hasher.update(path.as_os_str().as_encoded_bytes());
    hasher.update(b"\0");
    hasher.update(format!("{:?}", compress_kind));
    let hash = hasher.finalize();
    let filename = format!("{:x}", hash);
    let cache_file = out_dir.join(filename);
    if cache_file.exists() {
        return Ok(cache_file);
    }

    let content = std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let content = match compress_kind {
        CompressKind::None => content,
        #[cfg(feature = "brotli")]
        CompressKind::Brotli => {
            let mut buffer = Vec::with_capacity(4096);
            {
                let mut encoder = brotli::CompressorWriter::new(&mut buffer, 4096, 11, 22);
                encoder
                    .write_all(&content)
                    .map_err(|e| format!("Failed to write compressed content: {}", e))?;
                encoder
                    .flush()
                    .map_err(|e| format!("Failed to flush compressed content: {}", e))?;
            }
            buffer
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&cache_file)
        .map_err(|e| format!("Failed to open cache file: {}", e))?;

    file.write_all(&content)
        .map_err(|e| format!("Failed to write cache file: {}", e))?;
    Ok(cache_file)
}

fn resolve_path(caller_path: &std::path::Path, input_path: &str) -> Result<PathBuf, String> {
    let parent = caller_path.parent().unwrap_or(caller_path);

    let path = std::path::Path::new(&input_path);

    if path.is_absolute() {
        path.to_path_buf()
            .canonicalize()
            .map_err(|e| format!("failed to find file: {input_path}: {e:?}"))
    } else {
        let path = parent.join(path);
        path.canonicalize()
            .map_err(|e| format!("failed to find file: {path:?}: {e:?}"))
    }
}

/// Include file bytes with Brotli compression
///
/// Works similarly to `include_bytes!` but compresses the data with Brotli
/// and returns the compressed bytes
///
/// # Example
/// ```
/// const COMPRESSED_DATA: &[u8] = include_bytes_brotli!("large_file.json");
/// ```
#[cfg(feature = "brotli")]
#[proc_macro]
pub fn include_bytes_brotli(input: TokenStream) -> TokenStream {
    use syn::{LitStr, parse_macro_input};

    let input_str = parse_macro_input!(input as LitStr);

    let span = proc_macro2::Span::call_site();
    let Some(local_file) = span.local_file() else {
        return quote! {
            compile_error!("failed to get local file")
        }
        .into();
    };
    let path = match resolve_path(local_file.as_path(), &input_str.value()) {
        Ok(path) => path,
        Err(e) => {
            let error_msg = format!("Failed to resolve path: {}", e);
            return quote! {
                compile_error!(#error_msg)
            }
            .into();
        }
    };

    match cached_path_content(&path, CompressKind::Brotli) {
        Ok(cache_path) => {
            let cache_path_str = cache_path.to_str().expect("Invalid cache path");
            let expanded = quote! {
                include_bytes!(#cache_path_str)
            };
            expanded.into()
        }
        Err(e) => {
            let error_msg = format!("Failed to process file with Brotli compression: {}", e);
            quote! {
                compile_error!(#error_msg)
            }
            .into()
        }
    }
}
