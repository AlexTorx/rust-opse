use std::cmp::Ordering;

use std::f32::consts::PI;
use std::f32::EPSILON;


struct PRNG {
    coins: [u8; 32]
}

impl PRNG {
    fn numerify_coins (&self) -> u32 {
        let mut out: u32 = 0;
        for bit in self.coins.iter() {
            out = (out << 1) | *bit as u32;
        }
        out
    }
    fn draw (&self) -> f64 {
        (self.numerify_coins() as f64) / (2_u64.pow(32) - 1) as f64
    }
}

fn afc (index: &u32) -> f32 {
    // This function calculates logarithm of i factorial: ln(i!)
    // using Stirling's approximation
    //
    // The aim of this function is to have a much faster computation
    // compared to recursive factorial computation algorithm and to decrease
    // the use of memory for the whole computation.
    //
    // ln(n!) ~ n * ln(n) - n + 1 when n goes to infinity
    //
    // This value can be corrected with second or thrid order coefficients
    // when using Taylor's development to get more accuracy with lower values
    // of n.
    match index.cmp(&1) {
        Ordering::Less => 0.0,
        Ordering::Equal => 0.0,
        Ordering::Greater => {
            let index = *index as f32;
            let frac_12: f32 = 1.0 / 12.0;
            let frac_360: f32 = 1.0 / 360.0;
            let double_pi: f32 = 2.0 * PI;
            let frac_pi: f32 = 0.5 * double_pi.ln();
            (index + 0.5) * index.ln() - index + frac_12 / index - frac_360 / index / index / index + frac_pi
        }
    }
}

struct HGD {
    // Random variates from the hypergeometric distribution
    //
    // Returns the number of white balls drawn when kk balls are drawn
    // at random from an urn containing nn1 white and nn2 black balls
    // nn1 -- good
    // nn2 -- bad
}

impl HGD {
    fn loggam (x: &f32) -> f32 {
        // This method is aimed at implementing log-gamma function computation
        // to support some of the distributions.
        //
        // The algorithm comes from SPECFUN by Shanjie Zhang and Jiamming Jin
        // and their book "Computation of Special Functions", 1996, John Wiley & Sons.
        //
        // This formula is based on Rocktaeschel approximation :
        //
        // loggam(x) ~ (x - 0.5) * ln(x) - x + 0.5 * ln(2 * PI) as x goes to + infinity
        //
        // This approximation can be improved using some below values as corrections

        let a: Vec<f32> = vec![
            8.333_333_333_333_333e-02, -2.777_777_777_777_778e-03,
            7.936_507_936_507_937e-04, -5.952_380_952_380_952e-04,
            8.417_508_417_508_418e-04, -1.917_526_917_526_918e-03,
            6.410_256_410_256_410e-03, -2.955_065_359_477_124e-02,
            1.796_443_723_688_307e-01, -1.392_432_216_905_900e+00
        ];

        let mut x0: f32 = x.clone();
        let mut n: u32 = 0;

        if (x - 1.0).abs() < EPSILON || (x - 2.0).abs() < EPSILON {
            return 0.0
        }

        if x <= &7.0 {
            n = 7 - (*x as u32);
            x0 = x + (n as f32);
        }

        let x2: f32 = 1.0 / (x0 * x0);
        let xp: f32 = 2.0 * PI;
        let mut gl0: f32 = a[9];

        for k in (0..=8).rev() {
            gl0 *= x2;
            gl0 += a[k];
        }

        let mut gl: f32 = gl0 / x0 + 0.5 * xp.ln() + (x0 - 0.5) * x0.ln() - x0;

        if x <= &7.0 {
            for _k in 1..=n {
                gl -= (x0 - 1.0).ln();
                x0 -= 1.0;
            }
        }

        gl
    }
}

#[cfg(test)]
mod tests {

    use super::afc;
    use super::HGD;
    use super::PRNG;

    use std::f32::EPSILON;
    use std::f32::consts::LN_2;

    use std::f64::EPSILON as EPSILON_64;

