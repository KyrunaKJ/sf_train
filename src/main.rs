#![allow(unused_variables)]

extern crate serde;
extern crate serde_json;
extern crate clearscreen;
use std::fs::File;
use std::io::Read;
use rustyline::DefaultEditor;
use std::collections::HashMap;
use serde::Deserialize;

trait Game {
    fn new() -> Self where Self:Sized;
    fn play(&mut self) -> Result<(), std::io::Error>;
    fn cleanup(&mut self);
}

trait Commands {
    fn quit(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn move_command(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn look_around(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn interact(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn inventory(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn help(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn save(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn load(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn status(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
    fn map(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error>;
}

trait InputReader {
    fn read_player_input(&mut self);
}

#[derive(Deserialize)]
struct Location {
    name: String,
    description: String,
}

struct InputManager<G: Game> {
    input_map: HashMap<String, fn(&mut G, args: Option<Vec<String>>) -> Result<(), std::io::Error>>,
    input_args: HashMap<String, Vec<String>>,
    last_command: String,
}

struct LocationManager {
    current_position: (i32, i32),
    all_global_locations: HashMap<(i32, i32), Location>,
}

impl<G: Game> InputManager<G>{
    fn new() -> Self {
        let input_map = HashMap::new();
        let input_args = HashMap::new();
        
        InputManager { input_map, input_args, last_command: String::new() }
    }

    fn insert_command_and_abbreviations(
        &mut self, 
        command: &str,
        function: fn(&mut G, args: Option<Vec<String>>) -> Result<(), std::io::Error>,
        args: Vec<&str>,
        abbreviations: Vec<&str>
    ) {
        self.input_map.insert(command.to_string(), function);
        self.input_args.insert(command.to_string(), args.iter().map(|&s| s.to_string()).collect());
        for abbreviation in abbreviations {
            self.input_map.insert(abbreviation.to_string(), function);
            self.input_args.insert(abbreviation.to_string(), args.iter().map(|&s| s.to_string()).collect());            
        }
    }
}

impl LocationManager {
    fn new() -> Self {
        let all_locations = Self::load_locations_from_json().unwrap();
        let current_position = (-7, -7);
        
        LocationManager { 
            current_position,
            all_global_locations: all_locations,
        }
    }

    fn load_locations_from_json() -> Result<HashMap<(i32, i32), Location>, serde_json::Error> {
        let mut file = File::open("src/locations/locations.json").unwrap();
        let mut data = String::new();  // Replace with your JSON data.
        let _ = file.read_to_string(&mut data);
    
        let json_data: HashMap<String, Location> = serde_json::from_str(&data)?;
        let mut locations = HashMap::new();
    
        for (key, value) in json_data {
            let tuple_key: Vec<i32> = key
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            locations.insert((tuple_key[0], tuple_key[1]), value);
        }
    
        Ok(locations)
    }

    fn get_current_location_name(&self) -> String {
        self.all_global_locations.get(&self.current_position).unwrap().name.to_string()
    }

    fn get_current_location_description(&self) -> String {
        self.all_global_locations.get(&self.current_position).unwrap().description.to_string()
    }
}

struct TextAdventure {
    input_manager: InputManager<TextAdventure>,
    location_manager: LocationManager,
    play_loop: bool,
}

impl Commands for TextAdventure {
    fn quit(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        self.play_loop = false;
        
        Ok(())
    }

    fn move_command(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn look_around(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        let args = args.unwrap();
        if args.len() == 1 {
            println!("You look around");
            println!("You see you're at the '{}'", self.location_manager.get_current_location_name());
            return Ok(())
        }
        
        let arg = self.expand_abbreviation(args[1].as_str()).unwrap();
        println!("You look {}", arg);
        let pos = &self.location_manager.current_position;

        let dst_position = match args[1].as_str() {
            "north" | "n" => (pos.0, pos.1 + 1),
            "east" | "e" => (pos.0 + 1, pos.1),
            "west" | "w" => (pos.0 - 1, pos.1),
            "south" | "s" => (pos.0, pos.1 - 1),
            "northeast" | "ne" => (pos.0 + 1, pos.1 + 1),
            "northwest" | "nw" => (pos.0 - 1, pos.1 + 1),
            "southeast" | "se" => (pos.0 + 1, pos.1 - 1),
            "southwest" | "sw" => (pos.0 - 1, pos.1 - 1),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid direction")),
        };

        let dst_location = match self.location_manager.all_global_locations.get(&dst_position) {
            Some(location) => location,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid direction")),
        };

        println!("You see the '{}'", dst_location.name);

        Ok(())
    }

    fn interact(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn inventory(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn help(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn save(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn load(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn status(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }

    fn map(&mut self, args: Option<Vec<String>>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}

impl InputReader for TextAdventure {
    fn read_player_input(&mut self) {
        let mut rl = DefaultEditor::new().unwrap();
        let mut input = rl.readline(">> ").unwrap();        
        
        clearscreen::clear().unwrap();        
        input = input.trim().to_lowercase().to_string();
        let input_parts: Vec<String> = input.split_whitespace().into_iter().map(|s| s.to_string()).collect();

        let _ = match self.input_manager.input_map.get(&input_parts[0]) {
           Some(function) => {
               self.input_manager.last_command = input_parts[0].clone();
               function(self, Some(input_parts))
           },
           None => {
               println!("Not a command");
               Ok(())
           },
        };        
    }
}

impl Game for TextAdventure {
    fn new() -> Self {
        TextAdventure {
            input_manager: InputManager::new(),
            location_manager: LocationManager::new(),
            play_loop: true,
        }
    }

    fn play(&mut self) -> Result<(), std::io::Error> {
        self.load_default_commands();
        
        println!("You are in the {} - {}", self.location_manager.get_current_location_name(), self.location_manager.get_current_location_description());

        while self.play_loop {
            self.read_player_input();
        }
        println!("loop quit");

        Ok(())
    }

    fn cleanup(&mut self) {
        
    }
}

impl TextAdventure {
    fn load_default_commands(&mut self) {
        let directions = vec![
            "north", "n",
            "northeast", "ne",
            "east", "e",
            "southeast", "se",
            "south", "s",
            "southwest", "sw",
            "west", "w",
            "northwest", "nw",
        ];
        let mut insert = |
            command: &str, 
            function: fn(&mut TextAdventure, Option<Vec<String>>) -> Result<(), std::io::Error>, 
            args:Vec<&str>,
            abbreviations: Vec<&str>,
        | {
            self.input_manager.insert_command_and_abbreviations(command, function, args, abbreviations)
        };
        
        insert("move", Commands::move_command, directions.clone(), vec!["m"]);
        insert("look", Commands::look_around, directions.clone(), vec!["l"]);
        insert("interact", Commands::interact, vec![""], vec!["i"]);
        insert("inventory", Commands::inventory, vec![""], vec!["inv", "I"]);
        insert("map", Commands::map, vec![""], vec!["m"]);    
        insert("status", Commands::status, vec![""], vec!["stats"]);
        insert("save", Commands::save, vec![""], vec!["s", "sv"]);
        insert("load", Commands::load, vec![""], vec!["ld"]);
        insert("help", Commands::help, vec![""], vec!["h", "?"]);
        insert("quit", Commands::quit, vec![""], vec!["q", "exit", "e"]);
    }

    fn expand_abbreviation<'a>(&self, abbreviation: &'a str) -> Result<&'a str, std::io::Error> {
        match abbreviation {
            "n" => Ok("north"),
            "s" => Ok("south"),
            "e" => Ok("east"),
            "w" => Ok("west"),
            "ne" => Ok("northeast"),
            "nw" => Ok("northwest"),
            "se" => Ok("southeast"),
            "sw" => Ok("southwest"),
            _ => Ok(abbreviation), // Default case: return the abbreviation itself
        }
    }
}

struct MyApplication {
    game: Box<dyn Game>,
}

impl MyApplication {
    fn new() -> Self {
        MyApplication { 
            game: Box::new(TextAdventure::new()),
        }
    }

    fn run(&mut self) -> Result<(), std::io::Error> {
        self.game.play()?;
        self.game.cleanup();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = MyApplication::new();
    app.run()?;
    
    Ok(())
}

