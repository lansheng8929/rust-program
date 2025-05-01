use ecs_rust::{
    component::Component,
    entity_manager::{ EntityIdAccessor, EntityManager },
    system::System,
};

use crate::{ transform::Transform, WINDOW_HEIGHT, WINDOW_WIDTH };

#[derive(Debug)]
pub struct CollisionBox {
    pub width: u32,
    pub height: u32,
    pub is_trigger: bool, // 可选：用于确定是否只是触发事件而不进行物理碰撞
}

impl Component for CollisionBox {}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let ids = accessor.borrow_ids_for_pair::<Transform, CollisionBox>(manager).unwrap();
        for id in ids.iter() {
            {
                let collision_box = manager.borrow_component::<CollisionBox>(*id).unwrap();
                let (width, height) = (collision_box.width, collision_box.height);

                let transform_mut = manager.borrow_component_mut::<Transform>(*id).unwrap();
                let (x, y, vx, vy) = (
                    transform_mut.position.0,
                    transform_mut.position.1,
                    transform_mut.velocity.0,
                    transform_mut.velocity.1,
                );

                // 检查与墙壁的碰撞
                if x < 0 || x + width > WINDOW_WIDTH {
                    transform_mut.velocity.0 = -vx; // 反转水平速度
                }
                if y < 0 || y + height > WINDOW_HEIGHT {
                    transform_mut.velocity.1 = -vy; // 反转垂直速度
                }

                // 使用复制的数据进行其他操作
                // for other_id in ids.iter() {
                //     if id != other_id {
                //         let other_collision_box = manager
                //             .borrow_component::<CollisionBox>(*other_id)
                //             .unwrap();
                //         let (other_width, other_height) = (
                //             other_collision_box.width,
                //             other_collision_box.height,
                //         );
                //         let other_transform = manager
                //             .borrow_component::<Transform>(*other_id)
                //             .unwrap();
                //         let (other_x, other_y) = (
                //             other_transform.position.0,
                //             other_transform.position.1,
                //         );

                //         if
                //             is_colliding(
                //                 x,
                //                 y,
                //                 width,
                //                 height,
                //                 other_x,
                //                 other_y,
                //                 other_width,
                //                 other_height
                //             )
                //         {
                //             // 处理碰撞逻辑
                //             transform_mut.velocity.0 = -vx; // 反转水平速度
                //             transform_mut.velocity.1 = -vy; // 反转垂直速度

                //             // 调整位置以避免重叠
                //             if x < other_transform.position.0 {
                //                 transform_mut.position.0 -= 1;
                //             } else {
                //                 transform_mut.position.0 += 1;
                //             }

                //             if y < other_transform.position.1 {
                //                 transform_mut.position.1 -= 1;
                //             } else {
                //                 transform_mut.position.1 += 1;
                //             }
                //         }
                //     }
                // }
            }
        }
    }
}

fn is_colliding(x1: u32, y1: u32, w1: u32, h1: u32, x2: u32, y2: u32, w2: u32, h2: u32) -> bool {
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}
