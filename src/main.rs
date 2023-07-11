fn main() {
    println!("Hello, world!");

    let mut x = 0;
    let mut y = 1;
    loop{
        x = x+1;
        if x == 11{
            break;
        }
        print!("{x} ,");
    }
    loop{
        y = y+1;
        if y == 11{
            break;
        }
        print!("\n{y}");

    }
}
