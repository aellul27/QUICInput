use display_info::DisplayInfo;
pub fn get_display_size() -> (u32, u32) {
	let display_infos = DisplayInfo::all().unwrap();
    for display_info in display_infos.iter() {
        if display_info.is_primary {
            return (display_info.height, display_info.width);
        }
    }
    // fall back to the first display if no primary is flagged
    display_infos
        .first()
        .map(|info| (info.height, info.width))
        .unwrap_or((0, 0))
}

pub fn find_window_size() -> (u32, u32) {
    let display_info = get_display_size();
    (display_info.0 / 2, display_info.1 / 2)
}