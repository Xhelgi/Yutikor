mod content;
mod menu;
mod tools;

use std::fs;

use crate::{
    app::Yuti,
    data::{Node, Page},
};

impl eframe::App for Yuti {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        menu::save_graph(&self.graph_root_node, &self.path);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // 0. If CrtPage exist, then draw crtPage with Objects etc.
        if let Some(crt_page) = &mut self.crt_page {
            // ? ===== CONTENT WINDOW =====
            // ! ALL DRAW WORK | >>>>
            tools::create_top_menu(ctx, &mut self.state);
            if self.state.are_tools_visibled {
                tools::create_tools_area(ctx, &mut self.state, crt_page);
            }
            content::create_content_panel(ctx, crt_page, &mut self.node_state, &mut self.state);
            // ! <<<< | ALL DRAW WORK

            content::logic::sort_by_z(crt_page);
            content::logic::remove_obj_if_need(crt_page, &mut self.state.object_to_remove_id);
            content::logic::hotkey_process(ctx, &mut self.state, crt_page);
        }
        // 0. If CrtPage not exist, draw Menu with Graph and load crtPage
        else {
            // 1. If GraphRootNode exist - draw Graph
            if let Some(graph_root) = &mut self.graph_root_node {
                // * Create UI panel
                menu::create_graph_panel(
                    ctx,
                    &mut self.crt_page,
                    graph_root,
                    &mut self.node_state,
                    &self.path,
                );
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
                menu::save_page(to_save_path, &self.path, crt_page);
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
                menu::save_page(to_save_path, &self.path, crt_page);
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
            self.node_state.page_links =
                menu::get_links_by_path(path, root_node.expect("SwitchPage root not found!"));
            self.node_state.page_to_switch = None;
        }
    }
}
