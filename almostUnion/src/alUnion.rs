

pub struct al_union {
    sets: Vec<Vec<usize>>,
    locations: Vec<usize>,
    size: usize
}

impl al_union {
    pub fn new(size: usize) -> al_union {
        let mut sets: Vec<Vec<usize>> = Vec::with_capacity(size);
        let locations: Vec<usize> = (0..size).map(|v| v).collect();
        for i in 0..size {
            sets[i] = vec![i; 1];
        }
        al_union {
            sets,
            locations,
            size
        }
    }

    // union the sets that contain num1 and num2
    pub fn union(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);
        if set1 == set2 {
            return;
        }

        let mut tmp: Vec<usize> = Vec::new();
        tmp.append(&mut self.sets[set2]);
        tmp.append(&mut self.sets[set1]);

        self.sets[set1] = tmp;
        self.locations[num2 -1] = set1;
    }

    // move num2 to set containing num1
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);
        if set1 == set2 {
            return;
        }

        let rem_index = self.sets[set2].iter().position(|&x| x == num2).unwrap();

        self.sets[set1].push(num2);
        self.sets[set2].remove(rem_index);
        self.locations[num2 - 1] = set1;
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&self, num: usize) -> (usize, usize) {
        let set = self.find(num);

        (self.sets[set].len(), self.sets[set].iter().sum())
    }

    // returns which set num is in e.g. number 3 is in set number 2(sets are indexed from 0)
    fn find(&self, num: usize) -> usize {
        self.locations[num - 1]
    }
}