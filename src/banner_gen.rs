use mcbanner::{Banner, MCColor, Pattern};
use rand::Rng;
use crate::RequestBody;

fn get_pattern(pattern_str: &str) -> Option<Pattern> {
    let pattern_str = pattern_str.to_lowercase().replace('_', "");
    match pattern_str.as_str() {
        "squaretopleft" => Some(Pattern::SquareTopLeft),
        "squaretopright" => Some(Pattern::SquareTopRight),
        "squarebottomleft" => Some(Pattern::SquareBottomLeft),
        "squarebottomright" => Some(Pattern::SquareBottomRight),
        "stripebottom" => Some(Pattern::StripeBottom),
        "stripetop" => Some(Pattern::StripeTop),
        "stripeleft" => Some(Pattern::StripeLeft),
        "striperight" => Some(Pattern::StripeRight),
        "stripecenter" => Some(Pattern::StripeCenter),
        "stripemiddle" => Some(Pattern::StripeMiddle),
        "stripedownright" => Some(Pattern::StripeDownright),
        "stripedownleft" => Some(Pattern::StripeDownleft),
        "stripesmall" => Some(Pattern::StripeSmall),
        "cross" => Some(Pattern::Cross),
        "straightcross" => Some(Pattern::StraightCross),
        "border" => Some(Pattern::Border),
        "curlyborder" => Some(Pattern::CurlyBorder),
        "trianglebottom" => Some(Pattern::TriangleBottom),
        "triangletop" => Some(Pattern::TriangleTop),
        "trianglesbottom" => Some(Pattern::TrianglesBottom),
        "trianglestop" => Some(Pattern::TrianglesTop),
        "diagonalleft" => Some(Pattern::DiagonalTopLeft),
        "diagonalright" => Some(Pattern::DiagonalTopRight),
        "diagonalleftmirror" => Some(Pattern::DiagonalBottomLeft),
        "diagonalrightmirror" => Some(Pattern::DiagonalBottomRight),
        "circlemiddle" => Some(Pattern::CircleMiddle),
        "rhombusmiddle" => Some(Pattern::RhombusMiddle),
        "halfvertical" => Some(Pattern::HalfVertical),
        "halfverticalmirror" => Some(Pattern::HalfVerticalMirror),
        "halfhorizontal" => Some(Pattern::HalfHorizontal),
        "halfhorizontalmirror" => Some(Pattern::HalfHorizontalMirror),
        "creeper" => Some(Pattern::Creeper),
        "bricks" => Some(Pattern::Bricks),
        "gradient" => Some(Pattern::Gradient),
        "gradientup" => Some(Pattern::GradientUp),
        "skull" => Some(Pattern::Skull),
        "flower" => Some(Pattern::Flower),
        "mojang" => Some(Pattern::Thing),
        _ => None,
    }
}

fn get_color(color: &str) -> MCColor {
    match color {
        "black" => MCColor::Black,
        "blue" => MCColor::Blue,
        "brown" => MCColor::Brown,
        "cyan" => MCColor::Cyan,
        "gray" => MCColor::Gray,
        "green" => MCColor::Green,
        "light_blue" => MCColor::LightBlue,
        "light_gray" => MCColor::LightGray,
        "lime" => MCColor::Lime,
        "magenta" => MCColor::Magenta,
        "orange" => MCColor::Orange,
        "pink" => MCColor::Pink,
        "purple" => MCColor::Purple,
        "red" => MCColor::Red,
        "white" => MCColor::White,
        "yellow" => MCColor::Yellow,
        _ => MCColor::White,
    }
}

pub fn generate_banner(request: RequestBody) -> String{
    let mut rng = rand::thread_rng();
    let image_path = format!("images/{}.png", rng.gen::<u32>());

    let mut banner = Banner::new(get_color(request.base_color.as_str()));

    request.patterns.iter().for_each(|bodypattern| {
        let pattern = get_pattern(bodypattern.pattern.as_str()).unwrap();
        let color = get_color(bodypattern.color.as_str());
        banner.add_pattern(pattern, color);
    });
    banner.render().save(image_path.as_str()).unwrap();

    image_path
}