use eframe::egui::{self, Color32, FontId, Layout};

use crate::{
    app::{State, Yuti},
    data::{Object, Page},
};

impl Yuti {
    pub fn create_top_menu(ui: &mut egui::Ui, state: &mut State, crt_page: &mut Page) {
        // \\ CONSTS // \\
        let visible_height = 120.0;
        let hidden_height = 20.0;
        let first_block_width = 100.0;
        let last_block_width = 200.0;
        let sector_width = 150.0;
        // \\ CONSTS // \\

        // Set hiden|visible size
        let size = if state.is_inst_panel_visible {
            visible_height
        } else {
            hidden_height
        };

        ui.add_sized([ui.available_width(), size], |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                let height = ui.available_height();

                // ========= First Block
                if ui
                    .add_sized([first_block_width, height], egui::Button::new("Close Page"))
                    .clicked()
                {
                    state.page_to_close = true;
                }

                // ========= Last Block
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_sized([last_block_width, height], egui::Button::new("<|>"))
                        .clicked()
                    {
                        state.is_inst_panel_visible = !state.is_inst_panel_visible;
                    }

                    // ========= Center Content
                    ui.centered_and_justified(|ui| {
                        ui.horizontal(|ui| {
                            if state.is_inst_panel_visible
                                && let Some(selected_obj_id) = state.selected_object_id
                                && let Some(object) = crt_page.objects.get_mut(selected_obj_id)
                            {
                                // ====== IF PANEL IS VISIBLE AND OBJECT SELECTED

                                ui.add_space(10.0);
                                ui.separator();

                                // ! BACKGROUND SECTOR
                                Self::draw_background_sector(
                                    state,
                                    ui,
                                    object,
                                    sector_width,
                                    height,
                                );
                                ui.separator();

                                // ! FONTS SECTOR
                                Self::draw_fonts_sector(state, ui, object, sector_width, height);

                                ui.separator();

                                // ! STROKE SECTOR
                                Self::draw_stroke_sector(state, ui, object, sector_width, height);

                                ui.separator();

                                // ! POSITION SECTOR
                                Self::draw_position_sector(state, ui, object, sector_width, height);

                                ui.separator();

                                // ! TEXT SECTOR
                                Self::draw_text_sector(ui, object, sector_width, height);

                                ui.separator();

                                // ! Z-INDEX SECTOR
                                Self::draw_z_index_sector(state, ui, object, sector_width, height);

                                ui.separator();
                            } else {
                                // ====== IF PANEL IS INVISIBLE (just label)

                                ui.horizontal_centered(|ui| {
                                    ui.add(egui::Label::new(
                                        egui::RichText::new("Yutikora Core")
                                            .color(Color32::DARK_RED)
                                            .font(FontId::monospace(16.0)),
                                    ));
                                });
                            }
                        });
                    });
                });
            })
            .response
        });
    }

    fn draw_background_sector(
        state: &mut State,
        ui: &mut egui::Ui,
        object: &mut Object,
        sector_width: f32,
        height: f32,
    ) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Background Color:");
                let temp = ui.text_edit_singleline(&mut state.context_menu_color);
                if !temp.has_focus() {
                    state.context_menu_color = Object::hex_from_vec_u8(object.color)
                }
                if temp.changed() {
                    object.color = Object::vec_u8_from_hex(&state.context_menu_color);
                };
                state.sett_backgound_color = object.get_color();
                if ui
                    .color_edit_button_srgba(&mut state.sett_backgound_color)
                    .changed()
                {
                    object.color = Object::vec_u8_from_color32(state.sett_backgound_color);
                };
            })
            .response
        });
    }

    fn draw_fonts_sector(
        state: &mut State,
        ui: &mut egui::Ui,
        object: &mut Object,
        sector_width: f32,
        height: f32,
    ) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Font Color:");
                let temp = ui.text_edit_singleline(&mut state.context_menu_font_color);
                if !temp.has_focus() {
                    state.context_menu_font_color = Object::hex_from_vec_u8(object.font_color)
                }
                if temp.changed() {
                    object.font_color = Object::vec_u8_from_hex(&state.context_menu_font_color);
                };
                state.sett_font_color = object.get_font_color();
                if ui
                    .color_edit_button_srgba(&mut state.sett_font_color)
                    .changed()
                {
                    object.font_color = Object::vec_u8_from_color32(state.sett_font_color);
                };
                ui.separator();
                ui.add(
                    egui::DragValue::new(&mut object.font_size)
                        .speed(1)
                        .range(5..=100)
                        .prefix("Font Size: "),
                );
            })
            .response
        });
    }

    fn draw_stroke_sector(
        state: &mut State,
        ui: &mut egui::Ui,
        object: &mut Object,
        sector_width: f32,
        height: f32,
    ) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Stroke Color:");
                let temp = ui.text_edit_singleline(&mut state.context_menu_stroke_color);
                if !temp.has_focus() {
                    state.context_menu_stroke_color = Object::hex_from_vec_u8(object.stroke_color)
                }
                if temp.changed() {
                    object.stroke_color = Object::vec_u8_from_hex(&state.context_menu_stroke_color);
                };
                state.sett_stroke_color = object.get_stroke_color();
                if ui
                    .color_edit_button_srgba(&mut state.sett_stroke_color)
                    .changed()
                {
                    object.stroke_color = Object::vec_u8_from_color32(state.sett_stroke_color);
                };
                ui.separator();
                ui.add(
                    egui::DragValue::new(&mut object.stroke_width)
                        .speed(0.5)
                        .range(0..=1000)
                        .prefix("Stroke Width: "),
                );
                ui.add(
                    egui::DragValue::new(&mut object.corner_radius)
                        .speed(0.5)
                        .range(0..=1000)
                        .prefix("Corner Radius: "),
                );
            })
            .response
        });
    }

    fn draw_position_sector(
        state: &mut State,
        ui: &mut egui::Ui,
        object: &mut Object,
        sector_width: f32,
        height: f32,
    ) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Position:");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut object.pos.0)
                            .speed(state.grid_size)
                            .range(0..=999999)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut object.pos.1)
                            .speed(state.grid_size)
                            .range(0..=999999)
                            .prefix("y: "),
                    );
                });
                ui.separator();
                ui.label("Size:");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut object.size.0)
                            .speed(state.grid_size)
                            .range(0..=999999)
                            .prefix("Width: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut object.size.1)
                            .speed(state.grid_size)
                            .range(0..=999999)
                            .prefix("Height: "),
                    );
                });
            })
            .response
        });
    }

    fn draw_text_sector(ui: &mut egui::Ui, object: &mut Object, sector_width: f32, height: f32) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Text Offset:");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut object.text_offset.0)
                            .speed(0.5)
                            .range(0..=1000)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut object.text_offset.1)
                            .speed(0.5)
                            .range(0..=1000)
                            .prefix("y: "),
                    );
                });
                ui.separator();
                ui.label("Text Allign:");
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        ui.radio_value(&mut object.text_align, 0, "Center");
                        ui.radio_value(&mut object.text_align, 1, "Left");
                        ui.radio_value(&mut object.text_align, 2, "Right");
                    });
                });
            })
            .response
        });
    }

    fn draw_z_index_sector(
        state: &mut State,
        ui: &mut egui::Ui,
        object: &mut Object,
        sector_width: f32,
        height: f32,
    ) {
        ui.add_sized([sector_width, height], |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label("Z-Index");
                if ui
                    .add(
                        egui::DragValue::new(&mut object.z_index)
                            .speed(1)
                            .range(0..=999999)
                            .prefix("Z-Index: "),
                    )
                    .changed()
                {
                    state.selected_object_id = None;
                }
                ui.separator();
                if ui.button("up").clicked() {
                    object.z_index = if object.z_index > 999999 {
                        999999
                    } else {
                        object.z_index + 1
                    };
                    state.selected_object_id = None;
                };
                if ui.button("down").clicked() {
                    object.z_index = if object.z_index == 0 {
                        0
                    } else {
                        object.z_index - 1
                    };
                    state.selected_object_id = None;
                };
            })
            .response
        });
    }
}
