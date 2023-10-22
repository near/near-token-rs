use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::NearToken;

impl Serialize for NearToken{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = [0u8; 20];
        let remainder = {
            use std::io::Write;
            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.inner).map_err(|err| {
                Error::custom(format!("Failed to serialize: {}", err.to_string()))
            })?;
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len])
            .map_err(|err| Error::custom(format!("Failed to serialize: {}", err.to_string())))?;
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for NearToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<u128>()
            .map(NearToken::from_yoctonear)
            .map_err(|err| de::Error::custom(err.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::NearToken;

    #[test]
    fn json_ser() {
        fn test_json_ser(val: u128) {
            let gas = NearToken::from_yoctonear(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("\"{}\"", val));
            let de: NearToken= serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_yoctonear(), val);
        }

        test_json_ser(u128::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }
}
