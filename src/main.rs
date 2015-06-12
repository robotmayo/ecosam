pub struct Tile {
    food: i32,
    weather: i32
}

pub struct World {
    tiles: Vec<Tile>,
    height: usize,
    width: usize
}

impl World {
    fn new(h: usize, w: usize) -> World {
        World {
            tiles: (0..(h * w)).map(|_| {
                Tile {
                    food: 20,
                    weather: 1
                }
            }).collect::<Vec<_>>(),
            height: h,
            width: w
        }
    }
    fn getTile(&self, x: usize, y: usize) -> &Tile {
        return &self.tiles[x * (self.height - 1) + y];
    }
}

fn main() {
    let world = World::new(5, 5);
    println!("{}", world.getTile(1, 2).food);
}