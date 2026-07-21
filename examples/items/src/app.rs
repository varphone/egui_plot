use std::f64::consts::TAU;

use eframe::egui;
use eframe::egui::Response;
use egui::vec2;
use egui_plot::ArcLine;
use egui_plot::Arrows;
use egui_plot::HLine;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Pie;
use egui_plot::PieChart;
use egui_plot::Plot;
use egui_plot::PlotImage;
use egui_plot::PlotPoint;
use egui_plot::PlotPoints;
use egui_plot::Points;
use egui_plot::Polygon;
use egui_plot::Text;
use egui_plot::VLine;

#[derive(Default)]
pub struct ItemsExample {
    texture: Option<egui::TextureHandle>,
}

impl ItemsExample {
    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        let n = 100;
        let mut sin_values: Vec<_> = (0..=n)
            .map(|i| egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();

        let line = Line::new("sin(x)", sin_values.split_off(n / 2)).fill(-1.5);
        let polygon = Polygon::new(
            "polygon",
            PlotPoints::from_parametric_callback(
                |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
                0.0..TAU,
                100,
            ),
        );
        let points = Points::new("sin(x)", sin_values).stems(-1.5).radius(1.0);

        let arrows = {
            let pos_radius = 8.0;
            let tip_radius = 7.0;
            let arrow_origins =
                PlotPoints::from_parametric_callback(|t| (pos_radius * t.sin(), pos_radius * t.cos()), 0.0..TAU, 36);
            let arrow_tips =
                PlotPoints::from_parametric_callback(|t| (tip_radius * t.sin(), tip_radius * t.cos()), 0.0..TAU, 36);
            Arrows::new("arrows", arrow_origins, arrow_tips)
        };

        let texture: &egui::TextureHandle = self
            .texture
            .get_or_insert_with(|| ui.load_texture("plot_demo", egui::ColorImage::example(), Default::default()));
        let image = PlotImage::new(
            "image",
            texture,
            PlotPoint::new(0.0, 10.0),
            5.0 * vec2(texture.aspect_ratio(), 1.0),
        );

        let arc_line = ArcLine::new(
            "Arc line",
            PlotPoint::new(0.0, -10.0),
            3.0,
            225.0f32.to_radians(),
            135.0f32.to_radians(),
        );
        let pie = Pie::new(
            "Pie",
            PlotPoint::new(0.0, -10.0),
            3.0,
            -45.0f32.to_radians(),
            45.0f32.to_radians(),
        );
        let pie_chart_data = vec![16.41, 10.21, 9.76, 8.94, 6.77, 2.89, 1.85, 1.70, 1.61, 1.47, 38.39];
        let pie_chart_labels = vec![
            "Python".to_owned(),
            "C".to_owned(),
            "C++".to_owned(),
            "Java".to_owned(),
            "C#".to_owned(),
            "JavaScript".to_owned(),
            "Go".to_owned(),
            "Visual Basic".to_owned(),
            "Fortran".to_owned(),
            "SQL".to_owned(),
            "Others".to_owned(),
        ];
        let pie_chart = PieChart::new("TIOBE - April 2024 (L)", [-12.0, 0.0], 3.5, pie_chart_data.clone())
            .labels(pie_chart_labels.clone());
        let exploded_pies =
            PieChart::new("TIOBE - April 2024 (R)", [12.0, 0.0], 3.5, pie_chart_data).labels(pie_chart_labels);

        let plot = Plot::new("items_demo")
            .legend(
                Legend::default()
                    .position(egui_plot::Corner::RightBottom)
                    .title("Items"),
            )
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.hline(HLine::new("Lines horizontal", 9.0));
            plot_ui.hline(HLine::new("Lines horizontal", -9.0));
            plot_ui.vline(VLine::new("Lines vertical", 9.0));
            plot_ui.vline(VLine::new("Lines vertical", -9.0));
            plot_ui.line(line.name("Line with fill").id("line_with_fill"));
            plot_ui.polygon(polygon.name("Convex polygon").id("convex_polygon"));
            plot_ui.points(points.name("Points with stems").id("points_with_stems"));
            plot_ui.text(Text::new("Text", PlotPoint::new(-3.0, -3.0), "wow").id("text0"));
            plot_ui.text(Text::new("Text", PlotPoint::new(-2.0, 2.5), "so graph").id("text1"));
            plot_ui.text(Text::new("Text", PlotPoint::new(3.0, 3.0), "much color").id("text2"));
            plot_ui.text(Text::new("Text", PlotPoint::new(2.5, -2.0), "such plot").id("text3"));
            plot_ui.image(image.name("Image"));
            plot_ui.arrows(arrows.name("Arrows"));
            plot_ui.arc_line(arc_line);
            plot_ui.pie(pie);
            plot_ui.pie_chart(pie_chart);
            for slice in exploded_pies.to_pies() {
                plot_ui.pie(slice);
            }
        })
        .response
    }

    #[expect(clippy::unused_self, reason = "required by the example template")]
    pub fn show_controls(&self, ui: &mut egui::Ui) -> Response {
        // No controls for this example
        ui.scope(|_ui| {}).response
    }
}
