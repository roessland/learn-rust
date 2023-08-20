use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .size(640, 360)
        .simple_window(view)
        .run();
}

#[derive(Default)]
struct Model {
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn min_screen_dimension(_app: &App) -> f32 {
    let inner_size = _app.main_window().inner_size_points();
    f32::min(inner_size.0, inner_size.1)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let scale = f64::sin(app.duration.since_start.as_secs_f64()) as f32;
    let min_screen_dimension = min_screen_dimension(app);
    let padding =  0.05 * min_screen_dimension;
    let min_radius = padding;
    let max_radius =  min_screen_dimension  / 2.0 - padding;
    model.radius = min_radius + scale * (max_radius - min_radius);
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(PLUM);
    let draw = app.draw();

    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(model.radius)
        .color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}