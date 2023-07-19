use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::json;

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

        pokemon
            .iter_mut()
            .enumerate()
            .take(4)
            .for_each(|(i, poke)| {
                println!("getting sprite url for #{}-{}", i, poke.name);

                let sprite: serde_json::Value =
                    reqwest::blocking::get(&poke.url).unwrap().json().unwrap();

                let sprite_url = json!(sprite["sprites"]["front_default"]).to_string();

                poke.sprite = Some(sprite_url);
            });

        // todo!("write data to json file");

        // let pokemon_string = serde_json::to_string(&pokemon).unwrap();

        // let path = "pokemon.json";

        // let mut output = File::create(path).unwrap();
        // write!(output, "{pokemon_string}").unwrap();

        // todo!("Save images");
        pokemon.iter().enumerate().for_each(|(i, poke)| {
            println!("saving image {} for {}", i, poke.name);

            // let path = format!("image{}.png", i);

            match &poke.sprite {
                Some(u) => {
                    let url = format!("{}", u);

                    println!("{}", url);

                    // let mut file = std::fs::File::create(path).unwrap();
                    let res = reqwest::blocking::get(url);
                    let bytes = res.unwrap().bytes().unwrap();
                    println!("{:?}", bytes);
                }
                None => println!("There is no sprite"),
            };
        });
    }
}
