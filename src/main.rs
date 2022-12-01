mod strategy_pattern;

use strategy_pattern::*;

fn main() {
    let mallard_duck = strategy_pattern::create_mallard_duck();
    mallard_duck.fly_behaviour.fly();
    mallard_duck.quack_behaviour.quack();
}
