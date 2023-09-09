#![allow(dead_code)]

use std::time::Duration;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update_step)
        .size(640, 640)
        .simple_window(view)
        .loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(300) })
        .run();
}

#[derive(Default,Clone)]
struct Node {
    pos: Vec2,
    pos_old: Vec2,
}

#[derive(Default)]
struct Rope {
    nodes: Vec<Node>,
}

#[derive(Default)]
struct Model {
    rope: Rope,
}

fn model(_app: &App) -> Model {
    let mut m = Model::default();
    m.rope = Rope {
        nodes: vec![Node::default(); 100],
    };
    for i in 0..m.rope.nodes.len() {
        m.rope.nodes[i].pos = Vec2::new(0.2 * i.to_f32().unwrap(), 0.0);
        m.rope.nodes[i].pos_old = m.rope.nodes[i].pos;
    }
    m
}

fn update_step(app: &App, model: &mut Model, _update: Update) {
    let num_substeps = 2;
    for _ in 0..num_substeps {
        update_substep(app, model, _update, _update.since_last.as_secs_f32() / num_substeps.to_f32().unwrap());
    }
}

fn update_substep(app: &App, model: &mut Model, _update: Update, dt: f32) {
    let node_distance = 0.15;

    let gravity_acc = Vec2::new(0.0, -9.81);

    let nodes = &mut model.rope.nodes;

    // Apply gravity and continued movement to all nodes (except the first one)
    for i in 1..nodes.len() {
        // update node
        let pos_old = nodes[i].pos_old;
        let pos_curr = nodes[i].pos;
        let pos_next = pos_curr + (pos_curr - pos_old) + gravity_acc * dt * dt;
        nodes[i].pos = pos_next;
        nodes[i].pos_old = pos_curr;
    }

    // Apply constraints
    for _iterations in 0..10 {
        for i in 0..nodes.len()-1 {

            // move node 0 towards mouse
            if i == 0 {
                let mouse_pos = app.mouse.position() / 100.0;
                nodes[i].pos = mouse_pos;
            }

            let node1 = &nodes[i];
            let node2 = &nodes[i+1];


            let diff = node1.pos - node2.pos;
            let dist = diff.length();
            let diff_ratio = if dist > 0.0 { (node_distance - dist) / dist } else { 0.0 };
            nodes[i].pos += diff * diff_ratio * 0.5;
            nodes[i+1].pos -= diff * diff_ratio * 0.5;
        }
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(PLUM);
    let draw = app.draw();

    // loop nodes
    for i in 0..model.rope.nodes.len() {
        let node = &model.rope.nodes[i];
        draw.ellipse()
            .x_y(100.0*node.pos.x, 100.0*node.pos.y)
            .radius(100.0 * 0.08)
            .color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}