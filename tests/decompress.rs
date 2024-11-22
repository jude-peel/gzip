use std::fs;

use ::gzip::{deflate, gzip::GzipFile};

//#[test]
#[allow(dead_code)]
fn test_header() {
    let file = fs::read("./tests/data/block_type_0.gz").unwrap();

    let gzfile = GzipFile::build(&file).unwrap();
    let header = gzfile.header;

    println!(
        "{} {:?} {} {} {} {:?} {:?} {:?} {:?} {}",
        header.cm,
        header.flg,
        header.mtime,
        header.xfl,
        header.os,
        header.crc,
        header.fextra,
        header.fname,
        header.fcomment,
        header.end_idx
    );

    for byte in gzfile.deflate {
        println!("{:08b}", byte);
    }
}

//#[test]
#[allow(dead_code)]
fn test_deflate() {
    let type_0 = fs::read("./tests/data/block_type_0.gz").unwrap();
    let type_1 = fs::read("./tests/data/block_type_1.gz").unwrap();
    let type_2 = fs::read("./tests/data/block_type_2.gz").unwrap();

    let file_0 = GzipFile::build(&type_0).unwrap();
    let file_1 = GzipFile::build(&type_1).unwrap();
    let file_2 = GzipFile::build(&type_2).unwrap();

    let block_0 = deflate::DeflateBlock::build(&file_0.deflate).unwrap();
    let block_1 = deflate::DeflateBlock::build(&file_1.deflate).unwrap();
    let block_2 = deflate::DeflateBlock::build(&file_2.deflate).unwrap();

    let decompressed_0 = block_0.decompress().unwrap();
    let decompressed_1 = block_1.decompress().unwrap();
    let decompressed_2 = block_2.decompress().unwrap();

    let decompressed_0_string = String::from_utf8_lossy(&decompressed_0);
    let decompressed_1_string = String::from_utf8_lossy(&decompressed_1);
    let decompressed_2_string = String::from_utf8_lossy(&decompressed_2);

    assert_eq!(decompressed_0_string, "Lorem ipsum");
    assert_eq!(decompressed_1_string, "Lorem ipsum");
    assert_eq!(
        decompressed_2_string,
        "Lorem impsum dolor sit amet, consectetur adipiscing elit."
    );
}
