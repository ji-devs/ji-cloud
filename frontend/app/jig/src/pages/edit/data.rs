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

pub(super) mod module_service {
    use super::*;

    pub async fn add() -> Module {
        log::warn!("TODO - add new module!"); 
        Module {
            id: "blah_blah".to_string(),
            kind: None, 
        }
    }
    pub async fn delete(id:Id) {
        log::warn!("TODO - delete module!"); 
    }
    pub async fn reorder(from:usize, to:usize) {
        log::warn!("TODO - reorder module!"); 
    }
    pub async fn change_kind(id:Id, kind:ModuleKind) {
        log::warn!("TODO - change module kind!"); 
    }
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
    pub async fn mock(id:Id) -> Self {
        //TODO - really load it from backend
        Self {
            id,
            title: "hello world".to_string(),
            cover: None,
            ending: None,
            modules: vec!["3", "4", "5", "6"]
                .into_iter()
                .map(|x| Module {
                    id: x.to_string(),
                    kind: {
                        if x == "4" {
                            Some(ModuleKind::MemoryGame)
                        } else {
                            None
                        }
                    }
                })
                .collect()
        }
    }
}

pub type Id = String;

#[derive(Clone, Copy, Debug)]
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
