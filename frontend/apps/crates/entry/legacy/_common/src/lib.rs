use serde::{Serialize, Deserialize};
use serde_repr::*;

mod path;
pub use path::*;

mod design;
pub use design::*;

mod activities;
pub use activities::*;

mod module;
pub use module::*;

pub type Id = String;