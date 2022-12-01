mod strategy_pattern;

use strategy_pattern::*;

fn main() {
    let mut mallard_duck = strategy_pattern::create_mallard_duck();
    mallard_duck.fly_behaviour.fly();
    mallard_duck.quack_behaviour.quack();

    mallard_duck.set_fly_behaviour(Box::new(FlyRocketPowered {}));
    mallard_duck.fly_behaviour.fly();
}
