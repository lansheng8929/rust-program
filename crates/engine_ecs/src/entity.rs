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

	/// 从原始 ID 创建 Entity（用于与 ECS 系统的 usize entity_id 兼容）
	pub fn from_raw(id: usize) -> Self {
		Entity {
			id: id as u32,
			alive: true,
		}
	}

	/// 获取原始 ID（作为 usize）
	pub fn to_raw(&self) -> usize {
		self.id as usize
	}

}
