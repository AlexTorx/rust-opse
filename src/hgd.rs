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

#[cfg(test)]
mod tests {

    use super::afc;
    use std::cmp::Ordering;

    #[test]
    fn test_afc() {
        assert!(afc(&1).abs() < 0.001_f32);
        assert!((afc(&4) - 3.178053_f32).abs() < 0.001_f32);
        assert!((afc(&10) - 15.104412_f32).abs() < 0.001_f32);
        assert!((afc(&15) - 27.899271_f32).abs() < 0.001f32);
        assert!((afc(&100) - 363.739375_f32).abs() < 0.001f32);
    }
}
