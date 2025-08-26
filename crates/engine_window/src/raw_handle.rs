use std::sync::Arc;
use std::{any::Any, marker::PhantomData, ops::Deref};

use engine_ecs::component::Component;
use engine_platform::sync::Mutex;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};

/// 窗口的包装器。
///
/// 这样可以延长窗口的生命周期，避免在渲染管线还有帧未完成时窗口被提前释放。
///
/// 实现方式是将窗口的共享引用存储在 [`RawHandleWrapper`] 中，
/// 渲染器在提取阶段会获取这个引用。
#[derive(Debug)]
pub struct WindowWrapper<W> {
    reference: Arc<dyn Any + Send + Sync>,
    ty: PhantomData<W>,
}

impl<W: Send + Sync + 'static> WindowWrapper<W> {
    /// 从窗口对象创建一个 `WindowWrapper`。
    pub fn new(window: W) -> WindowWrapper<W> {
        WindowWrapper {
            reference: Arc::new(window),
            ty: PhantomData,
        }
    }
}

impl<W: 'static> Deref for WindowWrapper<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.reference.downcast_ref::<W>().unwrap()
    }
}

/// [`RawWindowHandle`] 和 [`RawDisplayHandle`] 的包装器，允许我们安全地在多个线程之间传递它。
///
/// 根据不同平台，底层包含指针的句柄并不能在所有线程上使用，
/// 因此我们不能简单地让它（或任何可以安全获取 [`RawWindowHandle`] 或 [`RawDisplayHandle`] 的类型）
/// 具有线程安全性。
#[derive(Debug, Clone)]
/// [`RawWindowHandle`] 和 [`RawDisplayHandle`] 的包装器，允许我们安全地在多个线程之间传递它。
pub struct RawHandleWrapper {
    /// 指向窗口的共享引用。
    /// 这样可以延长窗口的生命周期，
    /// 避免在渲染管线还有帧未完成时窗口被提前释放。
    _window: Arc<dyn Any + Send + Sync>,
    /// 指向窗口的原始句柄。
    window_handle: RawWindowHandle,
    /// 指向显示服务器的原始句柄。
    display_handle: RawDisplayHandle,
}

impl Component for RawHandleWrapper  {
    
}

impl RawHandleWrapper {
    /// Creates a `RawHandleWrapper` from a `WindowWrapper`.
    pub fn new<W: HasWindowHandle + HasDisplayHandle + 'static>(
        window: &WindowWrapper<W>,
    ) -> Result<RawHandleWrapper, HandleError> {
        Ok(RawHandleWrapper {
            _window: window.reference.clone(),
            window_handle: window.window_handle()?.as_raw(),
            display_handle: window.display_handle()?.as_raw(),
        })
    }

      /// 获取存储的窗口句柄。
    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }

    /// 设置窗口句柄。
    ///
    /// # 安全
    ///
    /// 传入的 [`RawWindowHandle`] 必须是有效的窗口句柄。
    // 注意：使用显式的 setter 而不是获取可变引用，是为了限制发生不安全行为的时间。
    // 如果我们直接返回一个可变引用，用户就必须在其生命周期内维护安全性约束。为了保持一致性，
    // 我们也倾向于返回句柄的副本而不是不可变引用。
    pub unsafe fn set_window_handle(&mut self, window_handle: RawWindowHandle) -> &mut Self {
        self.window_handle = window_handle;

        self
    }

       /// 获取存储的显示句柄。
    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }

    /// 设置显示句柄。
    ///
    /// # 安全
    ///
    /// 传入的 [`RawDisplayHandle`] 必须是有效的显示句柄。
    pub fn set_display_handle(&mut self, display_handle: RawDisplayHandle) -> &mut Self {
        self.display_handle = display_handle;

        self
    }
}

/// 用于多线程安全共享和管理窗口句柄的包装器。
pub struct RawHandleWrapperHolder(pub Arc<Mutex<Option<RawHandleWrapper>>>);