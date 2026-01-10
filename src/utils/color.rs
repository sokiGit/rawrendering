pub struct Color {
    pub value: u32,
}

impl Default for Color {
    fn default() -> Self {
        Color { value: 0xFF000000 } // Transparent
    }
}

impl Color {
    pub fn new(value_argb: u32) -> Self { Color { value: value_argb } }
    pub fn from_rgba(value_rgba: u32) -> Self {
        Color {
            value: value_rgba >> 24 & 0xFF | value_rgba << 8 & 0xFFFFFF00
        }
    }
    
    //IDEA: Maybe implement more from_<format> constructors?
    
    pub(crate) fn mix_overlapping_destroy_transparency(&self, overlapping: &Color) -> Color {
        let fg_a = overlapping.value >> 24 & 0xFF;
        let bg_a = self.value >> 24 & 0xFF;
        let fg_a_standard = 1f32 - (fg_a as f32 / 255f32);
        let bg_a_standard = 1f32 - (bg_a as f32 / 255f32);
        let fin_a = ((1f32 - (fg_a_standard + bg_a_standard * (1f32 - fg_a_standard))) * 255f32) as u32 & 0xFF;
        
        let alpha = fg_a as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), mask isn't strictly necessary for u32

        let bg_r = self.value >> 16 & 0xFF;  // 0xFF0000 -> 0x0000FF (4 hex sh = 16 bin sh)
        let bg_g = self.value >> 8 & 0xFF;   // 0x00FF00 -> 0x0000FF (2 hex sh = 8 bin sh)
        let bg_b = self.value & 0xFF;        // 0x0000FF -> 0x0000FF (0 hex sh = 0 bin sh)

        // New "foreground" color
        let fg_r = overlapping.value >> 16 & 0xFF;
        let fg_g = overlapping.value >> 8 & 0xFF;
        let fg_b = overlapping.value & 0xFF;

        // Mix bg and fg colors
        let fin_r = ((bg_r as f32 * alpha) + (fg_r as f32 * (1f32 - alpha))) as u32 & 0xFF;
        let fin_g = ((bg_g as f32 * alpha) + (fg_g as f32 * (1f32 - alpha))) as u32 & 0xFF;
        let fin_b = ((bg_b as f32 * alpha) + (fg_b as f32 * (1f32 - alpha))) as u32 & 0xFF;

        let fin_color = fin_a << 24 | fin_r << 16 | fin_g << 8 | fin_b;
        
        // Return resulting color
        Color::new(fin_color)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self { Color { value } }
}