use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Pokemon {
    name: String,
    url: String,
}

fn main() {
    let res: serde_json::Value =
        reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon?limit=151&offset=0")
            .unwrap()
            .json()
            .unwrap();

    let json = json!(res["results"]);

    let pokemon: Vec<Pokemon> = serde_json::from_value(json).unwrap();

    println!("Response: {:?}", pokemon);
}
