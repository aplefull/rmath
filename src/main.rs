use macroquad::prelude::*;
use rmath::*;
use egui_macroquad::egui;
use egui_macroquad::egui::{ComboBox, Slider};

#[derive(Clone)]
struct FunctionDescriptor {
    name: String,
    eval_fn: fn(f64) -> f64,
    domain_check: fn(f64) -> bool,
    default_bounds: (f32, f32, f32, f32),
}

impl FunctionDescriptor {
    fn new(
        name: &str,
        eval_fn: fn(f64) -> f64,
        domain_check: fn(f64) -> bool,
        default_bounds: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            name: name.to_string(),
            eval_fn,
            domain_check,
            default_bounds,
        }
    }

    fn evaluate(&self, x: f64) -> f64 {
        (self.eval_fn)(x)
    }

    fn is_valid_domain(&self, x: f64) -> bool {
        (self.domain_check)(x)
    }
}

fn wrapped_lucas(x: f64) -> f64 {
    lucas(x as i32) as f64
}

fn wrapped_fibonacci(x: f64) -> f64 {
    fibonacci(x as i32) as f64
}

fn wrapped_prime(x: f64) -> f64 {
    prime((x as i32).max(1)) as f64
}

fn wrapped_quotient(m: f64) -> f64 {
    quotient(7f64, m)
}

