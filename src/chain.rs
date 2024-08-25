use crate::circle::*;

pub struct Chain {
    pub circles: Vec<Circle>,
}

impl Chain {
    pub fn new(number: usize) -> Self {
        let circles = vec![Circle::default(); number]
            .into_iter()
            .map(|circle| circle.randomize_position())
            .collect();
        Self { circles }
    }
}
