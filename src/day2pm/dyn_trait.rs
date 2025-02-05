pub struct Dog {
    pub name: String,
    pub age: i8,
}

pub struct Cat {
    pub lives: i8,
}

pub trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

pub fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

pub fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}