use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use eframe::egui::{Color32, Pos2, Vec2};

use crate::data::{Node, Object, Page};

impl Default for Node {
    fn default() -> Self {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("???")
            .as_nanos();

        Node {
            name: String::from("NewNode"),
            path: PathBuf::from(format!("{:x}.json", id)),
            pos: (0.0, 0.0),
            sub_nodes: None,
        }
    }
}

impl Node {
    pub fn get_pos(&self) -> Pos2 {
        Pos2::new(self.pos.0, self.pos.1)
    }
}

impl Default for Page {
    fn default() -> Self {
        Page {
            objects: Vec::new(),
        }
    }
}

impl Object {
    pub fn new(text: &str, pos: Pos2) -> Self {
        Object {
            text: String::from(text),
            pos: (pos.x, pos.y),
            size: (80.0, 20.0),
            corner_radius: 10.0,
            color: (80, 80, 80, 255),
            font_color: (255, 255, 255, 255),
            font_size: 10.0,
            text_offset: (10.0, 10.0),
            text_align: 0,
            stroke_color: (255, 255, 255, 255),
            stroke_width: 2.0,
            z_index: 0,
        }
    }

    pub fn get_start_pos(&self) -> Pos2 {
        Pos2::new(self.pos.0, self.pos.1)
    }

    pub fn get_end_pos(&self) -> Pos2 {
        self.get_start_pos() + Vec2::new(self.size.0, self.size.1)
    }

    pub fn get_color(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.color.0, self.color.1, self.color.2, self.color.3)
    }

    pub fn get_stroke_color(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(
            self.stroke_color.0,
            self.stroke_color.1,
            self.stroke_color.2,
            self.stroke_color.3,
        )
    }

    pub fn get_font_color(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(
            self.font_color.0,
            self.font_color.1,
            self.font_color.2,
            self.font_color.3,
        )
    }

    pub fn vec_u8_from_hex(color_str: &str) -> (u8, u8, u8, u8) {
        let hex = color_str.trim_start_matches("#");

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                (r, g, b, a)
            }
            _ => return (0, 0, 0, 255),
        }
    }

    pub fn hex_from_vec_u8(color: (u8, u8, u8, u8)) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color.0, color.1, color.2, color.3
        )
    }

    pub fn vec_u8_from_color32(color: Color32) -> (u8, u8, u8, u8) {
        (color.r(), color.g(), color.b(), color.a())
    }
}
