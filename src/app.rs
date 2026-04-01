// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use crate::data::{Graphic, Node, Object, Page, PageLink};
use eframe::egui::Color32;
use std::path::PathBuf;

pub struct Yuti {
    pub path: Option<PathBuf>,
    pub graph_root_node: Option<Node>,
    pub crt_page: Option<Page>,
    pub editor_state: EditorState,
    pub graph_state: GraphState,
    pub folder_state: FolderState,
    pub graphic: Graphic,
}

pub struct EditorState {
    pub grid_size: f32,
    pub page_to_close: bool,
    pub object_to_remove_id: Option<usize>,
    pub selected_object_id: Option<usize>,
    pub is_selected_for_text_edit: bool,
    pub are_tools_visible: bool,
    pub copied_object: Option<Object>,
    pub tools_buf: ToolsBuffer,
}

pub struct ToolsBuffer {
    pub color: String,
    pub font_color: String,
    pub stroke_color: String,
    pub color_picker: Color32,
    pub font_color_picker: Color32,
    pub stroke_color_picker: Color32,
}

pub struct GraphState {
    pub node_to_remove_by_path: Option<PathBuf>,
    pub node_to_load_by_path: Option<PathBuf>,
    pub page_to_switch: Option<PathBuf>,
    pub page_links: Vec<PageLink>,
    pub start_coord: (f32, f32),
}

pub struct FolderState {
    pub path_line: String,
    pub is_path_to_clear: bool,
}

impl Yuti {
    pub fn new(_cc: &'_ eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&_cc.egui_ctx);

        Yuti {
            path: None,
            graph_root_node: None,
            crt_page: None,
            editor_state: EditorState {
                grid_size: 10.0,
                page_to_close: false,
                object_to_remove_id: None,
                selected_object_id: None,
                is_selected_for_text_edit: false,
                are_tools_visible: false,
                copied_object: None,
                tools_buf: ToolsBuffer {
                    color: String::new(),
                    font_color: String::new(),
                    stroke_color: String::new(),
                    color_picker: Color32::BLACK,
                    font_color_picker: Color32::BLACK,
                    stroke_color_picker: Color32::BLACK,
                },
            },
            graph_state: GraphState {
                node_to_remove_by_path: None,
                node_to_load_by_path: None,
                page_to_switch: None,
                page_links: Vec::new(),
                start_coord: (0.0, 0.0),
            },
            folder_state: FolderState {
                path_line: String::new(),
                is_path_to_clear: false,
            },
            graphic: Graphic::default(),
        }
    }

    pub fn reset_editor_state(&mut self) {
        self.editor_state.page_to_close = false;
        self.editor_state.are_tools_visible = false;
        self.editor_state.selected_object_id = None;
        self.editor_state.is_selected_for_text_edit = false;
        self.graph_state.page_links = Vec::new();
    }
}
