use rand::Rng;
use rand::rngs::ThreadRng;

pub struct RandomNumbers {
    pub generate: Vec<u32>
}

impl RandomNumbers {
    pub fn generate(size: u32) -> Vec<u32> {
        let mut rng: ThreadRng = rand::thread_rng();
        let numbers: Vec<u32> = (0..size).map(|_| rng.gen_range(0..=9)).collect();

        return numbers;
    }
}