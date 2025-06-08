use std::collections::hash_map::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::{char, env};

#[cfg(test)]
mod tests;

const ATTEMPTS: u64 = 10;
type Atom = char;
#[derive(Debug, PartialEq)]
struct Molecule {
    hashmap: HashMap<Atom, u64>,
    coefficient: u64,
}

impl Molecule {
    fn set_coefficient(&mut self, coefficient: u64) {
        // Must be used every time the coefficient is set to a value other than 1.
        for atom in self.hashmap.values_mut() {
            dbg!(self.coefficient);
            *atom = *atom / self.coefficient * coefficient;
        }
        self.coefficient = coefficient;
        dbg!(self.coefficient);
    }
}

type MoleculeSet = Vec<Molecule>;

#[derive(PartialEq, Debug)]
struct Equation(MoleculeSet, MoleculeSet);
impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for molecule in &self.0 {
            for atom in &molecule.hashmap {
                output.push_str(String::from(atom.1).);
            }
        }
        write!(f, "{output:#?}")
    }
}

fn parse_input(args: &Vec<String>) -> Equation {
    // TODO: Add compatibility for more than one digit or more than one letter in the element
    let mut products: MoleculeSet = vec![];
    let mut reagents: Option<MoleculeSet> = None;

    for molecule in args {
        if (&molecule).chars().all(char::is_alphanumeric) {
            let mut atoms = Molecule {
                coefficient: 1,
                hashmap: HashMap::new(),
            };
            for (i, element) in molecule.chars().enumerate() {
                if char::is_numeric(element) {
                    continue;
                } else {
                    let mut freq = 1;
                    if i < molecule.as_bytes().len() - 1 {
                        if let Ok(str) = String::from_utf8(vec![molecule.as_bytes()[i + 1]])
                            .unwrap()
                            .parse::<u64>()
                        {
                            freq = str
                        }
                    }
                    atoms.hashmap.insert(element, freq);
                }
            }
            products.push(atoms);
        } else {
            reagents = Some(products);
            products = vec![];
            continue;
        }
    }

    Equation(
        reagents.expect("There were no products in the equation"),
        products,
    ) // Products are moved into reagents after the arrow -> so if there are no products, reagents will be None
}

fn is_balanced(input: &Equation) -> bool {
    let mut reagent_sum = Molecule {
        coefficient: 1,
        hashmap: HashMap::new(),
    };
    let mut product_sum = Molecule {
        coefficient: 1,
        hashmap: HashMap::new(),
    };

    let reagents = &input.0;
    let products = &input.1;
    for reagent in reagents {
        for atom in &reagent.hashmap {
            if let Some(mutable_reagent_sum) = reagent_sum.hashmap.get_mut(&atom.0) {
                *mutable_reagent_sum += *atom.1;
            } else {
                reagent_sum.hashmap.insert(*atom.0, *atom.1);
            };
        }
    }

    for product in products {
        for atom in &product.hashmap {
            if let Some(mutable_product_sum) = product_sum.hashmap.get_mut(&atom.0) {
                *mutable_product_sum += *atom.1;
            } else {
                product_sum.hashmap.insert(*atom.0, *atom.1);
            };
        }
    }

    dbg!("Equality", product_sum.hashmap == reagent_sum.hashmap);
    product_sum.hashmap == reagent_sum.hashmap
}

fn balance(equation: &mut Equation, start_index: usize) -> &mut Equation {
    dbg!(&equation);
    // Assumes the input equation is unbalanced - may cause unnecessary modification if it is already balanced.
    for i in 1..ATTEMPTS {
        if is_balanced(&equation) {
            return equation;
        }
        let mut j = start_index;
        let equation_side = if start_index < equation.0.len() {
            &mut equation.0
        } else if start_index < equation.0.len() + equation.1.len() {
            equation.0.iter_mut().for_each(|x| x.set_coefficient(1));
            j = start_index - equation.0.len();
            &mut equation.1
        } else {
            panic!("Unable to balance equation at this time. (Exceeded bounds)");
        };

        equation_side[j].set_coefficient(i);
    }

    sleep(Duration::from_millis(1000)); // For debugging purposes
    balance(equation, start_index + 1)
}
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut equation: Equation = parse_input(&args);
    let i = 0;

    while !is_balanced(&input) {

        // Balance input
    }

    /* while !is_balanced(&equation) {
        println!("Unbalanced equation! The equation is {equation:?}");
        if i < equation.0.len() {
            let coeff = equation.0[i].coefficient;
            equation.0[i].set_coefficient(coeff + 1);
        } else if i < equation.1.len() + equation.0.len() {
            let j = i - equation.0.len();
            let coeff = equation.1[j].coefficient;
            equation.1[j].set_coefficient(coeff + 1);
        } else {
            panic!("Couldn't balance this equation at this time.\nThe equation is {equation:?}");
        }
        i += 1;
    } */
    println!("Balanced equation: {}", (&equation));
}
