pub mod helpers;
pub mod inst_panel;
pub mod main_panel;

use eframe::{
    self,
    egui::{self, Color32, Id, Key, Sense, UiBuilder},
};
use std::fs;

use crate::{
    app::Yuti,
    data::{Node, Page},
};

impl eframe::App for Yuti {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_graph();
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // 0. If CrtPage exist, then draw crtPage with Objects etc.
        if let Some(crt_page) = &mut self.crt_page {
            egui::TopBottomPanel::top("TopPanel")
                .frame(egui::Frame::NONE.fill(Color32::LIGHT_GRAY))
                .show(ctx, |ui| {
                    // ! TOP MENU
                    Self::create_top_menu(ui, &mut self.state, crt_page);
                });
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(Color32::WHITE))
                .show(ctx, |ui| {
                    // ! MAIN PANEL
                    Self::process_page_links(ctx, ui, &mut self.node_state);

                    let inner_rect = ui.available_rect_before_wrap().shrink(30.0);
                    ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
                        Self::process_background_events(ctx, ui, crt_page, &mut self.state);
                        Self::process_objects_list(ctx, ui, crt_page, &mut self.state);
                    });
                });
            // --> Sort by Z-Index
            crt_page.objects.sort_by_key(|a| a.z_index);
            // --> Remove object from List
            if let Some(id) = self.state.object_to_remove_id {
                crt_page.objects.remove(id);
                self.state.object_to_remove_id = None;
            }
            // --> Copy-Paste
            let (copy, paste) = ctx.input(|i| (i.key_pressed(Key::C), i.key_pressed(Key::V)));
            if copy
                && !self.state.is_selected_for_text_edit
                && let Some(index) = self.state.selected_object_id
                && let Some(object) = crt_page.objects.get(index)
            {
                self.state.copied_object = Some(object.clone());
            }
            if paste
                && !self.state.is_selected_for_text_edit
                && let Some(item) = self.state.copied_object.as_ref()
            {
                let mut object = item.clone();
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    object.pos.0 = mouse_pos.x;
                    object.pos.1 = mouse_pos.y;
                };
                crt_page.objects.push(object);
            }
            // --> Delete
            if ctx.input(|i| i.key_pressed(Key::Delete))
                && !self.state.is_selected_for_text_edit
                && let Some(index) = self.state.selected_object_id
            {
                crt_page.objects.remove(index);
            }
            // --> Close
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.state.page_to_close = true;
            }
        }
        // 0. If CrtPage not exist, draw Menu with Graph and load crtPage
        else {
            // 1. If GraphRootNode exist - draw Graph
            if let Some(graph_root) = &mut self.graph_root_node {
                // * Create UI panel
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(Color32::WHITE))
                    .show(ctx, |ui| {
                        // Clear state Vars
                        self.node_state.node_to_remove_by_path = None;
                        self.node_state.node_to_load_by_path = None;

                        let resp = ui.interact(ui.available_rect_before_wrap(), Id::new("DragAndDropPanel"), Sense::click_and_drag()); 
                        if resp.dragged() {
                            self.node_state.start_coord.0 += resp.drag_delta().x;
                            self.node_state.start_coord.1 += resp.drag_delta().y;
                        }
                        resp.context_menu(|ui| {
                            if ui.button("Go Home").clicked() {
                                self.node_state.start_coord = (0.0, 0.0);
                            }
                        });

                        Self::draw_node_recursiv(ui, graph_root, true, &mut self.node_state);
                        // If TO_REMOVE is Some -> remove
                        if let Some(to_remove_path) = &self.node_state.node_to_remove_by_path {
                            Self::search_and_remove_node_recursiv(graph_root, to_remove_path);
                            self.node_state.node_to_remove_by_path = None;
                        }
                        // If TO_LOAD is Some -> load and open
                        // Dont clear path-var (for save on page closing)
                        if let Some(to_load_path) = &self.node_state.node_to_load_by_path {
                            if let Ok(file_string) =
                                fs::read_to_string(self.path.join(to_load_path))
                                && let Ok(page) = serde_json::from_str(&file_string)
                            {
                                self.crt_page = Some(page);
                            } else {
                                let crt_page = Page::default();
                                let json_string = serde_json::to_string_pretty(&crt_page)
                                    .expect("Cannot serialize default Page");
                                fs::write(self.path.join(to_load_path), json_string)
                                    .expect("Cannot write new default Page file!");
                                self.crt_page = Some(crt_page);
                            }
                            self.node_state.page_links = Self::get_links_by_path(to_load_path, graph_root);
                        }
                    });
            }
            // 1. If GraphRootNode not exit - load GraphNode from Path
            else {
                let graph_path = self.path.join("graph.base");
                // 2. If GraphSaveFile exist -> Read
                if let Ok(json_string) = fs::read_to_string(&graph_path) {
                    match serde_json::from_str(&json_string) {
                        Ok(node) => self.graph_root_node = Some(node),
                        Err(e) => {
                            println!("Error bei reading GraphFile: {e}");
                            fs::remove_file(&graph_path).expect("Cannot remove braked GraphFile!")
                        }
                    }
                }
                // 2. If GraphSaveFile not exist -> Create new default Graph and save it
                else {
                    let def_graph = Node::default();
                    let def_graph_json_string = serde_json::to_string(&def_graph)
                        .expect("Cannot convert DefaultNode to Json string for new default File!");
                    fs::write(&graph_path, def_graph_json_string)
                        .expect("Cannot create default GraphFile");
                    self.graph_root_node = Some(def_graph);
                }
            }
        }
        // ! Commands to execute
        // Close Page Command
        if self.state.page_to_close {
            if let Some(to_save_path) = &self.node_state.node_to_load_by_path
                && let Some(crt_page) = &self.crt_page
            {
                Self::save_page(to_save_path, &self.path, crt_page);
            }
            self.crt_page = None;
            self.state.page_to_close = false;
            self.node_state.page_links = Vec::new();
        }
        // Switch Page Command
        if let Some(path) = &self.node_state.page_to_switch {
            if let Some(to_save_path) = &self.node_state.node_to_load_by_path
                && let Some(crt_page) = &self.crt_page
            {
                Self::save_page(to_save_path, &self.path, crt_page);
            }
            self.crt_page = None;
            self.state.page_to_close = false;
            self.node_state.page_links = Vec::new();
            
            if let Ok(file_string) = fs::read_to_string(self.path.join(path))
                && let Ok(page) = serde_json::from_str(&file_string)
            {
                self.crt_page = Some(page);
            } else {
                let crt_page = Page::default();
                let json_string =
                    serde_json::to_string_pretty(&crt_page).expect("Cannot serialize default Page");
                fs::write(self.path.join(path), json_string)
                    .expect("Cannot write new default Page file!");
                self.crt_page = Some(crt_page);
            }
            self.node_state.node_to_load_by_path = Some(path.clone());
            let root_node = self.graph_root_node.as_ref();
            self.node_state.page_links = Self::get_links_by_path(path, root_node.expect("SwitchPage root not found!"));
            self.node_state.page_to_switch = None;
        }
    }
}
