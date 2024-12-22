use crate::errors::{BitOutOfBoundsError, ByteOutOfBoundsError};

pub struct Password {
    bytes: [u8; 18]
}

impl Password {
    pub fn new() -> Password {
        Password {
            bytes: [0; 18]
        }
    }

    pub fn get_raw_bytes(&self) -> &[u8; 18] {
        &self.bytes
    }

    pub fn set_byte_value(&mut self, byte: u8, new_value: u8) -> Result<(), ByteOutOfBoundsError> {
        if byte > 17 { return Err(ByteOutOfBoundsError); }

        self.bytes[byte as usize] = new_value;

        Ok(())
    }

    pub fn set_bit(&mut self, bit: u8) -> Result<(), BitOutOfBoundsError> {
        if bit > 127 { return Err(BitOutOfBoundsError); }
        
        let byte = self.get_byte_from_bit(&bit);
        let bit = self.get_relative_bit(&bit);
        let shifts = self.get_needed_shifts(&bit);

        self.bytes[byte as usize] |= 0x01 << shifts;
        
        Ok(())
    }

    pub fn clear_bit(&mut self, bit: u8) -> Result<(), BitOutOfBoundsError> {
        if bit > 127 { return Err(BitOutOfBoundsError); }

        let byte = self.get_byte_from_bit(&bit);
        let bit = self.get_relative_bit(&bit);
        let shifts = self.get_needed_shifts(&bit);
        let mask = (0x01 << shifts) ^ 0xFF;

        self.bytes[byte as usize] &= mask;

        Ok(())
    }

    fn get_byte_from_bit(&self, bit: &u8) -> u8 {
        bit / 8
    }

    fn get_relative_bit(&self, absolute_bit: &u8) -> u8 {
        let byte = self.get_byte_from_bit(&absolute_bit);
        absolute_bit - (byte * 8)
    }

    fn get_needed_shifts(&self, bit: &u8) -> u8 {
        7 - bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_password() {
        Password::new();
    }

    #[test]
    fn should_get_zeroed_raw_bytes() {
        let pass = Password::new();
        let bytes = pass.get_raw_bytes();
        let expected = [0u8; 18];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn should_return_error_when_setting_byte_out_of_bounds() {
        let mut pass = Password::new();
        assert!(pass.set_byte_value(18, 0x0F).is_err());
    }

    #[test]
    fn should_set_whole_byte() {
        let mut pass = Password::new();
        assert!(!pass.set_byte_value(2, 0x0F).is_err());
        assert_eq!(pass.bytes[2], 0x0F);
    }

    #[test]
    fn should_return_error_when_setting_bit_out_of_bounds() {
        let mut pass = Password::new();
        let r = pass.set_bit(128);
        assert!(matches!(r, Err(BitOutOfBoundsError)));
    }

    #[test]
    fn should_set_last_bit_of_byte() {
        let mut pass = Password::new();
        let r = pass.set_bit(127);
        assert!(!r.is_err());
        assert_eq!(pass.bytes[15], 1);
    }

    #[test]
    fn should_set_first_and_last_bits_of_byte() {
        let mut pass = Password::new();
        assert!(!pass.set_bit(127).is_err());
        assert!(!pass.set_bit(120).is_err());
        assert_eq!(pass.bytes[15], 0b10000001);
    }

    #[test]
    fn should_return_error_when_clearing_bit_out_of_bounds() {
        let mut pass = Password::new();
        let r = pass.clear_bit(128);
        assert!(matches!(r, Err(BitOutOfBoundsError)));
    }

    #[test]
    fn should_clear_last_bit_of_byte() {
        let mut pass = Password::new();
        pass.bytes[15] = 0xFF;
        assert!(!pass.clear_bit(127).is_err());
        assert_eq!(pass.bytes[15], 0b11111110);
    }

    #[test]
    fn should_clear_first_and_last_bits_of_byte() {
        let mut pass = Password::new();
        pass.bytes[15] = 0xFF;
        assert!(!pass.clear_bit(120).is_err());
        assert!(!pass.clear_bit(127).is_err());
        assert_eq!(pass.bytes[15], 0b01111110);
    }

    #[test]
    fn should_get_byte_from_bit() {
        let password = Password::new();

        let byte = password.get_byte_from_bit(&10);
        assert_eq!(byte, 1);

        let byte = password.get_byte_from_bit(&19);
        assert_eq!(byte, 2);

        let byte = password.get_byte_from_bit(&23);
        assert_eq!(byte, 2);
    }
}