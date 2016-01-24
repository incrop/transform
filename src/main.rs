use std::io::{self, Read, Write};

enum Mode {
    Encode,
    Decode,
}

fn main() {
    parse_arguments()
        .and_then(run)
        .unwrap_or_else(|msg| println!("{}", msg));
}

fn run(mode: Mode) -> Result<(), String> {
    pipe(
        &mut io::stdin(),
        &mut io::stdout(),
        |input, finish| {
            match mode {
                Mode::Encode => encode(input, finish),
                Mode::Decode => decode(input, finish),
            }
        },
    ).map_err(|e| format!("{}", e).to_string())
}

fn read_arguments() -> Result<Vec<String>, String> {
    let os_args = std::env::args_os().skip(1);
    let mut result = Vec::new();
    for os_arg in os_args {
        let arg = try!(os_arg.into_string().map_err(|os_str| {
            "Broken parameter encoding: ".to_string() + &os_str.to_string_lossy().into_owned()
        }));
        result.push(arg);
    }
    Ok(result)
}

fn parse_arguments() -> Result<Mode, String> {
    let args = try!(read_arguments());
    let mut iter = args.iter();

    let mut mode_opt = None;
    loop {
        match iter.next() {
            Some(arg) => match arg.as_ref() {
                "--encode" | "-e" => mode_opt = Some(Mode::Encode),
                "--decode" | "-d" => mode_opt = Some(Mode::Decode),
                _ => return Err("Unknown parameter: ".to_string() + &arg),
            },
            None => return mode_opt.ok_or("Require --encode or --decode parameter".to_string()),
            // TODO: handle duplicate parameters
        }
    }
}

const BUF_SIZE: usize = 8192;

fn pipe<R: Read, W: Write, F>(from: &mut R, to: &mut W, transform: F) -> io::Result<()>
where F: Fn(&[u8], bool) -> (Vec<u8>, usize) {
    let mut buf_in = [0; BUF_SIZE];
    let mut extra_left = 0;
    
    loop {
        let read = try!(from.read(&mut buf_in[extra_left..]));
        let size = extra_left + read;

        let (buf_out, extra_right) = transform(&buf_in[..size], read == 0);
        try!(to.write_all(&buf_out));
        if read == 0 {
            assert_eq!(extra_right, 0);
            break;
        }

        for i in 0..extra_right {
            buf_in[i] = buf_in[size - extra_right + i];
        }
        extra_left = extra_right;
    }
    Ok(())
}

const BYTE_EQL: u8 = 61; // = TODO: constants for ascii?

fn decode(input: &[u8], finish: bool) -> (Vec<u8>, usize) {
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

fn decode_chunk(input: &mut [u8; 4], output: &mut [u8; 3]) -> usize {
    assert!(input[0] != BYTE_EQL);
    assert!(input[1] != BYTE_EQL);
    assert!(input[2] != BYTE_EQL || input[3] == BYTE_EQL);
    let len = if input[3] != BYTE_EQL { 3 }
        else if input[2] != BYTE_EQL { 2 }
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
        65...90  => input - 65, // A-Z
        97...122 => input - 71, // a-z
        48...57  => input + 4,  // 0-9
        43 => 62, // +
        47 => 63, // /
        61 => 0,  // =
        _ => panic!("Disallowed base64 symbol: {}", input),
    }
}

fn encode(input_all: &[u8], finish: bool) -> (Vec<u8>, usize) {
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

fn encode_chunk(input: &[u8; 3], output: &mut [u8; 4], input_size: usize) {
    output[0] = input[0] >> 2;
    output[1] = 0b00110000 & input[0] << 4 | input[1] >> 4;
    output[2] = 0b00111100 & input[1] << 2 | input[2] >> 6;
    output[3] = 0b00111111 & input[2];
    for i in 0..output.len() {
        output[i] = encode_byte(output[i]);
    }
    if input_size < 3 {
        output[3] = BYTE_EQL;
    } else if input_size < 2 {
        output[2] = BYTE_EQL;
    }
}

fn encode_byte(input: u8) -> u8 {
    match input {
        0...25  => input + 65, // A-Z
        26...51 => input + 71, // a-z
        52...61 => input - 4,  // 0-9
        62 => 43, // +
        63 => 47, // /
        _ => panic!("unreachable"),
    }
}