use rand::distributions::Distribution;
use rand::distributions::ChiSquared;

use rsgenetic::pheno::*;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;

use std::fmt;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct MyFitness {
    value: i32,
}

impl Fitness for MyFitness {
    // The zero value for our custom type
    fn zero() -> MyFitness {
        MyFitness { value: 0 }
    }

    // The absolute difference between two instances
    fn abs_diff(&self, other: &MyFitness) -> MyFitness {
        MyFitness {
            value: (self.value - other.value).abs()
        }
    }
}

const TARGET: i32 = 100;

#[derive(Copy, Clone)]
struct MyPheno {
    x: i32,
    y: i32,
}

impl Phenotype<i32> for MyPheno {
    // How fit is this individual?
    fn fitness(&self) -> i32 {
        TARGET - (self.x + self.y)
    }

    // Have two individuals create a new individual
    fn crossover(&self, other: &MyPheno) -> MyPheno {
        MyPheno {
            x: self.x,
            y: other.y,
        }
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> MyPheno {
        MyPheno {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}

impl fmt::Debug for MyPheno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
    // let mut r = rand::thread_rng();
    // let n = Normal::new(0.0, 1.0).unwrap();
    // for _ in 0..10 {
    //     println!("{}", n.sample(&mut r));
    // }

    #[allow(deprecated)]
    let mut population = (0..100).map(|i| MyPheno { x: i, y: 100 - i }).collect();
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(StochasticSelector::new(10)))
                    .set_max_iters(50)
                    .build();
    s.run();
    let result = s.get().unwrap(); // The best individual
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
    println!("Result: {:?} | Fitness: {}.", result, result.fitness());

    let chi = ChiSquared::new(11.0);
    let v = chi.sample(&mut rand::thread_rng());
    println!("{} is from a χ²(11) distribution", v)

}