pub mod character;
pub mod place;
pub mod scene;
pub mod story;
pub mod title;

pub use character::{Character, Characters};
pub use place::{Place, Places};
pub use scene::{Scene, Scenes};
pub use story::Story;
pub use title::Title;




#[derive(Debug, Clone)]
pub struct World {
    pub title: Title,
    pub story: Story,
    pub characters: Characters,
    pub places: Places,
    pub scenes: Scenes,

}

impl World {
    pub fn new(title: Title, story: Story, characters: Characters, places: Places, scenes: Scenes) -> World {
        World {
            title,
            story,
            characters,
            places,
            scenes,
        }
    }
}
    

#[derive(Debug, Clone)]
pub struct WorldBuilder {
    pub title: Title,
    pub story: Story,
    pub characters: Characters,
    pub places: Places,
    pub scenes: Scenes,
}


impl WorldBuilder {
    
    #[allow(dead_code)]

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    #[allow(unused_must_use)]
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            title: Title::new(String::from("")),
            story: Story::new(String::from("A tale of adventure and discovery")),
            characters: Characters::new(),
            places: Places::new(),
            scenes: Scenes::new(),
        }
    }

    pub fn build(&self) -> World {
        World {
            title: self.title.clone(),
            story: self.story.clone(),
            characters: self.characters.clone(),
            places: self.places.clone(),
            scenes: self.scenes.clone(),
        }
    }


    
    pub fn title(&mut self, title: Title) -> &mut WorldBuilder {
        
        self.title = title;
        self
    }
    
    pub fn story(&mut self, story: Story) -> &mut WorldBuilder {
        self.story = story;
        self
    }
    
    pub fn characters(&mut self, characters: Characters) -> &mut WorldBuilder {
        self.characters = characters;
        self
    }
    
    pub fn places(&mut self, places: Places) -> &mut WorldBuilder {
        self.places = places;
        self
    }
    
    pub fn scenes(&mut self, scenes: Scenes) -> &mut WorldBuilder {
        self.scenes = scenes;
        self
    }
    
    
}



