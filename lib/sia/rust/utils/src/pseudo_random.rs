pub trait Generate {
    fn seed(&mut self, seed_number: usize);
    fn generate(&mut self, from: f64, to: f64) -> f64;
    fn generate_unit(&mut self) -> f64;
}

pub struct PseudoRandom {
    pub seed: usize,
}

impl Generate for PseudoRandom {
    fn seed(&mut self, seed_number: usize) {
        self.seed = seed_number;
    }

    fn generate(&mut self, from: f64, to: f64) -> f64 {
        (self.generate_unit() * (to - from)) + from
    }

    fn generate_unit(&mut self) -> f64 {
        self.seed += 1;
        let a = self.seed as f64 * 15485863.;
        (a * a * a % 2038074743.) as f64 / 2038074743.
    }

}

#[cfg(test)]
mod tests {
    use crate::assert_approximately_equal;
    use crate::pseudo_random::{Generate, PseudoRandom};

    #[test]
    fn calculate_zernike_matrix_test() {
        let mut random = PseudoRandom { seed: 0 };

        let numbers: Vec<f64> = (1..100).map(|_| random.generate_unit()).collect();
        println!("random unit numbers: {:?}", numbers);
        numbers.iter().for_each(|n| assert!(0.<= *n && *n <= 1.));


        let numbers: Vec<f64> = (1..100).map(|_| random.generate(-2.,5.)).collect();
        println!("random numbers in (-2, 5): {:?}", numbers);
        numbers.iter().for_each(|n| assert!(-2.<= *n && *n <= 5.));

    }
}
