use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct Pokemon {
    name: String,
    url: String,
    sprite: Option<String>,
}

fn main() {
    // todo!("try to read json file if it does not exist call api then store");
    if std::path::Path::new("pokemon.json").exists() {
        println!("File exists");
    } else {
        let res: serde_json::Value =
            reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon?limit=151&offset=0")
                .unwrap()
                .json()
                .unwrap();

        let json = json!(res["results"]);

        let mut pokemon: Vec<Pokemon> = serde_json::from_value(json).unwrap();

        pokemon.iter_mut().enumerate().for_each(|(i, poke)| {
            println!("getting sprite url for #{}-{}", i + 1, poke.name);

            let sprite: serde_json::Value =
                reqwest::blocking::get(&poke.url).unwrap().json().unwrap();

            let sprite_url = sprite["sprites"]["front_default"].as_str();
            poke.sprite = Some(sprite_url.unwrap().to_string());

            // println!("{}", sprite_url.unwrap());

            let url = Url::parse(sprite_url.unwrap()).unwrap();

            let res = reqwest::blocking::get(url);
            let bytes = res.unwrap().bytes().unwrap();
            // println!("{:?}", bytes);

            println!("saving sprite for #{}-{}", i + 1, poke.name);

            let image_path = format!("images/image{}.png", i + 1);
            let mut file = File::create(image_path).unwrap();
            file.write_all(&bytes).unwrap();
        });

        // write data to json file
        let pokemon_string = serde_json::to_string(&pokemon).unwrap();
        let path = "pokemon.json";
        let mut output = File::create(path).unwrap();
        write!(output, "{pokemon_string}").unwrap();
    }
}
