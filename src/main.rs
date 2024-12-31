mod scanner;
mod smiles;
mod types;

use petgraph::{graph::NodeIndex, Graph};
use scanner::Scanner;
use types::{Atom, Bond, Mol};

pub struct SmilesParser {}

impl SmilesParser {
    pub fn parse(&self, smiles: &str) -> Mol {
        let mut scanner = Scanner::new(smiles);

        let mut graph = Graph::<Atom, Bond>::new();

        let mut last_node_index: Option<NodeIndex> = None;

        let mut is_in_ring = false;
        let mut ring_start: Option<NodeIndex> = None;

        while !scanner.is_done() {
            let c = scanner.pop();

            if let Some(c) = c {
                match &c {
                    'c' => {
                        let atom: Atom = Atom { number: 6 };
                        let node_index = graph.add_node(atom);
                        if let Some(last) = last_node_index {
                            let bond = Bond {
                                atom_start: last.index(),
                                atom_end: node_index.index(),
                            };
                            graph.add_edge(last, node_index, bond);
                        }
                        last_node_index = Some(node_index.clone());
                    }
                    '1' => {
                        if !is_in_ring {
                            ring_start = last_node_index;
                            is_in_ring = true;
                        } else {
                            // finishing the ring
                            let ring_start = ring_start.unwrap(); // finishing a ring, so must have been started
                            let last_node_index = last_node_index.unwrap(); // finishing a ring, so there must be at least a node before
                            let bond = Bond {
                                atom_start: ring_start.index(),
                                atom_end: last_node_index.index(),
                            };
                            graph.add_edge(ring_start, last_node_index, bond);
                        }
                    }
                    _ => {}
                }
            }
        }

        let mol = Mol { graph };

        mol
    }
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
}
