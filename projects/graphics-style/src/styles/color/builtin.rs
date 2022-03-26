use super::*;

// noinspection SpellCheckingInspection
impl RGBA {
    /// `#F0F8FF = rgb(240 248 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F0F8FF;"></div>
    pub const ALICE_BLUE: Self = Self::new(240, 248, 255, 255);

    /// `#FAEBD7 = rgb(250 235 215)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FAEBD7;"></div>
    pub const ANTIQUE_WHITE: Self = Self::new(250, 235, 215, 255);

    /// `#00FFFF = rgb(0 255 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00FFFF;"></div>
    pub const AQUA: Self = Self::new(0, 255, 255, 255);

    /// `#7FFFD4 = rgb(127 255 212)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #7FFFD4;"></div>
    pub const AQUA_MARINE: Self = Self::new(127, 255, 212, 255);

    /// `#F0FFFF = rgb(240 255 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F0FFFF;"></div>
    pub const AZURE: Self = Self::new(240, 255, 255, 255);

    /// `#F5F5DC = rgb(245 245 220)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F5F5DC;"></div>
    pub const BEIGE: Self = Self::new(245, 245, 220, 255);

    /// `#FFE4C4 = rgb(255 228 196)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFE4C4;"></div>
    pub const BISQUE: Self = Self::new(255, 228, 196, 255);

    /// `#000000 = rgb(0 0 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #000000;"></div>
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    /// `#FFEBCD = rgb(255 235 205)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFEBCD;"></div>
    pub const BLANCHEDALMOND: Self = Self::new(255, 235, 205, 255);

    /// `#0000FF = rgb(0 0 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #0000FF;"></div>
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    /// `#8A2BE2 = rgb(138 43 226)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #8A2BE2;"></div>
    pub const BLUE_VIOLET: Self = Self::new(138, 43, 226, 255);

    /// `#A52A2A = rgb(165 42 42)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #A52A2A;"></div>
    pub const BROWN: Self = Self::new(165, 42, 42, 255);

    /// `#DEB887 = rgb(222 184 135)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DEB887;"></div>
    pub const BURLYWOOD: Self = Self::new(222, 184, 135, 255);

    /// `#5F9EA0 = rgb(95 158 160)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #5F9EA0;"></div>
    pub const CADET_BLUE: Self = Self::new(95, 158, 160, 255);

    /// `#7FFF00 = rgb(127 255 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #7FFF00;"></div>
    pub const CHARTREUSE: Self = Self::new(127, 255, 0, 255);

    /// `#D2691E = rgb(210 105 30)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #D2691E;"></div>
    pub const CHOCOLATE: Self = Self::new(210, 105, 30, 255);

    /// `#FF7F50 = rgb(255 127 80)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF7F50;"></div>
    pub const CORAL: Self = Self::new(255, 127, 80, 255);

    /// `#6495ED = rgb(100 149 237)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #6495ED;"></div>
    pub const CORNFLOWER_BLUE: Self = Self::new(100, 149, 237, 255);

    /// `#FFF8DC = rgb(255 248 220)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFF8DC;"></div>
    pub const CORNSILK: Self = Self::new(255, 248, 220, 255);

    /// `#DC143C = rgb(220 20 60)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DC143C;"></div>
    pub const CRIMSON: Self = Self::new(220, 20, 60, 255);

    /// `#00FFFF = rgb(0 255 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00FFFF;"></div>
    pub const CYAN: Self = Self::new(0, 255, 255, 255);

    /// `#00008B = rgb(0 0 139)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00008B;"></div>
    pub const DARK_BLUE: Self = Self::new(0, 0, 139, 255);

    /// `#008B8B = rgb(0 139 139)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #008B8B;"></div>
    pub const DARK_CYAN: Self = Self::new(0, 139, 139, 255);

    /// `#B8860B = rgb(184 134 11)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #B8860B;"></div>
    pub const DARK_GOLDENROD: Self = Self::new(184, 134, 11, 255);

    /// `#A9A9A9 = rgb(169 169 169)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #A9A9A9;"></div>
    pub const DARK_GRAY: Self = Self::new(169, 169, 169, 255);

