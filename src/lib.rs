pub mod anim_id;
pub mod animate;
mod animation;
mod ase;
pub mod aseloader;
mod processing;
mod sprite;
mod tileset;
pub mod timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub use ase::AseId;
pub use tileset::{TileSize, Tileset, TilesetId, TilesetKey};
