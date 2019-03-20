mod sound;

mod plant;



fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

    crate::sound::abstract_sound();
    sound::abstract_sound();

    sound::instrument::brass::trumpet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it as v.id is private:
    // println!("The ID is {}", v.id);

}

fn a_super_fn(){
    // code
}
