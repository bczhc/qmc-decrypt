pub trait StreamExt {
    fn read_u32_be(&self, offset: usize) -> u32;
    fn read_u32_le(&self, offset: usize) -> u32;
    fn write_u32_be(&mut self, offset: usize, value: u32);
}

impl StreamExt for [u8] {
    #[inline]
    fn read_u32_be(&self, offset: usize) -> u32 {
        u32::from_be_bytes(self[offset..offset + 4].try_into().unwrap())
    }

    #[inline]
    fn read_u32_le(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self[offset..offset + 4].try_into().unwrap())
    }

    #[inline]
    fn write_u32_be(&mut self, offset: usize, value: u32) {
        self[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_u32_be_test() {
        assert_eq!([1, 2, 3, 4].read_u32_be(0), 0x01020304);
        assert_eq!([0x7f, 0xff, 0xee, 0xdd, 0xcc].read_u32_be(1), 0xffeeddcc);
    }

    #[test]
    fn read_u32_le_test() {
        assert_eq!([1, 2, 3, 4].read_u32_le(0), 0x04030201);
        assert_eq!([0x7f, 0xff, 0xee, 0xdd, 0xcc].read_u32_le(1), 0xccddeeff);
    }

    #[test]
    fn test_write_u32_be() {
        let mut v = [0x7fu8, 0xff, 0xee, 0xdd, 0xcc];
        v.write_u32_be(0, 0x01020304);
        assert_eq!(v, [1u8, 2, 3, 4, 0xcc]);
    }
}
