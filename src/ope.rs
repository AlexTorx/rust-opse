extern crate log;
extern crate crypto;

use crypto::mac::Mac;
use crypto::{hmac, aes, sha2};
use crypto::aes::KeySize;

#[cfg(not(test))]
use log::trace;
 
#[cfg(test)]
use std::println as trace;

use super::stat;
use super::util;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueRange {
    pub start: f64,
    pub end: f64,
}

impl ValueRange {
    pub fn new (start: f64, end: f64) -> ValueRange {

        if start > end {
            panic!("ValueRange : start value ({}) should not be greater than end value ({}).", start, end);
        }

        if start != start.floor() {
            panic!("ValueRange : start value should be a 0-decimal f64 number. Found {}", start);
        }

        if end != end.floor() {
            panic!("ValueRange : end value should be a 0-decimal f64 number. Found {}", end);
        }

        ValueRange { start: start, end: end }
    }

    pub fn size (&self) -> f64 {
        // This function is aimed at returning the number of values
        // in the current ValueRange object
        self.end - self.start + 1.0
    }

    pub fn contains (&self, number: &f64) -> bool {
        self.start <= *number && *number <= self.end
    }
}


#[derive(Debug)]
pub struct OPE {
    encryption_key: String,
    in_range: ValueRange,
    out_range: ValueRange,
}

impl OPE {
    pub fn new (encryption_key: &String, in_range: &ValueRange, out_range: &ValueRange) -> OPE {

        if in_range.size() > out_range.size() {
            panic!("in_range cannot have a bigger size than out_range. Current in_range is {:?} and current out_range is {:?}", in_range, out_range);
        }

        OPE { encryption_key: encryption_key.clone(), in_range: in_range.clone(), out_range: out_range.clone() }
    }

    pub fn encrypt (&self, plaintext: &f64) -> f64 {

        if !(self.in_range.contains(plaintext)) {
            panic!("Plaintext to encrypt must be in in_range ValueRange. Current in_range is {:?} and plaintext is {:?}", self.in_range, plaintext);
        }

        self.encrypt_recursive(plaintext, &(self.in_range.clone()), &(self.out_range.clone()))
    }

    fn encrypt_recursive (&self, plaintext: &f64, in_range: &ValueRange, out_range: &ValueRange) -> f64 {

        let in_size: f64 = self.in_range.size();
        let out_size: f64 = self.out_range.size();

        let in_edge: f64 = self.in_range.start - 1_f64;
        let out_edge: f64 = self.out_range.start - 1_f64;

        let mid: f64 = out_edge + (out_size / 2_f64).ceil();

        trace!("OPE::encrypt_recursive - in_size : {:?}", in_size);
        trace!("OPE::encrypt_recursive - out_size : {:?}", out_size);
        trace!("OPE::encrypt_recursive - in_edge : {:?}", in_edge);
        trace!("OPE::encrypt_recursive - out_edge : {:?}", out_edge);
        trace!("OPE::encrypt_recursive - mid : {:?}", mid);

        let mut coins: [u8; 128];
        if in_range.size().eq(&1_f64) {

            let in_range_min: f64 = in_range.start;
            coins = self.tape_gen(&in_range_min); 

            trace!("OPE::encrypt_recursive - in_range_min : {:?}", in_range_min);
            trace!("OPE::encrypt_recursive - coins : {:?}", coins);

            return stat::sample_uniform(out_range, &coins);
        }

        coins = self.tape_gen(&mid);
        let x: f64 = stat::sample_hgd(in_range, out_range, &mid, &coins);

        trace!("OPE::encrypt_recursive - coins : {:?}", coins);
        trace!("OPE::encrypt_recursive - x : {:?}", x);

        let new_in_range: ValueRange;
        let new_out_range: ValueRange;
        if plaintext.le(&x) {
            new_in_range = ValueRange::new(in_edge + 1_f64, x);
            new_out_range = ValueRange::new(out_edge + 1_f64, mid);
        } else {
            new_in_range = ValueRange::new(x + 1_f64, in_edge + in_size);
            new_out_range = ValueRange::new(mid + 1_f64, out_edge + out_size);
        }

        trace!("OPE::encrypt_recursive - new_in_range : {:?}", new_in_range);
        trace!("OPE::encrypt_recursive - new_out_range : {:?}", new_out_range);

        return self.encrypt_recursive(plaintext, &new_in_range, &new_out_range);
    }

