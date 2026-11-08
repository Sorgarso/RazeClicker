pub fn key_name(vk: usize) -> String {
    match vk {
        0x70..=0x87 => format!("F{}", vk - 0x6F),
        0x41..=0x5A | 0x30..=0x39 => (vk as u8 as char).to_string(),
        0x60..=0x69 => format!("Num {}", vk - 0x60),

        0x05 => "Mouse X1".into(),
        0x06 => "Mouse X2".into(),

        0x08 => "Backspace".into(),
        0x09 => "Tab".into(),
        0x0D => "Enter".into(),
        0x10 => "Shift".into(),
        0x11 => "Ctrl".into(),
        0x12 => "Alt".into(),
        0x14 => "Caps Lock".into(),
        0x20 => "Space".into(),
        0x21 => "Page Up".into(),
        0x22 => "Page Down".into(),
        0x23 => "End".into(),
        0x24 => "Home".into(),
        0x25 => "Left".into(),
        0x26 => "Up".into(),
        0x27 => "Right".into(),
        0x28 => "Down".into(),
        0x2D => "Insert".into(),
        0x2E => "Delete".into(),
        _ => format!("0x{vk:X}"),
    }
}
