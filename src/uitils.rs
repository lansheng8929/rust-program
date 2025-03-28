/// Checks if the given coordinates are within window boundaries
///
/// # Arguments
///
/// * `x` - The x-coordinate to check
/// * `y` - The y-coordinate to check
/// * `width` - The width of the window
/// * `height` - The height of the window
///
/// # Returns
///
/// `true` if the coordinates are within bounds, `false` otherwise
pub fn is_within_bounds(x: f32, y: f32, width: u32, height: u32) -> bool {
    x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32
}

/// Returns a tuple of coordinates that are guaranteed to be within window boundaries
///
/// # Arguments
///
/// * `x` - The x-coordinate to constrain
/// * `y` - The y-coordinate to constrain
/// * `width` - The width of the window
/// * `height` - The height of the window
///
/// # Returns
///
/// A tuple `(x, y)` with coordinates constrained to window boundaries
pub fn constrain_to_bounds(x: f32, y: f32, width: u32, height: u32) -> (f32, f32) {
    let x_constrained = x.clamp(0.0, width as f32 - 1.0);
    let y_constrained = y.clamp(0.0, height as f32 - 1.0);
    (x_constrained, y_constrained)
}