fn create_function_catalog() -> Vec<FunctionDescriptor> {
    vec![
        FunctionDescriptor::new("sin(x)", sin, |_| true, (-6.28, 6.28, -1.5, 1.5)),
        FunctionDescriptor::new("cos(x)", cos, |_| true, (-6.28, 6.28, -1.5, 1.5)),
        FunctionDescriptor::new("tan(x)", tan, |_| true, (-3.14, 3.14, -5.0, 5.0)),
        FunctionDescriptor::new("sinc(x)", sinc, |_| true, (-10.0, 10.0, -0.5, 1.2)),
        FunctionDescriptor::new("arcsin(x)", arcsin, |x| x >= -1.0 && x <= 1.0, (-1.2, 1.2, -2.0, 2.0)),
        FunctionDescriptor::new("arccos(x)", arccos, |x| x >= -1.0 && x <= 1.0, (-1.2, 1.2, 0.0, 3.5)),
        FunctionDescriptor::new("arctan(x)", arctan, |_| true, (-5.0, 5.0, -2.0, 2.0)),

        FunctionDescriptor::new("sinh(x)", sinh, |_| true, (-3.0, 3.0, -5.0, 5.0)),
        FunctionDescriptor::new("cosh(x)", cosh, |_| true, (-3.0, 3.0, 0.0, 8.0)),
        FunctionDescriptor::new("tanh(x)", tanh, |_| true, (-5.0, 5.0, -1.2, 1.2)),

        FunctionDescriptor::new("exp(x)", exp, |_| true, (-2.0, 3.0, 0.0, 8.0)),
        FunctionDescriptor::new("ln(x)", log, |x| x > 0.0, (0.1, 10.0, -3.0, 3.0)),
        FunctionDescriptor::new("log₁₀(x)", log10, |x| x > 0.0, (0.1, 100.0, -2.0, 2.0)),
        FunctionDescriptor::new("log₂(x)", log2, |x| x > 0.0, (0.1, 16.0, -4.0, 4.0)),

        FunctionDescriptor::new("√x", sqrt, |x| x >= 0.0, (0.0, 10.0, 0.0, 4.0)),
        FunctionDescriptor::new("x²", |x| power(x, 2.0), |_| true, (-3.0, 3.0, 0.0, 9.0)),
        FunctionDescriptor::new("x³", |x| power(x, 3.0), |_| true, (-2.0, 2.0, -8.0, 8.0)),
        FunctionDescriptor::new("1/x", |x| 1.0/x, |x| x != 0.0, (-5.0, 5.0, -5.0, 5.0)),

        FunctionDescriptor::new("W(x)", product_log, |x| x >= -1.0/std::f64::consts::E, (-0.35, 5.0, -2.0, 2.0)),
        FunctionDescriptor::new("erf(x)", erf, |_| true, (-3.0, 3.0, -1.2, 1.2)),
        FunctionDescriptor::new("erfc(x)", erfc, |_| true, (-3.0, 3.0, 0.0, 2.2)),

        FunctionDescriptor::new("floor(x)", floor, |_| true, (-5.0, 5.0, -5.0, 5.0)),
        FunctionDescriptor::new("ceil(x)", ceiling, |_| true, (-5.0, 5.0, -5.0, 5.0)),
        FunctionDescriptor::new("round(x)", round, |_| true, (-5.0, 5.0, -5.0, 5.0)),
        FunctionDescriptor::new("fract(x)", fractional_part, |_| true, (-3.0, 3.0, -1.0, 1.0)),
        
        FunctionDescriptor::new("Lucas(n)", wrapped_lucas, |_| true, (0.0, 20.0, 0.0, 30.0)),
        FunctionDescriptor::new("Fibonacci(n)", wrapped_fibonacci, |_| true, (0.0, 20.0, 0.0, 30.0)),
        FunctionDescriptor::new("Prime(n)", wrapped_prime, |_| true, (0.0, 100.0, 0.0, 30.0)),
        FunctionDescriptor::new("Quotient(m, n)", wrapped_quotient, |x| x != 0.0, (-10.0, 10.0, -10.0, 10.0)),

        FunctionDescriptor::new("smoothstep(x)", smoothstep, |_| true, (-0.5, 1.5, -0.2, 1.2)),
        FunctionDescriptor::new("smootherstep(x)", smootherstep, |_| true, (-0.5, 1.5, -0.2, 1.2)),
        FunctionDescriptor::new("clamp(x)", |x| rmath::easing::clamp(x, 0.0, 1.0), |_| true, (-2.0, 2.0, -0.5, 1.5)),
        FunctionDescriptor::new("lerp(0,1,x)", |x| lerp(0.0, 1.0, x), |_| true, (-0.5, 1.5, -0.5, 1.5)),

        FunctionDescriptor::new("square_wave(x)", square_wave, |_| true, (-2.0, 2.0, -1.5, 1.5)),
        FunctionDescriptor::new("sawtooth(x)", sawtooth, |_| true, (-2.0, 2.0, -1.5, 1.5)),
        FunctionDescriptor::new("triangle_wave(x)", triangle_wave, |_| true, (-2.0, 2.0, -1.5, 1.5)),
        FunctionDescriptor::new("pulse_wave(x,0.25)", |x| pulse_wave(x, 0.25), |_| true, (-2.0, 2.0, -1.5, 1.5)),

        FunctionDescriptor::new("step(0,x)", |x| step(0.0, x), |_| true, (-2.0, 2.0, -0.5, 1.5)),
        FunctionDescriptor::new("sign(x)", sign, |_| true, (-3.0, 3.0, -1.5, 1.5)),
        FunctionDescriptor::new("fmod(x,2)", |x| fmod(x, 2.0), |_| true, (-5.0, 5.0, -2.5, 2.5)),

        FunctionDescriptor::new("ease_in_quad(x)", ease_in_quad, |_| true, (-0.2, 1.2, -0.2, 1.2)),
        FunctionDescriptor::new("ease_out_quad(x)", ease_out_quad, |_| true, (-0.2, 1.2, -0.2, 1.2)),
        FunctionDescriptor::new("ease_in_out_quad(x)", ease_in_out_quad, |_| true, (-0.2, 1.2, -0.2, 1.2)),
        FunctionDescriptor::new("bounce(x)", bounce, |_| true, (-1.0, 3.0, -0.5, 2.5)),
        FunctionDescriptor::new("elastic(x)", elastic, |_| true, (-0.2, 1.2, -2.0, 2.0)),

        FunctionDescriptor::new("random(x)", random, |_| true, (-10.0, 10.0, -0.1, 1.1)),
        FunctionDescriptor::new("simple_hash(x)", |x| simple_hash(x) as f64 / (u32::MAX as f64), |_| true, (-10.0, 10.0, -0.1, 1.1)),

    ]
}

