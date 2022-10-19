wit_bindgen_rust::export!("../wit/console.wit");
wit_bindgen_rust::import!("../wit/random-thing.wit");

struct Console;

impl console::Console for Console {
    fn handle_console_input(input: String) -> String {
        let req = random_thing::Request::Joke;
        let t = random_thing::get_random_thing(req).unwrap_or("Oh no!".to_owned());
        
        format!("You said {}, I say {}", input, t)
    }
}
