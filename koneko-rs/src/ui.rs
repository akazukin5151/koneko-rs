use crate::data;
use crate::utils;

impl data::Image<'_> {
    pub fn open_image(&self) {
        utils::open_in_browser(self.image_id)
    }

    pub fn download_image(&self) {}
    pub fn show_full_res(&self) {}
    pub fn next_image(&self) {}
    pub fn previous_image(&self) {}
    pub fn jump_to_image(&self, selected_image_num: i32) {}
    fn jump(&self) {}
    fn prefetch_next_image(&self) {}
    pub fn leave(&self, force: bool) {}
    pub fn start_preview(&self) {}
    pub fn preview(&self) {}
}
