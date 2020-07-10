use ray::Color;

pub fn write_color(pixel_color: Color) -> String {
    unsafe {
        format!(
            "{} {} {}\n",
            (255.999 * pixel_color.x()).to_int_unchecked::<u16>(),
            (255.999 * pixel_color.y()).to_int_unchecked::<u16>(),
            (255.999 * pixel_color.z()).to_int_unchecked::<u16>()
        )
    }
}
