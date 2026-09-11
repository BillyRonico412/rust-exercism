#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

struct BucketItem {
    capacity: u8,
    value: u8,
    bucket: Bucket,
}

impl BucketItem {
    fn new(bucket: Bucket, capacity: u8) -> Self {
        Self {
            bucket,
            capacity,
            value: 0,
        }
    }

    fn fill(&mut self) {
        self.value = self.capacity
    }

    fn empty(&mut self) {
        self.value = 0;
    }

    fn poor(&mut self, target_bucket: &mut Self) {
        let value_poor = {
            let free_place_in_target_bucket = target_bucket.capacity - target_bucket.value;
            if self.value <= free_place_in_target_bucket {
                self.value
            } else {
                free_place_in_target_bucket
            }
        };
        target_bucket.value += value_poor;
        self.value -= value_poor;
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }

    fn is_filled(&self) -> bool {
        self.value == self.capacity
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }

    let mut bucket_item_1 = BucketItem::new(Bucket::One, capacity_1);
    let mut bucket_item_2 = BucketItem::new(Bucket::Two, capacity_2);
    let mut seq: Vec<(u8, u8)> = vec![];

    let mut moves = 0;
    let (start_bucket_item, other_bucket_item) = match start_bucket {
        Bucket::One => (&mut bucket_item_1, &mut bucket_item_2),
        Bucket::Two => (&mut bucket_item_2, &mut bucket_item_1),
    };
    loop {
        let current_seq = (start_bucket_item.value, other_bucket_item.value);
        if seq.contains(&current_seq) {
            break None;
        }
        seq.push(current_seq);

        let bucket_stats = [
            (&start_bucket_item, &other_bucket_item),
            (&other_bucket_item, &start_bucket_item),
        ]
        .iter()
        .find_map(|(bucket_item_1, bucket_item_2)| {
            (bucket_item_1.value == goal).then_some(BucketStats {
                goal_bucket: bucket_item_1.bucket.clone(),
                moves: moves,
                other_bucket: bucket_item_2.value,
            })
        });

        if bucket_stats.is_some() {
            break bucket_stats;
        }

        if start_bucket_item.is_empty() {
            start_bucket_item.fill();
        } else if other_bucket_item.capacity == goal {
            other_bucket_item.fill();
        } else if other_bucket_item.is_filled() {
            other_bucket_item.empty();
        } else {
            start_bucket_item.poor(other_bucket_item);
        }

        moves += 1;
    }
}
