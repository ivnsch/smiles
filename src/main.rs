mod scanner;
mod smiles;
mod types;

use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};
use scanner::Scanner;
use types::{Atom, Bond, Mol};

pub struct SmilesParser {}

impl SmilesParser {
    pub fn parse(&self, smiles: &str) -> Mol {
        let mut scanner = Scanner::new(smiles);

        let mut graph = Graph::<Atom, Bond>::new();

        let mut last_node_index: Option<NodeIndex> = None;

        // let mut is_in_ring = false;
        // let mut ring_start: Option<NodeIndex> = None;
        let mut rings: HashMap<char, NodeIndex> = HashMap::new();

        let mut branches_stack: Vec<NodeIndex> = vec![];

        while !scanner.is_done() {
            let c = scanner.pop();

            if let Some(c) = c {
                match &c {
                    'c' | 'f' => {
                        let number = atom_number(c);
                        let node_index = add_to_graph(&mut graph, number, last_node_index);
                        last_node_index = Some(node_index.clone());
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        if !rings.contains_key(c) {
                            // a ring starts
                            rings.insert(*c, last_node_index.unwrap()); // unwrap: smiles can't start with ring number (there's always a last node)
                        } else {
                            // ring ends
                            let ring_start = rings.get(c).unwrap(); // unwrap: finishing a ring, so must have been started
                            let ring_end = last_node_index.unwrap(); // unwrap: finishing a ring, so there must be at least a node before
                            let bond = Bond {
                                atom_start: ring_start.index(),
                                atom_end: ring_end.index(),
                            };
                            graph.add_edge(*ring_start, ring_end, bond);
                            rings.remove(c);
                        }
                    }
                    '(' => {
                        branches_stack.push(last_node_index.unwrap()); // unwrap: smiles can't start with a branch (there's always a last node)
                    }
                    ')' => {
                        let last_index_before_branch = branches_stack.pop();
                        // replace current last node index (in branch) with index before branch
                        last_node_index = last_index_before_branch;
                    }
                    _ => {}
                }
            }
        }

        let mol = Mol { graph };

        mol
    }
}

fn atom_number(char: &char) -> u32 {
    match char {
        'c' => 6,
        'f' => 9,
        _ => panic!("not supported: {}", char),
    }
}

fn add_to_graph(
    graph: &mut Graph<Atom, Bond>,
    atom_number: u32,
    last_node_index: Option<NodeIndex>,
) -> NodeIndex {
    let atom: Atom = Atom {
        number: atom_number,
    };
    let node_index = graph.add_node(atom);
    if let Some(last) = last_node_index {
        let bond = Bond {
            atom_start: last.index(),
            atom_end: node_index.index(),
        };
        graph.add_edge(last, node_index, bond);
    }
    node_index
}

pub fn string(string: &str) -> bool {
    let mut scanner = Scanner::new(string);

    loop {
        if !unit(&mut scanner) {
            break;
        }
    }

    scanner.cursor() > 0 && scanner.is_done()
}

fn unit(scanner: &mut Scanner) -> bool {
    scanner.take(&'*')
}

#[cfg(test)]
mod test {

    use crate::types::{Atom, Bond};

    use super::SmilesParser;

    #[test]
    fn parse_ccc() {
        let parser = SmilesParser {};
        let mol = parser.parse("ccc");

        assert_eq!(3, mol.num_atoms());
        assert_eq!(2, mol.num_bonds());
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(0));
    }

    #[test]
    fn parse_cyclopentane() {
        let parser = SmilesParser {};
        let mol = parser.parse("c1cccc1");

        assert_eq!(5, mol.num_atoms());
        assert_eq!(5, mol.num_bonds());
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(0));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(1));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(2));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(3));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(4));

        assert_eq!(
            Some(&Bond {
                atom_start: 0,
                atom_end: 1
            }),
            mol.bond_with_idx(0)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 1,
                atom_end: 2
            }),
            mol.bond_with_idx(1)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 2,
                atom_end: 3
            }),
            mol.bond_with_idx(2)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 3,
                atom_end: 4
            }),
            mol.bond_with_idx(3)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 0,
                atom_end: 4
            }),
            mol.bond_with_idx(4)
        );
    }

    #[test]
    fn parse_bicyclohexyl() {
        let parser = SmilesParser {};
        let mol = parser.parse("c1ccccc1c2ccccc2");

        assert_eq!(12, mol.num_atoms());
        assert_eq!(13, mol.num_bonds());
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(0));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(1));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(2));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(3));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(4));

        assert_eq!(
            Some(&Bond {
                atom_start: 0,
                atom_end: 1
            }),
            mol.bond_with_idx(0)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 1,
                atom_end: 2
            }),
            mol.bond_with_idx(1)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 2,
                atom_end: 3
            }),
            mol.bond_with_idx(2)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 3,
                atom_end: 4
            }),
            mol.bond_with_idx(3)
        );

        assert_eq!(
            Some(&Bond {
                atom_start: 4,
                atom_end: 5
            }),
            mol.bond_with_idx(4)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 0,
                atom_end: 5
            }),
            mol.bond_with_idx(5)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 5,
                atom_end: 6
            }),
            mol.bond_with_idx(6)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 6,
                atom_end: 7
            }),
            mol.bond_with_idx(7)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 7,
                atom_end: 8
            }),
            mol.bond_with_idx(8)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 8,
                atom_end: 9
            }),
            mol.bond_with_idx(9)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 9,
                atom_end: 10
            }),
            mol.bond_with_idx(10)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 10,
                atom_end: 11
            }),
            mol.bond_with_idx(11)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 6,
                atom_end: 11
            }),
            mol.bond_with_idx(12)
        );
    }

    #[test]
    fn parse_fluoroform() {
        let parser = SmilesParser {};
        let mol = parser.parse("fc(f)f");

        assert_eq!(4, mol.num_atoms());
        assert_eq!(3, mol.num_bonds());
        assert_eq!(Some(&Atom { number: 9 }), mol.atom_with_idx(0));
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(1));
        assert_eq!(Some(&Atom { number: 9 }), mol.atom_with_idx(2));
        assert_eq!(Some(&Atom { number: 9 }), mol.atom_with_idx(3));

        assert_eq!(
            Some(&Bond {
                atom_start: 0,
                atom_end: 1
            }),
            mol.bond_with_idx(0)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 1,
                atom_end: 2
            }),
            mol.bond_with_idx(1)
        );
        assert_eq!(
            Some(&Bond {
                atom_start: 1,
                atom_end: 3
            }),
            mol.bond_with_idx(2)
        );
    }
}
