/// Based on Linear Congruential Generators, generates seeded, deterministic random numbers between 0 and 9999
#[derive(Debug)]
pub struct Random {
    modulus: u64,
    multiplier: u64,
    increment: u64,
    seed: u64,
}

impl Random {
    pub fn new(seed: u64) -> Random {
        Random {
            seed,
            modulus: 10000,
            multiplier: 378,
            increment: 2310,
        }
    }

    pub fn peek(&self) -> u64 {
        ((self.seed * self.multiplier) + self.increment) % self.modulus
    }

    pub fn next(&mut self) -> u64 {
        let next = self.peek();
        self.seed = next;
        next
    }

    pub fn peek_between(&self, from: u64, to: u64) -> u64 {
        let next = self.peek();
        let range = to - from;
        let offset = from;

        offset + ((range * next) / 10000)
    }

    pub fn next_between(&mut self, from: u64, to: u64) -> u64 {
        let next = self.peek_between(from, to);
        self.next();
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut random = Random::new(1030307);

        assert_eq!(random.peek(), random.next());
    }

    #[test]
    fn it_gives_a_value_between() {
        let mut random = Random::new(1030307);

        for _ in 0..20 {
            let result = random.next_between(15, 30);
            assert!(result >= 15);
            assert!(result <= 30);
        }
    }
}
