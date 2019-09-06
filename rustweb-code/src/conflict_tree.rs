#[derive(Clone, Debug)]
pub struct ConflictTree<Item> {
    pub items: Vec<(Item, usize)>,
    pub children: Vec<Self>,
    sum_of_children: usize,
}

impl<Item: Copy> ConflictTree<Item> {
    pub fn new() -> Self {
        ConflictTree {
            items: vec![],
            children: vec![],
            sum_of_children: 0,
        }
    }

    pub fn add_item(&mut self, item: Item, weight: usize) {
        self.items.push((item, weight));
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn append(&mut self, other: Self) {
        self.items.extend(other.items.into_iter());
        self.children.extend(other.children.into_iter());
    }

    pub fn compute_sum_of_max(&mut self) -> usize {
        let mut sum = 0;
        for ch in &mut self.children {
            sum += ch.compute_sum_of_max();
        }

        self.sum_of_children = sum;

        let max_self = self.items.iter().map(|&(_, count)| count).max().unwrap_or(0);

        sum.max(max_self)
    }

    pub fn assign_best(&mut self, result: &mut Vec<Item>) -> bool {
        self.assign_best_(result);
        self.items.len() > 0 || self.children.len() > 0
    }

    pub fn assign_best_(&mut self, result: &mut Vec<Item>) -> usize {
        let mut best = -1;
        let mut second_best_count = 0;
        let mut best_count = 0;

        for (i, item) in self.items.iter().enumerate() {
            if item.1 >= best_count {
                //println!("{} >= {}", item.1, best_count);
                second_best_count = best_count;
                //dbg!(second_best_count);
                best = i as i32;
                //dbg!(best);
                best_count = item.1;
            } else if item.1 > second_best_count {
                second_best_count = item.1;
            }
        }

        let max_self;
        if best_count <= self.sum_of_children {
            // best in children
            let mut new_sum_of_children = 0;

            self.children.drain_filter(|child| {
                let child_max = child.assign_best_(result);
                new_sum_of_children += child_max;
                child_max == 0
            });

            self.sum_of_children = new_sum_of_children;
            max_self = best_count;
        } else {
            // assign 'id' to 'best'
            let item_id = self.items[best as usize].0;
            //println!("assigned {} to {}", id, item_id);
            result.push(item_id);
            self.items.swap_remove(best as usize);
            max_self = second_best_count;
        }

        self.sum_of_children.max(max_self)
    }
}