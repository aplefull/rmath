use macroquad::prelude::*;
use rmath::{
    trigonometric::*,
    hyperbolic::*,
    power::*,
    logarithmic::*,
    lambert_w::*,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlotFunction {
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Exp,
    Log,
    Sqrt,
    Arcsin,
    Arctan,
    ProductLog,
}

impl PlotFunction {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            PlotFunction::Sin => sin(x),
            PlotFunction::Cos => cos(x),
            PlotFunction::Tan => tan(x),
            PlotFunction::Sinh => sinh(x),
            PlotFunction::Cosh => cosh(x),
            PlotFunction::Exp => exp(x),
            PlotFunction::Log => log(x),
            PlotFunction::Sqrt => sqrt(x),
            PlotFunction::Arcsin => arcsin(x),
            PlotFunction::Arctan => arctan(x),
            PlotFunction::ProductLog => product_log(x),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            PlotFunction::Sin => "sin(x)",
            PlotFunction::Cos => "cos(x)",
            PlotFunction::Tan => "tan(x)",
            PlotFunction::Sinh => "sinh(x)",
            PlotFunction::Cosh => "cosh(x)",
            PlotFunction::Exp => "exp(x)",
            PlotFunction::Log => "ln(x)",
            PlotFunction::Sqrt => "√x",
            PlotFunction::Arcsin => "arcsin(x)",
            PlotFunction::Arctan => "arctan(x)",
            PlotFunction::ProductLog => "W(x)",
        }
    }

    fn color(&self) -> Color {
        match self {
            PlotFunction::Sin => RED,
            PlotFunction::Cos => BLUE,
            PlotFunction::Tan => GREEN,
            PlotFunction::Sinh => ORANGE,
            PlotFunction::Cosh => PURPLE,
            PlotFunction::Exp => MAGENTA,
            PlotFunction::Log => DARKGREEN,
            PlotFunction::Sqrt => BROWN,
            PlotFunction::Arcsin => PINK,
            PlotFunction::Arctan => SKYBLUE,
            PlotFunction::ProductLog => DARKBLUE,
        }
    }

    fn default_domain(&self) -> (f32, f32, f32, f32) {
        match self {
            PlotFunction::Sin | PlotFunction::Cos => (-6.28, 6.28, -1.5, 1.5),
            PlotFunction::Tan => (-3.14, 3.14, -5.0, 5.0),
            PlotFunction::Sinh | PlotFunction::Cosh => (-3.0, 3.0, -5.0, 5.0),
            PlotFunction::Exp => (-2.0, 3.0, 0.0, 8.0),
            PlotFunction::Log => (0.1, 10.0, -3.0, 3.0),
            PlotFunction::Sqrt => (0.0, 10.0, 0.0, 4.0),
            PlotFunction::Arcsin => (-1.0, 1.0, -2.0, 2.0),
            PlotFunction::Arctan => (-5.0, 5.0, -2.0, 2.0),
            PlotFunction::ProductLog => (-0.35, 5.0, -2.0, 2.0),
        }
    }
    

    fn is_valid_domain(&self, x: f64) -> bool {
        match self {
            PlotFunction::Log => x > 0.0,
            PlotFunction::Sqrt => x >= 0.0,
            PlotFunction::Arcsin => x >= -1.0 && x <= 1.0,
            PlotFunction::ProductLog => x >= -1.0 / std::f64::consts::E,
            _ => true,
        }
    }
}

const ALL_FUNCTIONS: &[PlotFunction] = &[
    PlotFunction::Sin,
    PlotFunction::Cos,
    PlotFunction::Tan,
    PlotFunction::Sinh,
    PlotFunction::Cosh,
    PlotFunction::Exp,
    PlotFunction::Log,
    PlotFunction::Sqrt,
    PlotFunction::Arcsin,
    PlotFunction::Arctan,
    PlotFunction::ProductLog,
];

struct PlotCamera {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    dragging: bool,
    drag_start_x: f32,
    drag_start_y: f32,
    last_mouse_x: f32,
    last_mouse_y: f32,
}

impl PlotCamera {
    fn new() -> Self {
        PlotCamera {
            x_min: -6.28,
            x_max: 6.28,
            y_min: -2.0,
            y_max: 2.0,
            dragging: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
        }
    }

    fn set_bounds(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        self.x_min = x_min;
        self.x_max = x_max;
        self.y_min = y_min;
        self.y_max = y_max;
    }

    fn screen_to_world(&self, screen_x: f32, screen_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let x = self.x_min + (screen_x / screen_width) * (self.x_max - self.x_min);
        let y = self.y_max - (screen_y / screen_height) * (self.y_max - self.y_min);
        (x, y)
    }

    fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let screen_x = ((world_x - self.x_min) / (self.x_max - self.x_min)) * screen_width;
        let screen_y = ((self.y_max - world_y) / (self.y_max - self.y_min)) * screen_height;
        (screen_x, screen_y)
    }

    fn update(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        let wheel_y = mouse_wheel().1;
        if wheel_y != 0.0 {
            let zoom_factor = if wheel_y > 0.0 { 0.9 } else { 1.1 };

            let (mouse_world_x, mouse_world_y) = self.screen_to_world(
                mouse_x, mouse_y, screen_width(), screen_height()
            );

            let dx = mouse_world_x - self.x_min;
            let new_dx = dx * zoom_factor;
            let delta_x_min = dx - new_dx;

            let dx2 = self.x_max - mouse_world_x;
            let new_dx2 = dx2 * zoom_factor;
            let delta_x_max = new_dx2 - dx2;

            let dy = mouse_world_y - self.y_min;
            let new_dy = dy * zoom_factor;
            let delta_y_min = dy - new_dy;

            let dy2 = self.y_max - mouse_world_y;
            let new_dy2 = dy2 * zoom_factor;
            let delta_y_max = new_dy2 - dy2;

            self.x_min += delta_x_min;
            self.x_max += delta_x_max;
            self.y_min += delta_y_min;
            self.y_max += delta_y_max;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.dragging = true;
            self.drag_start_x = mouse_x;
            self.drag_start_y = mouse_y;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let dx = mouse_x - self.last_mouse_x;
            let dy = mouse_y - self.last_mouse_y;

            if dx != 0.0 || dy != 0.0 {
                let dx_world = -dx * (self.x_max - self.x_min) / screen_width();
                let dy_world = dy * (self.y_max - self.y_min) / screen_height();

                self.x_min += dx_world;
                self.x_max += dx_world;
                self.y_min += dy_world;
                self.y_max += dy_world;
            }
        }

        self.last_mouse_x = mouse_x;
        self.last_mouse_y = mouse_y;
    }
}

struct PlotterApp {
    camera: PlotCamera,
    current_function: PlotFunction,
    show_ui: bool,
    sample_points: usize,
    line_thickness: f32,
}

impl PlotterApp {
    fn new() -> Self {
        PlotterApp {
            camera: PlotCamera::new(),
            current_function: PlotFunction::Sin,
            show_ui: true,
            sample_points: 2000,
            line_thickness: 2.5,
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Key1) { self.set_function(PlotFunction::Sin); }
        if is_key_pressed(KeyCode::Key2) { self.set_function(PlotFunction::Cos); }
        if is_key_pressed(KeyCode::Key3) { self.set_function(PlotFunction::Tan); }
        if is_key_pressed(KeyCode::Key4) { self.set_function(PlotFunction::Sinh); }
        if is_key_pressed(KeyCode::Key5) { self.set_function(PlotFunction::Cosh); }
        if is_key_pressed(KeyCode::Key6) { self.set_function(PlotFunction::Exp); }
        if is_key_pressed(KeyCode::Key7) { self.set_function(PlotFunction::Log); }
        if is_key_pressed(KeyCode::Key8) { self.set_function(PlotFunction::Sqrt); }
        if is_key_pressed(KeyCode::Key9) { self.set_function(PlotFunction::Arcsin); }
        if is_key_pressed(KeyCode::Key0) { self.set_function(PlotFunction::Arctan); }
        if is_key_pressed(KeyCode::Minus) { self.set_function(PlotFunction::ProductLog); }

        if is_key_pressed(KeyCode::H) {
            self.show_ui = !self.show_ui;
        }

        if is_key_pressed(KeyCode::R) {
            let (x_min, x_max, y_min, y_max) = self.current_function.default_domain();
            self.camera.set_bounds(x_min, x_max, y_min, y_max);
        }

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Up) {
            self.next_function();
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Down) {
            self.prev_function();
        }
    }

    fn set_function(&mut self, func: PlotFunction) {
        if self.current_function != func {
            self.current_function = func;
            let (x_min, x_max, y_min, y_max) = func.default_domain();
            self.camera.set_bounds(x_min, x_max, y_min, y_max);
        }
    }

    fn next_function(&mut self) {
        if let Some(current_idx) = ALL_FUNCTIONS.iter().position(|&f| f == self.current_function) {
            let next_idx = (current_idx + 1) % ALL_FUNCTIONS.len();
            self.set_function(ALL_FUNCTIONS[next_idx]);
        }
    }

    fn prev_function(&mut self) {
        if let Some(current_idx) = ALL_FUNCTIONS.iter().position(|&f| f == self.current_function) {
            let prev_idx = if current_idx == 0 { ALL_FUNCTIONS.len() - 1 } else { current_idx - 1 };
            self.set_function(ALL_FUNCTIONS[prev_idx]);
        }
    }
}

