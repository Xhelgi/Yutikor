pub mod data_impl;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub name: String,
    pub path: PathBuf,
    pub pos: (f32, f32),
    pub sub_nodes: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub objects: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    pub text: String,
    pub pos: (f32, f32),
    pub size: (f32, f32),
    pub color: (u8, u8, u8, u8),
    pub font_color: (u8, u8, u8, u8),
    pub font_size: f32,
    pub text_offset: (f32, f32),
    pub text_align: u8,
    pub corner_radius: f32,
    pub stroke_color: (u8, u8, u8, u8),
    pub stroke_width: f32,
    pub z_index: u32,
}
// Text Align:
//      0 - Center
//      1 - Left
//      2 - Right

#[derive(PartialEq)]
pub enum LinkType {
    ParentLink,
    ChildLink,
}

pub struct PageLink {
    pub link_type: LinkType,
    pub direction_vec: (f32, f32),
    pub file_name: PathBuf,
}
