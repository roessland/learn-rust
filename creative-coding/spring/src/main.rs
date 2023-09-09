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

#[derive(Default, Clone, Copy)]
struct Node {
    pos: Vec2,
    pos_old: Vec2,
    mass: f32,
}

fn vel(node: Node) -> Vec2 {
    node.pos - node.pos_old
}

fn pos(node: Node) -> Vec2 {
    0.5 * (node.pos - node.pos_old)
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
        nodes: vec![Node::default(); 2],
    };
    for i in 0..m.rope.nodes.len() {
        m.rope.nodes[i].pos = Vec2::new(0.2, -0.2 * i.to_f32().unwrap());
        m.rope.nodes[i].pos_old = m.rope.nodes[i].pos;
        m.rope.nodes[i].mass = 1.0;
    }
    m
}

fn update_step(app: &App, model: &mut Model, _update: Update) {
    let num_substeps = 20;
    for _ in 0..num_substeps {
        update_substep(app, model, _update, _update.since_last.as_secs_f32() / num_substeps.to_f32().unwrap());
    }
}

fn spring_force(node1: Node, node2: Node) -> Vec2 {
    let spring_constant = 50.0;
    let spring_length = 0.2;
    let delta = 0.5 * (node2.pos - node1.pos  + node2.pos_old - node1.pos_old);
    let delta_length = delta.length();
    let delta_normalized = delta / delta_length;
    let spring_force = delta_normalized * (delta_length - spring_length) * spring_constant;
    spring_force
}

fn spring_energy(node1: Node, node2: Node) -> f32 {
    let spring_constant = 50.0;
    let spring_length = 0.2;
    let delta_now = node2.pos - node1.pos;
    let delta_before = node2.pos_old - node1.pos_old;
    let delta = 0.5 * (delta_now + delta_before);
    let delta_length = delta.length();
    let spring_energy = 0.5 * spring_constant * (delta_length - spring_length) * (delta_length - spring_length);
    spring_energy
}

fn dashpot_force(node1: Node, node2: Node) -> Vec2 {
    let damping_constant = 1.0;
    let delta = node2.pos - node1.pos;
    let delta_length = delta.length();
    let delta_normalized = delta / delta_length;
    let delta_velocity = node2.pos - node1.pos;
    let dashpot_force = delta_normalized * delta_velocity.dot(delta_normalized) * damping_constant;
    dashpot_force
}

fn update_substep(app: &App, model: &mut Model, _update: Update, dt: f32) {
    let gravity_acc = Vec2::new(0.0, -9.81);
    let mass = 1.0;

    let nodes = &mut model.rope.nodes;

    // Apply gravity and continued movement to all nodes (except the first one)
    for i in 1..nodes.len() {
        // update node
        let pos_old = nodes[i].pos_old;
        let pos_curr = nodes[i].pos;
        let gravity_force = mass * gravity_acc;
        let spring_up_force = spring_force(nodes[i], nodes[i - 1]);
        let spring_down_force = if i < nodes.len() - 1 { spring_force(nodes[i], nodes[i + 1]) } else { Vec2::new(0.0, 0.0) };
        let total_force = gravity_force + spring_up_force + spring_down_force;
        let acc = total_force / mass;
        let pos_next = pos_curr + (pos_curr - pos_old) + acc * dt * dt;
        nodes[i].pos = pos_next;
        nodes[i].pos_old = pos_curr;
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn system_lagrangian(model: &Model) -> f32 {
    let nodes = &model.rope.nodes;
    let mut energy = 0.0;
    for i in 1..nodes.len() {
        let node = nodes[i];
        let node_prev = nodes[i - 1];
        let spring_energy = spring_energy(node, node_prev);
        let gravity_energy = 9.81 * pos(node).y * node.mass;
        let kinetic_energy = 0.5 * node.mass * vel(node).length_squared();
        energy = energy + gravity_energy + kinetic_energy + spring_energy;
    }
    energy
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PLUM);
    let draw = app.draw();

    // print system energy
    let energy = system_lagrangian(model);
    draw.text(&format!("Energy: {}", energy))
        .x_y(10.0, 10.0)
        .color(BLACK);

    // loop nodes
    for i in 0..model.rope.nodes.len() {
        let node = &model.rope.nodes[i];
        draw.ellipse()
            .x_y(100.0 * node.pos.x, 100.0 * node.pos.y)
            .radius(100.0 * 0.08)
            .color(STEELBLUE);
        draw.text(&format!("{}", i))
            .x_y(100.0 * node.pos.x, 100.0 * node.pos.y)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}