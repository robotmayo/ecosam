pub struct Tile {
    food: i32,
    weather: i32
}

fn main() {
    let world = gen_world(5);
    for y in world{
        for x in y{
            print!("{} ", x.food);
        }
        println!("");
    }
}

fn gen_world(size: i32) -> Vec<Vec<Tile>> {
    let mut outer_vec = vec![];
    for i in 1..size{
        let mut inner = vec![];
        for j in 1..size{
            inner.push(Tile {food : 20, weather : 1});
        }
        outer_vec.push(inner)
    }
    return outer_vec
}