struct PlotCamera {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    dragging: bool,
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

    fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let width = screen_width();
        let height = screen_height();
        let x = self.x_min + (screen_x / width) * (self.x_max - self.x_min);
        let y = self.y_max - (screen_y / height) * (self.y_max - self.y_min);
        (x, y)
    }

    fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        let width = screen_width();
        let height = screen_height();
        let screen_x = ((world_x - self.x_min) / (self.x_max - self.x_min)) * width;
        let screen_y = ((self.y_max - world_y) / (self.y_max - self.y_min)) * height;
        (screen_x, screen_y)
    }

    fn update(&mut self, egui_wants_input: bool) {
        let (mouse_x, mouse_y) = mouse_position();

        if !egui_wants_input {
            let wheel_y = mouse_wheel().1;
            if wheel_y != 0.0 {
                let zoom_factor = if wheel_y > 0.0 { 0.9 } else { 1.1 };
                let (mouse_world_x, mouse_world_y) = self.screen_to_world(mouse_x, mouse_y);

                let x_range = self.x_max - self.x_min;
                let y_range = self.y_max - self.y_min;

                let new_x_range = x_range * zoom_factor;
                let new_y_range = y_range * zoom_factor;

                let x_center = mouse_world_x;
                let y_center = mouse_world_y;

                self.x_min = x_center - new_x_range * (mouse_world_x - self.x_min) / x_range;
                self.x_max = self.x_min + new_x_range;
                self.y_min = y_center - new_y_range * (mouse_world_y - self.y_min) / y_range;
                self.y_max = self.y_min + new_y_range;
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                self.dragging = true;
            }

            if is_mouse_button_released(MouseButton::Left) {
                self.dragging = false;
            }

            if self.dragging {
                let dx = mouse_x - self.last_mouse_x;
                let dy = mouse_y - self.last_mouse_y;

                let dx_world = -dx * (self.x_max - self.x_min) / screen_width();
                let dy_world = dy * (self.y_max - self.y_min) / screen_height();

                self.x_min += dx_world;
                self.x_max += dx_world;
                self.y_min += dy_world;
                self.y_max += dy_world;
            }
        } else {
            if self.dragging {
                self.dragging = false;
            }
        }

        self.last_mouse_x = mouse_x;
        self.last_mouse_y = mouse_y;
    }
}

struct PlotterApp {
    camera: PlotCamera,
    functions: Vec<FunctionDescriptor>,
    current_function_index: usize,
    sample_points: usize,
    line_thickness: f32,
    show_coordinates: bool,
    show_crosshair: bool,
    function_color: [f32; 3],
}

impl PlotterApp {
    fn new() -> Self {
        let functions = create_function_catalog();
        PlotterApp {
            camera: PlotCamera::new(),
            functions,
            current_function_index: 0,
            sample_points: 2000,
            line_thickness: 2.5,
            show_coordinates: true,
            show_crosshair: false,
            function_color: [0.2, 0.4, 1.0],
        }
    }

    fn current_function(&self) -> &FunctionDescriptor {
        &self.functions[self.current_function_index]
    }

