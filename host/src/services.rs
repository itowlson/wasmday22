wit_bindgen_wasmtime::export!({paths: ["../wit/random-thing.wit"]});

pub struct RandomThing;

impl random_thing::RandomThing for RandomThing {
    fn get_random_thing(&mut self, req: random_thing::Request) -> Result<String, random_thing::Error> {
        let endpoint = match req {
            random_thing::Request::Joke => "https://some-random-api.ml/joke",
            random_thing::Request::AnimalFact(random_thing::AnimalType::Cat) => "https://some-random-api.ml/facts/cat",
            random_thing::Request::AnimalFact(random_thing::AnimalType::Dog) => "https://some-random-api.ml/facts/dog",
        };
        let response = reqwest::blocking::get(endpoint)
            .map_err(|e| match e.status() {
                Some(st) => random_thing::Error::Service(st.as_u16()),
                None => random_thing::Error::Network("network error".to_owned()),
            })?
            .text()
            .map_err(|_| random_thing::Error::Response)?;
        Ok(response)
    }
}
