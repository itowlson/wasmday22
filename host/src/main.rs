use std::io::Write;

mod console;
mod services;

fn main() {
    let wasm_path_str = std::env::args().nth(1).expect("must provide Wasm module path");
    let wasm_path = std::path::PathBuf::from(wasm_path_str);

    let random_thing_external_path = if std::env::args().len() >= 3 {
        let rtp_path_str = std::env::args().nth(2).unwrap();
        Some(std::path::PathBuf::from(rtp_path_str))
    } else {
        None
    };

    let handler = console::ConsoleHandler::load(&wasm_path, &random_thing_external_path).unwrap();
    loop {
        print!("Enter a command: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let response = handler.handle_input(input.trim());
        println!("{}", response.unwrap());
    }
}
