#[cfg(not(test))]
use log::trace;
 
#[cfg(test)]
use std::println as trace;


pub fn get_bits_list(numbers_list: &[u8; 16]) -> [u8; 128] {

    let mut total_bits: [u8; 128] = [0_u8; 128];

    for index in 0..16 {
        
        let bits: [u8; 8] = get_bits_for_u8(&numbers_list[index]);

        for bit_index in 0..8 {
            total_bits[8*index + bit_index] = bits[bit_index];
        }
    }

    trace!("util - get_bits_list : {:?} -> {:?}", numbers_list, total_bits);

    total_bits
}

fn get_bits_for_u8(number: &u8) -> [u8; 8] {

    let mut bits: [u8; 8] = [0_u8; 8];
    for n in 0..8 {
        bits[n] = ((number & (1 << (7 - n))) != 0) as u8;
    }

    trace!("util - get_bits_for_u8 : {:?} -> {:?}", number, bits);

    bits
}

#[cfg(test)]
mod tests {
    use super::{get_bits_for_u8, get_bits_list};

    #[test]
    fn test_get_bits_for_u8() {

        let mut test_u8: u8;

        test_u8 = 0;
        assert_eq!(get_bits_for_u8(&test_u8), [0_u8; 8]);

        test_u8 = 255;
        assert_eq!(get_bits_for_u8(&test_u8), [1_u8; 8]);
    }

    #[test]
    fn test_get_bits_list() {

        let mut test_list: [u8; 16];

        test_list = [0_u8; 16];
        assert_eq!(get_bits_list(&test_list), [0_u8; 128]);

        test_list = [255_u8; 16];
        assert_eq!(get_bits_list(&test_list), [1_u8; 128]);
    }
}
