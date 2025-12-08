use numpy::PyArrayMethods;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use wgpu::util::DeviceExt;
use winit::keyboard::KeyCode;

use crate::{Float, Mat3, Mat4, Quat, Vec3, Vec4};

pub const SENSITIVITY_ZOOM: Float = 6e2;

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 1.0),
);

#[cfg(feature = "calc_f32")]
pub const SAFE_FRAC_PI_2: Float = std::f32::consts::FRAC_PI_2 - 0.0001;
#[cfg(not(feature = "calc_f32"))]
pub const SAFE_FRAC_PI_2: Float = std::f64::consts::FRAC_PI_2 - 0.0001;

pub type ModelStateRaw = [[Float; 4]; 4];

#[cfg(feature = "calc_f32")]
const MODEL_STATE_ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
    7 => Float32x4,
    8 => Float32x4,
    9 => Float32x4,
    10 => Float32x4
];
#[cfg(not(feature = "calc_f32"))]
const MODEL_STATE_ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
    7 => Float64x4,
    8 => Float64x4,
    9 => Float64x4,
    10 => Float64x4
];

#[pyclass]
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelState {
    pub p: Vec3,
    pub m: Mat3,
}

impl ModelState {
    pub fn new() -> Self {
        Self {
            p: Vec3::ZERO,
            m: Mat3::from_quat(Quat::IDENTITY),
            // m: glam::Mat3::from_axis_angle(
            //     (0.0, 1.0, 0.0).into(),
            //     180.0 * std::f32::consts::PI / 180.0,
            // ),
        }
    }

    pub fn to_raw(&self) -> ModelStateRaw {
        Mat4::from_mat3_translation(self.m, self.p).to_cols_array_2d()
    }

    pub fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelStateRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &MODEL_STATE_ATTRIBS,
        }
    }
}

#[pymethods]
impl ModelState {
    #[new]
    #[pyo3(signature = (p: "numpy.array | None" = None, m: "numpy.array | None" = None) -> "None")]
    pub fn py_new(
        p: Option<numpy::PyReadonlyArray1<Float>>,
        m: Option<numpy::PyReadonlyArray2<Float>>,
    ) -> Self {
        let mut state = ModelState::new();

        if let Some(p) = p {
            state.p = Vec3::from_slice(p.as_slice().unwrap());
        }

        if let Some(m) = m {
            state.m = Mat3::from_cols_slice(m.as_slice().unwrap());
        }

        state
    }

    // Getter numpy.ndarray read and write.
    #[getter]
    fn p<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray1<Float>> {
        let slice = &slf.borrow().p;
        let slice2 = slice.as_ref();
        let arr = numpy::ndarray::ArrayView1::from(slice2);
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter numpy.ndarray to allow shorthand operators.
    #[setter]
    pub fn set_p<'py>(&mut self, p: Bound<'py, numpy::PyArray1<Float>>) {
        let p = unsafe { p.as_slice().unwrap() };
        self.p.x = p[0];
        self.p.y = p[1];
        self.p.z = p[2];
    }

    // Getter numpy.ndarray read and write.
    #[getter]
    fn m<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray2<Float>> {
        let slice = &slf.borrow().m;
        let slice2 = slice.as_ref();
        let arr = numpy::ndarray::ArrayView2::from_shape((3, 3), slice2).unwrap();
        unsafe { numpy::PyArray2::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter numpy.ndarray to allow shorthand operators.
    #[setter]
    pub fn set_m<'py>(&mut self, m: Bound<'py, numpy::PyArray2<Float>>) {
        let m = unsafe { m.as_slice().unwrap() };

        self.m.x_axis = Vec3::new(m[0], m[1], m[2]);
        self.m.y_axis = Vec3::new(m[3], m[4], m[5]);
        self.m.z_axis = Vec3::new(m[6], m[7], m[8]);
    }
}

pub trait CreateBuffer {
    fn create_buffer(self, device: &wgpu::Device) -> wgpu::Buffer;
}

impl CreateBuffer for ModelState {
    fn create_buffer(self, device: &wgpu::Device) -> wgpu::Buffer {
        let instance_data = vec![self.to_raw()];
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        })

