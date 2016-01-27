pub fn encode(input_all: &[u8], finish: bool) -> (Vec<u8>, usize) {
    let bytes_left = input_all.len() % 3;
    let input = &input_all[..(input_all.len() - bytes_left)];

    let mut output = Vec::with_capacity(((input.len() / 3) + 1) * 4);
    for chunk in input.chunks(3) {
        let in_chunk = [chunk[0], chunk[1], chunk[2]];
        let mut out_chunk = [0; 4];
        encode_chunk(&in_chunk, &mut out_chunk, 3);
        output.extend_from_slice(&out_chunk);
    }
    if finish && bytes_left > 0 {
        let mut in_chunk = [0; 3];
        for i in 0..bytes_left {
            in_chunk[i] = input_all[input.len() + i];
        }
        let mut out_chunk = [0; 4];
        encode_chunk(&in_chunk, &mut out_chunk, bytes_left);
        output.extend_from_slice(&out_chunk);
        (output, 0)
    } else {
        (output, bytes_left)
    }
}

pub fn decode(input: &[u8], finish: bool) -> (Vec<u8>, usize) {
    let bytes_left = input.len() % 4;
    assert!(bytes_left == 0 || !finish);
    let input = &input[..(input.len() - bytes_left)];

    let mut output = Vec::with_capacity(input.len() / 4 * 3);
    for chunk in input.chunks(4) {
        let mut in_chunk = [chunk[0], chunk[1], chunk[2], chunk[3]];
        let mut out_chunk = [0; 3];
        let size = decode_chunk(&mut in_chunk, &mut out_chunk);
        output.extend_from_slice(&out_chunk[..size]);
    }
    (output, bytes_left)
}

fn encode_chunk(input: &[u8; 3], output: &mut [u8; 4], input_size: usize) {
    output[0] = input[0] >> 2;
    output[1] = 0b00110000 & input[0] << 4 | input[1] >> 4;
    output[2] = 0b00111100 & input[1] << 2 | input[2] >> 6;
    output[3] = 0b00111111 & input[2];
    for i in 0..output.len() {
        output[i] = encode_byte(output[i]);
    }
    if input_size < 3 {
        output[3] = b'=';
        if input_size < 2 {
            output[2] = b'=';
        }
    }
}

fn encode_byte(input: u8) -> u8 {
    match input {
        0...25  => input + b'A',
        26...51 => input + b'a' - 26,
        52...61 => input + b'0' - 52,
        62 => b'+',
        63 => b'/',
        _ => panic!("unreachable"),
    }
}

fn decode_chunk(input: &mut [u8; 4], output: &mut [u8; 3]) -> usize {
    assert!(input[0] != b'=');
    assert!(input[1] != b'=');
    assert!(input[2] != b'=' || input[3] == b'=');
    let len = if input[3] != b'=' { 3 }
        else if input[2] != b'=' { 2 }
        else { 1 };
    for i in 0..input.len() {
        input[i] = decode_byte(input[i]);
    }
    output[0] = input[0] << 2 | input[1] >> 4;
    output[1] = input[1] << 4 | input[2] >> 2;
    output[2] = input[2] << 6 | input[3];
    len
}

fn decode_byte(input: u8) -> u8 {
    match input {
        b'A'...b'Z' => input - b'A',
        b'a'...b'z' => input - b'a' + 26,
        b'0'...b'9' => input - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        b'=' => 0, // TODO
        _ => panic!("Disallowed base64 symbol: {}", input),
    }
}
