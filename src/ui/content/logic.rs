use eframe::egui;

use crate::{app::State, data::Page};

pub fn sort_by_z(crt_page: &mut Page) {
    crt_page.objects.sort_by_key(|a| a.z_index);
}

pub fn remove_obj_if_need(crt_page: &mut Page, id_to_remove: &mut Option<usize>) {
    if let Some(id) = id_to_remove {
        crt_page.objects.remove(*id);
        *id_to_remove = None;
    }
}

pub fn hotkey_process(ctx: &egui::Context, state: &mut State, crt_page: &mut Page) {
    let (copy, paste, delete, escape) = ctx.input(|i| {
        (
            i.key_pressed(egui::Key::C),
            i.key_pressed(egui::Key::V),
            i.key_pressed(egui::Key::Delete),
            i.key_pressed(egui::Key::Escape),
        )
    });

    let mouse_pos = ctx
        .input(|i| i.pointer.interact_pos())
        .unwrap_or(egui::Pos2::new(0.0, 0.0));

    copy_paste_process(copy, paste, state, crt_page, mouse_pos);
    delete_process(delete, state, crt_page);
    close_process(escape, state);
}

fn copy_paste_process(
    copy: bool,
    paste: bool,
    state: &mut State,
    crt_page: &mut Page,
    mouse_pos: egui::Pos2,
) {
    if copy
        && !state.is_selected_for_text_edit
        && let Some(index) = state.selected_object_id
        && let Some(object) = crt_page.objects.get(index)
    {
        state.copied_object = Some(object.clone());
    }
    if paste
        && !state.is_selected_for_text_edit
        && let Some(item) = state.copied_object.as_ref()
    {
        let mut object = item.clone();
        object.pos.0 = mouse_pos.x;
        object.pos.1 = mouse_pos.y;
        crt_page.objects.push(object);
    }
}

fn delete_process(delete: bool, state: &mut State, crt_page: &mut Page) {
    if delete
        && !state.is_selected_for_text_edit
        && let Some(index) = state.selected_object_id
    {
        crt_page.objects.remove(index);
    }
}

fn close_process(close: bool, state: &mut State) {
    if close {
        state.page_to_close = true;
    }
}
