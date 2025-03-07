use eframe::{egui};
use egui::Widget;
use egui_nodes::{Context, LinkArgs, NodeArgs, NodeConstructor, PinArgs, PinShape};

struct MyApp {
    ctx: Context,
    links: Vec<(usize, usize)>,
}

pub fn example_graph(ctx: &mut Context, links: &mut Vec<(usize, usize)>, ui: &mut egui::Ui) {
    let mut node1 = NodeConstructor::new(
        0,
        NodeArgs {
            outline: Some(egui::Color32::LIGHT_BLUE),
            ..Default::default()
        },
    );

    node1.with_origin([50.0, 150.0].into())
    .with_title(|ui| ui.label("Example Node A"))
    .with_input_attribute(
        0,
        PinArgs {
            shape: PinShape::Triangle,
            ..Default::default()
        },
        |ui| ui.label("Input"),
    )
    .with_static_attribute(1, |ui| ui.label("Can't Connect to Me"))
    .with_output_attribute(
        2,
        PinArgs {
            shape: PinShape::TriangleFilled,
            ..Default::default()
        },
        |ui| ui.label("Output"),
    );
    let mut node2 =    NodeConstructor::new(1, Default::default());

        node2.with_origin([225.0, 150.0].into())
        .with_title(|ui| ui.label("Example Node B"))
        .with_static_attribute(3, |ui| ui.label("Can't Connect to Me"))
        .with_output_attribute(4, Default::default(), |ui| ui.label("Output"))
        .with_input_attribute(5, Default::default(), |ui| ui.label("Input"));
    // add nodes with attributes
    let nodes = vec![node1, node2];
     
    ctx.show(
        nodes,
        links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
        ui,
    );

    // remove destroyed links
    if let Some(idx) = ctx.link_destroyed() {
        links.remove(idx);
    }

    // add created links
    if let Some((start, end, _)) = ctx.link_created() {
        links.push((start, end))
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ctx: Context::default(),
            links: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            egui::Slider::new(&mut self.ctx.style.link_bezier_offset_coefficient.x, 0.0..=1.0_f32).ui(ui);
            egui::Slider::new(&mut self.ctx.style.link_line_segments_per_length, 0.0..=1.0_f32).ui(ui);
            example_graph(&mut self.ctx, &mut self.links, ui);

        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    eframe::run_native("My egui App", eframe::NativeOptions::default(), Box::new(|_| Box::new(MyApp::default())));
}
