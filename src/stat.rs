use super::hgd::HGD;
use super::ope::ValueRange;

fn sample_hgd(in_range: &ValueRange, out_range: &ValueRange, nsample: &f64, seed_coins: &[u8; 32]) -> f64 {

    // Get a sample from the hypergeometric distribution, using the provided bit list (seed coins)
    // as a source of randomness.

    let in_size: f64 = in_range.size();
    let out_size: f64 = out_range.size();

    if in_size < 1_f64 {
        panic!("in_range must have a positive size. Current size is : {:?}", in_size);
    }

    if out_size < 1_f64 {
        panic!("out_range must have a positive size. Current size is : {:?}", out_size);
    }

    if !(in_range.contains(nsample)) {
        panic!("nsample must be in in_range. Current nsample is {:?}, current in_range is {:?}.", nsample, in_range);
    }

    let nsample_index: f64 = nsample - out_range.start + 1_f64;
    if in_size.eq(&out_size) {
        return in_range.start + nsample_index - 1_f64;
    } 

    let in_sample_num: f64 = HGD::rhyper(&nsample_index, &in_size, &(out_size - in_size), seed_coins); 

    if in_sample_num == 0_f64 {
        return in_range.start;
    } else {
        let in_sample = in_range.start + in_sample_num - 1_f64;

        if !(in_range.contains(&in_sample)) {
            panic!("Error with in_range value. Current in_range is {:?}", in_range);
        }

        return in_sample;
    }
}

fn sample_uniform(in_range: &ValueRange, seed_coins: &[u8; 32]) -> f64 {

    // Uniformly select a number from the range using the provided bit list (seed_coins)
    // as a source of randomness.

    let mut current_range: ValueRange = (*in_range).clone();

    if current_range.size() == 0_f64 {
        panic!("Provided range has zero size. Current range {:?}", in_range);
    }

    let mut bit_counter: usize = 0;
    while current_range.size() > 1_f64 {

        let mid: f64 = (current_range.start + current_range.end).div_euclid(2_f64); 

        // Check if bit_counter exceeds seed_coins length (32)
        if bit_counter > 31 {
            panic!("Not enough coins.");
        }

        let bit: u8 = seed_coins[bit_counter];

        if bit == 0_u8 {
            current_range.end = mid;
        } else if bit == 1_u8 {
            current_range.start = mid + 1_f64;
        } else {
            panic!("Coins must be binary units. Found {:?}", bit);
        }

        bit_counter += 1_usize;
    }

    current_range.start
}


#[cfg(test)]
mod tests {

    use super::ValueRange;
    use super::sample_hgd;
    use super::sample_uniform;

    #[test]
    fn test_sample_hgd () {

        let mut in_range: ValueRange = ValueRange::new(1_f64, 100_f64);
        let mut out_range: ValueRange = ValueRange::new(1_f64, 300_f64);
        let mut seed_coins: [u8; 32] = [1; 32];

        assert_eq!(sample_hgd(&in_range, &out_range, &10_f64, &seed_coins), 10_f64);
        assert_eq!(sample_hgd(&in_range, &out_range, &2_f64, &seed_coins), 2_f64);

        seed_coins = [0; 32];
        seed_coins[31] = 1;

        assert_eq!(sample_hgd(&in_range, &out_range, &10_f64, &seed_coins), 1_f64);
        assert_eq!(sample_hgd(&in_range, &out_range, &8_f64, &seed_coins), 1_f64);

        in_range = ValueRange::new(-1_000_f64, 100_000_f64);
        out_range = ValueRange::new(-100_000_f64, 1_000_000_f64);
        
        seed_coins = [0; 32];
        seed_coins[0] = 1_u8;
        seed_coins[2] = 1_u8;
        seed_coins[3] = 1_u8;

        assert_eq!(sample_hgd(&in_range, &out_range, &2000_f64, &seed_coins), 8406_f64);
    }

    #[test]
    fn test_sample_uniform () {

        let mut in_range: ValueRange = ValueRange::new(1_f64, 1000_f64);
        let mut seed_coins: [u8; 32] = [1; 32];

        assert_eq!(sample_uniform(&in_range, &seed_coins), 1000_f64);

        in_range = ValueRange::new(-1000_f64, 100_000_f64);
        seed_coins = [0; 32];
        seed_coins[0] = 1_u8;
        seed_coins[2] = 1_u8;
        seed_coins[3] = 1_u8;

        assert_eq!(sample_uniform(&in_range, &seed_coins), 68439_f64);
    }
}