    #[test]
    fn test_afc () {
        // To test the result values, a few values were computed
        // using other methods.
        assert!(afc(&1).abs() < EPSILON);

        // For low values (2 and 3), precision is not good enough to under
        // EPSILON precision. just use 1e-4 as boundary
        assert!((afc(&2) - LN_2).abs() < 1e-04_f32);
        assert!((afc(&3) - 1.791_759).abs() < 1e-04_f32);

        assert!((afc(&4) - 3.178_053).abs() < EPSILON);
        assert!((afc(&10) - 15.104_412).abs() < EPSILON);
        assert!((afc(&15) - 27.899_271).abs() < EPSILON);
        assert!((afc(&100) - 363.739_375).abs() < EPSILON);
    }

    #[test]
    fn test_hgd_loggam () {
        // Low values do not have enough precision so take 1e-04 as boundary
        assert!((HGD::loggam(&0.5) - 0.572_364).abs() < 1e-04_f32);
        assert!((HGD::loggam(&3.0) - 0.693_147).abs() < 1e-04_f32);
        assert!((HGD::loggam(&3.5) - 1.200_973).abs() < 1e-04_f32);

        // These are precisely computed since their values are known
        assert!(HGD::loggam(&1.0).abs() <  EPSILON);
        assert!(HGD::loggam(&2.0).abs() < EPSILON);

        // These values are large enough to be compared to std::f32::EPSILON
        assert!((HGD::loggam(&5.0) - 3.178_053).abs() < EPSILON);
        assert!((HGD::loggam(&15.0) - 25.191_221).abs() < 1e-04_f32);
        assert!((HGD::loggam(&50.0) - 144.565_744).abs() < EPSILON);
        assert!((HGD::loggam(&100.0) - 359.134_205).abs() < EPSILON);
        assert!((HGD::loggam(&1000.0) - 5_905.220_423).abs() < EPSILON);
    }

    #[test]
    fn test_prng_numerify_coins () {
        let mut coins: [u8; 32] = [0; 32];
        let prng = PRNG { coins: coins};
        assert_eq!(prng.numerify_coins(), 0);

        let mut coins: [u8; 32] = [0; 32];
        coins[31] = 1;
        let prng = PRNG { coins: coins};
        assert_eq!(prng.numerify_coins(), 1);

        let mut coins: [u8; 32] = [0; 32];
        coins[30] = 1;
        coins[31] = 1;
        let prng = PRNG { coins: coins};
        assert_eq!(prng.numerify_coins(), 3);

        let mut coins: [u8; 32] = [0; 32];
        coins[0] = 1;
        let prng = PRNG { coins: coins};
        assert_eq!(prng.numerify_coins(), 2_u32.pow(31));

        let mut coins: [u8; 32] = [1; 32];
        let prng = PRNG { coins: coins};
        assert_eq!(prng.numerify_coins(), (2_u64.pow(32) - 1) as u32);
    }

    #[test]
    fn test_prng_draw () {
        let mut coins: [u8; 32] = [0; 32];
        let prng = PRNG { coins: coins};
        assert_eq!(prng.draw(), 0.0_f64);

        let mut coins: [u8; 32] = [0; 32];
        coins[31] = 1;
        let prng = PRNG { coins: coins};
        assert!((prng.draw() - 2.328_306_437e-10_f64).abs() < EPSILON_64);

        let mut coins: [u8; 32] = [0; 32];
        coins[30] = 1;
        coins[31] = 1;
        let prng = PRNG { coins: coins};
        assert!((prng.draw() - 6.984_919_311e-10_f64).abs() < EPSILON_64);

        let mut coins: [u8; 32] = [0; 32];
        coins[0] = 1;
        let prng = PRNG { coins: coins};
        assert!((prng.draw() - 0.500_000_000_116_415_3_f64).abs() < EPSILON_64);

        let mut coins: [u8; 32] = [1; 32];
        let prng = PRNG { coins: coins};
        assert_eq!(prng.draw(), 1.0_f64);
    }
}
