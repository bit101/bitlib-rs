extern crate rand;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b, a: 1.0 }
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    pub fn number(value: i32) -> Color {
        let r = (value >> 16 & 0xff) as f64;
        let g = (value >> 8 & 0xff) as f64;
        let b = (value & 0xff) as f64;
        Color::rgb(r, g, b)
    }

    pub fn number_with_alpha(value: i32) -> Color {
        let a = (value >> 24) as f64;
        let r = (value >> 16 & 0xff) as f64;
        let g = (value >> 8 & 0xff) as f64;
        let b = (value & 0xff) as f64;
        Color::rgba(r, g, b, a)
    }

    pub fn rgb_hex(r: i32, g: i32, b: i32) -> Color {
        Color::rgba_hex(r, g, b, 255)
    }

    pub fn rgba_hex(r: i32, g: i32, b: i32, a: i32) -> Color {
        Color::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
    }

    pub fn random_rgb() -> Color {
        let r = rand::random::<f64>();
        let g = rand::random::<f64>();
        let b = rand::random::<f64>();
        Color::rgb(r, g, b)
    }

    pub fn hsv(h: f64, s: f64, v: f64) -> Color {
        let h = h / 360.0;
        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        match (i % 6.0) as i32 {
            0 => Color::rgb(v, t, p),
            1 => Color::rgb(q, v, p),
            2 => Color::rgb(p, v, t),
            3 => Color::rgb(p, q, v),
            4 => Color::rgb(t, p, v),
            5 => Color::rgb(v, p, q),
            _ => Color::rgb(0.0, 0.0, 0.0),
        }
    }

    pub fn grey(shade: f64) -> Color {
        Color::rgb(shade, shade, shade)
    }

    pub fn grey_hex(shade: i32) -> Color {
        Color::grey(shade as f64 / 255.0)
    }

    pub fn random_grey() -> Color {
        Color::random_grey_range(0.0, 1.0)
    }

    pub fn random_grey_range(min: f64, max: f64) -> Color {
        Color::grey(min + rand::random::<f64>() * (max - min))
    }

    pub fn blueviolet() -> Color { Color::rgb_hex(138,43,226) }
    pub fn brown() -> Color { Color::rgb_hex(165,42,42) }
    pub fn aliceblue() -> Color { Color::rgb_hex(240,248,255) }
    pub fn antiquewhite() -> Color { Color::rgb_hex(250,235,215) }
    pub fn aqua() -> Color { Color::rgb_hex(0,255,255) }
    pub fn aquamarine() -> Color { Color::rgb_hex(127,255,212) }
    pub fn azure() -> Color { Color::rgb_hex(240,255,255) }
    pub fn beige() -> Color { Color::rgb_hex(245,245,220) }
    pub fn bisque() -> Color { Color::rgb_hex(255,228,196) }
    pub fn black() -> Color { Color::rgb_hex(0,0,0) }
    pub fn blanchedalmond() -> Color { Color::rgb_hex(255,235,205) }
    pub fn blue() -> Color { Color::rgb_hex(0,0,255) }
    pub fn burlywood() -> Color { Color::rgb_hex(222,184,135) }
    pub fn cadetblue() -> Color { Color::rgb_hex(95,158,160) }
    pub fn chartreuse() -> Color { Color::rgb_hex(127,255,0) }
    pub fn chocolate() -> Color { Color::rgb_hex(210,105,30) }
    pub fn coral() -> Color { Color::rgb_hex(255,127,80) }
    pub fn cornflowerblue() -> Color { Color::rgb_hex(100,149,237) }
    pub fn cornsilk() -> Color { Color::rgb_hex(255,248,220) }
    pub fn crimson() -> Color { Color::rgb_hex(220,20,60) }
    pub fn cyan() -> Color { Color::rgb_hex(0,255,255) }
    pub fn darkblue() -> Color { Color::rgb_hex(0,0,139) }
    pub fn darkcyan() -> Color { Color::rgb_hex(0,139,139) }
    pub fn darkgoldenrod() -> Color { Color::rgb_hex(184,134,11) }
    pub fn darkgray() -> Color { Color::rgb_hex(169,169,169) }
    pub fn darkgreen() -> Color { Color::rgb_hex(0,100,0) }
    pub fn darkgrey() -> Color { Color::rgb_hex(169,169,169) }
    pub fn darkkhaki() -> Color { Color::rgb_hex(189,183,107) }
    pub fn darkmagenta() -> Color { Color::rgb_hex(139,0,139) }
    pub fn darkolivegreen() -> Color { Color::rgb_hex(85,107,47) }
    pub fn darkorange() -> Color { Color::rgb_hex(255,140,0) }
    pub fn darkorchid() -> Color { Color::rgb_hex(153,50,204) }
    pub fn darkred() -> Color { Color::rgb_hex(139,0,0) }
    pub fn darksalmon() -> Color { Color::rgb_hex(233,150,122) }
    pub fn darkseagreen() -> Color { Color::rgb_hex(143,188,143) }
    pub fn darkslateblue() -> Color { Color::rgb_hex(72,61,139) }
    pub fn darkslategray() -> Color { Color::rgb_hex(47,79,79) }
    pub fn darkslategrey() -> Color { Color::rgb_hex(47,79,79) }
    pub fn darkturquoise() -> Color { Color::rgb_hex(0,206,209) }
    pub fn darkviolet() -> Color { Color::rgb_hex(148,0,211) }
    pub fn deeppink() -> Color { Color::rgb_hex(255,20,147) }
    pub fn deepskyblue() -> Color { Color::rgb_hex(0,191,255) }
    pub fn dimgray() -> Color { Color::rgb_hex(105,105,105) }
    pub fn dimgrey() -> Color { Color::rgb_hex(105,105,105) }
    pub fn dodgerblue() -> Color { Color::rgb_hex(30,144,255) }
    pub fn firebrick() -> Color { Color::rgb_hex(178,34,34) }
    pub fn floralwhite() -> Color { Color::rgb_hex(255,250,240) }
    pub fn forestgreen() -> Color { Color::rgb_hex(34,139,34) }
    pub fn fuchsia() -> Color { Color::rgb_hex(255,0,255) }
    pub fn gainsboro() -> Color { Color::rgb_hex(220,220,220) }
    pub fn ghostwhite() -> Color { Color::rgb_hex(248,248,255) }
    pub fn gold() -> Color { Color::rgb_hex(255,215,0) }
    pub fn goldenrod() -> Color { Color::rgb_hex(218,165,32) }
    pub fn gray() -> Color { Color::rgb_hex(128,128,128) }
    pub fn green() -> Color { Color::rgb_hex(0,128,0) }
    pub fn greenyellow() -> Color { Color::rgb_hex(173,255,47) }
    pub fn honeydew() -> Color { Color::rgb_hex(240,255,240) }
    pub fn hotpink() -> Color { Color::rgb_hex(255,105,180) }
    pub fn indianred() -> Color { Color::rgb_hex(205,92,92) }
    pub fn indigo() -> Color { Color::rgb_hex(75,0,130) }
    pub fn ivory() -> Color { Color::rgb_hex(255,255,240) }
    pub fn khaki() -> Color { Color::rgb_hex(240,230,140) }
    pub fn lavender() -> Color { Color::rgb_hex(230,230,250) }
    pub fn lavenderblush() -> Color { Color::rgb_hex(255,240,245) }
    pub fn lawngreen() -> Color { Color::rgb_hex(124,252,0) }
    pub fn lemonchiffon() -> Color { Color::rgb_hex(255,250,205) }
    pub fn lightblue() -> Color { Color::rgb_hex(173,216,230) }
    pub fn lightcoral() -> Color { Color::rgb_hex(240,128,128) }
    pub fn lightcyan() -> Color { Color::rgb_hex(224,255,255) }
    pub fn lightgoldenrodyellow() -> Color { Color::rgb_hex(250,250,210) }
    pub fn lightgray() -> Color { Color::rgb_hex(211,211,211) }
    pub fn lightgreen() -> Color { Color::rgb_hex(144,238,144) }
    pub fn lightgrey() -> Color { Color::rgb_hex(211,211,211) }
    pub fn lightpink() -> Color { Color::rgb_hex(255,182,193) }
    pub fn lightsalmon() -> Color { Color::rgb_hex(255,160,122) }
    pub fn lightseagreen() -> Color { Color::rgb_hex(32,178,170) }
    pub fn lightskyblue() -> Color { Color::rgb_hex(135,206,250) }
    pub fn lightslategray() -> Color { Color::rgb_hex(119,136,153) }
    pub fn lightslategrey() -> Color { Color::rgb_hex(119,136,153) }
    pub fn lightsteelblue() -> Color { Color::rgb_hex(176,196,222) }
    pub fn lightyellow() -> Color { Color::rgb_hex(255,255,224) }
    pub fn lime() -> Color { Color::rgb_hex(0,255,0) }
    pub fn limegreen() -> Color { Color::rgb_hex(50,205,50) }
    pub fn linen() -> Color { Color::rgb_hex(250,240,230) }
    pub fn magenta() -> Color { Color::rgb_hex(255,0,255) }
    pub fn maroon() -> Color { Color::rgb_hex(128,0,0) }
    pub fn mediumaquamarine() -> Color { Color::rgb_hex(102,205,170) }
    pub fn mediumblue() -> Color { Color::rgb_hex(0,0,205) }
    pub fn mediumorchid() -> Color { Color::rgb_hex(186,85,211) }
    pub fn mediumpurple() -> Color { Color::rgb_hex(147,112,219) }
    pub fn mediumseagreen() -> Color { Color::rgb_hex(60,179,113) }
    pub fn mediumslateblue() -> Color { Color::rgb_hex(123,104,238) }
    pub fn mediumspringgreen() -> Color { Color::rgb_hex(0,250,154) }
    pub fn mediumturquoise() -> Color { Color::rgb_hex(72,209,204) }
    pub fn mediumvioletred() -> Color { Color::rgb_hex(199,21,133) }
    pub fn midnightblue() -> Color { Color::rgb_hex(25,25,112) }
    pub fn mintcream() -> Color { Color::rgb_hex(245,255,250) }
    pub fn mistyrose() -> Color { Color::rgb_hex(255,228,225) }
    pub fn moccasin() -> Color { Color::rgb_hex(255,228,181) }
    pub fn navajowhite() -> Color { Color::rgb_hex(255,222,173) }
    pub fn navy() -> Color { Color::rgb_hex(0,0,128) }
    pub fn oldlace() -> Color { Color::rgb_hex(253,245,230) }
    pub fn olive() -> Color { Color::rgb_hex(128,128,0) }
    pub fn olivedrab() -> Color { Color::rgb_hex(107,142,35) }
    pub fn orange() -> Color { Color::rgb_hex(255,165,0) }
    pub fn orangered() -> Color { Color::rgb_hex(255,69,0) }
    pub fn orchid() -> Color { Color::rgb_hex(218,112,214) }
    pub fn palegoldenrod() -> Color { Color::rgb_hex(238,232,170) }
    pub fn palegreen() -> Color { Color::rgb_hex(152,251,152) }
    pub fn paleturquoise() -> Color { Color::rgb_hex(175,238,238) }
    pub fn palevioletred() -> Color { Color::rgb_hex(219,112,147) }
    pub fn papayawhip() -> Color { Color::rgb_hex(255,239,213) }
    pub fn peachpuff() -> Color { Color::rgb_hex(255,218,185) }
    pub fn peru() -> Color { Color::rgb_hex(205,133,63) }
    pub fn pink() -> Color { Color::rgb_hex(255,192,203) }
    pub fn plum() -> Color { Color::rgb_hex(221,160,221) }
    pub fn powderblue() -> Color { Color::rgb_hex(176,224,230) }
    pub fn purple() -> Color { Color::rgb_hex(128,0,128) }
    pub fn rebeccapurple() -> Color { Color::rgb_hex(102,51,153) }
    pub fn red() -> Color { Color::rgb_hex(255,0,0) }
    pub fn rosybrown() -> Color { Color::rgb_hex(188,143,143) }
    pub fn royalblue() -> Color { Color::rgb_hex(65,105,225) }
    pub fn saddlebrown() -> Color { Color::rgb_hex(139,69,19) }
    pub fn salmon() -> Color { Color::rgb_hex(250,128,114) }
    pub fn sandybrown() -> Color { Color::rgb_hex(244,164,96) }
    pub fn seagreen() -> Color { Color::rgb_hex(46,139,87) }
    pub fn seashell() -> Color { Color::rgb_hex(255,245,238) }
    pub fn sienna() -> Color { Color::rgb_hex(160,82,45) }
    pub fn silver() -> Color { Color::rgb_hex(192,192,192) }
    pub fn skyblue() -> Color { Color::rgb_hex(135,206,235) }
    pub fn slateblue() -> Color { Color::rgb_hex(106,90,205) }
    pub fn slategray() -> Color { Color::rgb_hex(112,128,144) }
    pub fn slategrey() -> Color { Color::rgb_hex(112,128,144) }
    pub fn snow() -> Color { Color::rgb_hex(255,250,250) }
    pub fn springgreen() -> Color { Color::rgb_hex(0,255,127) }
    pub fn steelblue() -> Color { Color::rgb_hex(70,130,180) }
    pub fn tan() -> Color { Color::rgb_hex(210,180,140) }
    pub fn teal() -> Color { Color::rgb_hex(0,128,128) }
    pub fn thistle() -> Color { Color::rgb_hex(216,191,216) }
    pub fn tomato() -> Color { Color::rgb_hex(255,99,71) }
    pub fn turquoise() -> Color { Color::rgb_hex(64,224,208) }
    pub fn violet() -> Color { Color::rgb_hex(238,130,238) }
    pub fn wheat() -> Color { Color::rgb_hex(245,222,179) }
    pub fn white() -> Color { Color::rgb_hex(255,255,255) }
    pub fn whitesmoke() -> Color { Color::rgb_hex(245,245,245) }
    pub fn yellow() -> Color { Color::rgb_hex(255,255,0) }
    pub fn yellowgreen() -> Color { Color::rgb_hex(154,205,50) }
}
