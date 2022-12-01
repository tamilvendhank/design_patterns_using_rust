pub trait FlyBehaviour {
    fn fly(&self);
}

pub trait QuackBehaviour {
    fn quack(&self);
}

pub struct Duck {
    pub fly_behaviour: Box<dyn FlyBehaviour>,
    pub quack_behaviour: Box<dyn QuackBehaviour>,
}

pub struct FlyWithWings;
pub struct FlyNoWay;
pub struct Quack;
pub struct MuteQuack;
pub struct Squeak;

impl Duck {
    pub fn perform_fly(&self) {
        self.fly_behaviour.fly();
    }

    pub fn perform_quack(&self) {
        self.quack_behaviour.quack();
    }

    pub fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying!!");
    }
}

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly");
    }
}

impl QuackBehaviour for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

impl QuackBehaviour for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}

impl QuackBehaviour for Squeak {
    fn quack(&self) {
        println!("Squeak");
    }
}

pub fn create_mallard_duck() -> Duck {
    Duck {
        fly_behaviour: Box::new(FlyWithWings {}),
        quack_behaviour: Box::new(Quack {}),
    }
}
