use palette::{FromColor, Oklch, Srgb};

fn parse_hex(hex: &str) -> Result<Srgb, String> {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue")?;
        Ok(Srgb::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    } else if hex.len() == 4 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "Invalid red")? * 17;
        let g = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "Invalid green")? * 17;
        let b = u8::from_str_radix(&hex[3..4], 16).map_err(|_| "Invalid blue")? * 17;
        Ok(Srgb::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        ))
    } else {
        Err("Invalid hex format".to_string())
    }
}

pub fn parse_color(color_str: &str) -> Result<Oklch, String> {
    if color_str.starts_with('#') {
        let srgb = parse_hex(color_str)?;
        Ok(Oklch::from_color(srgb))
    } else if color_str.starts_with("oklch(") && color_str.ends_with(')') {
        let inner = &color_str[6..color_str.len() - 1];
        let parts: Vec<&str> = inner.split_whitespace().collect();
        if parts.len() == 3 {
            let l: f32 = parts[0]
                .parse()
                .map_err(|_| "Invalid lightness".to_string())?;
            let c: f32 = parts[1].parse().map_err(|_| "Invalid chroma".to_string())?;
            let h: f32 = parts[2].parse().map_err(|_| "Invalid hue".to_string())?;
            Ok(Oklch::new(l, c, h))
        } else {
            Err(format!("Invalid oklch format: {color_str}"))
        }
    } else {
        Err(format!("Unsupported color format: {color_str}"))
    }
}

pub fn color_to_hex(color: Oklch) -> String {
    let srgb = Srgb::from_color(color);
    format!(
        "#{:02x}{:02x}{:02x}",
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8
    )
}

fn parse_percentage(amount: &str) -> Result<f32, String> {
    if amount.ends_with('%') {
        let num: f32 = amount
            .trim_end_matches('%')
            .parse()
            .map_err(|_| "Invalid percentage".to_string())?;
        Ok(num / 100.0)
    } else {
        amount.parse().map_err(|_| "Invalid amount".to_string())
    }
}

pub fn darken(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt = parse_percentage(amount)?;
    oklch.l = (oklch.l - amt).max(0.0);
    Ok(color_to_hex(oklch))
}

pub fn lighten(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt = parse_percentage(amount)?;
    oklch.l = (oklch.l + amt).min(1.0);
    Ok(color_to_hex(oklch))
}

pub fn temperature_up(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt: f32 = amount.parse().map_err(|_| "Invalid amount".to_string())?;
    if amt == 0.0 || amt % 360.0 == 0.0 {
        return Ok(color_to_hex(oklch));
    }
    use palette::OklabHue;
    oklch.hue = OklabHue::new((oklch.hue.into_degrees() + amt) % 360.0);
    Ok(color_to_hex(oklch))
}

pub fn temperature_down(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt: f32 = amount.parse().map_err(|_| "Invalid amount".to_string())?;
    if amt == 0.0 || amt % 360.0 == 0.0 {
        return Ok(color_to_hex(oklch));
    }
    use palette::OklabHue;
    oklch.hue = OklabHue::new((oklch.hue.into_degrees() - amt + 360.0) % 360.0);
    Ok(color_to_hex(oklch))
}

pub fn saturate(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt = parse_percentage(amount)?;
    oklch.chroma = (oklch.chroma + amt).min(0.4);
    Ok(color_to_hex(oklch))
}

pub fn desaturate(color: &str, amount: &str) -> Result<String, String> {
    let mut oklch = parse_color(color)?;
    let amt = parse_percentage(amount)?;
    oklch.chroma = (oklch.chroma - amt).max(0.0);
    Ok(color_to_hex(oklch))
}

