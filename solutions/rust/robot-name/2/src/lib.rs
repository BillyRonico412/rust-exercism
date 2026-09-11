use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rand::{Rng, RngExt};

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.

type NameSet = Rc<RefCell<HashSet<String>>>;

fn gen_random_name<R: Rng>(rng: &mut R) -> String {
    format!(
        "{}{}{}{}{}",
        rng.random_range('A'..='Z'),
        rng.random_range('A'..='Z'),
        rng.random_range(0..=9),
        rng.random_range(0..=9),
        rng.random_range(0..=9)
    )
}

pub struct RobotFactory {
    name_used_set: NameSet,
}

pub struct Robot {
    name: String,
    name_used_set: NameSet,
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory {
            name_used_set: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let mut new_robot = Robot {
            name: String::new(),
            name_used_set: self.name_used_set.clone(),
        };
        new_robot.reset(rng);
        new_robot
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        let new_name = gen_random_name(rng);

        self.name_used_set.borrow_mut().remove(self.name());

        if self.name_used_set.borrow().contains(&new_name) {
            self.reset(rng);
        } else {
            let mut name_used_set = self.name_used_set.borrow_mut();
            self.name = new_name.clone();
            name_used_set.insert(new_name.clone());
        }
    }
}
