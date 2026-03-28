use std::path::PathBuf;

use eframe::egui::Color32;

use crate::data::{Node, Object, Page, PageLink};

pub struct Yuti {
    pub path: PathBuf,
    pub graph_root_node: Option<Node>,
    pub crt_page: Option<Page>,

    pub state: State,
    pub node_state: NodeState,
}

pub struct NodeState {
    pub node_to_remove_by_path: Option<PathBuf>,
    pub node_to_load_by_path: Option<PathBuf>,
    pub page_to_switch: Option<PathBuf>,
    pub page_links: Vec<PageLink>,
    pub start_coord: (f32, f32),
}

pub struct State {
    pub grid_size: f32,

    pub page_to_close: bool,

    pub object_to_remove_id: Option<usize>,
    pub selected_object_id: Option<usize>,
    pub is_selected_for_text_edit: bool,

    pub context_menu_color: String,
    pub context_menu_font_color: String,
    pub context_menu_stroke_color: String,
    pub is_inst_panel_visible: bool,

    pub sett_backgound_color: Color32,
    pub sett_font_color: Color32,
    pub sett_stroke_color: Color32,

    pub copied_object: Option<Object>,
}

impl Yuti {
    pub fn new(_cc: &'_ eframe::CreationContext) -> Self {
        let default_path = "TestSaveFolder/";

        let state = State {
            grid_size: 10.0,

            page_to_close: false,

            selected_object_id: None,
            is_selected_for_text_edit: false,
            object_to_remove_id: None,

            context_menu_color: String::new(),
            context_menu_stroke_color: String::new(),
            context_menu_font_color: String::new(),
            is_inst_panel_visible: false,

            sett_backgound_color: Color32::BLACK,
            sett_font_color: Color32::BLACK,
            sett_stroke_color: Color32::BLACK,

            copied_object: None,
        };

        let node_state = NodeState {
            node_to_remove_by_path: None,
            node_to_load_by_path: None,
            page_to_switch: None,
            page_links: Vec::new(),
            start_coord: (0.0, 0.0),
        };

        Yuti {
            path: PathBuf::from(default_path),
            graph_root_node: None,
            crt_page: None,

            state,
            node_state,
        }
    }
}
