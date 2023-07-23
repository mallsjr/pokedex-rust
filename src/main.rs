use std::fs::{self, File};
use std::io::Write;

use egui::emath::RectTransform;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use egui::{Color32, FontId, Pos2, Rect, RichText, Rounding, Stroke, Vec2};

#[derive(Debug, Deserialize, Serialize)]
struct Pokemon {
    name: String,
    url: String,
    sprite: Option<String>,
}

fn main() -> eframe::Result<()> {
    // read json file into memory
    let mut pokemon: Vec<Pokemon>;
    if std::path::Path::new("pokemon.json").exists() {
        println!("File exists");

        let contents =
            fs::read_to_string("pokemon.json").expect("Should have been able to read the file");

        pokemon = serde_json::from_str(&contents).unwrap();

        // println!("{:?}", pokemon);
    } else {
        let res: serde_json::Value =
            reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon?limit=151&offset=0")
                .unwrap()
                .json()
                .unwrap();

        let json = json!(res["results"]);

        pokemon = serde_json::from_value(json).unwrap();

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

    // Create egui screen and pass in pokemon

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1024.0, 700.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Pokedex",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, pokemon))),
    )
}

struct App {
    pokemon: Vec<Pokemon>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>, pokemon: Vec<Pokemon>) -> Self {
        Self { pokemon }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(RichText::new("Pokedex").strong().size(50.0));
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for i in 0..self.pokemon.len() {
                        let (_, rect) = ui.allocate_space(egui::vec2(150.0, 250.0));
                        ui.painter().rect(
                            rect,
                            Rounding::from(5.0),
                            Color32::TRANSPARENT,
                            Stroke::new(1.0, Color32::WHITE),
                        );

                        // Get the relative position of our "canvas"
                        let to_screen = RectTransform::from_to(
                            Rect::from_min_size(Pos2::ZERO, rect.size()),
                            rect,
                        );

                        // Create an absolute point
                        let point = Pos2 { x: 75.0, y: 230.0 };
                        // Make the absolute point relative to the "canvas" container
                        let point_in_screen = to_screen.transform_pos(point);

                        ui.painter_at(rect).text(
                            point_in_screen,
                            egui::Align2::CENTER_CENTER,
                            &self.pokemon[i].name,
                            FontId::monospace(20.0),
                            Color32::WHITE,
                        );
                    }
                });
            });
        });
    }
}
