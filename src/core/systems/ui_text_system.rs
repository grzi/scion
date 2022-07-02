use std::collections::HashSet;
use hecs::{Entity};

use crate::core::components::{
    maths::{coordinates::Coordinates, hierarchy::Parent, transform::Transform},
    ui::{
        font::Font,
        ui_image::UiImage,
        ui_text::{UiText, UiTextImage},
        UiComponent,
    },
};

pub (crate) fn ui_text_bitmap_update_system(world: &mut crate::core::world::World){

    let mut parentToRemove: HashSet<Entity> = HashSet::new();
    let mut toAdd: Vec<(UiTextImage, UiComponent, Transform, Parent)> = Vec::new();

    for (e, (ui_text, transform)) in world.query_mut::<(&mut UiText, &Transform)>(){
        if ui_text.dirty {
            parentToRemove.insert(e);
            let Font::Bitmap {
                texture_path,
                chars,
                width,
                height,
                texture_columns,
                texture_lines,
            } = ui_text.font();
            let texture_width = texture_columns * width;
            let texture_height = texture_lines * height;

            for (index, character) in ui_text.text().chars().enumerate() {
                let (line, column) =
                    Font::find_line_and_column(&chars, *texture_columns, character);

                let uvs = [
                    Coordinates::new(
                        (column * width) / texture_width,
                        (line * height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width) / texture_width,
                        (line * height + height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width + width) / texture_width,
                        (line * height + height) / texture_height,
                    ),
                    Coordinates::new(
                        (column * width + width) / texture_width,
                        (line * height) / texture_height,
                    ),
                ];

                let mut char_transform = Transform::from_xy(index as f32 * (width + 1.), 0.);
                char_transform.set_z(transform.translation().z());
                toAdd.push((
                    UiTextImage(UiImage::new_with_uv_map(
                        *width as f32,
                        *height as f32,
                        texture_path.clone(),
                        uvs,
                    )),
                    UiComponent,
                    char_transform,
                    Parent(e),
                ));
            }
            ui_text.dirty = false;
        }
    }

    let eToRemove = world.query::<(&UiTextImage, &Parent)>().iter()
        .filter(|(_e, (_, p))| parentToRemove.contains(&p.0))
        .map(|(e, _)| e).collect::<Vec<_>>();

    eToRemove.iter().for_each(|e| { let _r = world.remove(*e); });

    toAdd.drain(0..).for_each(|c| {
        world.push(c);
    });
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::components::{
        maths::transform::Transform,
        ui::{
            font::Font,
            ui_text::{UiText, UiTextImage},
        },
    };
    use crate::core::world::World;

    fn get_test_ui_text() -> UiText {
        // First we add an UiText to the world
        let font = Font::Bitmap {
            texture_path: "test".to_string(),
            chars: "abcdefg".to_string(),
            texture_columns: 7.,
            texture_lines: 1.,
            width: 5.,
            height: 5.,
        };

        UiText::new("abf".to_string(), font)
    }

    #[test]
    fn ui_text_without_transform_should_not_generate_ui_image() {
        let mut world = World::default();


        let _entity = world.push((get_test_ui_text(),));

        ui_text_bitmap_update_system(&mut world);

        let cpt = world.query::<&UiTextImage>().iter().count();
        assert_eq!(0, cpt);
    }

    #[test]
    fn ui_text_with_transform_should_generate_ui_image() {
        let mut world = World::default();

        let _entity = world.push((get_test_ui_text(), Transform::default()));

        ui_text_bitmap_update_system(&mut world);

        let cpt  = world.query::<&UiTextImage>().iter().count();
        assert_eq!(3, cpt);
    }
}
