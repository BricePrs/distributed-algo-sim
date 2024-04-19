use std::collections::HashSet;
use std::fmt::{Formatter, write};

type NodeID = u32; // TODO: better way ? All function takes u32 as input too ...

pub struct Node {
	neighbors: HashSet<NodeID> // todo: Hashset ??
}

pub struct Graph {
	nodes: Vec<Node>,
}

impl Graph {
	pub fn add_node(&mut self, neighbors: Option<Vec<NodeID>>) -> NodeID {
		self.nodes.push(Node{ neighbors: neighbors.unwrap_or(vec![]).into_iter().collect::<HashSet<NodeID>>() });
		(self.nodes.len()-1) as NodeID
	}

	pub fn connect_nodes(&mut self, a: NodeID, b: NodeID) {
		self.nodes[a as usize].neighbors.insert(b);
		self.nodes[b as usize].neighbors.insert(a);
	}

	pub fn connect_to_all(&mut self, node_index: NodeID) {
		for index in 0..self.nodes.len() {
			if index != node_index as usize {
				self.connect_nodes(node_index, index as NodeID);
			}
		}
	}

	pub fn empty() -> Self {
		Self{
			nodes: vec![],
		}
	}

	pub fn clique(n: u32) -> Self {
		let mut graph = Self::empty();
		for _ in 0..n {
			let node = graph.add_node(None);
			graph.connect_to_all(node);
		}
		graph
	}

	pub fn grid(n: u32) -> Self {
		Self::empty()
	}

	pub fn torus(n: u32) -> Self {
		Self::empty()
	}
}


impl std::fmt::Display for Graph {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for (nodeId, node) in self.nodes.iter().enumerate() {
			write!(f, "{:#?} {:?}\n", nodeId, node.neighbors);
		}
		Ok(())
	}
}