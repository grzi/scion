use miniquad::{Context, Buffer, BufferType, Bindings, Shader, Pipeline, BufferLayout, VertexAttribute, VertexFormat};
use crate::renderer::bidimensional::gl_representations::{ColoredGlVertex, GlVec2, GlColor, GlUniform, create_glmat4, TexturedGlVertex};
use crate::renderer::bidimensional::material::{Material2D, Texture2D};
use ultraviolet::{Isometry3, Similarity3, Vec4, Vec3, Rotor3, Rotor2, Similarity2, Vec2, Mat4};
use crate::renderer::bidimensional::renderer::Renderable2D;
use crate::renderer::bidimensional::transform::{Transform2D, Position2D};
use image::{GenericImageView};
use crate::renderer::color::Color;

pub struct Triangle {
    pub vertices: [Position2D; 3],
    pub uvs: Option<[Position2D; 3]>,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertices: [
                Position2D { x: -0.5, y: -0.5 },
                Position2D { x: 0.5, y: -0.5 },
                Position2D { x: 0., y: 0.5 }
            ],
            uvs: Some(
                [
                    Position2D { x: 0., y: 0. },
                    Position2D { x: 1., y: 0. },
                    Position2D { x: 0.5, y: 1. }
                ]
            ),
        }
    }
}

impl Triangle {
    pub fn render_colored(&self, context: &mut Context, transform: &Transform2D, color: &Color) {
        let color: GlColor = color.into();
        let vertices: [ColoredGlVertex; 3] = [
            ColoredGlVertex { pos: GlVec2::from(&self.vertices[0]), color: color.clone() },
            ColoredGlVertex { pos: GlVec2::from(&self.vertices[1]), color: color.clone() },
            ColoredGlVertex { pos: GlVec2::from(&self.vertices[2]), color },
        ];

        let vertex_buffer = Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = Shader::new(context, shader::VERTEX_COLORED, shader::FRAGMENT_COLORED, shader::meta(vec![])).unwrap();

        let pipeline = Pipeline::new(
            context,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("color", VertexFormat::Float4),
            ],
            shader,
        );

        context.apply_pipeline(&pipeline);
        context.apply_bindings(&bindings);

        let mut transform_rotate = Isometry3::identity();
        transform_rotate.append_translation(Vec3 {
            x: transform.position.x,
            y: transform.position.y,
            z: 1.0,
        });
        transform_rotate.prepend_rotation(Rotor3::from_rotation_xy(transform.angle).normalized());

        let mut scale = Similarity3::identity();
        scale.append_scaling(transform.scale);

        let mut transform_rotate = transform_rotate.into_homogeneous_matrix();
        let mut scale = scale.into_homogeneous_matrix();

        context.apply_uniforms(&GlUniform { offset: (0., 0.), trans: create_glmat4(&mut transform_rotate), scale: create_glmat4(&mut scale) });

        context.draw(0, 3, 1);
    }

    pub fn render_textured(&self, context: &mut Context, transform: &Transform2D, texture: &Texture2D) {
        let uvs = self.uvs.unwrap_or_else(|| {
            log::error!("No uv map found for shape with Texture2D material. Using default positions.");
            [Position2D { x: 0.0, y: 0.0 }; 3]
        });
        let vertices: [TexturedGlVertex; 3] = [
            TexturedGlVertex { pos: GlVec2::from(&self.vertices[0]), uv: GlVec2::from(&uvs[0]) },
            TexturedGlVertex { pos: GlVec2::from(&self.vertices[1]), uv: GlVec2::from(&uvs[1]) },
            TexturedGlVertex { pos: GlVec2::from(&self.vertices[2]), uv: GlVec2::from(&uvs[2]) },
        ];
        let vertex_buffer = Buffer::immutable(context, BufferType::VertexBuffer, &vertices);
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![miniquad::Texture::from_rgba8(context, texture.width, texture.height, &texture.bytes)],
        };

        let shader = Shader::new(context, shader::VERTEX_TEXTURED, shader::FRAGMENT_TEXTURED, shader::meta(vec!["tex".to_string()])).unwrap();

        let pipeline = Pipeline::new(
            context,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        context.apply_pipeline(&pipeline);
        context.apply_bindings(&bindings);

        let mut transform_rotate = Isometry3::identity();
        transform_rotate.append_translation(Vec3 {
            x: transform.position.x,
            y: transform.position.y,
            z: 1.0,
        });
        transform_rotate.prepend_rotation(Rotor3::from_rotation_xy(transform.angle).normalized());

        let mut scale = Similarity3::identity();
        scale.append_scaling(transform.scale);

        let mut transform_rotate = transform_rotate.into_homogeneous_matrix();
        let mut scale = scale.into_homogeneous_matrix();

        context.apply_uniforms(&GlUniform { offset: (0., 0.), trans: create_glmat4(&mut transform_rotate), scale: create_glmat4(&mut scale) });

        context.draw(0, 3, 1);
    }
}

impl Renderable2D for Triangle {
    fn render(&self, context: &mut Context, material: Option<&Material2D>, transform: &Transform2D) {
        match material.expect("Render function must not be called without a material") {
            Material2D::Color(color) => self.render_colored(context, transform, &color),
            Material2D::Texture(texture) => self.render_textured(context, transform, &texture)
        }
    }
}


mod shader {
    use miniquad::*;

    pub const VERTEX_TEXTURED: &str =
        r#"
            #version 330 core
            in vec2 pos;
            in vec2 uv;

            uniform mat4 trans;
            uniform mat4 scale;
            uniform vec2 offset;

            out lowp vec2 texcoord;
            void main() {
                gl_Position = (trans * vec4(pos, 0, 1)) * scale;
                texcoord = uv;
            }
        "#;

    pub const VERTEX_COLORED: &str =
        r#"
            #version 330 core
            in vec2 pos;
            in vec4 color;

            uniform mat4 trans;
            uniform mat4 scale;
            uniform vec2 offset;

            out lowp vec4 color_lowp;
            void main() {
                gl_Position = (trans * vec4(pos, 0, 1)) * scale;
                color_lowp = color;
            }
        "#;

    pub const FRAGMENT_COLORED: &str =
        r#"
            #version 330 core
            in lowp vec4 color_lowp;
            out vec4 FragColor;
            void main() {
                FragColor = color_lowp;
            }
        "#;

    pub const FRAGMENT_TEXTURED: &str =
        r#"
            #version 330 core
            in lowp vec2 texcoord;
            out vec4 FragColor;
            uniform sampler2D tex;
            void main() {
                FragColor = texture(tex, texcoord);
            }
        "#;

    pub fn meta(images: Vec<String>) -> ShaderMeta {
        ShaderMeta {
            images,
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("trans", UniformType::Mat4),
                    UniformDesc::new("scale", UniformType::Mat4)],
            },
        }
    }
}