fn draw_grid(camera: &PlotCamera) {
    let width = screen_width();
    let height = screen_height();

    draw_rectangle(0.0, 0.0, width, height, WHITE);

    let grid_color = Color::new(0.9, 0.9, 0.9, 1.0);
    let axis_color = Color::new(0.3, 0.3, 0.3, 1.0);

    let x_range = camera.x_max - camera.x_min;
    let y_range = camera.y_max - camera.y_min;

    let x_grid_step = calculate_grid_step(x_range);
    let y_grid_step = calculate_grid_step(y_range);

    let x_start = (camera.x_min / x_grid_step).floor() * x_grid_step;
    let x_end = (camera.x_max / x_grid_step).ceil() * x_grid_step;

    for x in (0..=((x_end - x_start) / x_grid_step) as i32).map(|i| x_start + i as f32 * x_grid_step) {
        let (screen_x, _) = camera.world_to_screen(x, 0.0, width, height);

        if screen_x >= 0.0 && screen_x <= width {
            let line_color = if x.abs() < 1e-6 { axis_color } else { grid_color };
            let thickness = if x.abs() < 1e-6 { 2.0 } else { 1.0 };
            draw_line(screen_x, 0.0, screen_x, height, thickness, line_color);

            if x.abs() > 1e-6 {
                let label = if x_grid_step >= 1.0 {
                    format!("{:.0}", x)
                } else {
                    format!("{:.2}", x)
                };
                let text_params = TextParams {
                    font_size: 12,
                    color: DARKGRAY,
                    ..Default::default()
                };
                draw_text_ex(&label, screen_x + 2.0, height - 5.0, text_params);
            }
        }
    }

    let y_start = (camera.y_min / y_grid_step).floor() * y_grid_step;
    let y_end = (camera.y_max / y_grid_step).ceil() * y_grid_step;

    for y in (0..=((y_end - y_start) / y_grid_step) as i32).map(|i| y_start + i as f32 * y_grid_step) {
        let (_, screen_y) = camera.world_to_screen(0.0, y, width, height);

        if screen_y >= 0.0 && screen_y <= height {
            let line_color = if y.abs() < 1e-6 { axis_color } else { grid_color };
            let thickness = if y.abs() < 1e-6 { 2.0 } else { 1.0 };
            draw_line(0.0, screen_y, width, screen_y, thickness, line_color);

            if y.abs() > 1e-6 {
                let label = if y_grid_step >= 1.0 {
                    format!("{:.0}", y)
                } else {
                    format!("{:.2}", y)
                };
                let text_params = TextParams {
                    font_size: 12,
                    color: DARKGRAY,
                    ..Default::default()
                };
                draw_text_ex(&label, 5.0, screen_y - 2.0, text_params);
            }
        }
    }
}

fn calculate_grid_step(range: f32) -> f32 {
    let raw_step = 10.0_f32.powf((range.log10() - 0.7).floor());
    if range / raw_step > 8.0 {
        raw_step * 2.0
    } else if range / raw_step > 4.0 {
        raw_step
    } else if range / raw_step > 2.0 {
        raw_step / 2.0
    } else {
        raw_step / 4.0
    }
}

fn draw_function(camera: &PlotCamera, func: PlotFunction, app: &PlotterApp) {
    let width = screen_width();
    let height = screen_height();
    let num_points = app.sample_points;
    let dx = (camera.x_max - camera.x_min) / (num_points as f32);

    let mut segments = Vec::new();
    let mut current_segment = Vec::new();

    for i in 0..=num_points {
        let x = camera.x_min + i as f32 * dx;

        if !func.is_valid_domain(x as f64) {
            if !current_segment.is_empty() {
                segments.push(current_segment);
                current_segment = Vec::new();
            }
            continue;
        }

        let y = func.evaluate(x as f64) as f32;

        if y.is_nan() || y.is_infinite() {
            if !current_segment.is_empty() {
                segments.push(current_segment);
                current_segment = Vec::new();
            }
            continue;
        }

        let y_buffer = (camera.y_max - camera.y_min) * 0.1;
        if y < camera.y_min - y_buffer || y > camera.y_max + y_buffer {
            if !current_segment.is_empty() {
                segments.push(current_segment);
                current_segment = Vec::new();
            }
            continue;
        }

        let (screen_x, screen_y) = camera.world_to_screen(x, y, width, height);
        current_segment.push((screen_x, screen_y, x, y));
    }

    if !current_segment.is_empty() {
        segments.push(current_segment);
    }

    let color = func.color();
    for segment in segments {
        if segment.len() < 2 {
            continue;
        }

        for thickness_offset in 0..3 {
            let thickness = app.line_thickness + thickness_offset as f32 * 0.5;
            let alpha = match thickness_offset {
                0 => 1.0,
                1 => 0.3,
                2 => 0.1,
                _ => 0.0,
            };

            let segment_color = Color::new(color.r, color.g, color.b, color.a * alpha);

            for i in 1..segment.len() {
                let (x1, y1, _, _) = segment[i - 1];
                let (x2, y2, _, _) = segment[i];

                let dx = x2 - x1;
                let dy = y2 - y1;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < 100.0 {
                    draw_line(x1, y1, x2, y2, thickness, segment_color);
                }
            }
        }
    }

    match func {
        PlotFunction::ProductLog => {
            let branch_point = -1.0 / std::f64::consts::E;
            let (critical_x, critical_y) = camera.world_to_screen(
                branch_point as f32,
                -1.0,
                width,
                height
            );

            if critical_x >= 0.0 && critical_x <= width && critical_y >= 0.0 && critical_y <= height {
                draw_circle(critical_x, critical_y, 6.0, func.color());
                draw_circle(critical_x, critical_y, 4.0, WHITE);
            }
        }
        _ => {}
    }
}

