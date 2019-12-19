use std::cmp::Ordering;
use std::f32::consts::PI;

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
    match index.cmp(&0) {
        Ordering::Less => 0.0, // TODO : necessary with unsigned int ??
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
            8.333333333333333e-02, -2.777777777777778e-03,
            7.936507936507937e-04, -5.952380952380952e-04,
            8.417508417508418e-04, -1.917526917526918e-03,
            6.410256410256410e-03, -2.955065359477124e-02,
            1.796443723688307e-01, -1.39243221690590e+00
        ];

        let mut x0: f32 = x.clone();
        let mut n: u32 = 0;

        if x == &1.0 || x == &2.0 {
            return 0.0
        }

        if x <= &7.0 {
            n = 7 - (*x as u32);
            x0 = x + (n as f32);
        }

        let x2: f32 = 1.0 / (x0 * x0);
        let xp: f32 = 2.0 * PI;
        let mut gl0: f32 = a[9];

        for k in (0..9).rev() {
            gl0 *= x2;
            gl0 += a[k];
        }

        let mut gl: f32 = gl0 / x0 + 0.5 * xp.ln() + (x0 - 0.5) * x0.ln() - x0;

        if x <= &7.0 {
            for k in 1..(n + 1) {
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

    #[test]
    fn test_afc () {
        assert!(afc(&1).abs() < 0.001_f32);
        assert!((afc(&4) - 3.178053_f32).abs() < 0.001_f32);
        assert!((afc(&10) - 15.104412_f32).abs() < 0.001_f32);
        assert!((afc(&15) - 27.899271_f32).abs() < 0.001_f32);
        assert!((afc(&100) - 363.739375_f32).abs() < 0.001_f32);
    }

    #[test]
    fn test_hgd_loggam () {
        assert!((HGD::loggam(&0.5) - 0.572364).abs() < 0.001_f32);
        assert_eq!(HGD::loggam(&1.0), 0.0);
        assert_eq!(HGD::loggam(&2.0), 0.0);
        assert!((HGD::loggam(&3.0) - 0.693147).abs() < 0.001_f32);
        assert!((HGD::loggam(&3.5) - 1.200973).abs() < 0.001_f32);
        assert!((HGD::loggam(&5.0) - 3.178053).abs() < 0.001_f32);
        assert!((HGD::loggam(&15.0) - 25.191221).abs() < 0.001_f32);
        assert!((HGD::loggam(&100.0) - 359.134205).abs() < 0.001_f32);
        assert!((HGD::loggam(&1000.0) - 5905.220423).abs() < 0.001_f32);
    }
}
