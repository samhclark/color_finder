#[derive(Clone, Debug)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn to_hex(self: Self) -> String {
        format!("#{:02X?}{:02X?}{:02X?}", self.red, self.green, self.blue)
    }
}

fn main() {
    let white = Color { red: 255, green: 255, blue: 255 };
    let black = Color { red: 0, green: 0, blue: 0 };
    let primary = Color { red: 24, green: 120, blue: 209 };

    // let color_under_test = Color { red: 103, green: 77, blue: 137 };
    // let contrast_against_white = contrast(&color_under_test, &white);
    // let contrast_against_black = contrast(&color_under_test, &black);
    // // println!("Checking {:?}", color_under_test);
    // println!("({:?} has {}:1 contrast against black, {}:1 contrast against white", color_under_test, contrast_against_black, contrast_against_white);
    // if contrast_against_black >= 3.0 && contrast_against_white >= 7.0 {
    //     println!("({:?} has {}:1 contrast against black, {}:1 contrast against white", color_under_test, contrast_against_black, contrast_against_white);
    // }

    let mut passing_colors: Vec<Color> = vec![];
    let mut greatest_min_contrast: (Color, f64) = (Color { red: 0, green: 0, blue: 0 }, 0.0);
    let mut greatest_cume_contrast: (Color, f64) = (Color { red: 0, green: 0, blue: 0 }, 0.0);

    for i in 0u8..=255 {
        for j in 0u8..=255 {
            for k in 0u8..=255 {
                let color_under_test = Color { red: i, green: j, blue: k };
                let contrast_against_primary = contrast(&color_under_test, &primary);
                let contrast_against_white = contrast(&color_under_test, &white);
                let contrast_against_black = contrast(&color_under_test, &black);
                let lightness = lightness(&color_under_test);
                if contrast_against_primary > 3.0 && contrast_against_white > 4.5 {
                    passing_colors.push(color_under_test.clone());
                    if contrast_against_black.min(contrast_against_white) > greatest_min_contrast.1 {
                        greatest_min_contrast = (color_under_test.clone(), contrast_against_black.min(contrast_against_white));
                    }
                    if (contrast_against_black + contrast_against_white) > greatest_cume_contrast.1 {
                        greatest_cume_contrast = (color_under_test.clone(), contrast_against_black + contrast_against_white);
                    }
                    // println!("({:?} has {}:1 contrast against black, {}:1 contrast against white", color_under_test.to_hex(), contrast_against_black, contrast_against_white);
                }
            }
        }
    }

    // println!("{} has greatest minimum contrast", {greatest_min_contrast.0.to_hex()});
    // println!("{} has greatest cummulative contrast", {greatest_cume_contrast.0.to_hex()});

    print!("<doctype! html>");
    for color in passing_colors {
        let hex = color.to_hex();
        print!("<div style=\"background-color: {}; width: 128px; height: 128px;\" title=\"{}\"></div>", &hex, &hex);
    }
    println!("");
}

fn srgb_from(component: u8) -> f64 {
    f64::from(component) / 255.0
}

fn normalize(value: f64) -> f64 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn rel_luminance(color: &Color) -> f64 {
    let rs = srgb_from(color.red);
    let gs = srgb_from(color.green);
    let bs = srgb_from(color.blue);
    let r = normalize(rs);
    let g = normalize(gs);
    let b = normalize(bs);
    (0.2126 * r) + (0.7152 * g) + (0.0722 * b)
}

fn contrast_ratio(lighter: f64, darker: f64) -> f64 {
    assert!(lighter <= darker, "lighter must be less than or equal to darker");
    (lighter + 0.05) / (darker + 0.05)
}

fn contrast(a: &Color, b: &Color) -> f64 {
    let l1 = rel_luminance(a);
    let l2 = rel_luminance(b);
    if l1 < l2 {
        1.0 / contrast_ratio(l1, l2)
    } else {
        1.0 / contrast_ratio(l2, l1)
    }
}

fn lightness(color: &Color) -> f64 {
    let r = f64::from(color.red);
    let g = f64::from(color.green);
    let b = f64::from(color.blue);
    // let r = normalize(rs);
    // let g = normalize(gs);
    // let b = normalize(bs);

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    (((max + min) / 2.0) / 255.0) * 100.0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn contrast_of_white_and_black_is_21() {
        let white = Color { red: 255, green: 255, blue: 255 };
        let black = Color { red: 0, green: 0, blue: 0 };

        assert_eq!(contrast(&white, &black), 21.0)
    }

}