use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

#[derive(PartialEq)]
struct Event {
	node_id: usize,
	time: f64,
	//callback: dyn FnOnce()
}

impl Eq for Event {}

impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		other.time.partial_cmp(&self.time).unwrap_or(other.node_id.cmp(&self.node_id))
	}
}

impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		other.time.partial_cmp(&self.time)
	}
}


pub(crate) struct DiscreteEventSystem {
	priority_queue: BinaryHeap<Box<Event>>,
	current_time: f64,
}

impl DiscreteEventSystem {
	pub fn new() -> Self {
		Self {
			current_time: 0_f64,
			priority_queue: BinaryHeap::new(),
		}
	}

	pub fn next_event(&mut self) {
		if self.priority_queue.is_empty() { return; }
	}

	pub fn execute_events_to_current_time(&mut self) {
		while self.priority_queue.peek()
			.is_some_and(|x| x.time < self.current_time)
		{
			//self.priority_queue.pop()?.callback();
		}
	}

	pub fn execute_all_events(&mut self) {
		while let Some(event) = self.priority_queue.peek()
		{
			//event.callback(/* Callback data */);
		}
	}

	pub fn exectute_events_in_time_step(&mut self, dt: f64) {
		self.current_time += dt;
		self.execute_events();
	}

	pub fn execute_events(&mut self) {
		todo!()
	}

	pub fn insert_event(&mut self, event: Box<Event>) {

	}
}