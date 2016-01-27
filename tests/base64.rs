extern crate transform;

use transform::plugins::base64::{encode, decode};

fn assert_transform<F>(transform: F, input: &[u8], output: &[u8])
where F: Fn(&[u8], bool) -> (Vec<u8>, usize) {
    let (result, rest) = transform(input, true);
    assert_eq!(rest, 0);
    assert_eq!(&result[..], output);
}

#[test]
fn encode_empty() {
    assert_transform(encode, b"", b"");
}
#[test]
fn encode_string() {
    assert_transform(encode, b"pleasure.", b"cGxlYXN1cmUu");
}
#[test]
fn encode_with_padding() {
    //assert_transform(encode, b"sure.", b"c3VyZS4=");
    //assert_transform(encode, b"sure.sure.", b"c3VyZS5zdXJlLg==");
    assert_transform(encode, b".", b"Lg==");
}
#[test]
fn encode_transparent_gif() {
    assert_transform(encode,
        &[
            0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x01, 0x44, 0x00, 0x3b,
        ],
        b"R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7");
}

#[test]
fn decode_empty() {
    assert_transform(decode, b"", b"");
}
#[test]
fn decode_string() {
    assert_transform(decode, b"cGxlYXN1cmUu", b"pleasure.");
}
#[test]
fn decode_with_padding() {
    assert_transform(decode, b"c3VyZS4=", b"sure.");
    assert_transform(decode, b"c3VyZS5zdXJlLg==", b"sure.sure.");
}
#[test]
fn decode_concatenated() {
    assert_transform(decode, b"c3VyZS4=c3VyZS4=", b"sure.sure.");
}
#[test]
fn decode_transparent_gif() {
    assert_transform(decode,
        b"R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7",
        &[
            0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff, 0xff, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x01, 0x44, 0x00, 0x3b,
        ]);
}
