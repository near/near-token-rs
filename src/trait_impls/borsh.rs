#[cfg(test)]
mod test {
    use borsh::{to_vec, BorshDeserialize};

    use crate::NearToken;

    #[test]
    fn borsh() {
        fn test_borsh_ser(val: u128, expected_serialized_value: [u8; 16]) {
            let gas = NearToken::from_yoctonear(val);
            let ser = to_vec(&gas).unwrap();
            // println!("{:?}", ser);
            assert_eq!(expected_serialized_value, ser.as_slice());
            let de: NearToken = NearToken::try_from_slice(&ser).unwrap();
            assert_eq!(de.as_yoctonear(), val);
        }

        test_borsh_ser(
            u128::MAX,
            [
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ],
        );
        test_borsh_ser(8, [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        test_borsh_ser(0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
