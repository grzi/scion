use hecs::Component;

use crate::core::components::{Square, Triangle};
use crate::core::components::material::Material;
use crate::core::components::maths::camera::Camera;
use crate::core::components::maths::transform::Transform;
use crate::core::components::shapes::line::Line;
use crate::core::components::shapes::polygon::Polygon;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::components::tiles::sprite::Sprite;
use crate::core::components::tiles::tilemap::Tilemap;
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::UiTextImage;
use crate::core::components::ui::UiComponent;
use crate::core::world::{GameData, World};
use crate::graphics::rendering::{Renderable2D, RenderingUpdate};
use crate::graphics::rendering::shaders::gl_representations::{GlUniform, UniformData};
use crate::graphics::rendering::scion2d::pre_renderer::Scion2DPreRenderer;

pub(crate) fn call(renderer: &mut Scion2DPreRenderer, data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    updates.append(&mut update_transforms_for_type::<Triangle>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Square>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Rectangle>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Sprite>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Line>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Polygon>(renderer, data));
    updates.append(&mut update_transforms_for_type::<UiImage>(renderer, data));
    updates.append(&mut update_transforms_for_type::<UiTextImage>(renderer, data));
    updates.append(&mut update_transforms_for_type::<Tilemap>(renderer, data));
    updates
}

fn update_transforms_for_type<T: Component + Renderable2D>(
    _renderer: &mut Scion2DPreRenderer,
    data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    let camera1 = {
        let mut t = Transform::default();
        let mut c = Camera::new(1.0, 1.0);

        for (_, (cam, tra)) in data.query::<(&Camera, &Transform)>().iter() {
            c = cam.clone();
            t = *tra;
        }
        (c, t)
    };
    let camera = (&camera1.0, &camera1.1);
    for (entity, (transform, optional_ui_component, renderable, optional_material)) in
    data.query::<(&Transform, Option<&UiComponent>, &T, Option<&Material>)>().iter() {
        // TODO : update only if needed ?
        let uniform = GlUniform::from(UniformData {
            transform,
            camera,
            is_ui_component: optional_ui_component.is_some(),
            pivot_offset: renderable.get_pivot_offset(optional_material),
        });
        updates.push(RenderingUpdate::TransformUniform {
            entity,
            uniform,
        });
    }
    updates
}

