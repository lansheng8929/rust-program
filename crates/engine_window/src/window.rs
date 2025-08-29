
use engine_ecs::prelude::*;
use engine_math::prelude::*;

/// 标记主窗口组件
#[derive(Default, Debug, Copy, Clone)]
pub struct PrimaryWindow;

impl Component for PrimaryWindow {
    
}

/// 窗口引用（主窗口或指定实体窗口）
#[derive(Default, Copy, Clone, Debug)]
pub enum WindowRef {
    #[default]
    Primary,
    Entity(Entity),
}

/// 窗口组件，存储窗口的主要属性
#[derive(Debug, Clone)]
pub struct Window {
    pub title: String,
    pub resolution: WindowResolution,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub visible: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "App".to_owned(),
            resolution: Default::default(),
            resizable: true,
            decorations: true,
            transparent: false,
            visible: true,
        }
    }
}

impl Component for Window  {
    
}

impl Window {
    /// 获取窗口逻辑宽度
    pub fn width(&self) -> f32 {
        self.resolution.width()
    }
    /// 获取窗口逻辑高度
    pub fn height(&self) -> f32 {
        self.resolution.height()
    }
    /// 获取窗口逻辑尺寸
    pub fn size(&self) -> Vec2 {
        self.resolution.size()
    }
    /// 获取窗口物理宽度
    pub fn physical_width(&self) -> u32 {
        self.resolution.physical_width()
    }
    /// 获取窗口物理高度
    pub fn physical_height(&self) -> u32 {
        self.resolution.physical_height()
    }
    /// 获取窗口物理尺寸
    pub fn physical_size(&self) -> UVec2 {
        self.resolution.physical_size()
    }
}

/// 窗口分辨率
#[derive(Debug, Clone, PartialEq)]
pub struct WindowResolution {
    physical_width: u32,
    physical_height: u32,
    scale_factor: f32,
}

impl Default for WindowResolution {
    fn default() -> Self {
        WindowResolution {
            physical_width: 1280,
            physical_height: 720,
            scale_factor: 1.0,
        }
    }
}

impl WindowResolution {
    pub fn width(&self) -> f32 {
        self.physical_width as f32 / self.scale_factor
    }
    pub fn height(&self) -> f32 {
        self.physical_height as f32 / self.scale_factor
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }
    pub fn physical_width(&self) -> u32 {
        self.physical_width
    }
    pub fn physical_height(&self) -> u32 {
        self.physical_height
    }
    pub fn physical_size(&self) -> UVec2 {
        UVec2::new(self.physical_width, self.physical_height)
    }
}