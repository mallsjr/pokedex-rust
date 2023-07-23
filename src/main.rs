use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use egui::emath::RectTransform;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use egui::{pos2, Color32, FontId, Pos2, Rect, RichText, Rounding, Stroke, TextureHandle, Vec2};

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
    images: Vec<TextureHandle>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, pokemon: Vec<Pokemon>) -> Self {
        let mut images: Vec<TextureHandle> = Vec::new();
        //TODO: Figure out how to load images on initial creation of app
        pokemon.iter().enumerate().for_each(|(i, _poke)| {
            let image_name = format!("image{}.png", i + 1);
            let path = Path::new("").join("images").join(&image_name);
            let image = load_image_from_path(&path).unwrap();

            //TODO: Figure out how to then convert into textures
            let texture: egui::TextureHandle =
                cc.egui_ctx
                    .load_texture(image_name, image, Default::default());

            images.push(texture);
        });

        //TODO: Put image textures into an vector to be used by update method
        Self { pokemon, images }
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
                    ui.spacing_mut().item_spacing = Vec2::new(25.0, 25.0);
                    for i in 0..self.pokemon.len() {
                        //The below of loading the email might be expensive which hurts scrolling
                        // let image_name = format!("image{}.png", i + 1);
                        // let path = Path::new("").join("images").join(&image_name);
                        // let image = load_image_from_path(&path).unwrap();

                        //This also could be very expensive which is why they say to do once
                        // let texture: egui::TextureHandle =
                        //     ui.ctx().load_texture(image_name, image, Default::default());

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

                        // Create an absolute point for name
                        let name_point = Pos2 { x: 75.0, y: 230.0 };

                        // Make the absolute point relative to the "canvas" container
                        let name_point_in_screen = to_screen.transform_pos(name_point);

                        ui.painter_at(rect).image(
                            self.images[i].id(),
                            rect,
                            Rect::from_two_pos(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );

                        ui.painter_at(rect).text(
                            name_point_in_screen,
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

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
