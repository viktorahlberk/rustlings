fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let name = cat.0;
    // let age=cat.1;
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
