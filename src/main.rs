#[allow(unused_must_use)]
pub mod config;
mod render;
mod resources {
    pub mod cpu;
    pub mod art;
    pub mod ram;
    pub mod gpu;
    pub mod xorgenv;
}

#[allow(unused_imports)]
use resources::art;
    #[allow(unused_must_use)] // I just didn't want to see the warns lol.
fn main()  {
    art::display_art();
    render::render();
}
