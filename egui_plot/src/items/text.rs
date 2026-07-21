use core::hash::Hash;
use std::ops::RangeInclusive;

use egui::Color32;
use egui::Shape;
use egui::Stroke;
use egui::TextStyle;
use egui::Ui;
use egui::WidgetText;
use egui::epaint::TextShape;
use emath::Align2;
use emath::Rot2;

use crate::axis::PlotTransform;
use crate::bounds::PlotBounds;
use crate::bounds::PlotPoint;
use crate::item_id::ItemId;
use crate::items::PlotGeometry;
use crate::items::PlotItem;
use crate::items::PlotItemBase;

impl Text {
    pub fn new(name: impl Into<String>, position: PlotPoint, text: impl Into<WidgetText>) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            text: text.into(),
            position,
            color: Color32::TRANSPARENT,
            anchor: Align2::CENTER_CENTER,
            angle: 0.0,
        }
    }

    /// Text color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    /// Anchor position of the text. Default is `Align2::CENTER_CENTER`.
    #[inline]
    pub fn anchor(mut self, anchor: Align2) -> Self {
        self.anchor = anchor;
        self
    }

    /// Rotate text by this many radians clockwise around its anchor point.
    #[inline]
    pub fn angle(mut self, angle: impl Into<f32>) -> Self {
        self.angle = angle.into();
        self
    }

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you
    /// can use it to change the name dynamically between frames without
    /// losing the item's state. You should make sure the name passed to
    /// [`Self::new`] is unique and stable for each item, or set unique and
    /// stable ids explicitly via [`Self::id`].
    #[expect(clippy::needless_pass_by_value, reason = "to allow various string types")]
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.base_mut().name = name.to_string();
        self
    }

    /// Highlight this plot item, typically by scaling it up.
    ///
    /// If false, the item may still be highlighted via user interaction.
    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.base_mut().highlight = highlight;
        self
    }

    /// Allowed hovering this item in the plot. Default: `true`.
    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.base_mut().allow_hover = hovering;
        self
    }

    /// Sets the [`ItemId`] of this plot item.
    ///
    /// The id only has to be unique within the plot.
    ///
    /// By default the id is derived from the name passed to [`Self::new`],
    /// but it can be explicitly set to a different value.
    #[inline]
    pub fn id(mut self, id: impl Hash) -> Self {
        self.base_mut().id = ItemId::new(id);
        self
    }
}

impl PlotItem for Text {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        let color = if self.color == Color32::TRANSPARENT {
            ui.style().visuals.text_color()
        } else {
            self.color
        };

        let galley =
            self.text
                .clone()
                .into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, TextStyle::Small);

        let pos = transform.position_from_point(&self.position);
        let size = galley.size();
        let rect = self.anchor.anchor_size(pos, size);

        let text_pos = if self.angle == 0.0 {
            rect.min
        } else {
            let offset = rect.min - pos;
            let rotated_offset = Rot2::from_angle(self.angle) * offset;
            pos + rotated_offset
        };

        shapes.push(TextShape::new(text_pos, galley, color).with_angle(self.angle).into());

        if self.base.highlight {
            if self.angle == 0.0 {
                shapes.push(Shape::rect_stroke(
                    rect.expand(1.0),
                    1.0,
                    Stroke::new(0.5, color),
                    egui::StrokeKind::Outside,
                ));
            } else {
                let rect = self.anchor.anchor_size(pos, size).expand(1.0);
                let rotation = Rot2::from_angle(self.angle);
                let corners = [
                    rect.left_top(),
                    rect.right_top(),
                    rect.right_bottom(),
                    rect.left_bottom(),
                ];

                let rotated_corners = corners
                    .iter()
                    .map(|&corner| {
                        let offset = corner - pos;
                        let rotated_offset = rotation * offset;
                        pos + rotated_offset
                    })
                    .collect();

                shapes.push(Shape::closed_line(rotated_corners, Stroke::new(0.5, color)));
            }
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.color
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        bounds.extend_with(&self.position);
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// Text inside the plot.
#[derive(Clone)]
pub struct Text {
    base: PlotItemBase,
    pub(crate) text: WidgetText,
    pub(crate) position: PlotPoint,
    pub(crate) color: Color32,
    pub(crate) anchor: Align2,
    pub(crate) angle: f32,
}
