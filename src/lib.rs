pub mod assets;
pub mod camera;
pub mod gameplay;
pub mod items;
pub mod levels;
pub mod mainmenu;
pub mod physics;
pub mod player;
pub mod startup;
pub mod state;
pub mod paused;
pub mod debug;

pub mod prelude {
    pub use crate::assets::*;
    pub use crate::camera::*;
    pub use crate::gameplay::*;
    pub use crate::items::inventory::*;
    pub use crate::items::*;
    pub use crate::levels::*;
    pub use crate::mainmenu::*;
    pub use crate::physics::component::*;
    pub use crate::physics::*;
    pub use crate::player::component::*;
    pub use crate::player::*;
    pub use crate::startup::*;
    pub use crate::state::*;
    pub use crate::paused::*;
    pub use crate::debug::*;
}