fn draw_ui(camera: &PlotCamera, app: &PlotterApp) {
    if !app.show_ui {
        return;
    }

    let mouse_pos = mouse_position();
    let (world_x, world_y) = camera.screen_to_world(
        mouse_pos.0,
        mouse_pos.1,
        screen_width(),
        screen_height()
    );

    let func_value = if app.current_function.is_valid_domain(world_x as f64) {
        app.current_function.evaluate(world_x as f64)
    } else {
        f64::NAN
    };

    let coords_text = format!(
        "x: {:.4}, y: {:.4}, {}({:.4}): {:.4}",
        world_x,
        world_y,
        app.current_function.name(),
        world_x,
        func_value
    );

    let text_params = TextParams {
        font_size: 14,
        color: BLACK,
        ..Default::default()
    };

    draw_rectangle(10.0, 10.0, 400.0, 25.0, Color::new(1.0, 1.0, 1.0, 0.9));
    draw_text_ex(&coords_text, 15.0, 30.0, text_params.clone());

    let title = format!("Function Plotter - {}", app.current_function.name());
    draw_text_ex(
        &title,
        screen_width() / 2.0 - 100.0,
        30.0,
        TextParams {
            font_size: 20,
            color: app.current_function.color(),
            ..Default::default()
        }
    );

    let help_lines = [
        "Controls:",
        "• Mouse: Pan (drag) & Zoom (wheel)",
        "• 1-9,0,-: Select function",
        "• ← → ↑ ↓: Cycle functions",
        "• R: Reset view",
        "• H: Toggle help",
    ];

    let help_y_start = screen_height() - (help_lines.len() as f32 * 20.0) - 10.0;
    draw_rectangle(
        10.0,
        help_y_start - 5.0,
        280.0,
        (help_lines.len() as f32 * 20.0) + 10.0,
        Color::new(1.0, 1.0, 1.0, 0.9)
    );

    for (i, line) in help_lines.iter().enumerate() {
        draw_text_ex(
            line,
            15.0,
            help_y_start + (i as f32 * 20.0) + 15.0,
            TextParams {
                font_size: 12,
                color: DARKGRAY,
                ..Default::default()
            }
        );
    }

    let functions_per_row = 6;
    let start_x = screen_width() - 400.0;
    let start_y = 60.0;

    draw_rectangle(start_x - 5.0, start_y - 5.0, 390.0, 80.0, Color::new(1.0, 1.0, 1.0, 0.9));

    draw_text_ex(
        "Available Functions:",
        start_x,
        start_y + 10.0,
        TextParams {
            font_size: 14,
            color: BLACK,
            ..Default::default()
        }
    );

    for (i, &func) in ALL_FUNCTIONS.iter().enumerate() {
        let row = i / functions_per_row;
        let col = i % functions_per_row;
        let x = start_x + (col as f32 * 65.0);
        let y = start_y + 25.0 + (row as f32 * 20.0);

        let color = if func == app.current_function {
            func.color()
        } else {
            DARKGRAY
        };

        let key = match i {
            0..=8 => format!("{}:", i + 1),
            9 => "0:".to_string(),
            10 => "-:".to_string(),
            _ => "".to_string(),
        };

        draw_text_ex(
            &format!("{}{}", key, func.name()),
            x,
            y,
            TextParams {
                font_size: 11,
                color,
                ..Default::default()
            }
        );
    }
}

#[macroquad::main("Plotter")]
async fn main() {
    let mut app = PlotterApp::new();

    loop {
        clear_background(WHITE);

        app.handle_input();

        app.camera.update();

        draw_grid(&app.camera);

        draw_function(&app.camera, app.current_function, &app);

        draw_ui(&app.camera, &app);

        next_frame().await;
    }
}