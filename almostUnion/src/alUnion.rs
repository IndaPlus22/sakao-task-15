#[derive(Debug)]
struct Element {
    value: usize,
    parent: usize,
}

impl Element {
    fn new(val: usize, par: usize) -> Element {
        Element {
            value: val,
            parent: par,
        }
    }
}

pub struct BetUnion {
    values: Vec<Element>,
    // locations: Vec<usize>
}

impl BetUnion {
    pub fn new(size: usize) -> BetUnion {
        let mut values: Vec<Element> = Vec::new();
        // let locations: Vec<usize> = (0..size).map(|v| v).collect();
        for i in 0..size {
            values.push(Element::new(i + 1, i));
        }

        BetUnion {
            values, /*locations */
        }
    }

    // union the sets that contain num1 and num2
    pub fn union(&mut self, num1: usize, num2: usize) {
        let old_parent = self.find(num1);
        let new_parent = self.find(num2);
        if old_parent == new_parent {
            return;
        }
        let tmp: Vec<bool> = self.values.iter().map(|x| x.parent == old_parent).collect();

        // println!("debug:::union: {:?}", tmp);
        for i in 0..tmp.len() {
            if tmp[i] {
                self.values[i].parent = new_parent;
                // self.locations[i] = new_parent  ;
            }
        }

        // println!("res: {:?}", self.values);
        // println!("loc: {:?}", self.locations);
    }

    // move num1 to set containing num2
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let new_parent = self.find(num2);
        if new_parent == self.find(num1) {
            return;
        }
        self.values[num1 - 1].parent = new_parent;
        // println!("new parent {}, num1: {}", new_parent, num1);
        // self.locations[num1   - 1] = new_parent  ;

        // println!("res: {:?}", self.values);
        // println!("loc: {:?}", self.locations);
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&self, num: usize) -> (usize, usize) {
        let parent = self.find(num);
        let tmp: Vec<bool> = self.values.iter().map(|x| x.parent == parent).collect();
        let mut val: usize = 0;
        let mut sum: usize = 0;

        // println!("debug:::get: {:?}", tmp);
        for i in 0..tmp.len() {
            if tmp[i] {
                sum += i + 1;
                val += 1;
            }
        }

        (val, sum)
    }

    // returns which set number is in e.g. number 3 is in set number 2(sets are indexed from 0)
    fn find(&self, num: usize) -> usize {
        self.values[num - 1].parent
    }
}


// old version
pub struct AlUnion {
    sets: Vec<Vec<usize>>,
    locations: Vec<usize>,
}

impl AlUnion {
    pub fn new(size: usize) -> AlUnion {
        let mut sets: Vec<Vec<usize>> = Vec::new();
        for i in 0..size {
            sets.push(Vec::new());
        }
        let locations: Vec<usize> = (0..size).map(|v| v).collect();

        for i in 0..size {
            sets[i] = vec![i + 1; 1];
        }
        AlUnion { sets, locations }
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
        self.locations[(num2 - 1)] = set1;

        // println!("res: {:?}", self.sets);
        // println!("loc: {:?}", self.locations);
    }

    // move num1 to set containing num2
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);
        if set1 == set2 {
            return;
        }

        let rem_index = self.sets[set1].iter().position(|&x| x == num1).unwrap();

        self.sets[set2].push(num1);
        self.sets[set1].remove(rem_index);
        self.locations[(num1 - 1)] = set2;

        // println!("res: {:?}", self.sets);
        // println!("loc: {:?}", self.locations);
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&self, num: usize) -> (usize, usize) {
        let set = self.find(num);

        (self.sets[set].len(), self.sets[set].iter().sum())
    }

    // returns which set num is in e.g. number 3 is in set number 2(sets are indexed from 0)
    fn find(&self, num: usize) -> usize {
        self.locations[(num - 1)]
    }
}
