use eframe::egui;

use crate::{
    app::NodeState,
    data::{LinkType, Node, Page, PageLink},
};

pub fn create_graph_panel(
    ctx: &egui::Context,
    crt_page: &mut Option<Page>,
    graph_root: &mut Node,
    node_state: &mut NodeState,
    path: &std::path::Path,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            node_state.node_to_remove_by_path = None;
            node_state.node_to_load_by_path = None;

            let resp = ui.interact(
                ui.available_rect_before_wrap(),
                egui::Id::new("DragAndDropPanel"),
                egui::Sense::click_and_drag(),
            );
            if resp.dragged() {
                node_state.start_coord.0 += resp.drag_delta().x;
                node_state.start_coord.1 += resp.drag_delta().y;
            }
            resp.context_menu(|ui| {
                if ui.button("Go Home").clicked() {
                    node_state.start_coord = (0.0, 0.0);
                }
            });

            draw_node_recursiv(ui, graph_root, true, node_state);

            if let Some(to_remove_path) = &node_state.node_to_remove_by_path {
                search_and_remove_node_recursiv(graph_root, &to_remove_path);
                node_state.node_to_remove_by_path = None;
            }
            if let Some(to_load_path) = &node_state.node_to_load_by_path {
                if let Ok(file_string) = std::fs::read_to_string(path.join(to_load_path))
                    && let Ok(page) = serde_json::from_str(&file_string)
                {
                    *crt_page = Some(page);
                } else {
                    let def_page = Page::default();
                    let json_string = serde_json::to_string_pretty(&def_page)
                        .expect("Cannot serialize default Page");
                    std::fs::write(path.join(&to_load_path), json_string)
                        .expect("Cannot write new default Page file!");
                    *crt_page = Some(def_page);
                }
                node_state.page_links = get_links_by_path(&to_load_path, graph_root);
            }
        });
}

pub fn save_page(to_save_path: &std::path::Path, path: &std::path::Path, crt_page: &Page) {
    let json_string = serde_json::to_string_pretty(crt_page).expect("Cannot serialize User Page");
    std::fs::write(path.join(to_save_path), json_string).expect("Cannot save UserPage file!");
}

pub fn get_links_by_path(path: &std::path::Path, root_node: &Node) -> Vec<PageLink> {
    let mut links: Vec<PageLink> = Vec::new();
    let root_pos = root_node.get_pos();
    if root_node.path == path {
        for sub_page in root_node.sub_nodes.iter().flatten() {
            let sub_pos = sub_page.get_pos();
            let vec = (sub_pos - root_pos).normalized() * 10_000.0;
            links.push(PageLink {
                link_type: LinkType::ChildLink,
                direction_vec: (vec.x, vec.y),
                file_name: sub_page.path.clone(),
            });
        }
        return links;
    } else {
        for sub_page in root_node.sub_nodes.iter().flatten() {
            if sub_page.path == path {
                let sub_pos = sub_page.get_pos();
                let vec = (root_pos - sub_pos).normalized() * 10_000.0;
                links.push(PageLink {
                    link_type: LinkType::ParentLink,
                    direction_vec: (vec.x, vec.y),
                    file_name: root_node.path.clone(),
                });
                for sub_sub_page in sub_page.sub_nodes.iter().flatten() {
                    let sub_sub_pos = sub_sub_page.get_pos();
                    let sub_vec = (sub_sub_pos - sub_pos).normalized() * 10_000.0;
                    links.push(PageLink {
                        link_type: LinkType::ChildLink,
                        direction_vec: (sub_vec.x, sub_vec.y),
                        file_name: sub_sub_page.path.clone(),
                    });
                }
                return links;
            }
        }
        for sub_page in root_node.sub_nodes.iter().flatten() {
            let res = get_links_by_path(path, sub_page);
            if res.len() > 0 {
                return res;
            }
        }
    };
    Vec::new()
}

pub fn draw_node_recursiv(
    ui: &mut egui::Ui,
    node: &mut Node,
    is_root: bool,
    node_state: &mut NodeState,
) {
    let circle_radius = 30.0;
    let circle_color = if is_root {
        egui::Color32::RED
    } else {
        egui::Color32::GRAY
    };
    let circle_stroke = egui::Stroke::new(2.0, egui::Color32::DARK_RED);
    let line_stroke = egui::Stroke::new(4.0, egui::Color32::LIGHT_RED);
    let arrow_length = 50.0;
    let font_size = 16.0;
    let font_color = egui::Color32::BLACK;

    let start_coord = node_state.start_coord;
    let start_coord_vec = egui::Vec2::new(start_coord.0, start_coord.1);
    let node_pos = node.get_pos() + start_coord_vec;
    let node_pos_clear = node.get_pos();

    if let Some(sub_nodes) = &mut node.sub_nodes {
        for sub_node in sub_nodes.iter_mut() {
            {
                let painter = ui.painter();
                let sub_node_pos = sub_node.get_pos() + start_coord_vec;
                let vec_to_sub_node = (sub_node_pos - node_pos).normalized();
                painter.line_segment([node_pos, sub_node_pos], line_stroke);
                painter.arrow(node_pos, vec_to_sub_node * arrow_length, line_stroke);
            }
            draw_node_recursiv(ui, sub_node, false, node_state);
        }
    }
    {
        let painter = ui.painter();
        painter.circle(node_pos, circle_radius, circle_color, circle_stroke);
        painter.text(
            node_pos,
            egui::Align2::CENTER_CENTER,
            &node.name,
            egui::FontId::monospace(font_size),
            font_color,
        );

        let resp: egui::Response = ui.allocate_rect(
            egui::Rect::from_center_size(node_pos, egui::Vec2::splat(circle_radius * 2.0)),
            egui::Sense::click_and_drag(),
        );

        if resp.clicked() {
            node_state.node_to_load_by_path = Some(node.path.clone());
        }
        if resp.dragged() {
            node.pos.0 += resp.drag_delta().x;
            node.pos.1 += resp.drag_delta().y;
        }
        resp.context_menu(|ui| {
            if ui.button("Add SubNode").clicked() {
                let mut new_node = Node::default();
                new_node.pos = (node_pos_clear.x + 80.0, node_pos_clear.y + 40.0);
                if let Some(sub_nodes) = &mut node.sub_nodes {
                    sub_nodes.push(new_node);
                } else {
                    node.sub_nodes = Some(vec![new_node]);
                }
            }
            if ui.button("Remove Node").clicked() {
                node_state.node_to_remove_by_path = Some(node.path.clone());
            }
            ui.text_edit_singleline(&mut node.name);
        });
    }
}

pub fn search_and_remove_node_recursiv(node: &mut Node, path: &std::path::Path) {
    if let Some(sub_nodes) = &mut node.sub_nodes {
        if let Some((index, _)) = sub_nodes.iter().enumerate().find(|(_, n)| n.path == path) {
            sub_nodes.remove(index);
            return;
        }
        for sub_node in sub_nodes.iter_mut() {
            search_and_remove_node_recursiv(sub_node, path);
        }
    }
}

pub fn save_graph(root_node: &Option<Node>, path: &std::path::Path) {
    if let Some(graph_root) = root_node {
        if let Ok(json_string) = serde_json::to_string(graph_root) {
            std::fs::write(path.join("graph.base"), &json_string).expect("Cannot save GraphFile!");
        }
    }
}
