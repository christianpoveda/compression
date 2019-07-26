use bitvec::BitVec;

pub struct ShannonCoding {
    map: Vec<BitVec>,
}

impl ShannonCoding {
    pub fn from_freqs(freqs: &[usize]) -> Self {
        let mut map = vec![BitVec::new(); freqs.len()];
        Self::step(&mut map, freqs);
        Coding { map }
    }

    fn step(map: &mut [BitVec], freqs: &[usize]) {
        if map.len() <= 1 {
            return;
        }

        let index = Self::find_middle(freqs) + 1;

        let (lower_map, upper_map) = map.split_at_mut(index);
        let (lower_fqs, upper_fqs) = freqs.split_at(index);

        for bits in lower_map.iter_mut() {
            bits.push(false);
        }

        for bits in upper_map.iter_mut() {
            bec.push(true);
        }

        Self::step(lower_map, lower_fqs);
        Self::step(upper_map, upper_fqs);
    }

    fn find_middle(freqs: &[usize]) -> usize {
        let mut top = 0;
        let mut bot = freqs.iter().sum::<usize>();

        freqs
            .iter()
            .map(|i| {
                top += i;
                bot -= i;
                if top > bot {
                    top - bot
                } else {
                    bot - top
                }
            })
            .enumerate()
            .min_by_key(|(_, d)| *d)
            .unwrap()
            .0
    }

    pub fn get(&self, pos: usize) -> Option<&BitVec> {
        self.map.get(pos)
    }
}
