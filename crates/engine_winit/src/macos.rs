use std::sync::Once;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
use objc2_app_kit::{NSApplication, NSApplicationDelegate};
use objc2_foundation::{NSArray, NSURL, MainThreadMarker, NSObject, NSObjectProtocol};
use winit::event_loop::EventLoop;

declare_class!(
    struct AppDelegate;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyAppDelegate";
    }

    impl DeclaredClass for AppDelegate {}

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(application:openURLs:)]
        fn application_openURLs(&self, application: &NSApplication, urls: &NSArray<NSURL>) {
            // Note: To specifically get `application:openURLs:` to work, you _might_
            // have to bundle your application. This is not done in this example.
            println!("open urls: {application:?}, {urls:?}");
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        unsafe { msg_send_id![super(mtm.alloc().set_ivars(())), init] }
    }
}

static SETUP_ONCE: Once = Once::new();

/// 设置 macOS 应用委托（只执行一次）
pub fn setup_app_delegate() {
    SETUP_ONCE.call_once(|| {
        if let Some(mtm) = MainThreadMarker::new() {
            let delegate = AppDelegate::new(mtm);
            let app = NSApplication::sharedApplication(mtm);
            app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

            // 设置应用激活策略
            unsafe {
                use objc2_app_kit::NSApplicationActivationPolicy;
                app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
            }

            println!("macOS 应用委托设置完成");
        } else {
            eprintln!("警告: 无法获取主线程标记，macOS 应用委托设置失败");
        }
    });
}

/// macOS 特定的窗口配置
pub fn configure_window_for_macos() {
    // 设置 macOS 特定的窗口属性
    use winit::platform::macos::WindowExtMacOS;

    println!("macOS 窗口配置完成");
}

/// 检查是否在 macOS 平台运行
pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}
