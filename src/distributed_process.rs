
pub trait DistributedProcessAtomaton<S, M> {
	fn recv_msg(&mut self, current_state: S, msg: M) -> S;
	fn send_msg(&mut self, current_state: S, msg: M) -> S;
}

