
use std::collections::HashMap;
use engine_ecs::prelude::*;
use engine_window::prelude::*;
use winit::window::{Window as WinitWindow, WindowId};
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;

/// 管理 Winit 窗口与实体之间的映射关系
#[derive(Debug, Default)]
pub struct WinitWindows {
    /// 存储窗口包装器
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
    /// 实体到窗口ID的映射
    pub entity_to_winit: HashMap<Entity, WindowId>,
    /// 窗口ID到实体的映射
    pub winit_to_entity: HashMap<WindowId, Entity>,
}

impl WinitWindows {
    /// 创建新的窗口管理器
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            entity_to_winit: HashMap::new(),
            winit_to_entity: HashMap::new(),
        }
    }

    /// 创建新的 winit 窗口并关联到实体
    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        entity: Entity,
        window: &Window,
    ) -> &WindowWrapper<WinitWindow> {

        let mut winit_window_attributes = WinitWindow::default_attributes();

        winit_window_attributes = winit_window_attributes
            .with_title(&window.title)
            .with_inner_size(LogicalSize::new(window.width(), window.height()))
            .with_visible(false) // 先设为不可见，创建完成后再显示
            .with_resizable(window.resizable)
            .with_decorations(window.decorations)
            .with_transparent(window.transparent);

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowAttributesExtMacOS;

            winit_window_attributes = winit_window_attributes
                // 允许通过拖动窗口背景移动窗口
                .with_movable_by_window_background(true)
                // 窗口阴影
                .with_has_shadow(true)
                // 如果没有装饰，隐藏标题栏
                .with_titlebar_hidden(!window.decorations)
                // 标题栏透明度（可根据需要调整）
                .with_titlebar_transparent(window.transparent);
        }

        // 创建窗口
        let winit_window = event_loop
            .create_window(winit_window_attributes)
            .expect("Failed to create window");

        // 设置可见性
        winit_window.set_visible(window.visible);

        let window_id = winit_window.id();

        // 建立映射关系
        self.entity_to_winit.insert(entity, window_id);
        self.winit_to_entity.insert(window_id, entity);

        // 存储窗口包装器并返回引用
        self.windows
            .entry(window_id)
            .or_insert(WindowWrapper::new(winit_window))
    }

    /// 插入新窗口并建立映射关系
    pub fn insert_window(&mut self, entity: Entity, window_id: WindowId, window: WinitWindow) {
        self.entity_to_winit.insert(entity, window_id);
        self.winit_to_entity.insert(window_id, entity);
        self.windows.insert(window_id, WindowWrapper::new(window));
    }

    /// 通过实体获取窗口
    pub fn get_window(&self, entity: Entity) -> Option<&WindowWrapper<WinitWindow>> {
        self.entity_to_winit
            .get(&entity)
            .and_then(|winit_id| self.windows.get(winit_id))
    }

    /// 通过窗口ID获取实体
    pub fn get_window_entity(&self, winit_id: WindowId) -> Option<Entity> {
        self.winit_to_entity.get(&winit_id).copied()
    }

    /// 移除窗口及其所有映射关系
    pub fn remove_window(&mut self, entity: Entity) -> Option<WindowWrapper<WinitWindow>> {
        let winit_id = self.entity_to_winit.remove(&entity)?;
        self.winit_to_entity.remove(&winit_id);
        self.windows.remove(&winit_id)
    }


}
