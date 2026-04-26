use std::io::Read;

use include_compress_bytes::include_bytes_brotli;

#[test]
fn test_include_bytes_brotli() {
    let content = include_bytes_brotli!("./content/hello.txt");
    let mut data = Vec::with_capacity(8 * 1024);
    {
        let mut decoder = brotli::Decompressor::new(content as &[u8], 4096);
        let mut buf = [0u8; 4096];
        loop {
            match decoder.read(&mut buf[..]) {
                Ok(0) => break,
                Ok(read) => data.extend_from_slice(&buf[..read]),
                Err(e) => panic!("Failed to read compressed content: {}", e),
            }
        }
    }
    assert_eq!(data, b"Hello, world!");
}
