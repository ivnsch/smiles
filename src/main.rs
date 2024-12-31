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

    use crate::types::Atom;

    use super::SmilesParser;

    #[test]
    fn parse_c() {
        let parser = SmilesParser {};
        let mol = parser.parse("ccc");

        assert_eq!(3, mol.num_atoms());
        assert_eq!(2, mol.num_bonds());
        assert_eq!(Some(&Atom { number: 6 }), mol.atom_with_idx(0));

        // assert!(mol.gr)
    }
}
