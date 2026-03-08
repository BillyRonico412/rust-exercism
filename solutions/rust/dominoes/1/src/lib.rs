pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    fn chain_rec(rest: &mut Vec<(u8, u8)>, stack: &mut Vec<(u8, u8)>) -> bool {
        if rest.len() == 0 {
            match (stack.first(), stack.last()) {
                (Some(first), Some(last)) => first.0 == last.1,
                (None, None) => true,
                _ => unreachable!(),
            }
        } else {
            match stack.last().cloned() {
                None => {
                    let removed_domino = rest.pop().unwrap();
                    stack.push(removed_domino);
                    let res = chain_rec(rest, stack);
                    if !res {
                        stack.pop();
                        rest.push(removed_domino);
                    }
                    res
                }
                Some((_, l)) => rest
                    .clone()
                    .iter()
                    .enumerate()
                    .filter_map(|(i, d)| match (l == d.0, l == d.1) {
                        (false, false) => None,
                        (true, _) => Some((i, (d.0, d.1))),
                        _ => Some((i, (d.1, d.0))),
                    })
                    .any(|(i, d)| {
                        let removed_domino = rest.remove(i);
                        stack.push(d);
                        let res = chain_rec(rest, stack);
                        if !res {
                            stack.pop();
                            rest.insert(i, removed_domino);
                        }
                        res
                    }),
            }
        }
    }
    let mut rest = input.iter().copied().collect::<Vec<_>>();
    let mut stack = Vec::with_capacity(input.len());
    match chain_rec(&mut rest, &mut stack) {
        false => None,
        true => Some(stack),
    }
}