        // impl CreateBuffer for &[ModelState] {
        //         let instance_data = self
        //             .iter()
        //             .map(super::scene::ModelState::to_raw)
        //             .collect::<Vec<_>>();
    }
}

pub fn create_buffer(device: &wgpu::Device, instances: &[ModelState]) -> wgpu::Buffer {
    let instance_data = instances
        .iter()
        .map(super::scene::ModelState::to_raw)
        .collect::<Vec<_>>();
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CameraType {
    Arcball,
    WASD,
}

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ProjectionType {
    Orthographic,

    Perspective,
}

// pub fn matrix(&self, near: Float, far: Float, aspect: Float) -> Mat4 {
//     match self {
//         ProjectionMode::Orthographic => {
//             let side = far;
//             glm::ortho(side * aspect, side * aspect, side, side, near, far)
//         }
//         ProjectionMode::Perspective(fovy) => glm::perspective(aspect, *fovy, near, far),
//     }
// }

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub anchor: Vec3,
    pub typ: CameraType,
    pub up_world: Vec3,
}

impl Camera {
    pub fn target(&self) -> Vec3 {
        self.pos + self.dir
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn bottom(&self) -> Vec3 {
        -self.up
    }

    pub fn front(&self) -> Vec3 {
        self.dir
    }

    pub fn back(&self) -> Vec3 {
        -self.dir
    }

    pub fn right(&self) -> Vec3 {
        self.dir.cross(self.up).normalize()
    }

    pub fn left(&self) -> Vec3 {
        -self.right()
    }

    pub fn lookto(&self) -> Mat4 {
        Mat4::look_to_rh(self.pos, self.dir, self.up)
    }

    pub fn mat(&self) -> Mat3 {
        Mat3::from_cols(self.dir, self.right(), self.up)
    }

    pub fn fix_up(&mut self) {
        self.up = self.right().cross(self.dir).normalize();
    }

    pub fn look_anchor(&mut self) {
        self.dir = (self.anchor - self.pos).normalize();
        self.fix_up();
    }

    pub fn toggle_type(&mut self) {
        self.typ = match self.typ {
            CameraType::Arcball => CameraType::WASD,
            CameraType::WASD => CameraType::Arcball,
        };
    }

    pub fn update_with_controller(&mut self, ctrl: &mut Controller, dt: std::time::Duration) {
        match self.typ {
            CameraType::Arcball => self.arcball_rotate(ctrl, dt),
            CameraType::WASD => self.wasd_with_conroller(ctrl, dt),
        };

        // reset mouse amounts
        ctrl.horizontal = 0.0;
        ctrl.vertical = 0.0;
        ctrl.zoom = 0.0;
    }

    pub fn arcball_rotate(&mut self, ctrl: &mut Controller, dt: std::time::Duration) {
        let dt = dt.as_secs_f64() as Float;

        // zoom
        self.pos += self.dir * ctrl.zoom * ctrl.sensitivity * SENSITIVITY_ZOOM * dt;

        // calc matrices
        let m1 = Mat3::from_axis_angle(self.up_world, ctrl.horizontal * ctrl.sensitivity * dt);
        let m2 = Mat3::from_axis_angle(self.right(), ctrl.vertical * ctrl.sensitivity * dt);
        let m = m1 * m2;

        // update movement and cam dir
        self.pos = self.anchor + m * (self.pos - self.anchor);
        self.up = m * self.up;
        self.look_anchor();
    }

    pub fn wasd_with_conroller(&mut self, ctrl: &mut Controller, dt: std::time::Duration) {
        let dt = dt.as_secs_f64() as Float;

        // movement
        self.pos += self.dir * (ctrl.forward - ctrl.backward) * ctrl.speed * dt;
        self.pos += self.right() * (ctrl.right - ctrl.left) * ctrl.speed * dt;
        self.pos += self.up * (ctrl.up - ctrl.down) * ctrl.speed * dt;

        // look around
        let m1 = Mat3::from_axis_angle(self.up, -ctrl.horizontal * ctrl.sensitivity * dt);
        let m2 = Mat3::from_axis_angle(self.right(), -ctrl.vertical * ctrl.sensitivity * dt);
        let m = m1 * m2;
        self.up = m * self.up;
        self.dir = m * self.dir;
    }
}

#[derive(Debug, Clone)]
pub struct CameraFpsWGPU {
    pub position: glam::Vec3,
    pub yaw: f32,   // rad
    pub pitch: f32, // rad
}

impl CameraFpsWGPU {
    pub fn new(position: glam::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
        }
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        glam::Mat4::look_to_rh(
            self.position,
            glam::Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            glam::Vec3::Y,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Projection {
    pub aspect: Float,
    pub fovy: Float, // fovy in rad
    pub znear: Float,
    pub zfar: Float,
    pub typ: ProjectionType,
}

impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        fovy: Float,
        znear: Float,
        zfar: Float,
        typ: ProjectionType,
    ) -> Self {
        Self {
            aspect: width as Float / height as Float,
            znear,
            fovy,
            zfar,
            typ,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as Float / height as Float;
    }

    // right-handed, Z axis points out of the screen
    // i dont like and want that, I want +Z is up
    pub fn mat(&self) -> Mat4 {
        // OPENGL_TO_WGPU_MATRIX *
        match self.typ {
            ProjectionType::Orthographic => {
                let side = self.zfar;
                Mat4::orthographic_rh(
                    side * self.aspect,
                    side * self.aspect,
                    side,
                    side,
                    self.znear,
                    self.zfar,
                )
            }
            ProjectionType::Perspective => {
                Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [Float; 4],
    pub view: [[Float; 4]; 4],
    pub view_proj: [[Float; 4]; 4],
    pub inv_view: [[Float; 4]; 4],
    pub inv_proj: [[Float; 4]; 4],
}

impl CameraUniform {
    pub fn init() -> Self {
        Self {
            view_position: [0.0; 4],
            view: Mat4::IDENTITY.to_cols_array_2d(),
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            inv_view: Mat4::IDENTITY.to_cols_array_2d(),
            inv_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn new(camera: &Camera, projection: &Projection) -> Self {
        let mut u = Self::init();
        u.update_view_proj(camera, projection);
        u
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = [camera.pos.x, camera.pos.y, camera.pos.z, 1.0];
        let view = camera.lookto();
        let proj = projection.mat();
        self.view = view.to_cols_array_2d();
        self.view_proj = (proj * view).to_cols_array_2d();
        self.inv_view = view.transpose().to_cols_array_2d();
        self.inv_proj = proj.inverse().to_cols_array_2d();
    }
}

impl super::buffer::UniformBindTrait for CameraUniform {}

#[derive(Debug)]
pub struct Controller {
    // amounts
    pub left: Float,
    pub right: Float,
    pub forward: Float,
    pub backward: Float,
    pub up: Float,
    pub down: Float,
    pub horizontal: Float, // rad
    pub vertical: Float,   // rad
    pub zoom: Float,
    pub speed: Float,
    pub sensitivity: Float,
}

impl Controller {
    pub fn new(speed: Float, sensitivity: Float) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            forward: 0.0,
            backward: 0.0,
            up: 0.0,
            down: 0.0,
            horizontal: 0.0,
            vertical: 0.0,
            zoom: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool {
        let amount = if is_pressed { 1.0 } else { 0.0 };

        match key {
            KeyCode::KeyW => {
                self.forward = amount;
            }
            KeyCode::KeyS => {
                self.backward = amount;
            }
            KeyCode::KeyA => {
                self.left = amount;
            }
            KeyCode::KeyD => {
                self.right = amount;
            }
            KeyCode::Space => {
                self.up = amount;
            }
            KeyCode::ShiftLeft => {
                self.down = amount;
            }
            _ => {
                return false;
            }
        }

        true
    }

    pub fn mouse_motion(&mut self, dx: Float, dy: Float) {
        self.horizontal = dx;
        self.vertical = dy;
    }

    pub fn zoom(&mut self, delta: Float) {
        self.zoom = delta;
    }
}
