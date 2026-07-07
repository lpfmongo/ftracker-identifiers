use arbitrary::{Arbitrary, Unstructured};

use super::Isin;
use super::validation::{BASE_LEN, compute_check_digit};

const LETTERS: &[u8; 26] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUMERIC: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl<'a> Arbitrary<'a> for Isin {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut base = [0u8; BASE_LEN];

        // Positions 1-2: ISO 3166-1 alpha-2 country code (letters only).
        base[0] = LETTERS[u.arbitrary::<u8>()? as usize % LETTERS.len()];
        base[1] = LETTERS[u.arbitrary::<u8>()? as usize % LETTERS.len()];

        // Positions 3-11: alphanumeric NSIN.
        for slot in base[2..].iter_mut() {
            *slot = ALPHANUMERIC[u.arbitrary::<u8>()? as usize % ALPHANUMERIC.len()];
        }

        let check = compute_check_digit(&base);
        let mut bytes = [0u8; 12];
        bytes[..BASE_LEN].copy_from_slice(&base);
        bytes[BASE_LEN] = check + b'0';

        Isin::from_bytes(bytes).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_produces_valid_isins() {
        for seed in 0u32..256 {
            let data = seed.to_le_bytes().repeat(8);
            let mut u = Unstructured::new(&data);
            let isin = Isin::arbitrary(&mut u).expect("arbitrary should always succeed");
            // Re-validating via parse() proves the value round-trips through the exact same
            // checks a hand-typed input would.
            assert!(Isin::parse(isin.as_str()).is_ok());
        }
    }
}
