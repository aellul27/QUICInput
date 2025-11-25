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

pub fn find_window_size() -> (f64, f64) {
    let (height, width) = get_display_size();
    (f64::from(height) / 2.0, f64::from(width) / 2.0)
}