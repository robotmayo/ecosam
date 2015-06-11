fn main() {
    let world = gen_world(5);
    for y in world{
        for x in y{
            print!("{}", x);
        }
        println!("");
    }
}

fn gen_world(size: i32) -> Vec<Vec<i32>> {
    let mut outer_vec = vec![];
    for i in 1..size{
        let mut inner = vec![];
        for j in 1..size{
            inner.push(0);
        }
        outer_vec.push(inner)
    }
    return outer_vec
}