    /// `#006400 = rgb(0 100 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #006400;"></div>
    pub const DARK_GREEN: Self = Self::new(0, 100, 0, 255);

    /// `#A9A9A9 = rgb(169 169 169)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #A9A9A9;"></div>
    pub const DARK_GREY: Self = Self::new(169, 169, 169, 255);

    /// `#BDB76B = rgb(189 183 107)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #BDB76B;"></div>
    pub const DARK_KHAKI: Self = Self::new(189, 183, 107, 255);

    /// `#8B008B = rgb(139 0 139)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #8B008B;"></div>
    pub const DARK_MAGENTA: Self = Self::new(139, 0, 139, 255);

    /// `#556B2F = rgb(85 107 47)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #556B2F;"></div>
    pub const DARK_OLIVE_GREEN: Self = Self::new(85, 107, 47, 255);

    /// `#FF8C00 = rgb(255 140 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF8C00;"></div>
    pub const DARK_ORANGE: Self = Self::new(255, 140, 0, 255);

    /// `#9932CC = rgb(153 50 204)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #9932CC;"></div>
    pub const DARK_ORCHID: Self = Self::new(153, 50, 204, 255);

    /// `#8B0000 = rgb(139 0 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #8B0000;"></div>
    pub const DARK_RED: Self = Self::new(139, 0, 0, 255);

    /// `#E9967A = rgb(233 150 122)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #E9967A;"></div>
    pub const DARK_SALMON: Self = Self::new(233, 150, 122, 255);

    /// `#8FBC8F = rgb(143 188 143)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #8FBC8F;"></div>
    pub const DARK_SEAGREEN: Self = Self::new(143, 188, 143, 255);

    /// `#483D8B = rgb(72 61 139)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #483D8B;"></div>
    pub const DARK_SLATE_BLUE: Self = Self::new(72, 61, 139, 255);

    /// `#2F4F4F = rgb(47 79 79)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #2F4F4F;"></div>
    pub const DARK_SLATEGRAY: Self = Self::new(47, 79, 79, 255);

    /// `#2F4F4F = rgb(47 79 79)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #2F4F4F;"></div>
    pub const DARK_SLATEGREY: Self = Self::new(47, 79, 79, 255);

    /// `#00CED1 = rgb(0 206 209)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00CED1;"></div>
    pub const DARK_TURQUOISE: Self = Self::new(0, 206, 209, 255);

    /// `#9400D3 = rgb(148 0 211)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #9400D3;"></div>
    pub const DARK_VIOLET: Self = Self::new(148, 0, 211, 255);

    /// `#FF1493 = rgb(255 20 147)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF1493;"></div>
    pub const DEEP_PINK: Self = Self::new(255, 20, 147, 255);

    /// `#00BFFF = rgb(0 191 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00BFFF;"></div>
    pub const DEEP_SKYBLUE: Self = Self::new(0, 191, 255, 255);

    /// `#696969 = rgb(105 105 105)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #696969;"></div>
    pub const DIMGRAY: Self = Self::new(105, 105, 105, 255);

    /// `#696969 = rgb(105 105 105)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #696969;"></div>
    pub const DIMGREY: Self = Self::new(105, 105, 105, 255);

    /// `#1E90FF = rgb(30 144 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #1E90FF;"></div>
    pub const DODGER_BLUE: Self = Self::new(30, 144, 255, 255);

    /// `#B22222 = rgb(178 34 34)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #B22222;"></div>
    pub const FIREBRICK: Self = Self::new(178, 34, 34, 255);

    /// `#FFFAF0 = rgb(255 250 240)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFAF0;"></div>
    pub const FLORAL_WHITE: Self = Self::new(255, 250, 240, 255);

    /// `#228B22 = rgb(34 139 34)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #228B22;"></div>
    pub const FORESTGREEN: Self = Self::new(34, 139, 34, 255);

    /// `#FF00FF = rgb(255 0 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF00FF;"></div>
    pub const FUCHSIA: Self = Self::new(255, 0, 255, 255);

