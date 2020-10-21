//Probably going to move to shared or core
//
#[derive(Clone, Debug)]
pub struct Jig {
    pub id: Id,
    pub title: String,
    pub cover: Option<Id>,
    pub ending: Option<Id>,
    pub modules: Vec<Module>
}


#[derive(Clone, Debug)]
pub struct Module {
    pub id: Id,
    pub kind: Option<ModuleKind>, 
}

impl Module {
    //TODO - all these need backend API stuff
    pub async fn load_new() -> Module {
        Module {
            id: "blah_blah".to_string(),
            kind: None, 
        }
    }
    pub async fn change_kind(&mut self, kind:ModuleKind) {
        self.kind = Some(kind);
    }
}

//BACKEND STUFF TODO
pub async fn delete_module(id:Id) {

}
impl Jig {
    pub async fn load(id:Id) -> Self {
        //TODO - really load it from backend
        Self {
            id,
            title: "".to_string(),
            cover: None,
            ending: None,
            modules: vec![
                Module {
                    id: "foo".to_string(),
                    kind: None
                }
            ],
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
