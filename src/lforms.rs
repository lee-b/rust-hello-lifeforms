use std::rand;
use std::rand::Rng;

type Fitness = f64;

#[deriving(PartialEq,Clone,Show)]
struct Individual {
    genes : u32
}

struct Population<'a> {
    individuals : Vec<Individual>
}

impl<'a> Population<'a> {
    fn new<R: Rng>(pop_size: u32, rng: &mut R) -> Population {
        let mut indivs = Vec::new();

        for _ in range(0, pop_size) {
            let indiv = Individual { genes: rng.gen_range(1, 32) };
            indivs.push(indiv);
        }

        Population { individuals: indivs }
    }
}

struct Evaluator;

impl Evaluator {
    fn evaluate(&self,  indiv: &Individual) -> Fitness {
        ((indiv.genes as f64) / 1_000_000.0f64)
    }
}

struct Evolver;


fn select_indiv<'a, R: Rng>(pop: &'a [Individual], rng: &mut R) -> &'a Individual {
    rng.choose(pop).expect("select_indiv(): rng.choose() returned None!")
}

fn naturally_select_indiv<'a, R: Rng>(pop: &'a Population, rng: &mut R, eval: &mut Evaluator) -> &'a Individual {
    let pop_slice = pop.individuals.as_slice();

    let suitor1 = select_indiv(pop_slice, rng);
    let suitor2 = select_indiv(pop_slice, rng);

    let suitor1_fitness = eval.evaluate(suitor1);
    let suitor2_fitness = eval.evaluate(suitor2);

    if suitor1_fitness >= suitor2_fitness {
        suitor1
    } else {
        suitor2
    }
}

fn select_parents<'a, R: Rng>(pop: &'a Population, rng: &mut R, eval: &mut Evaluator) -> (&'a Individual, &'a Individual) {
    assert!(pop.individuals.len() > 1);

    // FIXME: this doesn't avoid mom and dad being the same Individual
    let mom = naturally_select_indiv(pop, rng, eval);
    let dad = naturally_select_indiv(pop, rng, eval);

    return (mom, dad)
}

impl Evolver {
    fn evolve<'a, R: Rng>(&self, old_pop: &Population, rng : &mut R, eval: &mut Evaluator) -> Population {
        let mut new_indivs = Vec::new();
        let pop_size = old_pop.individuals.len();

        assert!(pop_size >= 2);
 
        for _child_num in range(1, pop_size+1) {
//            println!("    Evolving individual {} of {}", child_num, pop_size);
            let (mom, dad) = select_parents(old_pop, rng, eval);
//            println!("Selected parents: {} and {}", mom, dad);

            let child = breed(mom, dad);
            new_indivs.push(child);
        }

        Population { individuals: new_indivs }
    }
}

fn breed(mom : &Individual, dad: &Individual) -> Individual {
    Individual { genes: ((mom.genes + dad.genes) as f64 * 1.2) as u32 }
}

fn main() {
    println!("Building population...");

    let mut rng = rand::task_rng();

    let mut pop : Population = Population::new(1_000_000, &mut rng);
    let evolver = Evolver;
    let mut evaluator = Evaluator;
    let mut peak_fitness = 0f64 as Fitness;

    loop {
        let mut best_fitness : Fitness = 0.0f64;
        for indiv in pop.individuals.iter() {
            let fitness = evaluator.evaluate(indiv);

            if fitness > best_fitness {
                best_fitness = fitness;
            }
        }

        if best_fitness > peak_fitness {
            peak_fitness = best_fitness;
        }

        if peak_fitness > 90f64 {
            println!("{}% fitness achieved.  Good enough.", peak_fitness);
            break
        } else {
            println!("{}% fitness achieved.  Insufficient.  Evolving population...", peak_fitness);
            let new_pop = evolver.evolve(&pop, &mut rng, &mut evaluator);
            pop = new_pop;
        }
    }
}

