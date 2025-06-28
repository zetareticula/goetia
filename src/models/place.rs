use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashMap;

impl Hash for Place {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.description.hash(state);
        format!("{},{}", self.coordinates.0, self.coordinates.1).hash(state);
    }
}

impl Eq for Place {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Place {
    pub name: String,
    pub description: String,
    pub coordinates: (f64, f64), // latitude and longitude
}

impl Place {
    pub fn new(name: String, description: String, coordinates: (f64, f64)) -> Self {
        Place {
            name,
            description,
            coordinates,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (self.coordinates.0, self.coordinates.1)
    }

    pub fn from_string(text: &str, prefix: &str) -> Self {
        let text = text.trim();
        let mut lines = text.splitn(2, '\n');
        let name = lines.next().unwrap_or("Unknown Place").to_string();
        let description = lines.next().unwrap_or("No description").to_string();
        // Use default coordinates if not specified
        let coordinates = (0.0, 0.0);
        
        Self {
            name,
            description,
            coordinates,
        }
    }

    pub fn display(&self) -> String {
        format!("Place: {}\nDescription: {}\nCoordinates: ({}, {})", 
                self.name, self.description, self.coordinates.0, self.coordinates.1)
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.display())
    }
}

impl PartialEq for Place {
    fn eq(&self, other: &Place) -> bool {
        self.name == other.name &&
        self.description == other.description &&
        self.coordinates == other.coordinates
    }
}
    
#[derive(Debug, Clone)]
pub struct Places {
    pub places: Vec<Place>,
}

impl Places {
    pub fn new() -> Self {
        Places {
            places: Vec::new(),
        }
    }

    pub fn add_place(&mut self, place: Place) {
        if !self.places.iter().any(|p| p.name == place.name) {
            self.places.push(place);
        }
    }

    pub fn remove_place(&mut self, name: &str) {
        self.places.retain(|place| place.name != name);
    }

    pub fn get_place(&self, name: &str) -> Option<&Place> {
        self.places.iter().find(|place| place.name == name)
    }

    pub fn get_places(&self) -> &Vec<Place> {
        &self.places
    }
}