pub fn mix(color1: &str, color2: &str, ratio: &str) -> Result<String, String> {
    let c1 = parse_color(color1)?;
    let c2 = parse_color(color2)?;
    let r: f32 = ratio.parse().map_err(|_| "Invalid ratio".to_string())?;
    let r = r.clamp(0.0, 1.0);

    let mixed = Oklch::new(
        c1.l * (1.0 - r) + c2.l * r,
        c1.chroma * (1.0 - r) + c2.chroma * r,
        c1.hue.into_degrees() * (1.0 - r) + c2.hue.into_degrees() * r,
    );
    Ok(color_to_hex(mixed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_valid() {
        let result = parse_hex("#ff0000");
        assert!(result.is_ok());
        let srgb = result.unwrap();
        assert_eq!(srgb.red, 1.0);
        assert_eq!(srgb.green, 0.0);
        assert_eq!(srgb.blue, 0.0);
    }

    #[test]
    fn test_parse_hex_invalid_length() {
        assert!(parse_hex("#ff00").is_err());
        assert!(parse_hex("#ff000000").is_err());
    }

    #[test]
    fn test_parse_hex_invalid_chars() {
        assert!(parse_hex("#gg0000").is_err());
    }

    #[test]
    fn test_parse_hex_no_hash() {
        assert!(parse_hex("ff0000").is_err());
    }

    #[test]
    fn test_parse_hex_3_digit() {
        let result = parse_hex("#f00");
        assert!(result.is_ok());
        let srgb = result.unwrap();
        assert_eq!(srgb.red, 1.0);
        assert_eq!(srgb.green, 0.0);
        assert_eq!(srgb.blue, 0.0);
    }

    #[test]
    fn test_parse_hex_3_digit_mixed_case() {
        let result = parse_hex("#A0F");
        assert!(result.is_ok());
        let srgb = result.unwrap();
        assert!((srgb.red - 170.0 / 255.0).abs() < 0.01);
        assert_eq!(srgb.green, 0.0);
        assert!((srgb.blue - 255.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_color_hex() {
        let result = parse_color("#ff0000");
        assert!(result.is_ok());
        let oklch = result.unwrap();
        assert!((oklch.l - 0.637).abs() < 0.01);
    }

    #[test]
    fn test_parse_color_hex_3_digit() {
        let result = parse_color("#f00");
        assert!(result.is_ok());
        let oklch = result.unwrap();
        assert!((oklch.l - 0.637).abs() < 0.01);
    }

    #[test]
    fn test_parse_color_oklch() {
        let result = parse_color("oklch(0.5 0.1 120)");
        assert!(result.is_ok());
        let oklch = result.unwrap();
        assert_eq!(oklch.l, 0.5);
        assert_eq!(oklch.chroma, 0.1);
        assert_eq!(oklch.hue, 120.0);
    }

    #[test]
    fn test_parse_color_oklch_invalid() {
        assert!(parse_color("oklch(0.5 0.1)").is_err());
        assert!(parse_color("oklch(invalid)").is_err());
        assert!(parse_color("oklch(0.5 invalid 120)").is_err());
    }

    #[test]
    fn test_parse_color_unsupported() {
        assert!(parse_color("hsl(120, 100%, 50%)").is_err());
        assert!(parse_color("rgb(255, 0, 0)").is_err());
    }

    #[test]
    fn test_color_to_hex() {
        let oklch = Oklch::new(0.5, 0.1, 120.0);
        let hex = color_to_hex(oklch);
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);
        // Should be able to parse it back
        assert!(parse_hex(&hex).is_ok());
    }

    #[test]
    fn test_parse_percentage_with_percent() {
        let result = parse_percentage("50%");
        assert_eq!(result.unwrap(), 0.5);
    }

    #[test]
    fn test_parse_percentage_decimal() {
        let result = parse_percentage("0.25");
        assert_eq!(result.unwrap(), 0.25);
    }

    #[test]
    fn test_parse_percentage_invalid() {
        assert!(parse_percentage("abc%").is_err());
        assert!(parse_percentage("invalid").is_err());
    }

    #[test]
    fn test_darken_normal() {
        let result = darken("#ffffff", "50%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.l < 0.5); // Should be darker than mid-gray
    }

    #[test]
    fn test_darken_to_black() {
        let result = darken("#ffffff", "100%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.l < 0.01);
    }

    #[test]
    fn test_darken_invalid_color() {
        assert!(darken("invalid", "10%").is_err());
    }

    #[test]
    fn test_darken_invalid_amount() {
        assert!(darken("#ff0000", "invalid").is_err());
    }

    #[test]
    fn test_lighten_normal() {
        let result = lighten("#000000", "50%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!((color.l - 0.5).abs() < 0.01); // Should be mid-gray
    }

    #[test]
    fn test_lighten_to_white() {
        let result = lighten("#000000", "100%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.l > 0.99);
    }

    #[test]
    fn test_temperature_up() {
        let result = temperature_up("#ff0000", "60").unwrap();
        assert!(result.starts_with('#'));
        assert_ne!(result, "#ff0000");
    }

    #[test]
    fn test_temperature_up_wrap_around() {
        let result = temperature_up("#ff0000", "360").unwrap();
        // Should wrap around to same color
        assert_eq!(result, "#ff0000");
    }

    #[test]
    fn test_temperature_up_large_amount() {
        let result = temperature_up("#ff0000", "400").unwrap();
        // 400 % 360 = 40, so should be same as temperature_up("#ff0000", "40")
        let result40 = temperature_up("#ff0000", "40").unwrap();
        assert_eq!(result, result40);
    }

    #[test]
    fn test_temperature_down() {
        let result = temperature_down("#ff0000", "60").unwrap();
        assert!(result.starts_with('#'));
        assert_ne!(result, "#ff0000");
    }

    #[test]
    fn test_temperature_down_wrap_around() {
        let result = temperature_down("#ff0000", "360").unwrap();
        // Should wrap around to same color
        assert_eq!(result, "#ff0000");
    }

    #[test]
    fn test_temperature_down_negative_wrap() {
        let result = temperature_down("#ff0000", "400").unwrap();
        // Should handle negative properly
        assert!(result.starts_with('#'));
        assert_ne!(result, "#ff0000");
    }

    #[test]
    fn test_saturate() {
        let result = saturate("#808080", "0.5").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma > 0.0);
    }

    #[test]
    fn test_saturate_cap_chroma() {
        let result = saturate("#ff0000", "1.0").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma <= 0.4);
    }

    #[test]
    fn test_desaturate() {
        let result = desaturate("#ff0000", "0.8").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma < 0.237); // Original red chroma
    }

    #[test]
    fn test_desaturate_to_gray() {
        let result = desaturate("#ff0000", "1.0").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma < 0.01);
    }

    #[test]
    fn test_edge_case_lightness_bounds() {
        // Test that lightness stays within [0, 1]
        let very_dark = darken("#000000", "100%").unwrap();
        let color = parse_color(&very_dark).unwrap();
        assert!(color.l >= 0.0);

        let very_light = lighten("#ffffff", "100%").unwrap();
        let color = parse_color(&very_light).unwrap();
        assert!(color.l <= 1.0);
    }

    #[test]
    fn test_edge_case_chroma_bounds() {
        // Test that chroma doesn't go negative
        let desaturated = desaturate("#ff0000", "2.0").unwrap();
        let color = parse_color(&desaturated).unwrap();
        assert!(color.chroma >= 0.0);
    }

    #[test]
    fn test_saturate_already_saturated() {
        let result = saturate("#ff0000", "0.1").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma <= 0.4);
    }

    #[test]
    fn test_temperature_functions_with_zero() {
        let result_up = temperature_up("#ff0000", "0").unwrap();
        assert_eq!(result_up, "#ff0000");
        let result_down = temperature_down("#ff0000", "0").unwrap();
        assert_eq!(result_down, "#ff0000");
    }

    #[test]
    fn test_lighten_darken_round_trip() {
        let original = "#808080";
        let lightened = lighten(original, "20%").unwrap();
        let darkened = darken(&lightened, "20%").unwrap();
        // Should be close to original, but not exactly due to floating point
        let orig_color = parse_color(original).unwrap();
        let final_color = parse_color(&darkened).unwrap();
        assert!((orig_color.l - final_color.l).abs() < 0.01);
    }

    #[test]
    fn test_mix_equal_colors() {
        let result = mix("#ff0000", "#ff0000", "0.5").unwrap();
        assert_eq!(result, "#ff0000");
    }

    #[test]
    fn test_mix_red_blue() {
        let result = mix("#ff0000", "#0000ff", "0.5").unwrap();
        assert!(result.starts_with("#"));
        // Should be a purple-ish color
        let color = parse_color(&result).unwrap();
        assert!(color.chroma > 0.0);
    }

    #[test]
    fn test_mix_ratio_zero() {
        let result = mix("#ff0000", "#0000ff", "0.0").unwrap();
        assert_eq!(result, "#ff0000");
    }

    #[test]
    fn test_mix_ratio_one() {
        let result = mix("#ff0000", "#0000ff", "1.0").unwrap();
        // Due to floating point precision, it might be very close but not exact
        assert!(result.starts_with("#0000"));
        assert!(result.ends_with("ff") || result.ends_with("fe"));
    }

    #[test]
    fn test_mix_invalid_color() {
        assert!(mix("invalid", "#0000ff", "0.5").is_err());
    }

    #[test]
    fn test_mix_invalid_ratio() {
        assert!(mix("#ff0000", "#0000ff", "invalid").is_err());
    }

    #[test]
    fn test_mix_ratio_out_of_bounds() {
        let result = mix("#ff0000", "#0000ff", "2.0").unwrap();
        // Should clamp to 1.0, but due to floating point, close to #0000ff
        assert!(result.starts_with("#0000"));
        assert!(result.ends_with("ff") || result.ends_with("fe"));
    }
}
