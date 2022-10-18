use std::io::Write;

mod console;

fn main() {
    let wasm_path_str = std::env::args().nth(1).expect("must provide Wasm module path");
    let wasm_path = std::path::PathBuf::from(wasm_path_str);
    let handler = console::ConsoleHandler::load(&wasm_path).unwrap();
    loop {
        print!("Enter a command: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let response = handler.handle_input(input.trim());
        println!("{}", response);
    }
}
