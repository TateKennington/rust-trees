use nannou::prelude::*;

#[derive(Clone)]
pub enum TurtleInt {
    Forward(f32),
    Turn(f32),
    Push,
    Pop,
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
        .size_pixels(400, 400)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        count: 0,
        turtle_string: vec![
            TurtleInt::Forward(200.0),
            TurtleInt::Turn(120.0),
            TurtleInt::Forward(200.0),
            TurtleInt::Turn(120.0),
            TurtleInt::Forward(200.0),
        ],
    }
}

fn koch_snowflake(turtle_string: &[TurtleInt]) -> Vec<TurtleInt> {
    turtle_string
        .iter()
        .flat_map(|int| {
            if let TurtleInt::Forward(amount) = int {
                vec![
                    TurtleInt::Forward(amount / 3.0),
                    TurtleInt::Turn(-60.0),
                    TurtleInt::Forward(amount / 3.0),
                    TurtleInt::Turn(120.0),
                    TurtleInt::Forward(amount / 3.0),
                    TurtleInt::Turn(-60.0),
                    TurtleInt::Forward(amount / 3.0),
                ]
            } else {
                vec![int.clone()]
            }
        })
        .collect()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.count % 60 == 0 {
        model.turtle_string = koch_snowflake(&model.turtle_string);
    }
    model.count += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let mut turtle_pos = Vec2::new(win.w() / 2.0 - 80.0, -win.h() / 2.0 + 20.0);
    let mut turtle_dir = Vec2::Y;
    for int in &model.turtle_string {
        match int {
            TurtleInt::Forward(amount) => {
                draw.line()
                    .weight(1.0)
                    .caps_round()
                    .color(WHITE)
                    .start(turtle_pos)
                    .end(turtle_pos + turtle_dir * *amount);
                turtle_pos += turtle_dir * *amount
            }
            TurtleInt::Turn(deg) => {
                let rad = deg.to_radians();
                turtle_dir = turtle_dir.rotate(rad);
            }
            _ => unimplemented!("Turtle instruction not yet implemented"),
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
