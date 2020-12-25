const INITIAL_NUMBER: u64 = 1;
const MODULUS: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

pub struct KeyPair(Key, Key);

impl KeyPair {
    pub fn new(pub1: u64, pub2: u64) -> KeyPair {
        KeyPair(Key::new(pub1, pub2), Key::new(pub2, pub1))
    }

    fn step(&mut self) -> Option<u64> {
        self.0.step().or_else(|| self.1.step())
    }

    pub fn find_encryption_key(&mut self) -> Option<u64> {
        self.find(|v| v.is_some()).flatten()
    }
}

impl Iterator for KeyPair {
    type Item = Option<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step())
    }
}

struct Key {
    public_key: u64,
    other: u64,
    temporary: u64,
    encryption_key: u64,
}

impl Key {
    fn new(public_key: u64, other: u64) -> Key {
        Key {
            public_key,
            other,
            temporary: INITIAL_NUMBER,
            encryption_key: INITIAL_NUMBER,
        }
    }

    fn step(&mut self) -> Option<u64> {
        if self.temporary == self.public_key {
            Some(self.encryption_key)
        } else {
            self.temporary = transform(self.temporary, SUBJECT_NUMBER);
            self.encryption_key = transform(self.encryption_key, self.other);
            None
        }
    }
}

fn transform(nr: u64, subject: u64) -> u64 {
    (nr * subject) % MODULUS
}
