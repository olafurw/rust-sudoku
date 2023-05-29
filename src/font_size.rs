use macroquad::text::{measure_text, Font, TextDimensions};

pub fn estimate_font_size(text: &str, font: Option<Font>, cell_size: f32) -> u16 {
    let measure_fn = measure_text_curry(text, font);
    find_best_font_size(measure_fn, cell_size)
}

fn measure_text_curry(text: &str, font: Option<Font>) -> Box<dyn Fn(u16) -> TextDimensions> {
    let text = text.to_owned();
    Box::new(move |font_size: u16| measure_text(&text, font, font_size, 1.0))
}

fn find_best_font_size(measure_fn: Box<dyn Fn(u16) -> TextDimensions>, cell_size: f32) -> u16 {
    if cell_size < 1.0 {
        return 1;
    }

    let mut start_size: u16 = 1;
    let mut end_size: u16 = 400;

    while start_size <= end_size {
        let test_size = (start_size + end_size) / 2;
        let measurement = measure_fn(test_size);
        let ratio = measurement.height / cell_size;

        if ratio > 0.59 && ratio < 0.62 {
            return test_size;
        } else if ratio >= 0.62 {
            end_size = test_size - 1;
        } else {
            start_size = test_size + 1;
        }
    }

    start_size
}

#[cfg(test)]
mod tests {
    use crate::font_size::find_best_font_size;
    use macroquad::text::TextDimensions;

    #[test]
    fn empty_measure() {
        let measure_fn = Box::new(move |font_size: u16| TextDimensions {
            width: font_size as f32,
            height: font_size as f32,
            offset_y: 0.0,
        });

        assert_eq!(find_best_font_size(measure_fn, 0.0), 1);
    }

    #[test]
    fn simple_measure() {
        let measure_fn = Box::new(move |font_size: u16| TextDimensions {
            width: font_size as f32,
            height: font_size as f32,
            offset_y: 0.0,
        });

        assert_eq!(find_best_font_size(measure_fn, 64.0), 38);
    }

    #[test]
    fn huge_measure() {
        let measure_fn = Box::new(move |font_size: u16| TextDimensions {
            width: font_size as f32,
            height: font_size as f32,
            offset_y: 0.0,
        });

        assert_eq!(find_best_font_size(measure_fn, 2000.0), 401);
    }
}
