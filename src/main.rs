use std::collections::hash_map::HashMap;
use std::{char, env};

type Atom = char;
// type Molecule = HashMap<Atom, u64>;
#[derive(Debug)]
struct Molecule {
    hashmap: HashMap<Atom, u64>,
    coefficient: u64,
}

impl Molecule {
    fn set_coefficient(&mut self, coefficient: u64) {
        // Must be used every time the coefficient is set to a value other than 1.
        self.coefficient = coefficient;
        for (i, atom) in self.hashmap.iter().enumerate() {
            let hmap = &self.hashmap;
            dbg!(atom);
        }
    }
}

impl PartialEq for Molecule {
    fn eq(&self, other: &Self) -> bool {
        other.hashmap == self.hashmap
    }

    fn ne(&self, other: &Self) -> bool {
        other.hashmap != self.hashmap
    }
}
type MoleculeSet = Vec<Molecule>;
type Equation = (MoleculeSet, MoleculeSet);

#[cfg(test)]
mod tests;
fn parse_input(args: &Vec<String>) -> Equation {
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

    (
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
                *mutable_reagent_sum = *atom.1;
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

    // dbg!("Products", &product_sum);
    // dbg!("Reagents", &reagent_sum);
    dbg!("Equality", product_sum.hashmap == reagent_sum.hashmap);
    product_sum.hashmap == reagent_sum.hashmap
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let input: Equation = parse_input(&args);

    while !is_balanced(&input) {
        // Balance input
    }
}
