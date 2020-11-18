pub type Id = String;
use shared::domain::jig::ModuleKind;
use utils::settings::SETTINGS;

pub trait ModuleKindExt {
    fn get_thumbnail(self) -> String;
}

impl ModuleKindExt for Option<ModuleKind> {
    fn get_thumbnail(self) -> String {

        let media_url = unsafe {
            SETTINGS.get_unchecked().remote_target.media_ui_url()
        };
        let icon_path = {
            match self {
                None => "JIG_Gear@2x.png",
                Some(kind) => {
                    match kind {
                        ModuleKind::Poster => "icn-module-poster2.png",
                        ModuleKind::MemoryGame => "module-memory-game.svg",
                        _ => {
                            panic!("don't have the icon for that module kind!");
                        }
                    }
                }
            }
        };
        format!("{}/{}", media_url, icon_path)
    }
}

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
            modules: vec!["3", "4", "5", "6", "7", "8", "9", "10"]
                .into_iter()
                .map(|x| Module {
                    id: x.to_string(),
                    kind: {
                        if x == "4" {
                            Some(ModuleKind::MemoryGame)
                            //None
                        } else {
                            None
                        }
                    }
                })
                .collect()
        }
    }
}
