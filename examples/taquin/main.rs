use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            maths::{coordinates::Coordinates, transform::Transform},
            tiles::{sprite::Sprite, tileset::Tileset},
        },
        scene::{Scene},
        legion_ext::{ScionResourcesExtension, ScionWorldExtension},
        resources::inputs::inputs_controller::InputsController,
    },
    legion::{Resources, system, World},
    Scion,
    utils::file::app_base_path,
};

#[derive(Debug)]
struct Case(Coordinates);

enum MoveDirection {
    Left,
    Top,
    Right,
    Bottom,
    None,
}

struct Taquin {
    board: [[bool; 4]; 4],
}

impl Taquin {
    fn new() -> Self {
        let mut board = [[true; 4]; 4];
        board[3][3] = false;
        Self { board }
    }

    fn try_move(&mut self, column: usize, line: usize) -> MoveDirection {
        self.board[column][line] = false;
        if column > 0 && !self.board[column - 1][line] {
            self.board[column - 1][line] = true;
            MoveDirection::Left
        } else if line > 0 && !self.board[column][line - 1] {
            self.board[column][line - 1] = true;
            MoveDirection::Top
        } else if column < 3 && !self.board[column + 1][line] {
            self.board[column + 1][line] = true;
            MoveDirection::Right
        } else if line < 3 && !self.board[column][line + 1] {
            self.board[column][line + 1] = true;
            MoveDirection::Bottom
        } else {
            self.board[column][line] = true;
            MoveDirection::None
        }
    }
}

#[system(for_each)]
fn taquin(
    #[resource] inputs: &InputsController,
    #[resource] taquin: &mut Taquin,
    case: &mut Case,
    transform: &mut Transform,
) {
    inputs.on_left_click_pressed(|mouse_x, mouse_y| {
        if mouse_x > (case.0.x() * 192.) as f64
            && mouse_y > (case.0.y() * 192.) as f64
            && mouse_x < (case.0.x() * 192. + 192.) as f64
            && mouse_y < (case.0.y() * 192. + 192.) as f64
        {
            match taquin.try_move(case.0.x() as usize, case.0.y() as usize) {
                MoveDirection::Left => {
                    case.0.set_x(case.0.x() - 1.);
                    transform.append_translation(-192., 0.);
                }
                MoveDirection::Top => {
                    case.0.set_y(case.0.y() - 1.);
                    transform.append_translation(0., -192.);
                }
                MoveDirection::Right => {
                    case.0.set_x(case.0.x() + 1.);
                    transform.append_translation(192., 0.);
                }
                MoveDirection::Bottom => {
                    case.0.set_y(case.0.y() + 1.);
                    transform.append_translation(0., 192.);
                }
                MoveDirection::None => {}
            };
        }
    })
}

#[derive(Default)]
struct MainScene;

impl Scene for MainScene {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        let tileset_ref = resources.assets().register_tileset(Tileset::new(
            app_base_path().join("examples/taquin/assets/taquin.png").get(),
            4,
            4,
            192,
        ));
        for line in 0..4 {
            for column in 0..4 {
                if !(line == 3 && column == 3) {
                    let square = (
                        Transform::from_xy(column as f32 * 192., line as f32 * 192.),
                        tileset_ref.clone(),
                        Sprite::new(line * 4 + column),
                        Case(Coordinates::new(column as f32, line as f32)),
                    );
                    world.push(square);
                }
            }
        }
        world.add_default_camera();

        resources.insert(Taquin::new());
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new().with_resizable(false).with_dimensions((768, 768)).get(),
            )
            .get(),
    )
        .with_system(taquin_system())
        .with_scene::<MainScene>()
        .run();
}