    /// `#DCDCDC = rgb(220 220 220)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DCDCDC;"></div>
    pub const GAINSBORO: Self = Self::new(220, 220, 220, 255);

    /// `#F8F8FF = rgb(248 248 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F8F8FF;"></div>
    pub const GHOST_WHITE: Self = Self::new(248, 248, 255, 255);

    /// `#DAA520 = rgb(218 165 32)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DAA520;"></div>
    pub const GOLDEN_ROD: Self = Self::new(218, 165, 32, 255);

    /// `#FFD700 = rgb(255 215 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFD700;"></div>
    pub const GOLD: Self = Self::new(255, 215, 0, 255);

    /// `#808080 = rgb(128 128 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #808080;"></div>
    pub const GRAY: Self = Self::new(128, 128, 128, 255);

    /// `#008000 = rgb(0 128 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #008000;"></div>
    pub const GREEN: Self = Self::new(0, 128, 0, 255);

    /// `#ADFF2F = rgb(173 255 47)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #ADFF2F;"></div>
    pub const GREEN_YELLOW: Self = Self::new(173, 255, 47, 255);

    /// `#808080 = rgb(128 128 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #808080;"></div>
    pub const GREY: Self = Self::new(128, 128, 128, 255);

    /// `#F0FFF0 = rgb(240 255 240)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F0FFF0;"></div>
    pub const HONEYDEW: Self = Self::new(240, 255, 240, 255);

    /// `#FF69B4 = rgb(255 105 180)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF69B4;"></div>
    pub const HOTPINK: Self = Self::new(255, 105, 180, 255);

    /// `#CD5C5C = rgb(205 92 92)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #CD5C5C;"></div>
    pub const INDIANRED: Self = Self::new(205, 92, 92, 255);

    /// `#4B0082 = rgb(75 0 130)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #4B0082;"></div>
    pub const INDIGO: Self = Self::new(75, 0, 130, 255);

    /// `#FFFFF0 = rgb(255 255 240)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFFF0;"></div>
    pub const IVORY: Self = Self::new(255, 255, 240, 255);

    /// `#F0E68C = rgb(240 230 140)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F0E68C;"></div>
    pub const KHAKI: Self = Self::new(240, 230, 140, 255);

    /// `#FFF0F5 = rgb(255 240 245)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFF0F5;"></div>
    pub const LAVENDERBLUSH: Self = Self::new(255, 240, 245, 255);

    /// `#E6E6FA = rgb(230 230 250)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #E6E6FA;"></div>
    pub const LAVENDER: Self = Self::new(230, 230, 250, 255);

    /// `#7CFC00 = rgb(124 252 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #7CFC00;"></div>
    pub const LAWN_GREEN: Self = Self::new(124, 252, 0, 255);

    /// `#FFFACD = rgb(255 250 205)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFACD;"></div>
    pub const LEMON_CHIFFON: Self = Self::new(255, 250, 205, 255);

    /// `#ADD8E6 = rgb(173 216 230)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #ADD8E6;"></div>
    pub const LIGHT_BLUE: Self = Self::new(173, 216, 230, 255);

    /// `#F08080 = rgb(240 128 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F08080;"></div>
    pub const LIGHT_CORAL: Self = Self::new(240, 128, 128, 255);

    /// `#E0FFFF = rgb(224 255 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #E0FFFF;"></div>
    pub const LIGHT_CYAN: Self = Self::new(224, 255, 255, 255);

    /// `#FAFAD2 = rgb(250 250 210)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FAFAD2;"></div>
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::new(250, 250, 210, 255);

    /// `#D3D3D3 = rgb(211 211 211)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #D3D3D3;"></div>
    pub const LIGHT_GRAY: Self = Self::new(211, 211, 211, 255);

    /// `#90EE90 = rgb(144 238 144)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #90EE90;"></div>
    pub const LIGHT_GREEN: Self = Self::new(144, 238, 144, 255);

    /// `#D3D3D3 = rgb(211 211 211)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #D3D3D3;"></div>
    pub const LIGHT_GREY: Self = Self::new(211, 211, 211, 255);

