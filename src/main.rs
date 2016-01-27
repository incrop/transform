extern crate transform;

use std::io;
use transform::{Mode, Method, pipe};
use transform::plugins::base64;

fn main() {
    parse_arguments()
        .and_then(|(method, mode)| run(method, mode))
        .unwrap_or_else(|msg| println!("{}", msg));
}

pub fn run(method: Method, mode: Mode) -> Result<(), String> {
    pipe(
        &mut io::stdin(),
        &mut io::stdout(),
        |input, finish| {
            match method {
                Method::Base64 => match mode {
                    Mode::Encode => base64::encode(input, finish),
                    Mode::Decode => base64::decode(input, finish),
                },
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

fn parse_arguments() -> Result<(Method, Mode), String> {
    let args = try!(read_arguments());

    let mut mode = None;
    let mut method = None;

    for arg in args.iter() {
        match arg.as_ref() {
            "--base64" => match method {
                Some((_, m)) => return Err("Conflicting parameters: '".to_string() + m + "', '" + arg + "'"),
                None => method = Some((Method::Base64, arg)),
            },
            "--encode" | "-e" => match mode {
                Some((_, e)) => return Err("Conflicting parameters: '".to_string() + e + "', '" + arg + "'"),
                None => mode = Some((Mode::Encode, arg)),
            },
            "--decode" | "-d" => match mode {
                Some((_, d)) => return Err("Conflicting parameters: '".to_string() + d + "', '" + arg + "'"),
                None => mode = Some((Mode::Decode, arg)),
            },
            _ => return Err("Unknown parameter: ".to_string() + &arg),
        }
    }

    let mode_res = mode.ok_or("Require --encode or --decode parameter".to_string());
    let method_res = method.ok_or("Require --base64 parameter".to_string());

    mode_res.and_then(|(mode, _)| {
        method_res.map(|(method, _)| {
            (method, mode)
        })
    })
}
