use std::time::{Duration, Instant};

struct GifPlayer {
    frames: Vec<(Duration, egui::TextureHandle)>,
    start_time: Instant,
    current_frame: usize,
    accumulated_time: Duration,
}

impl GifPlayer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load your GIF file here
        let gif_data = include_bytes!("../.assets/dl_gif_handsome-handsome_final.gif");

        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let mut decoder = decoder.read_info(&gif_data[..]).unwrap();

        let mut frames = Vec::new();
        let mut index = 0;

        while let Some(frame) = decoder.read_next_frame().unwrap() {
            let delay = Duration::from_millis((frame.delay as u64) * 10);
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [frame.width as usize, frame.height as usize],
                &frame.buffer,
            );

            let texture = cc.egui_ctx.load_texture(
                format!("frame-{}", index),
                color_image,
                Default::default(),
            );

            frames.push((delay, texture));
            index += 1;
        }

        GifPlayer {
            frames,
            start_time: Instant::now(),
            current_frame: 0,
            accumulated_time: Duration::ZERO,
        }
    }
}

impl eframe::App for GifPlayer {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.frames.is_empty() {
            return;
        }

        let elapsed = self.start_time.elapsed();
        let total_duration = self.frames.iter().map(|(d, _)| *d).sum::<Duration>();

        let loop_time =
            Duration::from_nanos((elapsed.as_nanos() % total_duration.as_nanos()) as u64);
        let mut accumulated = Duration::ZERO;

        for (i, (delay, _)) in self.frames.iter().enumerate() {
            accumulated += *delay;
            if accumulated > loop_time {
                self.current_frame = i;
                break;
            }
        }

        egui::Area::new("gif_area".into())
            .movable(false)
            .interactable(false)
            .show(ctx, |ui| {
                if let Some((_, texture)) = self.frames.get(self.current_frame) {
                    ui.image(texture);
                }
            });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_decorations(true)
            .with_inner_size([400.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Transparent GIF Viewer",
        options,
        Box::new(|cc| Ok(Box::new(GifPlayer::new(cc)))),
    )
}
