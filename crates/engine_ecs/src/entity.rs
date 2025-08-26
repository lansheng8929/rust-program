#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Entity {
    id: u32,
    alive: bool,
}

impl Entity {
	pub fn new() -> Self {
		Entity {
			id: Entity::next_id(),
			alive: true
		}
	}

	fn next_id() -> u32 {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

	pub fn is_alive(&self) -> bool {
		self.alive
	}

	pub fn invalid(&mut self) {
		self.alive = false;
	}

	pub fn reset(&mut self) {
		self.alive = true;
	}
}