    fn tape_gen (&self, data: &f64) -> [u8; 128] {

        let mut hmac: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new(sha2::Sha256::new(), self.encryption_key.as_bytes());
        hmac.input(data.to_string().as_bytes());

        let hmac_result = hmac.result();

        trace!("OPE::tape_gen - hmac_result : {:?}", hmac_result.code());

        let mut aes_code = aes::ctr(KeySize::KeySize256, hmac_result.code(), &[0;16]);

        let mut coins: [u8; 16] = [0_u8; 16];
        aes_code.process(&[0;16], &mut coins);

        trace!("OPE::tape_gen - coins : {:?}", coins);

        let bits: [u8; 128] = util::get_bits_list(&coins);

        trace!("OPE::tape_gen - bits : {:?}", bits);

        bits
    }
}

#[cfg(test)]
mod tests {

    mod test_value_range {

        use super::super::ValueRange;

        fn create_value_range (start: f64, end: f64) -> ValueRange {
            ValueRange::new(start, end)
        }

        #[test]
        fn test_print_debug () {

            let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);
            assert_eq!(format!("{:?}", range), "ValueRange { start: 0.0, end: 100.0 }");
        }

        #[test]
        fn test_equal () {

            let range_1: ValueRange = create_value_range(0.0_f64, 100.0_f64);
            let range_2: ValueRange = create_value_range(0.0_f64, 100.0_f64);
            assert_eq!(range_1, range_2);

            let range_3: ValueRange = create_value_range(1.0_f64, 100.0_f64);
            assert!(range_1 != range_3);
        }

        #[test]
        fn test_size () {

            let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);
            assert_eq!(range.size(), 101.0);

            let range: ValueRange = create_value_range(100.0_f64, 100.0_f64);
            assert_eq!(range.size(), 1.0);
        }

        #[test]
        fn test_contains () {

            let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);

            assert_eq!(range.contains(&0.0_f64), true);
            assert_eq!(range.contains(&100.0_f64), true);
            assert_eq!(range.contains(&50.0_f64), true);
            assert_eq!(range.contains(&101.0_f64), false);
            assert_eq!(range.contains(&-1.0_f64), false);
        }
    }

    mod test_ope {

        use super::super::OPE;
        use super::super::ValueRange;

        #[test]
        fn test_new () {

            let encryption_key: String = String::from("new_encryption_key");
            let in_range: ValueRange = ValueRange::new(1_f64, 100_f64);
            let out_range: ValueRange = ValueRange::new(-100_f64, 800_f64);

            let mut ope: OPE = OPE::new(&encryption_key, &in_range, &out_range);

            assert_eq!(ope.encryption_key, encryption_key);
            assert_eq!(ope.in_range, in_range);
            assert_eq!(ope.out_range, out_range);
        }

        #[test]
        fn test_encrypt () {

            let in_range: ValueRange = ValueRange::new(0_f64, 1e20_f64);
            let out_range: ValueRange = ValueRange::new(0_f64, 1e50_f64);

            let ope: OPE = OPE::new(&String::from("encryption_key"), &in_range, &out_range);

            let plaintext: f64 = 30792318992869221_f64;

            assert_eq!(ope.encrypt(&plaintext), 30792319112322099345020992978448823790582026526_f64);
        }

        #[test]
        fn test_tape_gen () {

            let data: f64 = 23_f64;
            let in_range: ValueRange = ValueRange::new(0_f64, 1e20_f64);
            let out_range: ValueRange = ValueRange::new(0_f64, 1e50_f64);

            let ope: OPE = OPE::new(&String::from("encryption_key"), &in_range, &out_range);

            let mut result: [u8; 128] = ope.tape_gen(&data);
            let mut expected_result: [u8; 128] = [
                1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0,
                0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0,
                1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1,
                1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1,
                1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1,
                0, 1
            ];

            assert_eq!(result, expected_result);

            let ope: OPE = OPE::new(&String::from("new_encryption_key"), &in_range, &out_range);

            result = ope.tape_gen(&data);
            expected_result = [
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1,
                1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1,
                0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0,
                1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1,
                1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1,
                1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0,
                0, 1
            ];

            assert_eq!(result, expected_result);

        }
    }
}
