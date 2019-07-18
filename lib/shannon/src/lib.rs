use bitvec::BitVec;

pub struct Code {
    map: Vec<BitVec>,
}

impl Code {
    pub fn new(freqs: &[usize]) -> Self {
        let mut map = vec![BitVec::new(); freqs.len()];
        Self::step(&mut map, freqs);
        Code { map }
    }

    fn step(map: &mut [BitVec], freqs: &[usize]) {
        println!("{:?} {:?}", map.len(), freqs);
        if map.len() <= 1 {
            return;
        }

        let index = Self::find_middle(freqs) + 1;

        for bec in map[..index].iter_mut() {
            bec.push(false);
        }

        for bec in map[index..].iter_mut() {
            bec.push(true);
        }

        Self::step(&mut map[..index], &freqs[..index]);
        Self::step(&mut map[index..], &freqs[index..]);
    }

    fn find_middle(freqs: &[usize]) -> usize {
        let mut top = 0;
        let mut bot = freqs.iter().sum::<usize>();

        freqs
            .iter()
            .map(|i| {
                top += i;
                bot -= i;
                println!("{} {} {}", i, top, bot);
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
