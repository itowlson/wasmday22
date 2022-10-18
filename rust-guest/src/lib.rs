wit_bindgen_rust::export!("../wit/console.wit");

struct Console;

impl console::Console for Console {
    fn handle_console_input(input: String) -> String {
        format!("{} {}", input, input)
    }
    // fn exec(source: String) -> Result<String, String> {
    //     let mut builder = String::with_capacity(source.len() * 2);
    //     for c in source.chars() {
    //         builder.push(c);
    //         builder.push(CLAP);
    //     }
    //     let result = builder.trim_end_matches(CLAP);
    //     Ok(result.to_owned())
    // }
}
