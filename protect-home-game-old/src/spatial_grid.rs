use std::collections::{HashMap, HashSet};

// 网格单元坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

// 通用空间分区网格
pub struct SpatialGrid<T> {
    cell_size: f32,
    // 将网格单元映射到该单元中的实体ID集合
    grid: HashMap<GridCell, HashSet<T>>,
}

impl<T: Copy + PartialEq + Eq + std::hash::Hash> SpatialGrid<T> {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    // 将世界坐标转换为网格单元
    pub fn get_cell(&self, x: f32, y: f32) -> GridCell {
        GridCell {
            x: (x / self.cell_size).floor() as i32,
            y: (y / self.cell_size).floor() as i32,
        }
    }

    // 获取矩形覆盖的所有网格单元
    pub fn get_cells_for_rect(&self, x: f32, y: f32, width: f32, height: f32) -> Vec<GridCell> {
        let min_cell = self.get_cell(x, y);
        let max_cell = self.get_cell(x + width, y + height);

        let mut cells = Vec::new();
        for cell_x in min_cell.x..=max_cell.x {
            for cell_y in min_cell.y..=max_cell.y {
                cells.push(GridCell {
                    x: cell_x,
                    y: cell_y,
                });
            }
        }
        cells
    }

    // 将实体插入到其边界覆盖的单元格中
    pub fn insert(&mut self, entity_id: T, x: f32, y: f32, width: f32, height: f32) {
        let cells = self.get_cells_for_rect(x, y, width, height);

        for cell in cells {
            self.grid
                .entry(cell)
                .or_insert_with(HashSet::new)
                .insert(entity_id);
        }
    }

    // 从所有单元格中移除实体
    pub fn remove(&mut self, entity_id: &T) {
        for cell_entities in self.grid.values_mut() {
            cell_entities.remove(entity_id);
        }

        // 清理空单元格
        self.grid.retain(|_, entities| !entities.is_empty());
    }

    // 获取实体可能碰撞的候选对象
    pub fn query_potential_collisions(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> HashSet<T> {
        let cells = self.get_cells_for_rect(x, y, width, height);
        let mut result = HashSet::new();

        for cell in cells {
            if let Some(entities) = self.grid.get(&cell) {
                result.extend(entities);
            }
        }

        result
    }

    // 清除网格中的所有实体
    pub fn clear(&mut self) {
        self.grid.clear();
    }
}
