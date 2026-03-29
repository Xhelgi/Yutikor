use eframe::egui;

pub fn is_mouse_in_link_area(ui: &egui::Ui, aviable_rect: egui::Rect, margin: f32) -> bool {
    let mut in_link_area = false;

    // Left Side
    if ui
        .interact(
            egui::Rect::from_min_max(
                aviable_rect.left_top(),
                aviable_rect.left_bottom() + egui::Vec2::new(margin, 0.0),
            ),
            egui::Id::new("left_link_panel"),
            egui::Sense::click(),
        )
        .clicked()
    {
        in_link_area = true;
    }
    // Right Side
    if ui
        .interact(
            egui::Rect::from_min_max(
                aviable_rect.right_top() + egui::Vec2::new(-margin, 0.0),
                aviable_rect.right_bottom(),
            ),
            egui::Id::new("right_link_panel"),
            egui::Sense::click(),
        )
        .clicked()
    {
        in_link_area = true;
    }
    // Top Side
    if ui
        .interact(
            egui::Rect::from_min_max(
                aviable_rect.left_top(),
                aviable_rect.right_top() + egui::Vec2::new(0.0, margin),
            ),
            egui::Id::new("top_link_panel"),
            egui::Sense::click(),
        )
        .clicked()
    {
        in_link_area = true;
    }
    // Bottom Side
    if ui
        .interact(
            egui::Rect::from_min_max(
                aviable_rect.left_bottom() + egui::Vec2::new(0.0, -margin),
                aviable_rect.right_bottom(),
            ),
            egui::Id::new("bottom_link_panel"),
            egui::Sense::click(),
        )
        .clicked()
    {
        in_link_area = true;
    }

    in_link_area
}
