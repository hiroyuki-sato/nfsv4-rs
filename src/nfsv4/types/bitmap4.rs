#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap4(Vec<u32>);

impl Bitmap4 {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&word| word == 0)
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.0
    }

    pub fn into_inner(self) -> Vec<u32> {
        self.0
    }

    pub fn contains(&self, bit: u32) -> bool {
        let word = (bit / 32) as usize;
        let mask = 1u32 << (bit % 32);
        self.0.get(word).map(|w| (*w & mask) != 0).unwrap_or(false)
    }

    pub fn insert(&mut self, bit: u32) {
        let word = (bit / 32) as usize;
        let mask = 1u32 << (bit % 32);
        if self.0.len() <= word {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= mask;
    }

    pub fn remove(&mut self, bit: u32) {
        let word = (bit / 32) as usize;
        let mask = 1u32 << (bit % 32);
        if let Some(w) = self.0.get_mut(word) {
            *w &= !mask;
        }
    }

    pub fn set(&mut self, bit: u32, value: bool) {
        if value {
            self.insert(bit);
        } else {
            self.remove(bit);
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Default for Bitmap4 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<u32>> for Bitmap4 {
    fn from(bits: Vec<u32>) -> Self {
        Self(bits)
    }
}

impl From<Bitmap4> for Vec<u32> {
    fn from(bitmap: Bitmap4) -> Self {
        bitmap.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bitmap() {
        let bitmap = Bitmap4::new();
        assert!(bitmap.is_empty());
        assert_eq!(bitmap.as_slice(), &[]);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut bitmap = Bitmap4::new();
        bitmap.insert(0);
        bitmap.insert(31);
        bitmap.insert(32);
        bitmap.insert(64);

        assert!(bitmap.contains(0));
        assert!(bitmap.contains(31));
        assert!(bitmap.contains(32));
        assert!(bitmap.contains(64));

        assert!(!bitmap.contains(1));
        assert!(!bitmap.contains(63));
    }

    #[test]
    fn test_remove() {
        let mut bitmap = Bitmap4::new();
        bitmap.insert(2);
        bitmap.insert(64);

        bitmap.remove(2);
        assert!(!bitmap.contains(2));
        assert!(bitmap.contains(64));

        bitmap.remove(64);
        assert!(!bitmap.contains(64));
    }

    #[test]
    fn test_set() {
        let mut bitmap = Bitmap4::new();
        bitmap.set(33, true);
        assert!(bitmap.contains(33));

        bitmap.set(33, false);
        assert!(!bitmap.contains(33));
    }

    #[test]
    fn test_clear() {
        let mut bitmap = Bitmap4::new();
        bitmap.insert(10);
        bitmap.insert(100);

        bitmap.clear();
        assert!(bitmap.is_empty());
        assert_eq!(bitmap.as_slice(), &[]);
    }

    #[test]
    fn test_from_vec_and_into_vec() {
        let bitmap = Bitmap4::from(vec![0x1, 0x2]);
        assert_eq!(bitmap.as_slice(), &[0x1, 0x2]);

        let raw: Vec<u32> = bitmap.into();
        assert_eq!(raw, vec![0x1, 0x2]);
    }
}
