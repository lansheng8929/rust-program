// pub fn create_windows<F: QueryFilter + 'static>(
//     event_loop: &ActiveEventLoop,
//     (
//         mut commands,
//         mut created_windows,
//         mut window_created_events,
//         mut handlers,
//         accessibility_requested,
//         monitors,
//     ): SystemParamItem<CreateWindowParams<F>>,
// ) {
//     WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
//         ACCESS_KIT_ADAPTERS.with_borrow_mut(|adapters| {
//             for (entity, mut window, cursor_options, handle_holder) in &mut created_windows {
//                 if winit_windows.get_window(entity).is_some() {
//                     continue;
//                 }

//                 info!("Creating new window {} ({})", window.title.as_str(), entity);

//                 let winit_window = winit_windows.create_window(
//                     event_loop,
//                     entity,
//                     &window,
//                     cursor_options,
//                     adapters,
//                     &mut handlers,
//                     &accessibility_requested,
//                     &monitors,
//                 );

//                 if let Some(theme) = winit_window.theme() {
//                     window.window_theme = Some(convert_winit_theme(theme));
//                 }

//                 window
//                     .resolution
//                     .set_scale_factor_and_apply_to_physical_size(winit_window.scale_factor() as f32);

//                 commands.entity(entity).insert((
//                     CachedWindow(window.clone()),
//                     CachedCursorOptions(cursor_options.clone()),
//                     WinitWindowPressedKeys::default(),
//                 ));

//                 if let Ok(handle_wrapper) = RawHandleWrapper::new(winit_window) {
//                     commands.entity(entity).insert(handle_wrapper.clone());
//                     if let Some(handle_holder) = handle_holder {
//                         *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
//                     }
//                 }

//                 #[cfg(target_arch = "wasm32")]
//                 {
//                     if window.fit_canvas_to_parent {
//                         let canvas = winit_window
//                             .canvas()
//                             .expect("window.canvas() can only be called in main thread.");
//                         let style = canvas.style();
//                         style.set_property("width", "100%").unwrap();
//                         style.set_property("height", "100%").unwrap();
//                     }
//                 }

//                 #[cfg(target_os = "ios")]
//                 {
//                     winit_window.recognize_pinch_gesture(window.recognize_pinch_gesture);
//                     winit_window.recognize_rotation_gesture(window.recognize_rotation_gesture);
//                     winit_window.recognize_doubletap_gesture(window.recognize_doubletap_gesture);
//                     if let Some((min, max)) = window.recognize_pan_gesture {
//                         winit_window.recognize_pan_gesture(true, min, max);
//                     } else {
//                         winit_window.recognize_pan_gesture(false, 0, 0);
//                     }
//                 }

//                 window_created_events.write(WindowCreated { window: entity });
//             }
//         });
//     });
// }
