use std::fmt::Debug;

pub trait CanSpeak: Debug {
    fn speak(&self) -> String;
}

pub trait CanAge: Clone {
    fn age_one_year(&mut self);
    fn age(&self) -> u32;
    fn rebirth(self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Dog {
    pub name: String,
    pub age: u32,
    pub puppies: Vec<Dog>,
}

impl Dog {
    pub fn new(name: String) -> Dog {
        Dog {
            name,
            age: 0,
            puppies: vec![]
        }
    }

    pub fn add_puppy(&mut self, pup: Dog) {
        self.puppies.push(pup);
    }
}

impl CanSpeak for Dog {
    fn speak(&self) -> String { format!("({}) says 'Woof!'", self.name) }
}

impl CanAge for Dog {
    fn age_one_year(&mut self) {
        self.age += 1;
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn rebirth(self) -> Self {
        Dog {
            name: self.name,
            age: 0,
            puppies: self.puppies
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cat {
    pub name: String,
    pub age: u32,
    pub kittens: Vec<Cat>,
}

impl Cat {
    pub fn new(name: String) -> Cat {
        Cat {
            name,
            age: 0,
            kittens: vec![]
        }
    }

    pub fn add_kitten(&mut self, kitty: Cat) {
        self.kittens.push(kitty);
    }
}

impl CanSpeak for Cat {
    fn speak(&self) -> String { format!("({}) says 'Meow!'", self.name) }
}

impl CanAge for Cat {
    fn age_one_year(&mut self) {
        self.age += 1;
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn rebirth(self) -> Self {
        Cat {
            name: self.name,
            age: 0,
            kittens: self.kittens
        }
    }
}

#[derive(Clone)]
pub struct Rose {
    pub num_petals: u32
}

impl Rose {
    pub fn new() -> Rose {
        Rose {
            num_petals: 50
        }
    }
}

impl CanAge for Rose {
    fn age_one_year(&mut self) {
        self.num_petals -= 10;
    }

    fn age(&self) -> u32 {
        (50 - self.num_petals) / 10 as u32
    }
}

pub fn interview(subject: &(impl CanSpeak + CanAge)) {
    println!("Tell me your name -> {}", subject.speak());
    println!("How old are you? -> {} years old", subject.age());
    println!("How do you feel? -> {}", subject.speak());
}

pub fn interview2<T>(subject: &T) where T: CanSpeak + CanAge {
    println!("Again, Tell me your name -> {}", subject.speak());
    println!("Again, How old are you? -> {} years old", subject.age());
    println!("Again, How do you feel? -> {}", subject.speak());
}

// Same as above:
//
//pub fn interview2<T: CanSpeak + CanAge>(subject: &T) {
//    println!("Again, Tell me your name -> {}", subject.speak());
//    println!("Again, How old are you? -> {} years old", subject.age());
//    println!("Again, How do you feel? -> {}", subject.speak());
//}

pub fn time_slip(target: &mut impl CanAge, years: u32) {
    for _ in 0..years {
        target.age_one_year();
    }
}

pub struct Kennel<T> where T: CanSpeak {
    objects: Vec<T>
}

impl<T> Kennel<T> where T: CanSpeak {
    pub fn new() -> Kennel<T> {
        Kennel {
            objects: vec![]
        }
    }

    pub fn add(&mut self, member: T) {
        self.objects.push(member);
    }

    pub fn all_speak(&self) -> Vec<String> {
        self.objects.iter().map(|m| m.speak()).collect()
    }

}

#[derive(Clone, Debug)]
pub enum Animal {
    Cat(Cat),
    Dog(Dog)
}

impl CanSpeak for Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Cat(c) => c.speak(),
            Animal::Dog(d) => d.speak(),
        }
    }
}

impl Into<Animal> for Dog {
    fn into(self) -> Animal {
        Animal::Dog(self)
    }
}

impl Into<Animal> for Cat {
    fn into(self) -> Animal {
        Animal::Cat(self)
    }
}

fn main() {
    let mut fido = Dog::new("Fido".into());
    println!("Speak Fido! -> {}", fido.speak());

    let fluffy = Cat::new("Fluffy".into());
    println!("Speak Fluffy! -> {}", fluffy.speak());

    interview(&fido);
    interview2(&fluffy);

    println!("Fido is now {} year(s) old", fido.age);
    time_slip(&mut fido, 10);
    println!("Fido is now {} year(s) old", fido.age);

    let mut rose = Rose::new();
    // This won't compile because Rose doesn't implement 'CanSpeak'
    // interview(&rose);

    println!("Rose is now {} year(s) old", rose.age());
    time_slip(&mut rose, 2);
    println!("Rose is now {} year(s) old", rose.age());

    // This won't compile because Kennel is already instanced as a Kennel<Dog> from the first line.
    // let mut kennel = Kennel::new();
    // kennel.add(fido);
    // kennel.add(fluffy);

    let mut kennel = Kennel::<Animal>::new();
    kennel.add(fido.into());
    kennel.add(fluffy.into());

    println!("Everyone, say Hi! -> {}", kennel.all_speak().join(", "));
}
