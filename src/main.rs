pub mod raws;
#[macro_use]
extern crate lazy_static;

/* ...
gs.ecs.register::<Door>();
gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

raws::load_raws();

gs.ecs.insert(Map::new(1, 64, 64));
... */
fn main() {
    println!("Hello, world!");
}
