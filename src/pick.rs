use cgmath::{
    EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Transform, Vector3, Vector4,
};
use amethyst_core::GlobalTransform;
use amethyst_renderer::Camera;
use collision::Ray3;

fn mouse_to_screen(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
    ((2.0 * x) / width - 1.0, 1.0 - (2.0 * y) / height)
}

fn clip_to_eye(clip: Vector4<f32>, inverse_projection: Matrix4<f32>) -> Vector4<f32> {
    let eye = inverse_projection * clip;
    Vector4::new(eye.x, eye.y, -1.0, 0.0)
}

fn eye_to_world(eye: Vector4<f32>, camera_transform: &Matrix4<f32>) -> Vector3<f32> {
    (camera_transform * eye).normalize().truncate()
}

/// Generate a ray for picking, based on the clicked position in pixel coordinates.
///
/// ## Parameters:
///
/// - `window_pos`: clicked position in pixel coordinates
/// - `window_size`: window size in pixels
/// - `camera`: `Camera`, used to convert from screen space to eye space
/// - `camera_transform`: camera transform, used to convert from eye space to world space
pub fn pick_ray(
    window_pos: (f32, f32), // pixel coordinates
    window_size: (f32, f32),
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Ray3<f32> {
    let clip = mouse_to_screen(window_pos.0, window_pos.1, window_size.0, window_size.1);
    pick_ray_screen(clip, camera, camera_transform)
}

/// Generate a ray for picking, based on the clicked position in screen space coordinates.
///
/// ## Parameters:
///
/// - `window_pos`: clicked position in normalized device coordinates, i.e. in the range (-1..1)
/// - `camera`: `Camera`, used to convert from screen space to eye space
/// - `camera_transform`: camera transform, used to convert from eye space to world space
pub fn pick_ray_screen(
    window_pos: (f32, f32), // ndc coordinates (-1..1)
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Ray3<f32> {
    let clip = Vector4::new(window_pos.0, window_pos.1, 0.0, 0.0);
    let eye = clip_to_eye(
        clip,
        camera
            .proj
            .invert()
            .expect("degenerate projection matrix caused breakdown in matrix inversion"),
    );
    let world_dir = eye_to_world(eye, &camera_transform.0);
    let origin = camera_transform.0.transform_point(Point3::origin());
    Ray3::new(origin, world_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!(
            "{:?}",
            pick_ray(
                (0., 0.),
                (1024., 768.),
                &Camera::standard_3d(1024., 768.),
                &GlobalTransform::default()
            )
        );
    }
}
