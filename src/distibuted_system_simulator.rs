use log::{info, log, warn};
use crate::distributed_process::DistributedProcessAtomaton;
use crate::graph::Graph;
use crate::discrete_event::DiscreteEventSystem;

pub struct DistributedSystemSimulator<S, M> {
	graph: Graph,
	process: Box<dyn DistributedProcessAtomaton<S, M>>,
	event_system: DiscreteEventSystem,
}

impl<S, M> DistributedSystemSimulator<S, M> {
	pub fn new(graph: Graph, automaton: Box<dyn DistributedProcessAtomaton<S, M>>) -> Self {
		info!("Creating a DistributedSystemSimulator with network graph : \n{}", graph);
		Self {
			graph,
			process: automaton,
			event_system: DiscreteEventSystem::new()
		}
	}

	pub fn start(&mut self) {
		self.event_system.execute_all_events()
	}
}