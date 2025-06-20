#[derive(Debug, Clone)]
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

    pub fn display(&self) {
        println!("Place: {}\nDescription: {}\nCoordinates: ({}, {})", 
                 self.name, self.description, self.coordinates.0, self.coordinates.1);
    }
}