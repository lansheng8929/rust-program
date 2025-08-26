
use std::{cell::RefCell, collections::HashMap};
use engine_ecs::entity::Entity;
use engine_window::raw_handle::WindowWrapper;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    error::ExternalError,
    event_loop::ActiveEventLoop,
    monitor::{MonitorHandle, VideoModeHandle},
    window::{CursorGrabMode as WinitCursorGrabMode, Fullscreen, Window as WinitWindow, WindowId},
};



#[derive(Debug, Default)]
pub struct WinitWindows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
    /// Maps entities to `winit` window identifiers.
    pub entity_to_winit: HashMap<Entity, WindowId>,
    /// Maps `winit` window identifiers to entities.
    pub winit_to_entity: HashMap<WindowId, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl WinitWindows {
    /// Creates a new instance of `WinitWindows`.
    pub const fn new() -> Self {
        Self {
            windows: HashMap::new(),
            entity_to_winit: EntityHashMap::new(),
            winit_to_entity: HashMap::new(),
            _not_send_sync: core::marker::PhantomData,
        }
    }

    /// Creates a `winit` window and associates it with our entity.
    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        entity: Entity,
        window: &Window,
        cursor_options: &CursorOptions,
        adapters: &mut AccessKitAdapters,
        handlers: &mut WinitActionRequestHandlers,
        accessibility_requested: &AccessibilityRequested,
        monitors: &WinitMonitors,
    ) -> &WindowWrapper<WinitWindow> {
        let mut winit_window_attributes = WinitWindow::default_attributes();

        // Due to a UIA limitation, winit windows need to be invisible for the
        // AccessKit adapter is initialized.
        winit_window_attributes = winit_window_attributes.with_visible(false);

        let maybe_selected_monitor = &match window.mode {
            WindowMode::BorderlessFullscreen(monitor_selection)
            | WindowMode::Fullscreen(monitor_selection, _) => select_monitor(
                monitors,
                event_loop.primary_monitor(),
                None,
                &monitor_selection,
            ),
            WindowMode::Windowed => None,
        };

        winit_window_attributes = match window.mode {
            WindowMode::BorderlessFullscreen(_) => winit_window_attributes
                .with_fullscreen(Some(Fullscreen::Borderless(maybe_selected_monitor.clone()))),
            WindowMode::Fullscreen(monitor_selection, video_mode_selection) => {
                let select_monitor = &maybe_selected_monitor
                    .clone()
                    .expect("Unable to get monitor.");

                if let Some(video_mode) =
                    get_selected_videomode(select_monitor, &video_mode_selection)
                {
                    winit_window_attributes.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)))
                } else {
                    warn!(
                        "Could not find valid fullscreen video mode for {:?} {:?}",
                        monitor_selection, video_mode_selection
                    );
                    winit_window_attributes
                }
            }
            WindowMode::Windowed => {
                if let Some(position) = winit_window_position(
                    &window.position,
                    &window.resolution,
                    monitors,
                    event_loop.primary_monitor(),
                    None,
                ) {
                    winit_window_attributes = winit_window_attributes.with_position(position);
                }
                let logical_size = LogicalSize::new(window.width(), window.height());
                if let Some(sf) = window.resolution.scale_factor_override() {
                    let inner_size = logical_size.to_physical::<f64>(sf.into());
                    winit_window_attributes.with_inner_size(inner_size)
                } else {
                    winit_window_attributes.with_inner_size(logical_size)
                }
            }
        };

        // It's crucial to avoid setting the window's final visibility here;
        // as explained above, the window must be invisible until the AccessKit
        // adapter is created.
        winit_window_attributes = winit_window_attributes
            .with_window_level(convert_window_level(window.window_level))
            .with_theme(window.window_theme.map(convert_window_theme))
            .with_resizable(window.resizable)
            .with_enabled_buttons(convert_enabled_buttons(window.enabled_buttons))
            .with_decorations(window.decorations)
            .with_transparent(window.transparent)
            .with_active(window.focused);

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowAttributesExtWindows;
            winit_window_attributes =
                winit_window_attributes.with_skip_taskbar(window.skip_taskbar);
            winit_window_attributes =
                winit_window_attributes.with_clip_children(window.clip_children);
        }

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowAttributesExtMacOS;
            winit_window_attributes = winit_window_attributes
                .with_movable_by_window_background(window.movable_by_window_background)
                .with_fullsize_content_view(window.fullsize_content_view)
                .with_has_shadow(window.has_shadow)
                .with_titlebar_hidden(!window.titlebar_shown)
                .with_titlebar_transparent(window.titlebar_transparent)
                .with_title_hidden(!window.titlebar_show_title)
                .with_titlebar_buttons_hidden(!window.titlebar_show_buttons);
        }

        #[cfg(target_os = "ios")]
        {
            use crate::converters::convert_screen_edge;
            use winit::platform::ios::WindowAttributesExtIOS;

            let preferred_edge =
                convert_screen_edge(window.preferred_screen_edges_deferring_system_gestures);

            winit_window_attributes = winit_window_attributes
                .with_preferred_screen_edges_deferring_system_gestures(preferred_edge);
            winit_window_attributes = winit_window_attributes
                .with_prefers_home_indicator_hidden(window.prefers_home_indicator_hidden);
            winit_window_attributes = winit_window_attributes
                .with_prefers_status_bar_hidden(window.prefers_status_bar_hidden);
        }

        let display_info = DisplayInfo {
            window_physical_resolution: (
                window.resolution.physical_width(),
                window.resolution.physical_height(),
            ),
            window_logical_resolution: (window.resolution.width(), window.resolution.height()),
            monitor_name: maybe_selected_monitor
                .as_ref()
                .and_then(MonitorHandle::name),
            scale_factor: maybe_selected_monitor
                .as_ref()
                .map(MonitorHandle::scale_factor),
            refresh_rate_millihertz: maybe_selected_monitor
                .as_ref()
                .and_then(MonitorHandle::refresh_rate_millihertz),
        };
        bevy_log::debug!("{display_info}");

        #[cfg(any(
            all(
                any(feature = "wayland", feature = "x11"),
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                )
            ),
            target_os = "windows"
        ))]
        if let Some(name) = &window.name {
            #[cfg(all(
                feature = "wayland",
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                )
            ))]
            {
                winit_window_attributes =
                    winit::platform::wayland::WindowAttributesExtWayland::with_name(
                        winit_window_attributes,
                        name.clone(),
                        "",
                    );
            }

            #[cfg(all(
                feature = "x11",
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                )
            ))]
            {
                winit_window_attributes = winit::platform::x11::WindowAttributesExtX11::with_name(
                    winit_window_attributes,
                    name.clone(),
                    "",
                );
            }
            #[cfg(target_os = "windows")]
            {
                winit_window_attributes =
                    winit::platform::windows::WindowAttributesExtWindows::with_class_name(
                        winit_window_attributes,
                        name.clone(),
                    );
            }
        }

        let constraints = window.resize_constraints.check_constraints();
        let min_inner_size = LogicalSize {
            width: constraints.min_width,
            height: constraints.min_height,
        };
        let max_inner_size = LogicalSize {
            width: constraints.max_width,
            height: constraints.max_height,
        };

        let winit_window_attributes =
            if constraints.max_width.is_finite() && constraints.max_height.is_finite() {
                winit_window_attributes
                    .with_min_inner_size(min_inner_size)
                    .with_max_inner_size(max_inner_size)
            } else {
                winit_window_attributes.with_min_inner_size(min_inner_size)
            };

        #[expect(clippy::allow_attributes, reason = "`unused_mut` is not always linted")]
        #[allow(
            unused_mut,
            reason = "This variable needs to be mutable if `cfg(target_arch = \"wasm32\")`"
        )]
        let mut winit_window_attributes = winit_window_attributes.with_title(window.title.as_str());

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            if let Some(selector) = &window.canvas {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let canvas = document
                    .query_selector(selector)
                    .expect("Cannot query for canvas element.");
                if let Some(canvas) = canvas {
                    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok();
                    winit_window_attributes = winit_window_attributes.with_canvas(canvas);
                } else {
                    panic!("Cannot find element: {selector}.");
                }
            }

            winit_window_attributes =
                winit_window_attributes.with_prevent_default(window.prevent_default_event_handling);
            winit_window_attributes = winit_window_attributes.with_append(true);
        }

        let winit_window = event_loop.create_window(winit_window_attributes).unwrap();
        let name = window.title.clone();
        prepare_accessibility_for_window(
            event_loop,
            &winit_window,
            entity,
            name,
            accessibility_requested.clone(),
            adapters,
            handlers,
        );

        // Now that the AccessKit adapter is created, it's safe to show
        // the window.
        winit_window.set_visible(window.visible);

        // Do not set the grab mode on window creation if it's none. It can fail on mobile.
        if cursor_options.grab_mode != CursorGrabMode::None {
            let _ = attempt_grab(&winit_window, cursor_options.grab_mode);
        }

        winit_window.set_cursor_visible(cursor_options.visible);

        // Do not set the cursor hittest on window creation if it's false, as it will always fail on
        // some platforms and log an unfixable warning.
        if !cursor_options.hit_test
            && let Err(err) = winit_window.set_cursor_hittest(cursor_options.hit_test)
        {
            warn!(
                "Could not set cursor hit test for window {}: {}",
                window.title, err
            );
        }

        self.entity_to_winit.insert(entity, winit_window.id());
        self.winit_to_entity.insert(winit_window.id(), entity);

        self.windows
            .entry(winit_window.id())
            .insert(WindowWrapper::new(winit_window))
            .into_mut()
    }

    /// Get the winit window that is associated with our entity.
    pub fn get_window(&self, entity: Entity) -> Option<&WindowWrapper<WinitWindow>> {
        self.entity_to_winit
            .get(&entity)
            .and_then(|winit_id| self.windows.get(winit_id))
    }

    /// Get the entity associated with the winit window id.
    ///
    /// This is mostly just an intermediary step between us and winit.
    pub fn get_window_entity(&self, winit_id: WindowId) -> Option<Entity> {
        self.winit_to_entity.get(&winit_id).cloned()
    }

    /// Remove a window from winit.
    ///
    /// This should mostly just be called when the window is closing.
    pub fn remove_window(&mut self, entity: Entity) -> Option<WindowWrapper<WinitWindow>> {
        let winit_id = self.entity_to_winit.remove(&entity)?;
        self.winit_to_entity.remove(&winit_id);
        self.windows.remove(&winit_id)
    }
}

