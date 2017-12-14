#[derive(Debug, Eq, PartialEq)]
pub struct KnotHasher {
    pub list: Vec<u8>,
    pub pos: usize,
    pub skip: usize,
}

impl KnotHasher {
    pub fn new(size: u32) -> KnotHasher {
        KnotHasher {
            list: (0..size).map(|n| n as u8).collect(),
            pos: 0,
            skip: 0,
        }
    }

    pub fn hash(&mut self, length: u8) {
        let length = length as usize;
        let len = self.list.len();

        let mut section = self.list
            .iter()
            .cloned()
            .cycle()
            .skip(self.pos)
            .take(length)
            .collect::<Vec<_>>();
        section.reverse();

        for (i, &reversed) in section.iter().enumerate().take(length) {
            self.list[(self.pos + i) % len] = reversed;
        }

        self.pos = (self.pos + length + self.skip) % len;

        self.skip += 1;
    }

    pub fn dense_hash(&self) -> Vec<u8> {
        let mut hash = Vec::with_capacity(16);

        for i in 0..16 {
            let mut xor = None;
            for j in 0..16 {
                let val = self.list[i * 16 + j];
                xor = if let Some(xor) = xor.take() {
                    Some(xor ^ val)
                } else {
                    Some(val)
                }
            }

            hash.push(xor.expect("failed to compute XOR"));
        }

        hash
    }

    pub fn dense_hash_from_key(key: &str) -> Vec<u8> {
        let mut key = key.chars().map(|c| c as u8).collect::<Vec<_>>();
        key.push(17);
        key.push(31);
        key.push(73);
        key.push(47);
        key.push(23);

        let mut hasher = KnotHasher::new(256);

        for _ in 0..64 {
            for &length in &key {
                hasher.hash(length)
            }
        }

        hasher.dense_hash()
    }
}
