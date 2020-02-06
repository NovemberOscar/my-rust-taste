mod guees_game_pack {
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

fn main() {
    //    let g = guees_game_pack::Guess { value: 55 };
    //    println!("{}", g.value);

    let g = guees_game_pack::Guess::new(43);
    println!("{}", g.value());
}
