use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::BufReader;

const HANDRANKS_SIZE: usize = 32487834;

pub struct HandEval {
    data: Vec<u32>,
}

impl HandEval {
    pub fn new(filename: &str) -> Self {
        let mut hand_eval = HandEval { data: Vec::new() };
        hand_eval.load_data(filename);
        hand_eval
    }

    fn load_data(self: &mut Self, filename: &str) {
        let mut file = BufReader::new(File::open(filename).unwrap());
        self.data.reserve(HANDRANKS_SIZE);
        for _ in 0..HANDRANKS_SIZE {
            self.data.push(file.read_u32::<LittleEndian>().unwrap());
        }
    }

    pub fn eval(self: &Self, cards: &[u8]) -> u32 {
        let a = (self.data[53 + cards[0] as usize]) as usize;
        let b = (self.data[a + cards[1] as usize]) as usize;
        let c = (self.data[b + cards[2] as usize]) as usize;
        let d = (self.data[c + cards[3] as usize]) as usize;
        let e = (self.data[d + cards[4] as usize]) as usize;
        let f = (self.data[e + cards[5] as usize]) as usize;
        let g = self.data[f + cards[6] as usize];
        g
    }

    // same as eval but with cards in 0..52
    pub fn eval0(self: &Self, cards: &[u8]) -> u32 {
        let a = (self.data[54 + cards[0] as usize]) as usize;
        let b = (self.data[a + 1 + cards[1] as usize]) as usize;
        let c = (self.data[b + 1 + cards[2] as usize]) as usize;
        let d = (self.data[c + 1 + cards[3] as usize]) as usize;
        let e = (self.data[d + 1 + cards[4] as usize]) as usize;
        let f = (self.data[e + 1 + cards[5] as usize]) as usize;
        let g = self.data[f + 1 + cards[6] as usize];
        g
    }
}

#[cfg(test)]
mod tests {
    use crate::HandEval;

    #[test]
    fn haha() {
        let eval = HandEval::new("handranks.dat");
        let mut category_counts = [0u32; 10];
        for a in 0..52 {
            for b in a + 1..52 {
                for c in b + 1..52 {
                    for d in c + 1..52 {
                        for e in d + 1..52 {
                            for f in e + 1..52 {
                                for g in f + 1..52 {
                                    let score = eval.eval0(&[a, b, c, d, e, f, g]);
                                    let rank = score >> 12;
                                    category_counts[rank as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(
            category_counts,
            [0, 23294460, 58627800, 31433400, 6461620, 6180020, 4047644, 3473184, 224848, 41584]
        );
    }
}