    fn set_function(&mut self, index: usize) {
        if index < self.functions.len() {
            self.current_function_index = index;
        }
    }


    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::R) {
            let bounds = self.current_function().default_bounds;
            self.camera.set_bounds(bounds.0, bounds.1, bounds.2, bounds.3);
        }
    }

    fn draw_ui(&mut self, egui_ctx: &egui::Context) {
        egui_ctx.set_visuals(egui::Visuals::dark());
        
        egui::Window::new("Function Plotter")
            .default_pos(egui::pos2(10.0, 10.0))
            .default_size(egui::vec2(320.0, 350.0))
            .resizable(true)
            .show(egui_ctx, |ui| {
                ui.heading("Function Selection");
                
                let current_name = self.functions[self.current_function_index].name.clone();
                let mut selected_index = None;
                ComboBox::from_label("Function")
                    .selected_text(&current_name)
                    .show_ui(ui, |ui| {
                        for (i, func) in self.functions.iter().enumerate() {
                            let is_selected = i == self.current_function_index;
                            if ui.selectable_label(is_selected, &func.name).clicked() {
                                selected_index = Some(i);
                            }
                        }
                    });
                
                if let Some(index) = selected_index {
                    self.set_function(index);
                }
                
                ui.separator();
                
                ui.heading("Plot Settings");
                ui.add(Slider::new(&mut self.sample_points, 100..=5000).text("Sample Points"));
                ui.add(Slider::new(&mut self.line_thickness, 0.5..=5.0).text("Line Thickness"));
                
                ui.label("Function Color:");
                ui.color_edit_button_rgb(&mut self.function_color);
                
                ui.checkbox(&mut self.show_coordinates, "Show coordinates");
                ui.checkbox(&mut self.show_crosshair, "Show crosshair lines");
                
                ui.separator();
                
                if ui.button("Reset View (R)").clicked() {
                    let bounds = self.current_function().default_bounds;
                    self.camera.set_bounds(bounds.0, bounds.1, bounds.2, bounds.3);
                }
                
                ui.separator();
                
                ui.label("Controls:");
                ui.label("• Mouse drag: Pan");
                ui.label("• Mouse wheel: Zoom");
                ui.label("• R key: Reset view");
            });
        
        if self.show_coordinates {
            let (mouse_x, mouse_y) = mouse_position();
            let (world_x, world_y) = self.camera.screen_to_world(mouse_x, mouse_y);
            
            let func_value = if self.current_function().is_valid_domain(world_x as f64) {
                self.current_function().evaluate(world_x as f64)
            } else {
                f64::NAN
            };
            
            egui::Window::new("Coordinates")
                .default_pos(egui::pos2(10.0, screen_height() - 120.0))
                .default_size(egui::vec2(200.0, 100.0))
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.label(format!("x: {:.4}", world_x));
                    ui.label(format!("y: {:.4}", world_y));
                    if !func_value.is_nan() {
                        ui.label(format!("{}({:.4}) = {:.4}", 
                            self.current_function().name, world_x, func_value));
                    }
                });
        }
    }
}

fn draw_grid(camera: &PlotCamera) {
    let width = screen_width();
    let height = screen_height();
    
    let grid_color = Color::new(0.3, 0.3, 0.3, 1.0);
    let axis_color = Color::new(0.6, 0.6, 0.6, 1.0);

    let x_range = camera.x_max - camera.x_min;
    let y_range = camera.y_max - camera.y_min;

    let max_range = x_range.max(y_range);
    
    let target_divisions = 6.0;
    let raw_step = max_range / target_divisions;
    
    let log_step = raw_step.log10().floor();
    let grid_step = 10.0_f32.powf(log_step);
    
    let x_grid_step = grid_step;
    let y_grid_step = grid_step;

    let x_start = (camera.x_min / x_grid_step).floor() * x_grid_step;
    let x_end = (camera.x_max / x_grid_step).ceil() * x_grid_step;
    let mut x = x_start;
    while x <= x_end {
        let (screen_x, _) = camera.world_to_screen(x, 0.0);
        if screen_x >= 0.0 && screen_x <= width {
            let is_axis = x.abs() < x_grid_step * 0.01;
            let color = if is_axis { axis_color } else { grid_color };
            let thickness = if is_axis { 2.0 } else { 1.0 };
            draw_line(screen_x, 0.0, screen_x, height, thickness, color);

            if !is_axis {
                let label = format!("{:.1}", x);
                draw_text(&label, screen_x + 2.0, height - 5.0, 12.0, LIGHTGRAY);
            }
        }
        x += x_grid_step;
    }

    let y_start = (camera.y_min / y_grid_step).floor() * y_grid_step;
    let y_end = (camera.y_max / y_grid_step).ceil() * y_grid_step;
    let mut y = y_start;
    while y <= y_end {
        let (_, screen_y) = camera.world_to_screen(0.0, y);
        if screen_y >= 0.0 && screen_y <= height {
            let is_axis = y.abs() < y_grid_step * 0.01;
            let color = if is_axis { axis_color } else { grid_color };
            let thickness = if is_axis { 2.0 } else { 1.0 };
            draw_line(0.0, screen_y, width, screen_y, thickness, color);

            if !is_axis {
                let label = format!("{:.1}", y);
                draw_text(&label, 5.0, screen_y - 2.0, 12.0, LIGHTGRAY);
            }
        }
        y += y_grid_step;
    }
}

