use ultraviolet::{Mat4, Rotor3, Similarity3, Vec3, Vec4};

use crate::core::components::maths::{
    camera::Camera, coordinates::Coordinates, transform::Transform,
};
use crate::utils::maths::Vector;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlVec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&Coordinates> for GlVec3 {
    fn from(position: &Coordinates) -> Self {
        Self { x: position.x(), y: position.y(), z: 0. }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vec4> for GlVec4 {
    fn from(vec: Vec4) -> Self {
        GlVec4 { x: vec.x, y: vec.y, z: vec.z, w: vec.w }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ColoredGlVertex {
    pub position: [f32;3],
    pub color: GlColor,
}

impl ColoredGlVertex {
    pub fn _desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColoredGlVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TexturedGlVertex {
    pub position: [f32;3],
    pub tex_translation: [f32;2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TexturedGlVertexWithLayer {
    pub position: [f32;3],
    pub tex_translation: [f32;2],
    pub layer: u32
}

impl TexturedGlVertexWithLayer {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedGlVertexWithLayer>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 2]>() + mem::size_of::<[f32; 3]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

impl TexturedGlVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedGlVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

impl From<(&Coordinates, &Coordinates)> for TexturedGlVertex {
    fn from(positions: (&Coordinates, &Coordinates)) -> Self {
        TexturedGlVertex {
            position: [positions.0.x(), positions.0.y(), 0.0 ],
            tex_translation: [positions.1.x(), positions.1.y],
        }
    }
}

impl From<(&Coordinates, &Coordinates, usize)> for TexturedGlVertexWithLayer {
    fn from(positions: (&Coordinates, &Coordinates, usize)) -> Self {
        TexturedGlVertexWithLayer {
            position: [positions.0.x(), positions.0.y(), 0.0 ],
            tex_translation: [positions.1.x(), positions.1.y()],
            layer: positions.2 as u32
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlUniform {
    pub model_trans: [[f32; 4]; 4],
    pub camera_view: [[f32; 4]; 4],
}

impl GlUniform {
    pub(crate) fn replace_with(&mut self, other: GlUniform) {
        self.model_trans = other.model_trans;
        self.camera_view = other.camera_view;
    }
}

pub(crate) fn create_glmat4(t: &mut Mat4) -> [[f32; 4]; 4] {
    [
        create_glmat(&t.cols[0]),
        create_glmat(&t.cols[1]),
        create_glmat(&t.cols[2]),
        create_glmat(&t.cols[3]),
    ]
}

pub(crate) fn create_glmat(t: &Vec4) -> [f32; 4] {
    [t.x, t.y, t.z, t.w]
}

pub(crate) struct UniformData<'a> {
    pub transform: &'a Transform,
    pub camera: (&'a Camera, &'a Transform),
    pub is_ui_component: bool,
    pub pivot_offset: Vector
}

impl From<UniformData<'_>> for GlUniform {
    fn from(uniform_data: UniformData) -> Self {
        let mut model_trans = Similarity3::identity();
        model_trans.prepend_scaling(uniform_data.transform.scale);
        model_trans.append_translation(Vec3 {
            x: uniform_data.transform.global_translation.x() + uniform_data.pivot_offset.x * uniform_data.transform.scale,
            y: uniform_data.transform.global_translation.y() + uniform_data.pivot_offset.y * uniform_data.transform.scale,
            z: uniform_data.transform.global_translation.z() as f32,
        });
        if !uniform_data.is_ui_component && !uniform_data.transform.use_screen_as_origin {
            model_trans.append_translation(Vec3 {
                x: -1. * uniform_data.camera.1.global_translation().x(),
                y: -1. * uniform_data.camera.1.global_translation().y(),
                z: 0.0,
            });
        }
        model_trans
            .prepend_rotation(Rotor3::from_rotation_xy(uniform_data.transform.global_angle).normalized());
        let mut model_trans = model_trans.into_homogeneous_matrix();
        let mut camera_view = ultraviolet::projection::lh_ydown::orthographic_wgpu_dx(
            uniform_data.camera.0.left,
            uniform_data.camera.0.right,
            uniform_data.camera.0.bottom ,
            uniform_data.camera.0.top,
            uniform_data.camera.0.near,
            uniform_data.camera.0.far,
        );
        GlUniform {
            model_trans: create_glmat4(&mut model_trans),
            camera_view: create_glmat4(&mut camera_view),
        }
    }
}
