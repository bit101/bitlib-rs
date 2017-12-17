use random::Random;

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

    pub fn rgb_int(r: i32, g: i32, b: i32) -> Color {
        Color::rgba_int(r, g, b, 255)
    }

    pub fn rgba_int(r: i32, g: i32, b: i32, a: i32) -> Color {
        Color::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
    }

    pub fn from_int(value: i64) -> Color {
        let a = (value >> 24) as i32;
        let r = (value >> 16 & 0xff) as i32;
        let g = (value >> 8 & 0xff) as i32;
        let b = (value & 0xff) as i32;
        Color::rgba_int(r, g, b, a)
    }

    pub fn random_rgb() -> Color {
        let mut rand = Random::new();
        let r = rand.float(0.0, 1.0); 
        let g = rand.float(0.0, 1.0); 
        let b = rand.float(0.0, 1.0); 
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

    pub fn random_hsv(hmin: f64, hmax: f64, smin: f64, smax: f64, vmin: f64, vmax: f64) -> Color {
        let mut rand = Random::new();
        Color::hsv(rand.float(hmin, hmax), rand.float(smin, smax), rand.float(vmin, vmax))
    }

    pub fn grey(shade: f64) -> Color {
        Color::rgb(shade, shade, shade)
    }

    pub fn grey_int(shade: i32) -> Color {
        Color::grey(shade as f64 / 255.0)
    }

    pub fn random_grey() -> Color {
        Color::random_grey_range(0.0, 1.0)
    }

    pub fn random_grey_range(min: f64, max: f64) -> Color {
        let mut rand = Random::new();
        Color::grey(rand.float(min, max))
    }

    pub fn from_string(src: &str) -> Color {
        if &src[0..1] == "#" {
            let num = i64::from_str_radix(&src[1..], 16)
                .expect("Could not parse color");
            let mut col = Color::from_int(num);
            if src.len() == 7 {
                col.a = 1.0;
            }
            col
        }
        else {
            Color::from_named_string(src)
        }
    }

    pub fn from_named_string(src: &str) -> Color {
        match src {
            "blueviolet" => Color::rgb_int(138,43,226),
            "brown" => Color::rgb_int(165,42,42),
            "aliceblue" => Color::rgb_int(240,248,255),
            "antiquewhite" => Color::rgb_int(250,235,215),
            "aqua" => Color::rgb_int(0,255,255),
            "aquamarine" => Color::rgb_int(127,255,212),
            "azure" => Color::rgb_int(240,255,255),
            "beige" => Color::rgb_int(245,245,220),
            "bisque" => Color::rgb_int(255,228,196),
            "black" => Color::rgb_int(0,0,0),
            "blanchedalmond" => Color::rgb_int(255,235,205),
            "blue" => Color::rgb_int(0,0,255),
            "burlywood" => Color::rgb_int(222,184,135),
            "cadetblue" => Color::rgb_int(95,158,160),
            "chartreuse" => Color::rgb_int(127,255,0),
            "chocolate" => Color::rgb_int(210,105,30),
            "coral" => Color::rgb_int(255,127,80),
            "cornflowerblue" => Color::rgb_int(100,149,237),
            "cornsilk" => Color::rgb_int(255,248,220),
            "crimson" => Color::rgb_int(220,20,60),
            "cyan" => Color::rgb_int(0,255,255),
            "darkblue" => Color::rgb_int(0,0,139),
            "darkcyan" => Color::rgb_int(0,139,139),
            "darkgoldenrod" => Color::rgb_int(184,134,11),
            "darkgray" => Color::rgb_int(169,169,169),
            "darkgreen" => Color::rgb_int(0,100,0),
            "darkgrey" => Color::rgb_int(169,169,169),
            "darkkhaki" => Color::rgb_int(189,183,107),
            "darkmagenta" => Color::rgb_int(139,0,139),
            "darkolivegreen" => Color::rgb_int(85,107,47),
            "darkorange" => Color::rgb_int(255,140,0),
            "darkorchid" => Color::rgb_int(153,50,204),
            "darkred" => Color::rgb_int(139,0,0),
            "darksalmon" => Color::rgb_int(233,150,122),
            "darkseagreen" => Color::rgb_int(143,188,143),
            "darkslateblue" => Color::rgb_int(72,61,139),
            "darkslategray" => Color::rgb_int(47,79,79),
            "darkslategrey" => Color::rgb_int(47,79,79),
            "darkturquoise" => Color::rgb_int(0,206,209),
            "darkviolet" => Color::rgb_int(148,0,211),
            "deeppink" => Color::rgb_int(255,20,147),
            "deepskyblue" => Color::rgb_int(0,191,255),
            "dimgray" => Color::rgb_int(105,105,105),
            "dimgrey" => Color::rgb_int(105,105,105),
            "dodgerblue" => Color::rgb_int(30,144,255),
            "firebrick" => Color::rgb_int(178,34,34),
            "floralwhite" => Color::rgb_int(255,250,240),
            "forestgreen" => Color::rgb_int(34,139,34),
            "fuchsia" => Color::rgb_int(255,0,255),
            "gainsboro" => Color::rgb_int(220,220,220),
            "ghostwhite" => Color::rgb_int(248,248,255),
            "gold" => Color::rgb_int(255,215,0),
            "goldenrod" => Color::rgb_int(218,165,32),
            "gray" => Color::rgb_int(128,128,128),
            "green" => Color::rgb_int(0,128,0),
            "greenyellow" => Color::rgb_int(173,255,47),
            "honeydew" => Color::rgb_int(240,255,240),
            "hotpink" => Color::rgb_int(255,105,180),
            "indianred" => Color::rgb_int(205,92,92),
            "indigo" => Color::rgb_int(75,0,130),
            "ivory" => Color::rgb_int(255,255,240),
            "khaki" => Color::rgb_int(240,230,140),
            "lavender" => Color::rgb_int(230,230,250),
            "lavenderblush" => Color::rgb_int(255,240,245),
            "lawngreen" => Color::rgb_int(124,252,0),
            "lemonchiffon" => Color::rgb_int(255,250,205),
            "lightblue" => Color::rgb_int(173,216,230),
            "lightcoral" => Color::rgb_int(240,128,128),
            "lightcyan" => Color::rgb_int(224,255,255),
            "lightgoldenrodyellow" => Color::rgb_int(250,250,210),
            "lightgray" => Color::rgb_int(211,211,211),
            "lightgreen" => Color::rgb_int(144,238,144),
            "lightgrey" => Color::rgb_int(211,211,211),
            "lightpink" => Color::rgb_int(255,182,193),
            "lightsalmon" => Color::rgb_int(255,160,122),
            "lightseagreen" => Color::rgb_int(32,178,170),
            "lightskyblue" => Color::rgb_int(135,206,250),
            "lightslategray" => Color::rgb_int(119,136,153),
            "lightslategrey" => Color::rgb_int(119,136,153),
            "lightsteelblue" => Color::rgb_int(176,196,222),
            "lightyellow" => Color::rgb_int(255,255,224),
            "lime" => Color::rgb_int(0,255,0),
            "limegreen" => Color::rgb_int(50,205,50),
            "linen" => Color::rgb_int(250,240,230),
            "magenta" => Color::rgb_int(255,0,255),
            "maroon" => Color::rgb_int(128,0,0),
            "mediumaquamarine" => Color::rgb_int(102,205,170),
            "mediumblue" => Color::rgb_int(0,0,205),
            "mediumorchid" => Color::rgb_int(186,85,211),
            "mediumpurple" => Color::rgb_int(147,112,219),
            "mediumseagreen" => Color::rgb_int(60,179,113),
            "mediumslateblue" => Color::rgb_int(123,104,238),
            "mediumspringgreen" => Color::rgb_int(0,250,154),
            "mediumturquoise" => Color::rgb_int(72,209,204),
            "mediumvioletred" => Color::rgb_int(199,21,133),
            "midnightblue" => Color::rgb_int(25,25,112),
            "mintcream" => Color::rgb_int(245,255,250),
            "mistyrose" => Color::rgb_int(255,228,225),
            "moccasin" => Color::rgb_int(255,228,181),
            "navajowhite" => Color::rgb_int(255,222,173),
            "navy" => Color::rgb_int(0,0,128),
            "oldlace" => Color::rgb_int(253,245,230),
            "olive" => Color::rgb_int(128,128,0),
            "olivedrab" => Color::rgb_int(107,142,35),
            "orange" => Color::rgb_int(255,165,0),
            "orangered" => Color::rgb_int(255,69,0),
            "orchid" => Color::rgb_int(218,112,214),
            "palegoldenrod" => Color::rgb_int(238,232,170),
            "palegreen" => Color::rgb_int(152,251,152),
            "paleturquoise" => Color::rgb_int(175,238,238),
            "palevioletred" => Color::rgb_int(219,112,147),
            "papayawhip" => Color::rgb_int(255,239,213),
            "peachpuff" => Color::rgb_int(255,218,185),
            "peru" => Color::rgb_int(205,133,63),
            "pink" => Color::rgb_int(255,192,203),
            "plum" => Color::rgb_int(221,160,221),
            "powderblue" => Color::rgb_int(176,224,230),
            "purple" => Color::rgb_int(128,0,128),
            "rebeccapurple" => Color::rgb_int(102,51,153),
            "red" => Color::rgb_int(255,0,0),
            "rosybrown" => Color::rgb_int(188,143,143),
            "royalblue" => Color::rgb_int(65,105,225),
            "saddlebrown" => Color::rgb_int(139,69,19),
            "salmon" => Color::rgb_int(250,128,114),
            "sandybrown" => Color::rgb_int(244,164,96),
            "seagreen" => Color::rgb_int(46,139,87),
            "seashell" => Color::rgb_int(255,245,238),
            "sienna" => Color::rgb_int(160,82,45),
            "silver" => Color::rgb_int(192,192,192),
            "skyblue" => Color::rgb_int(135,206,235),
            "slateblue" => Color::rgb_int(106,90,205),
            "slategray" => Color::rgb_int(112,128,144),
            "slategrey" => Color::rgb_int(112,128,144),
            "snow" => Color::rgb_int(255,250,250),
            "springgreen" => Color::rgb_int(0,255,127),
            "steelblue" => Color::rgb_int(70,130,180),
            "tan" => Color::rgb_int(210,180,140),
            "teal" => Color::rgb_int(0,128,128),
            "thistle" => Color::rgb_int(216,191,216),
            "tomato" => Color::rgb_int(255,99,71),
            "turquoise" => Color::rgb_int(64,224,208),
            "violet" => Color::rgb_int(238,130,238),
            "wheat" => Color::rgb_int(245,222,179),
            "white" => Color::rgb_int(255,255,255),
            "whitesmoke" => Color::rgb_int(245,245,245),
            "yellow" => Color::rgb_int(255,255,0),
            "yellowgreen" => Color::rgb_int(154,205,50),
            _ => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_int() {
        let c = Color::rgb_int(255, 0, 0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::rgba_int(255, 0, 0, 255);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_from_int() {
        let c = Color::from_int(0xff0000);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 0.0);

        let c = Color::from_int(0xffff0000);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_hsv() {
        let c = Color::hsv(0.0, 1.0, 1.0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::hsv(60.0, 1.0, 1.0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::hsv(120.0, 1.0, 1.0);
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::hsv(240.0, 1.0, 1.0);
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_greys() {
        let c = Color::grey(0.56);
        assert_eq!(c.r, 0.56);
        assert_eq!(c.g, 0.56);
        assert_eq!(c.b, 0.56);
        assert_eq!(c.a, 1.0);

        let c = Color::grey_int(255);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 1.0);

        let c = Color::random_grey_range(0.25, 0.35);
        assert!(c.r >= 0.25 && c.r <= 0.35);
        assert_eq!(c.r, c.g);
        assert_eq!(c.r, c.b);
        assert_eq!(c.a, 1.0);

    }


    #[test]
    fn test_from_string() {
        let c = Color::from_string("#ff0000");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("#00ff00");
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("#00ff0000");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 0.0);

        let c = Color::from_string("#ffffffff");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("#FFFFFFFF");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("red");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("lime");
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("blue");
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 1.0);

        let c = Color::from_string("yellow");
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);

        // bad color = black
        let c = Color::from_string("foo");
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);
    }
}