fn draw_crosshair(camera: &PlotCamera, func: &FunctionDescriptor, func_color: Color) {
    let (mouse_x, mouse_y) = mouse_position();
    let width = screen_width();
    let height = screen_height();
    
    if mouse_x >= 0.0 && mouse_x <= width && mouse_y >= 0.0 && mouse_y <= height {
        let crosshair_color = Color::new(0.8, 0.8, 0.8, 0.7);
        
        draw_line(mouse_x, 0.0, mouse_x, height, 1.0, crosshair_color);
        
        draw_line(0.0, mouse_y, width, mouse_y, 1.0, crosshair_color);
        
        let (world_x, _world_y) = camera.screen_to_world(mouse_x, mouse_y);
        if func.is_valid_domain(world_x as f64) {
            let func_y = func.evaluate(world_x as f64) as f32;
            if !func_y.is_nan() && !func_y.is_infinite() {
                let (dot_screen_x, dot_screen_y) = camera.world_to_screen(world_x, func_y);
                
                if dot_screen_x >= -10.0 && dot_screen_x <= width + 10.0 &&
                   dot_screen_y >= -10.0 && dot_screen_y <= height + 10.0 {
                    draw_circle(dot_screen_x, dot_screen_y, 4.0, func_color);
                    draw_circle_lines(dot_screen_x, dot_screen_y, 4.0, 1.0, WHITE);
                }
            }
        }
    }
}


fn draw_function(camera: &PlotCamera, func: &FunctionDescriptor, sample_points: usize, line_thickness: f32, color: Color) {
    let width = screen_width();
    let height = screen_height();
    let dx = (camera.x_max - camera.x_min) / (sample_points as f32);

    let mut last_point: Option<(f32, f32)> = None;

    for i in 0..=sample_points {
        let x = camera.x_min + i as f32 * dx;

        if !func.is_valid_domain(x as f64) {
            last_point = None;
            continue;
        }

        let y = func.evaluate(x as f64) as f32;

        if y.is_nan() || y.is_infinite() {
            last_point = None;
            continue;
        }

        let (screen_x, screen_y) = camera.world_to_screen(x, y);

        if screen_x >= -100.0 && screen_x <= width + 100.0 &&
            screen_y >= -100.0 && screen_y <= height + 100.0 {

            if let Some((last_x, last_y)) = last_point {
                let dx = screen_x - last_x;
                let dy = screen_y - last_y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < 200.0 && distance > 0.1 {
                    draw_line(last_x, last_y, screen_x, screen_y, line_thickness, color);
                }
            }

            last_point = Some((screen_x, screen_y));
        } else {
            last_point = None;
        }
    }
}


#[macroquad::main("Function Plotter with UI")]
async fn main() {
    let mut app = PlotterApp::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let mut egui_wants_input = false;
        egui_macroquad::ui(|egui_ctx| {
            app.draw_ui(egui_ctx);
            egui_wants_input = egui_ctx.wants_pointer_input() || egui_ctx.wants_keyboard_input();
        });
        
        app.handle_input();
        app.camera.update(egui_wants_input);

        draw_grid(&app.camera);
        let func_color = Color::new(app.function_color[0], app.function_color[1], app.function_color[2], 1.0);
        draw_function(&app.camera, app.current_function(), app.sample_points, app.line_thickness, func_color);
        
        if app.show_crosshair {
            draw_crosshair(&app.camera, app.current_function(), func_color);
        }

        egui_macroquad::draw();

        next_frame().await;
    }
}