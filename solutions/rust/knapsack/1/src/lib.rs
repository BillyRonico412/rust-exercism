#[derive(Debug, Default)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let len = items.len() as u32;
    (0..2u32.pow(len))
        .map(|n| {
            (0..len)
                .filter_map(|b| ((1 << b) & n != 0).then(|| &items[b as usize]))
                .fold(Item::default(), |acc, item| Item {
                    weight: acc.weight + item.weight,
                    value: acc.value + item.value,
                })
        })
        .fold(0, |acc, item| {
            if item.weight > max_weight {
                acc
            } else if item.value < acc {
                acc
            } else {
                item.value
            }
        })
}
