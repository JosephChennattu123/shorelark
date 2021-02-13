use crate::*;

/// Roulette-wheel selection via stochastic acceptance
#[derive(Clone, Debug)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionPolicy for RouletteWheelSelection {
    fn select<'a, I: Individual>(&self, population: &'a [I], rng: &mut dyn RngCore) -> &'a I {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let individuals: Vec<_> = (0..1000)
            .map(|_| RouletteWheelSelection::new().select(&population, &mut rng))
            .collect();

        let actual: BTreeMap<_, _> = (1..=4)
            .map(|individual_fitness| {
                let individual_count = individuals
                    .iter()
                    .filter(|individual| individual.fitness() as usize == individual_fitness)
                    .count();

                (individual_fitness, individual_count)
            })
            .collect();

        let expected = maplit::btreemap! {
            // individual's fitness => how many times this individual has been chosen
            1 => 98,
            2 => 202,
            3 => 278,
            4 => 422,
        };

        assert_eq!(actual, expected);
    }
}