    /// `#FFB6C1 = rgb(255 182 193)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFB6C1;"></div>
    pub const LIGHT_PINK: Self = Self::new(255, 182, 193, 255);

    /// `#FFA07A = rgb(255 160 122)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFA07A;"></div>
    pub const LIGHT_SALMON: Self = Self::new(255, 160, 122, 255);

    /// `#20B2AA = rgb(32 178 170)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #20B2AA;"></div>
    pub const LIGHT_SEA_GREEN: Self = Self::new(32, 178, 170, 255);

    /// `#87CEFA = rgb(135 206 250)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #87CEFA;"></div>
    pub const LIGHT_SKY_BLUE: Self = Self::new(135, 206, 250, 255);

    /// `#778899 = rgb(119 136 153)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #778899;"></div>
    pub const LIGHT_SLATE_GRAY: Self = Self::new(119, 136, 153, 255);

    /// `#778899 = rgb(119 136 153)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #778899;"></div>
    pub const LIGHT_SLATE_GREY: Self = Self::new(119, 136, 153, 255);

    /// `#B0C4DE = rgb(176 196 222)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #B0C4DE;"></div>
    pub const LIGHT_STEEL_BLUE: Self = Self::new(176, 196, 222, 255);

    /// `#FFFFE0 = rgb(255 255 224)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFFE0;"></div>
    pub const LIGHT_YELLOW: Self = Self::new(255, 255, 224, 255);

    /// `#00FF00 = rgb(0 255 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00FF00;"></div>
    pub const LIME: Self = Self::new(0, 255, 0, 255);

    /// `#32CD32 = rgb(50 205 50)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #32CD32;"></div>
    pub const LIME_GREEN: Self = Self::new(50, 205, 50, 255);

    /// `#FAF0E6 = rgb(250 240 230)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FAF0E6;"></div>
    pub const LINEN: Self = Self::new(250, 240, 230, 255);

