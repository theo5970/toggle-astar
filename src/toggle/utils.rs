#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BitArray {
    bytes: Vec<u8>,
    len: usize
}

impl BitArray {
    pub fn new(x: usize) -> BitArray {
        let size = ((x as f64) / 8.0).ceil() as usize;
        let mut result = BitArray {
            bytes: Vec::with_capacity(size),
            len: x
        };

        for _ in 0..size {
            result.bytes.push(0);
        }

        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn from(r: &[u8]) -> BitArray {
        BitArray { bytes: Vec::from(r), len: r.len() * 8}
    }

    pub fn resize(&mut self, new_length: usize) {
        let size = ((new_length as f64) / 8.0).ceil() as usize;

        if size > new_length {
            let diff = new_length - size;
            for _ in 0..diff {
                self.bytes.push(0);
            }
        }
        self.len = new_length;
    }

    pub fn get(&self, pos: usize) -> bool {
        let byte_index = pos / 8;
        let bit_pos = pos % 8;

        return (self.bytes[byte_index] & (1 << bit_pos)) != 0; 
    }

    pub fn set(&mut self, pos: usize, is_on: bool) {
        let byte_index = pos / 8;
        let bit_pos = pos % 8;

        if is_on {
            self.bytes[byte_index] |= 1 << bit_pos;
        } else {
            self.bytes[byte_index] &= !(1 << bit_pos);
        }
    }

    pub fn print(&self) {
        print!("output: ");
        for i in 0..self.bytes.len() {
            let mut byte = self.bytes[i];

            for k in 0..8 {
                if (i * 8 + k) > self.len {
                    break;
                }

                let bit = (byte >> 7) & 1;
                print!("{}", bit);
                byte <<= 1;
            }
            print!(" ");
        }
        println!("");
    }

    pub fn to_base64(&self) -> String {
        base64::encode(&self.bytes)
    }
}