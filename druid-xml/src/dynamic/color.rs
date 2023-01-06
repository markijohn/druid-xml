use druid::Color;

pub(crate) fn to_color(name:&str, default:Color) -> Color {
    match name {
        "lightsalmon"          => Color::rgb8(255,160,122),// #FFA07A	
        "salmon"               => Color::rgb8(250,128,114),// #FA8072	
        "darksalmon"           => Color::rgb8(233,150,122),//	#E9967A	
        "lightcoral"           => Color::rgb8(240,128,128),// #F08080
        "indianred"            => Color::rgb8(205,92,92),  // #CD5C5C
        "crimson"              => Color::rgb8(220,20,60),  // #DC143C
        "firebrick"            => Color::rgb8(178,34,34),  //	#B22222	
        "red"                  => Color::rgb8(255,0,0),    //	#FF0000	
        "darkred"              => Color::rgb8(139,0,0),    //	#8B0000	
        "coral"                => Color::rgb8(255,127,80), //	#FF7F50	
        "tomato"               => Color::rgb8(255,99,71),  //	#FF6347	
        "orangered"            => Color::rgb8(255,69,0),   //	#FF4500	
        "gold"                 => Color::rgb8(255,215,0),  //	#FFD700	
        "orange"               => Color::rgb8(255,165,0),  //	#FFA500	
        "darkorange"           => Color::rgb8(255,140,0),  //	#FF8C00	
        "lightyellow"          => Color::rgb8(255,255,224),//	#FFFFE0	
        "lemonchiffon"         => Color::rgb8(255,250,205),//	#FFFACD	
        "lightgoldenrodyellow" => Color::rgb8(250,250,210),//	#FAFAD2	
        "papayawhip"           => Color::rgb8(255,239,213),//	#FFEFD5	
        "moccasin"             => Color::rgb8(255,228,181),//	#FFE4B5	
        "peachpuff"            => Color::rgb8(255,218,185),//	#FFDAB9	
        "palegoldenrod"        => Color::rgb8(238,232,170),//	#EEE8AA	
        "khaki"                => Color::rgb8(240,230,140),//	#F0E68C	
        "darkkhaki"            => Color::rgb8(189,183,107),//	#BDB76B	
        "yellow"               => Color::rgb8(255,255,0),  //	#FFFF00	
        "lawngreen"            => Color::rgb8(124,252,0),  //	#7CFC00	
        "chartreuse"           => Color::rgb8(127,255,0),  //	#7FFF00	
        "limegreen"            => Color::rgb8(50,205,50),  //	#32CD32	
        "lime"                 => Color::rgb8(0.255.0),    //	#00FF00	
        "forestgreen"          => Color::rgb8(34,139,34),  //	#228B22	
        "green"                => Color::rgb8(0,128,0),    //	#008000	
        "darkgreen"            => Color::rgb8(0,100,0),    //	#006400	
        "greenyellow"          => Color::rgb8(173,255,47), //	#ADFF2F	
        "yellowgreen"          => Color::rgb8(154,205,50), //	#9ACD32	
        "springgreen"          => Color::rgb8(0,255,127),  //	#00FF7F	
        "mediumspringgreen"    => Color::rgb8(0,250,154),  //	#00FA9A	
        "lightgreen"           => Color::rgb8(144,238,144),//	#90EE90	
        "palegreen"            => Color::rgb8(152,251,152),//	#98FB98	
        "darkseagreen"         => Color::rgb8(143,188,143),//	#8FBC8F	
        "mediumseagreen"       => Color::rgb8(60,179,113), //	#3CB371	
        "seagreen"             => Color::rgb8(46,139,87),  //	#2E8B57	
        "olive"                => Color::rgb8(128,128,0),  //	#808000	
        "darkolivegreen"       => Color::rgb8(85,107,47),  //	#556B2F	
        "olivedrab"            => Color::rgb8(107,142,35), //	#6B8E23	
        "lightcyan"            => Color::rgb8(224,255,255),//	#E0FFFF	
        "cyan"                 => Color::rgb8(0,255,255),  //	#00FFFF	
        "aqua"                 => Color::rgb8(0,255,255),  //	#00FFFF	
        "aquamarine"           => Color::rgb8(127,255,212),//	#7FFFD4	
        "mediumaquamarine"     => Color::rgb8(102,205,170),//	#66CDAA	
        "paleturquoise"        => Color::rgb8(175,238,238),//	#AFEEEE	
        "turquoise"            => Color::rgb8(64,224,208), //	#40E0D0	
        "mediumturquoise"      => Color::rgb8(72,209,204), //	#48D1CC	
        "darkturquoise"        => Color::rgb8(0,206,209),  //	#00CED1	
        "lightseagreen"        => Color::rgb8(32,178,170), //	#20B2AA	
        "cadetblue"            => Color::rgb8(95,158,160), //	#5F9EA0	
        "darkcyan"             => Color::rgb8(0,139,139),  //	#008B8B	
        "teal"                 => Color::rgb8(0,128,128),  //	#008080	
        "powderblue"           => Color::rgb8(176,224,230),//	#B0E0E6	
        "lightblue"            => Color::rgb8(173,216,230),//	#ADD8E6	
        "lightskyblue"         => Color::rgb8(135,206,250),//	#87CEFA	
        "skyblue"              => Color::rgb8(135,206,235),//	#87CEEB	
        "deepskyblue"          => Color::rgb8(0,191,255),  //	#00BFFF	
        "lightsteelblue"       => Color::rgb8(176,196,222),//	#B0C4DE	
        "dodgerblue"           => Color::rgb8(30,144,255), //	#1E90FF	
        "cornflowerblue"       => Color::rgb8(100,149,237),//	#6495ED	
        "steelblue"            => Color::rgb8(70,130,180), //	#4682B4	
        "royalblue"            => Color::rgb8(65,105,225), //	#4169E1	
        "blue"                 => Color::rgb8(0,0,255),    //	#0000FF	
        "mediumblue"           => Color::rgb8(0,0,205),    //	#0000CD	
        "darkblue"             => Color::rgb8(0,0,139),    //	#00008B	
        "navy"                 => Color::rgb8(0,0,128),    //	#000080	
        "midnightblue"         => Color::rgb8(25,25,112),  //	#191970	
        "mediumslateblue"      => Color::rgb8(123,104,238),//	#7B68EE	
        "slateblue"            => Color::rgb8(106,90,205), //	#6A5ACD	
        "darkslateblue"        => Color::rgb8(72,61,139),  //	#483D8B	
        "lavender"             => Color::rgb8(230,230,250),//	#E6E6FA	
        "thistle"              => Color::rgb8(216,191,216),//	#D8BFD8	
        "plum"                 => Color::rgb8(221,160,221),//	#DDA0DD	
        "violet"               => Color::rgb8(238,130,238),//	#EE82EE	
        "orchid"               => Color::rgb8(218,112,214),//	#DA70D6	
        "fuchsia"              => Color::rgb8(255,0,255),  //	#FF00FF	
        "magenta"              => Color::rgb8(255,0,255),  //	#FF00FF	
        "mediumorchid"         => Color::rgb8(186,85,211), //	#BA55D3	
        "mediumpurple"         => Color::rgb8(147,112,219),//	#9370DB	
        "blueviolet"           => Color::rgb8(138,43,226), //	#8A2BE2	
        "darkviolet"           => Color::rgb8(148,0,211),  //	#9400D3	
        "darkorchid"           => Color::rgb8(153,50,204), //	#9932CC	
        "darkmagenta"          => Color::rgb8(139,0,139),  //	#8B008B	
        "purple"               => Color::rgb8(128,0,128),  //	#800080	
        "indigo"               => Color::rgb8(75,0,130),   //	#4B0082	
        "pink"                 => Color::rgb8(255,192,203),//	#FFC0CB	
        "lightpink"            => Color::rgb8(255,182,193),//	#FFB6C1	
        "hotpink"              => Color::rgb8(255,105,180),//	#FF69B4	
        "deeppink"             => Color::rgb8(255,20,147), //	#FF1493	
        "palevioletred"        => Color::rgb8(219,112,147),//	#DB7093	
        "mediumvioletred"      => Color::rgb8(199,21,133), //	#C71585	
        "white"                => Color::rgb8(255,255,255),//	#FFFFFF	
        "snow"                 => Color::rgb8(255,250,250),//	#FFFAFA	
        "honeydew"             => Color::rgb8(240,255,240),//	#F0FFF0	
        "mintcream"            => Color::rgb8(245,255,250),//	#F5FFFA	
        "azure"                => Color::rgb8(240,255,255),//	#F0FFFF	
        "aliceblue"            => Color::rgb8(240,248,255),//	#F0F8FF	
        "ghostwhite"           => Color::rgb8(248,248,255),//	#F8F8FF	
        "whitesmoke"           => Color::rgb8(245,245,245),//	#F5F5F5	
        "seashell"             => Color::rgb8(255,245,238),//	#FFF5EE	
        "beige"                => Color::rgb8(245,245,220),//	#F5F5DC	
        "oldlace"              => Color::rgb8(253,245,230),//	#FDF5E6	
        "floralwhite"          => Color::rgb8(255,250,240),//	#FFFAF0	
        "ivory"                => Color::rgb8(255,255,240),//	#FFFFF0	
        "antiquewhite"         => Color::rgb8(250,235,215),//	#FAEBD7	
        "linen"                => Color::rgb8(250,240,230),//	#FAF0E6	
        "lavenderblush"        => Color::rgb8(255,240,245),//	#FFF0F5	
        "mistyrose"            => Color::rgb8(255,228,225),//	#FFE4E1	
        "gainsboro"            => Color::rgb8(220,220,220),//	#DCDCDC	
        "lightgray"            => Color::rgb8(211,211,211),//	#D3D3D3	
        "silver"               => Color::rgb8(192,192,192),//	#C0C0C0	
        "darkgray"             => Color::rgb8(169,169,169),//	#A9A9A9	
        "gray"                 => Color::rgb8(128,128,128),//	#808080	
        "dimgray"              => Color::rgb8(105,105,105),//	#696969	
        "lightslategray"       => Color::rgb8(119,136,153),//	#778899	
        "slategray"            => Color::rgb8(112,128,144),//	#708090	
        "darkslategray"        => Color::rgb8(47,79,79),   //	#2F4F4F	
        "black"                => Color::rgb8(0,0,0),      //	#000000	
        "cornsilk"             => Color::rgb8(255,248,220),//	#FFF8DC	
        "blanchedalmond"       => Color::rgb8(255,235,205),//	#FFEBCD	
        "bisque"               => Color::rgb8(255,228,196),//	#FFE4C4	
        "navajowhite"          => Color::rgb8(255,222,173),//	#FFDEAD	
        "wheat"                => Color::rgb8(245,222,179),//	#F5DEB3	
        "burlywood"            => Color::rgb8(222,184,135),//	#DEB887	
        "tan"                  => Color::rgb8(210,180,140),//	#D2B48C	
        "rosybrown"            => Color::rgb8(188,143,143),//	#BC8F8F	
        "sandybrown"           => Color::rgb8(244,164,96), //	#F4A460	
        "goldenrod"            => Color::rgb8(218,165,32), //	#DAA520	
        "peru"                 => Color::rgb8(205,133,63), //	#CD853F	
        "chocolate"            => Color::rgb8(210,105,30), //	#D2691E	
        "saddlebrown"          => Color::rgb8(139,69,19),  //	#8B4513	
        "sienna"               => Color::rgb8(160,82,45),  //	#A0522D	
        "brown"                => Color::rgb8(165,42,42),  //	#A52A2A	
        "maroon"               => Color::rgb8(128,0,0),    //	#800000	
        _ => {
            if tv.starts_with('#') {
                Color::from_hex_str(tv)
            } else if tv.starts_with("rgb") && tv.ends_with(')') {
                let params = &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()].split(',');
                Color::rgb8(params.next().parse::<u8>().unwrap(), params.next().parse::<u8>().unwrap(), params.next().parse::<u8>().unwrap());
            } else if tv.starts_with("rgba") && tv.ends_with(')') {
                let params = &tv[tv.find('(').unwrap() .. tv.rfind(')').unwrap()].split(',');
                Color::rgba8(params.next().parse::<u8>().unwrap(), params.next().parse::<u8>().unwrap(), params.next().parse::<u8>().unwrap(), params.next().parse::<u8>().unwrap());
            } else {
                Color::black
            }
        }
    }
}