    /// `#FF00FF = rgb(255 0 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF00FF;"></div>
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);

    /// `#800000 = rgb(128 0 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #800000;"></div>
    pub const MAROON: Self = Self::new(128, 0, 0, 255);

    /// `#66CDAA = rgb(102 205 170)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #66CDAA;"></div>
    pub const MEDIUM_AQUA_MARINE: Self = Self::new(102, 205, 170, 255);

    /// `#0000CD = rgb(0 0 205)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #0000CD;"></div>
    pub const MEDIUM_BLUE: Self = Self::new(0, 0, 205, 255);

    /// `#BA55D3 = rgb(186 85 211)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #BA55D3;"></div>
    pub const MEDIUM_ORCHID: Self = Self::new(186, 85, 211, 255);

    /// `#9370DB = rgb(147 112 219)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #9370DB;"></div>
    pub const MEDIUM_PURPLE: Self = Self::new(147, 112, 219, 255);

    /// `#3CB371 = rgb(60 179 113)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #3CB371;"></div>
    pub const MEDIUM_SEA_GREEN: Self = Self::new(60, 179, 113, 255);

    /// `#7B68EE = rgb(123 104 238)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #7B68EE;"></div>
    pub const MEDIUM_SLATE_BLUE: Self = Self::new(123, 104, 238, 255);

    /// `#00FA9A = rgb(0 250 154)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00FA9A;"></div>
    pub const MEDIUM_SPRING_GREEN: Self = Self::new(0, 250, 154, 255);

    /// `#48D1CC = rgb(72 209 204)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #48D1CC;"></div>
    pub const MEDIUM_TURQUOISE: Self = Self::new(72, 209, 204, 255);

    /// `#C71585 = rgb(199 21 133)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #C71585;"></div>
    pub const MEDIUM_VIOLET_RED: Self = Self::new(199, 21, 133, 255);

    /// `#191970 = rgb(25 25 112)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #191970;"></div>
    pub const MIDNIGHT_BLUE: Self = Self::new(25, 25, 112, 255);

    /// `#F5FFFA = rgb(245 255 250)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F5FFFA;"></div>
    pub const MINTCREAM: Self = Self::new(245, 255, 250, 255);

    /// `#FFE4E1 = rgb(255 228 225)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFE4E1;"></div>
    pub const MISTYROSE: Self = Self::new(255, 228, 225, 255);

    /// `#FFE4B5 = rgb(255 228 181)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFE4B5;"></div>
    pub const MOCCASIN: Self = Self::new(255, 228, 181, 255);

    /// `#FFDEAD = rgb(255 222 173)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFDEAD;"></div>
    pub const NAVAJO_WHITE: Self = Self::new(255, 222, 173, 255);

    /// `#000080 = rgb(0 0 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #000080;"></div>
    pub const NAVY: Self = Self::new(0, 0, 128, 255);

    /// `#FDF5E6 = rgb(253 245 230)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FDF5E6;"></div>
    pub const OLDLACE: Self = Self::new(253, 245, 230, 255);

    /// `#808000 = rgb(128 128 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #808000;"></div>
    pub const OLIVE: Self = Self::new(128, 128, 0, 255);

    /// `#6B8E23 = rgb(107 142 35)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #6B8E23;"></div>
    pub const OLIVEDRAB: Self = Self::new(107, 142, 35, 255);

    /// `#FFA500 = rgb(255 165 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFA500;"></div>
    pub const ORANGE: Self = Self::new(255, 165, 0, 255);

    /// `#FF4500 = rgb(255 69 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF4500;"></div>
    pub const ORANGERED: Self = Self::new(255, 69, 0, 255);

    /// `#DA70D6 = rgb(218 112 214)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DA70D6;"></div>
    pub const ORCHID: Self = Self::new(218, 112, 214, 255);

    /// `#EEE8AA = rgb(238 232 170)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #EEE8AA;"></div>
    pub const PALE_GOLDEN_ROD: Self = Self::new(238, 232, 170, 255);

    /// `#98FB98 = rgb(152 251 152)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #98FB98;"></div>
    pub const PALE_GREEN: Self = Self::new(152, 251, 152, 255);

    /// `#AFEEEE = rgb(175 238 238)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #AFEEEE;"></div>
    pub const PALE_TURQUOISE: Self = Self::new(175, 238, 238, 255);

    /// `#DB7093 = rgb(219 112 147)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DB7093;"></div>
    pub const PALE_VIOLET_RED: Self = Self::new(219, 112, 147, 255);

    /// `#FFEFD5 = rgb(255 239 213)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFEFD5;"></div>
    pub const PAPAYAWHIP: Self = Self::new(255, 239, 213, 255);

    /// `#FFDAB9 = rgb(255 218 185)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFDAB9;"></div>
    pub const PEACHPUFF: Self = Self::new(255, 218, 185, 255);

    /// `#CD853F = rgb(205 133 63)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #CD853F;"></div>
    pub const PERU: Self = Self::new(205, 133, 63, 255);

    /// `#FFC0CB = rgb(255 192 203)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFC0CB;"></div>
    pub const PINK: Self = Self::new(255, 192, 203, 255);

    /// `#DDA0DD = rgb(221 160 221)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #DDA0DD;"></div>
    pub const PLUM: Self = Self::new(221, 160, 221, 255);

    /// `#B0E0E6 = rgb(176 224 230)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #B0E0E6;"></div>
    pub const POWDER_BLUE: Self = Self::new(176, 224, 230, 255);

    /// `#800080 = rgb(128 0 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #800080;"></div>
    pub const PURPLE: Self = Self::new(128, 0, 128, 255);

    /// `#663399 = rgb(102 51 153)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #663399;"></div>
    pub const REBECCA_PURPLE: Self = Self::new(102, 51, 153, 255);

    /// `#FF0000 = rgb(255 0 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF0000;"></div>
    pub const RED: Self = Self::new(255, 0, 0, 255);

    /// `#BC8F8F = rgb(188 143 143)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #BC8F8F;"></div>
    pub const ROSY_BROWN: Self = Self::new(188, 143, 143, 255);

    /// `#4169E1 = rgb(65 105 225)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #4169E1;"></div>
    pub const ROYAL_BLUE: Self = Self::new(65, 105, 225, 255);

    /// `#8B4513 = rgb(139 69 19)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #8B4513;"></div>
    pub const SADDLE_BROWN: Self = Self::new(139, 69, 19, 255);

    /// `#FA8072 = rgb(250 128 114)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FA8072;"></div>
    pub const SALMON: Self = Self::new(250, 128, 114, 255);

    /// `#F4A460 = rgb(244 164 96)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F4A460;"></div>
    pub const SANDY_BROWN: Self = Self::new(244, 164, 96, 255);

    /// `#2E8B57 = rgb(46 139 87)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #2E8B57;"></div>
    pub const SEA_GREEN: Self = Self::new(46, 139, 87, 255);

    /// `#FFF5EE = rgb(255 245 238)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFF5EE;"></div>
    pub const SEA_SHELL: Self = Self::new(255, 245, 238, 255);

    /// `#A0522D = rgb(160 82 45)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #A0522D;"></div>
    pub const SIENNA: Self = Self::new(160, 82, 45, 255);

    /// `#C0C0C0 = rgb(192 192 192)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #C0C0C0;"></div>
    pub const SILVER: Self = Self::new(192, 192, 192, 255);

    /// `#87CEEB = rgb(135 206 235)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #87CEEB;"></div>
    pub const SKY_BLUE: Self = Self::new(135, 206, 235, 255);

    /// `#6A5ACD = rgb(106 90 205)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #6A5ACD;"></div>
    pub const SLATE_BLUE: Self = Self::new(106, 90, 205, 255);

    /// `#708090 = rgb(112 128 144)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #708090;"></div>
    pub const SLATE_GRAY: Self = Self::new(112, 128, 144, 255);

    /// `#708090 = rgb(112 128 144)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #708090;"></div>
    pub const SLATE_GREY: Self = Self::new(112, 128, 144, 255);

    /// `#FFFAFA = rgb(255 250 250)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFAFA;"></div>
    pub const SNOW: Self = Self::new(255, 250, 250, 255);

    /// `#00FF7F = rgb(0 255 127)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #00FF7F;"></div>
    pub const SPRING_GREEN: Self = Self::new(0, 255, 127, 255);

    /// `#4682B4 = rgb(70 130 180)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #4682B4;"></div>
    pub const STEEL_BLUE: Self = Self::new(70, 130, 180, 255);

    /// `#D2B48C = rgb(210 180 140)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #D2B48C;"></div>
    pub const TAN: Self = Self::new(210, 180, 140, 255);

    /// `#008080 = rgb(0 128 128)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #008080;"></div>
    pub const TEAL: Self = Self::new(0, 128, 128, 255);

    /// `#D8BFD8 = rgb(216 191 216)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #D8BFD8;"></div>
    pub const THISTLE: Self = Self::new(216, 191, 216, 255);

    /// `#FF6347 = rgb(255 99 71)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FF6347;"></div>
    pub const TOMATO: Self = Self::new(255, 99, 71, 255);

    /// `#40E0D0 = rgb(64 224 208)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #40E0D0;"></div>
    pub const TURQUOISE: Self = Self::new(64, 224, 208, 255);

    /// `#EE82EE = rgb(238 130 238)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #EE82EE;"></div>
    pub const VIOLET: Self = Self::new(238, 130, 238, 255);

    /// `#F5DEB3 = rgb(245 222 179)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F5DEB3;"></div>
    pub const WHEAT: Self = Self::new(245, 222, 179, 255);

    /// `#FFFFFF = rgb(255 255 255)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFFFF;"></div>
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    /// `#F5F5F5 = rgb(245 245 245)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #F5F5F5;"></div>
    pub const WHITE_SMOKE: Self = Self::new(245, 245, 245, 255);

    /// `#FFFF00 = rgb(255 255 0)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #FFFF00;"></div>
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);

    /// `#9ACD32 = rgb(154 205 50)`
    ///
    /// <div style="display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: #9ACD32;"></div>
    pub const YELLOW_GREEN: Self = Self::new(154, 205, 50, 255);
}
