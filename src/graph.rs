
struct Node {}
struct Edge {}
struct Graph {
	nodes: Vec<Node>,
	edges: Vec<Edge>,
}

impl Graph {
	pub fn Empty() -> Self {
		Self{
			nodes: vec![],
			edges: vec![],
		}
	}

	pub fn Clique(n: u32) -> Self {
		Self::Empty()
	}

	pub fn Grid(n: u32) -> Self {
		Self::Empty()
	}

	pub fn Torus(n: u32) -> Self {
		Self::Empty()
	}
}
