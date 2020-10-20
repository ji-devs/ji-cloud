//Probably going to move to shared or core
//
#[derive(Clone, Debug)]
pub struct Jig {
    pub title: String,
    pub cover: Option<Id>,
    pub ending: Option<Id>,
    pub modules: Vec<Option<Module>>
}


#[derive(Clone, Debug)]
pub struct Module {
    pub id: Id,
    pub kind: ModuleKind, 
}

impl Jig {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            cover: None,
            ending: None,
            modules: vec![None],
        }
    }
    /*
    pub fn mock() -> Self {
        Self {
            title: "Hello World".to_string(),
            cover: Some("1".to_string()),
            ending: Some("2".to_string()),
            modules: vec!["3", "4", "5", "6"]
                .into_iter()
                .map(|x| Some(x.to_string()))
                .collect()
        }
    }
    */
}

pub type Id = String;

#[derive(Clone, Debug)]
pub enum ModuleKind{
    Poster,
    MemoryGame,
}

impl ModuleKind{
    pub fn from_str(s:&str) -> Option<Self> {
        match s {
            "poster" => Some(Self::Poster),
            "memory_game" => Some(Self::MemoryGame),
            _ => None
        }
    }
}
