use nannou::{prelude::*, rand::Rng};

#[derive(Clone)]
pub enum TurtleInt {
    Forward,
    Turn,
    Push,
    Pop,
    SetSpeed(f32),
    MultSpeed(f32),
    SetAngle(f32),
    MultAngle(f32),
    SetWeight(f32),
    MultWeight(f32),
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    count: usize,
    turtle_string: Vec<TurtleInt>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size_pixels(1000, 1000)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        count: 0,
        turtle_string: vec![],
    }
}

fn koch_snowflake(turtle_string: &[TurtleInt]) -> Vec<TurtleInt> {
    if turtle_string.is_empty() {
        return vec![
            TurtleInt::Forward,
            TurtleInt::Turn,
            TurtleInt::Forward,
            TurtleInt::Turn,
            TurtleInt::Forward,
        ];
    }
    turtle_string
        .iter()
        .flat_map(|int| {
            if let TurtleInt::Forward = int {
                vec![
                    TurtleInt::Forward,
                    TurtleInt::Turn,
                    TurtleInt::Forward,
                    TurtleInt::Turn,
                    TurtleInt::Forward,
                    TurtleInt::Turn,
                    TurtleInt::Forward,
                ]
            } else {
                vec![int.clone()]
            }
        })
        .collect()
}

fn crazy_tree(turtle_string: &[TurtleInt]) -> Vec<TurtleInt> {
    if turtle_string.is_empty() {
        return vec![TurtleInt::Push, TurtleInt::Forward, TurtleInt::Pop];
    }
    turtle_string
        .iter()
        .flat_map(|int| {
            if let TurtleInt::Pop = int {
                if random_range(0.0, 1.0) <= 0.5 {
                    vec![TurtleInt::Push, TurtleInt::Forward, TurtleInt::Pop]
                } else {
                    vec![
                        TurtleInt::Push,
                        TurtleInt::Turn,
                        TurtleInt::Forward,
                        TurtleInt::Pop,
                        TurtleInt::Forward,
                        TurtleInt::Pop,
                    ]
                }
            } else {
                vec![int.clone()]
            }
        })
        .collect()
}

fn tree(turtle_string: &[TurtleInt]) -> Vec<TurtleInt> {
    if turtle_string.is_empty() {
        return vec![TurtleInt::Push, TurtleInt::Forward, TurtleInt::Pop];
    }
    turtle_string
        .iter()
        .flat_map(|int| {
            if let TurtleInt::Pop = int {
                if random_range(0.0, 1.0) <= 0.1 {
                    vec![
                        TurtleInt::MultWeight(0.9),
                        TurtleInt::Forward,
                        TurtleInt::Pop,
                    ]
                } else {
                    let scale = random_range(0.6, 0.7)
                        * if random_range(0.0, 1.0) <= 0.5 {
                            -1.0
                        } else {
                            1.0
                        };
                    vec![
                        TurtleInt::Push,
                        TurtleInt::MultAngle(scale),
                        TurtleInt::MultSpeed(0.6),
                        TurtleInt::MultWeight(0.6),
                        TurtleInt::Turn,
                        TurtleInt::Forward,
                        TurtleInt::Pop,
                        TurtleInt::MultWeight(0.8),
                        TurtleInt::Forward,
                        TurtleInt::Pop,
                    ]
                }
            } else {
                vec![int.clone()]
            }
        })
        .collect()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.count > 60 * 15 {
        return;
    }
    if model.count % 60 == 0 {
        model.turtle_string = tree(&model.turtle_string);
    }
    model.count += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let mut turtle_pos = Vec2::new(0.0, -win.h() / 2.0 + 20.0);
    let mut turtle_dir = Vec2::Y;
    let mut turtle_speed = 50.0;
    let mut turtle_angle = 90.0;
    let mut turtle_weight = 10.0;
    let mut stack: Vec<(Vec2, Vec2, f32, f32, f32)> = Vec::default();
    for int in &model.turtle_string {
        match int {
            TurtleInt::Forward => {
                draw.line()
                    .caps_butt()
                    .weight(turtle_weight)
                    .color(if turtle_weight < 1.0 { GREEN } else { BROWN })
                    .start(turtle_pos)
                    .end(turtle_pos + turtle_dir * turtle_speed);
                turtle_pos += turtle_dir * turtle_speed
            }
            TurtleInt::Turn => {
                let rad = turtle_angle.to_radians();
                turtle_dir = turtle_dir.rotate(rad);
            }
            TurtleInt::Push => {
                stack.push((
                    turtle_pos,
                    turtle_dir,
                    turtle_speed,
                    turtle_angle,
                    turtle_weight,
                ));
            }
            TurtleInt::Pop => {
                let state = stack.pop().unwrap();
                turtle_pos = state.0;
                turtle_dir = state.1;
                turtle_speed = state.2;
                turtle_angle = state.3;
                turtle_weight = state.4;
            }
            TurtleInt::MultAngle(factor) => turtle_angle *= factor,
            TurtleInt::MultSpeed(factor) => turtle_speed *= factor,
            TurtleInt::MultWeight(factor) => turtle_weight *= factor,
            _ => unimplemented!("Turtle instruction not yet implemented"),
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
