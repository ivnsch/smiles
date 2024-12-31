use petgraph::{graph::NodeIndex, Graph};

#[derive(Debug, PartialEq, Eq)]
pub struct Atom {
    pub number: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bond {
    pub atom_start: usize, // graph index
    pub atom_end: usize,   // graph index
}

#[derive(Debug)]
pub struct Mol {
    pub graph: Graph<Atom, Bond>,
}

impl Mol {
    pub fn num_atoms(&self) -> usize {
        self.graph.node_count()
    }

    pub fn num_bonds(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn atom_with_idx(&self, idx: usize) -> Option<&Atom> {
        self.graph.node_weight(NodeIndex::new(idx))
    }
}
