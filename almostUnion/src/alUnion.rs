use std::{fmt, ops::*};

#[derive(Debug, Clone, Copy)]
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

impl Add for Element {
    type Output = usize;

    fn add(self, other: Element) -> usize {
        self.value + other.value
    }
}

pub struct BetUnion {
    // values: Vec<Element>,
    values: Vec<usize>,
    parent: Vec<usize>, // locations: Vec<usize>
}

impl BetUnion {
    pub fn new(size: usize) -> BetUnion {
        // let mut values: Vec<Element> = Vec::new();
        let values: Vec<usize> = (0..size).map(|v| v + 1).collect();
        let parent: Vec<usize> = (0..size).map(|v| v).collect();
        // for i in 0..size {
        //     values.push(Element::new(i + 1, i));
        // }

        BetUnion { values, parent }
    }

    // union the sets that contain num1 and num2
    pub fn union(&mut self, num1: usize, num2: usize) {
        let old_parent = self.find(num1);
        let new_parent = self.find(num2);
        if old_parent == new_parent {
            return;
        }
        let tmp: Vec<bool> = self.parent.iter().map(|x| x == &old_parent).collect();

        for i in 0..tmp.len() {
            if tmp[i] {
                self.parent[i] = new_parent;
            }
        }

        // -------thought this would be faster but wasnt----------------------
        // let mut tmp1: Vec<Element> = self.values.clone();
        // tmp1 = tmp1.drain(..).filter(|x| x.parent == old_parent).collect();

        // for i in 0..tmp1.len() {
        //     self.values[tmp1[i].value - 1].parent = new_parent;
        // }
        // -----------------------------

        // println!("debug:::union: {:?}", tmp);
        // println!("tmp1: {:?}", tmp1);
        // println!("res: {:?}", self.values);
    }

    // move num1 to set containing num2
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let new_parent = self.find(num2);
        if new_parent == self.find(num1) {
            return;
        }
        self.parent[num1 - 1] = new_parent;
        // println!("new parent {}, num1: {}", new_parent, num1);

        // println!("res: {:?}", self.values);
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&mut self, num: usize) -> (usize, usize) {
        let parent = self.find(num);
        let tmp1: Vec<bool> = self.parent.iter().map(|x| x == &parent).collect();
        // let tmp1: Vec<&Element> = self
        //     .values
        //     .iter()
        //     .filter(|x| x.parent == parent)
        //     .collect::<Vec<&Element>>();

        let mut sum: usize = 0;
        let mut vals: usize = 0;

        for i in 0..tmp1.len() {
            if tmp1[i] {
                vals += 1;
                sum += self.values[i];
            }
        }
        // for i in 0..tmp1.len() {
        //     sum += tmp1[i].value;
        // }
        (vals, sum)
    }

    // returns which set number is in e.g. number 3 is in set number 2(sets are indexed from 0)
    fn find(&self, num: usize) -> usize {
        let mut tmp = self.parent[num - 1];
        if tmp != num - 1 {
            tmp = self.find(tmp + 1);
        }
        tmp
    }
}

// ver 3
#[derive(Debug)]
pub struct ThUnion {
    sizes: Vec<usize>,
    sums: Vec<usize>,
    parent: Vec<usize>,
    size: usize,
}

impl ThUnion {
    pub fn new(size: usize) -> ThUnion {
        let sizes: Vec<usize> = vec![1; size];
        let parent: Vec<usize> = (0..size).map(|v| v).collect();
        let sums: Vec<usize> = (0..size).map(|v| v + 1).collect();

        ThUnion {
            sizes,
            sums,
            parent,
            size,
        }
    }

    // union the sets that contain num1 and num2
    pub fn union(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);
        if set1 == set2 {
            return;
        }

        // eprintln!("set1: {}, set2: {}", set1, set2);
        self.sizes[set1] += self.sizes[set2];
        self.sums[set1] += self.sums[set2];

        self.parent[set2] = set1;

        self.sizes[set2] = 0;
        self.sums[set2] = 0;

        // eprintln!("merged: {:?}", self);
    }

    // move num1 to set containing num2
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);
        if set1 == set2 {
            return;
        }

        // eprintln!("set1: {}, set2: {}", set1, set2);
        self.sizes[set1] -= 1;
        self.sizes[set2] += 1;

        self.sums[set1] -= num1;
        self.sums[set2] += num1;

        self.parent[num1 - 1] = set2;

        if set1 == num1 - 1 {
            // if num1 is the parent of the set
            let mut new_par: usize = 0;
            let mut found = false;
            for i in 0..self.size {
                if self.parent[i] == set1 {
                    // find the first kid of the set
                    if !found {
                        // if we havent found the new parent yet
                        found = true;
                        new_par = i;

                        self.sums[i] = self.sums[set1];
                        self.sizes[i] = self.sizes[set1];
                    }

                    self.parent[i] = new_par;

                    // eprintln!("num1: {}, kid {}, set1: {}", num1, i, set1);
                }
            }
            self.sizes[set1] = 0;
            self.sums[set1] = 0;
        }

        // eprintln!("moved: {:?}", self);
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&mut self, num: usize) -> (usize, usize) {
        let parent = self.find(num);

        (self.sizes[parent], self.sums[parent])
    }

    // returns parent by finding the root.
    fn find(&self, num: usize) -> usize {
        let mut tmp = self.parent[num - 1];
        if tmp != num - 1 {
            tmp = self.find(tmp + 1);
        }
        tmp
    }
}

impl fmt::Display for ThUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sizes: {:?}\nparent: {:?}\nsums: {:?}\nsize: {}\n",
            self.sizes, self.sums, self.parent, self.size
        )
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
