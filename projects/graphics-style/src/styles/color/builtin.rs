use super::*;

// noinspection SpellCheckingInspection

impl RGBA {
    /// `#F0F8FF = rgb(240 248 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F0F8FF;"></div>
    pub const ALICE_BLUE: Self = Self::new(0.941176, 0.972549, 1.0, 1.0);

    /// `#FAEBD7 = rgb(250 235 215)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FAEBD7;"></div>
    pub const ANTIQUE_WHITE: Self = Self::new(0.980392, 0.921569, 0.843137, 1.0);

    /// `#00FFFF = rgb(0 255 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00FFFF;"></div>
    pub const AQUA: Self = Self::new(0.0, 1.0, 1.0, 1.0);

    /// `#7FFFD4 = rgb(127 255 212)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #7FFFD4;"></div>
    pub const AQUA_MARINE: Self = Self::new(0.498039, 1.0, 0.831373, 1.0);

    /// `#F0FFFF = rgb(240 255 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F0FFFF;"></div>
    pub const AZURE: Self = Self::new(0.941176, 1.0, 1.0, 1.0);

    /// `#F5F5DC = rgb(245 245 220)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F5F5DC;"></div>
    pub const BEIGE: Self = Self::new(0.960784, 0.960784, 0.862745, 1.0);

    /// `#FFE4C4 = rgb(255 228 196)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFE4C4;"></div>
    pub const BISQUE: Self = Self::new(1.0, 0.894118, 0.768627, 1.0);

    /// `#000000 = rgb(0 0 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #000000;"></div>
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// `#FFEBCD = rgb(255 235 205)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFEBCD;"></div>
    pub const BLANCHEDALMOND: Self = Self::new(1.0, 0.921569, 0.803922, 1.0);

    /// `#0000FF = rgb(0 0 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #0000FF;"></div>
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    /// `#8A2BE2 = rgb(138 43 226)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #8A2BE2;"></div>
    pub const BLUE_VIOLET: Self = Self::new(0.541176, 0.168627, 0.886275, 1.0);

    /// `#A52A2A = rgb(165 42 42)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #A52A2A;"></div>
    pub const BROWN: Self = Self::new(0.647059, 0.164706, 0.164706, 1.0);

    /// `#DEB887 = rgb(222 184 135)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DEB887;"></div>
    pub const BURLYWOOD: Self = Self::new(0.870588, 0.721569, 0.529412, 1.0);

    /// `#5F9EA0 = rgb(95 158 160)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #5F9EA0;"></div>
    pub const CADET_BLUE: Self = Self::new(0.372549, 0.619608, 0.627451, 1.0);

    /// `#7FFF00 = rgb(127 255 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #7FFF00;"></div>
    pub const CHARTREUSE: Self = Self::new(0.498039, 1.0, 0.0, 1.0);

    /// `#D2691E = rgb(210 105 30)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #D2691E;"></div>
    pub const CHOCOLATE: Self = Self::new(0.823529, 0.411765, 0.117647, 1.0);

    /// `#FF7F50 = rgb(255 127 80)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF7F50;"></div>
    pub const CORAL: Self = Self::new(1.0, 0.498039, 0.313725, 1.0);

    /// `#6495ED = rgb(100 149 237)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #6495ED;"></div>
    pub const CORNFLOWER_BLUE: Self = Self::new(0.392157, 0.584314, 0.929412, 1.0);

    /// `#FFF8DC = rgb(255 248 220)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFF8DC;"></div>
    pub const CORNSILK: Self = Self::new(1.0, 0.972549, 0.862745, 1.0);

    /// `#DC143C = rgb(220 20 60)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DC143C;"></div>
    pub const CRIMSON: Self = Self::new(0.862745, 0.0784314, 0.235294, 1.0);

    /// `#00FFFF = rgb(0 255 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00FFFF;"></div>
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);

    /// `#00008B = rgb(0 0 139)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00008B;"></div>
    pub const DARK_BLUE: Self = Self::new(0.0, 0.0, 0.545098, 1.0);

    /// `#008B8B = rgb(0 139 139)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #008B8B;"></div>
    pub const DARK_CYAN: Self = Self::new(0.0, 0.545098, 0.545098, 1.0);

    /// `#B8860B = rgb(184 134 11)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #B8860B;"></div>
    pub const DARK_GOLDENROD: Self = Self::new(0.721569, 0.52549, 0.0431373, 1.0);

    /// `#A9A9A9 = rgb(169 169 169)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #A9A9A9;"></div>
    pub const DARK_GRAY: Self = Self::new(0.662745, 0.662745, 0.662745, 1.0);

    /// `#006400 = rgb(0 100 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #006400;"></div>
    pub const DARK_GREEN: Self = Self::new(0.0, 0.392157, 0.0, 1.0);

    /// `#A9A9A9 = rgb(169 169 169)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #A9A9A9;"></div>
    pub const DARK_GREY: Self = Self::new(0.662745, 0.662745, 0.662745, 1.0);

    /// `#BDB76B = rgb(189 183 107)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #BDB76B;"></div>
    pub const DARK_KHAKI: Self = Self::new(0.741176, 0.717647, 0.419608, 1.0);

    /// `#8B008B = rgb(139 0 139)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #8B008B;"></div>
    pub const DARK_MAGENTA: Self = Self::new(0.545098, 0.0, 0.545098, 1.0);

    /// `#556B2F = rgb(85 107 47)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #556B2F;"></div>
    pub const DARK_OLIVE_GREEN: Self = Self::new(0.333333, 0.419608, 0.184314, 1.0);

    /// `#FF8C00 = rgb(255 140 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF8C00;"></div>
    pub const DARK_ORANGE: Self = Self::new(1.0, 0.54902, 0.0, 1.0);

    /// `#9932CC = rgb(153 50 204)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #9932CC;"></div>
    pub const DARK_ORCHID: Self = Self::new(0.6, 0.196078, 0.8, 1.0);

    /// `#8B0000 = rgb(139 0 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #8B0000;"></div>
    pub const DARK_RED: Self = Self::new(0.545098, 0.0, 0.0, 1.0);

    /// `#E9967A = rgb(233 150 122)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #E9967A;"></div>
    pub const DARK_SALMON: Self = Self::new(0.913725, 0.588235, 0.478431, 1.0);

    /// `#8FBC8F = rgb(143 188 143)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #8FBC8F;"></div>
    pub const DARK_SEAGREEN: Self = Self::new(0.560784, 0.737255, 0.560784, 1.0);

    /// `#483D8B = rgb(72 61 139)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #483D8B;"></div>
    pub const DARK_SLATE_BLUE: Self = Self::new(0.282353, 0.239216, 0.545098, 1.0);

    /// `#2F4F4F = rgb(47 79 79)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #2F4F4F;"></div>
    pub const DARK_SLATEGRAY: Self = Self::new(0.184314, 0.309804, 0.309804, 1.0);

    /// `#2F4F4F = rgb(47 79 79)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #2F4F4F;"></div>
    pub const DARK_SLATEGREY: Self = Self::new(0.184314, 0.309804, 0.309804, 1.0);

    /// `#00CED1 = rgb(0 206 209)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00CED1;"></div>
    pub const DARK_TURQUOISE: Self = Self::new(0.0, 0.807843, 0.819608, 1.0);

    /// `#9400D3 = rgb(148 0 211)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #9400D3;"></div>
    pub const DARK_VIOLET: Self = Self::new(0.580392, 0.0, 0.827451, 1.0);

    /// `#FF1493 = rgb(255 20 147)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF1493;"></div>
    pub const DEEP_PINK: Self = Self::new(1.0, 0.0784314, 0.576471, 1.0);

    /// `#00BFFF = rgb(0 191 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00BFFF;"></div>
    pub const DEEP_SKYBLUE: Self = Self::new(0.0, 0.74902, 1.0, 1.0);

    /// `#696969 = rgb(105 105 105)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #696969;"></div>
    pub const DIMGRAY: Self = Self::new(0.411765, 0.411765, 0.411765, 1.0);

    /// `#696969 = rgb(105 105 105)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #696969;"></div>
    pub const DIMGREY: Self = Self::new(0.411765, 0.411765, 0.411765, 1.0);

    /// `#1E90FF = rgb(30 144 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #1E90FF;"></div>
    pub const DODGER_BLUE: Self = Self::new(0.117647, 0.564706, 1.0, 1.0);

    /// `#B22222 = rgb(178 34 34)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #B22222;"></div>
    pub const FIREBRICK: Self = Self::new(0.698039, 0.133333, 0.133333, 1.0);

    /// `#FFFAF0 = rgb(255 250 240)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFAF0;"></div>
    pub const FLORAL_WHITE: Self = Self::new(1.0, 0.980392, 0.941176, 1.0);

    /// `#228B22 = rgb(34 139 34)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #228B22;"></div>
    pub const FORESTGREEN: Self = Self::new(0.133333, 0.545098, 0.133333, 1.0);

    /// `#FF00FF = rgb(255 0 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF00FF;"></div>
    pub const FUCHSIA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    /// `#DCDCDC = rgb(220 220 220)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DCDCDC;"></div>
    pub const GAINSBORO: Self = Self::new(0.862745, 0.862745, 0.862745, 1.0);

    /// `#F8F8FF = rgb(248 248 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F8F8FF;"></div>
    pub const GHOST_WHITE: Self = Self::new(0.972549, 0.972549, 1.0, 1.0);

    /// `#DAA520 = rgb(218 165 32)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DAA520;"></div>
    pub const GOLDEN_ROD: Self = Self::new(0.854902, 0.647059, 0.12549, 1.0);

    /// `#FFD700 = rgb(255 215 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFD700;"></div>
    pub const GOLD: Self = Self::new(1.0, 0.843137, 0.0, 1.0);

    /// `#808080 = rgb(128 128 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #808080;"></div>
    pub const GRAY: Self = Self::new(0.501961, 0.501961, 0.501961, 1.0);

    /// `#008000 = rgb(0 128 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #008000;"></div>
    pub const GREEN: Self = Self::new(0.0, 0.501961, 0.0, 1.0);

    /// `#ADFF2F = rgb(173 255 47)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #ADFF2F;"></div>
    pub const GREEN_YELLOW: Self = Self::new(0.678431, 1.0, 0.184314, 1.0);

    /// `#808080 = rgb(128 128 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #808080;"></div>
    pub const GREY: Self = Self::new(0.501961, 0.501961, 0.501961, 1.0);

    /// `#F0FFF0 = rgb(240 255 240)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F0FFF0;"></div>
    pub const HONEYDEW: Self = Self::new(0.941176, 1.0, 0.941176, 1.0);

    /// `#FF69B4 = rgb(255 105 180)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF69B4;"></div>
    pub const HOTPINK: Self = Self::new(1.0, 0.411765, 0.705882, 1.0);

    /// `#CD5C5C = rgb(205 92 92)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #CD5C5C;"></div>
    pub const INDIANRED: Self = Self::new(0.803922, 0.360784, 0.360784, 1.0);

    /// `#4B0082 = rgb(75 0 130)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #4B0082;"></div>
    pub const INDIGO: Self = Self::new(0.294118, 0.0, 0.509804, 1.0);

    /// `#FFFFF0 = rgb(255 255 240)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFFF0;"></div>
    pub const IVORY: Self = Self::new(1.0, 1.0, 0.941176, 1.0);

    /// `#F0E68C = rgb(240 230 140)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F0E68C;"></div>
    pub const KHAKI: Self = Self::new(0.941176, 0.901961, 0.54902, 1.0);

    /// `#FFF0F5 = rgb(255 240 245)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFF0F5;"></div>
    pub const LAVENDERBLUSH: Self = Self::new(1.0, 0.941176, 0.960784, 1.0);

    /// `#E6E6FA = rgb(230 230 250)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #E6E6FA;"></div>
    pub const LAVENDER: Self = Self::new(0.901961, 0.901961, 0.980392, 1.0);

    /// `#7CFC00 = rgb(124 252 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #7CFC00;"></div>
    pub const LAWN_GREEN: Self = Self::new(0.486275, 0.988235, 0.0, 1.0);

    /// `#FFFACD = rgb(255 250 205)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFACD;"></div>
    pub const LEMON_CHIFFON: Self = Self::new(1.0, 0.980392, 0.803922, 1.0);

    /// `#ADD8E6 = rgb(173 216 230)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #ADD8E6;"></div>
    pub const LIGHT_BLUE: Self = Self::new(0.678431, 0.847059, 0.901961, 1.0);

    /// `#F08080 = rgb(240 128 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F08080;"></div>
    pub const LIGHT_CORAL: Self = Self::new(0.941176, 0.501961, 0.501961, 1.0);

    /// `#E0FFFF = rgb(224 255 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #E0FFFF;"></div>
    pub const LIGHT_CYAN: Self = Self::new(0.878431, 1.0, 1.0, 1.0);

    /// `#FAFAD2 = rgb(250 250 210)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FAFAD2;"></div>
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::new(0.980392, 0.980392, 0.823529, 1.0);

    /// `#D3D3D3 = rgb(211 211 211)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #D3D3D3;"></div>
    pub const LIGHT_GRAY: Self = Self::new(0.827451, 0.827451, 0.827451, 1.0);

    /// `#90EE90 = rgb(144 238 144)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #90EE90;"></div>
    pub const LIGHT_GREEN: Self = Self::new(0.564706, 0.933333, 0.564706, 1.0);

    /// `#D3D3D3 = rgb(211 211 211)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #D3D3D3;"></div>
    pub const LIGHT_GREY: Self = Self::new(0.827451, 0.827451, 0.827451, 1.0);

    /// `#FFB6C1 = rgb(255 182 193)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFB6C1;"></div>
    pub const LIGHT_PINK: Self = Self::new(1.0, 0.713725, 0.756863, 1.0);

    /// `#FFA07A = rgb(255 160 122)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFA07A;"></div>
    pub const LIGHT_SALMON: Self = Self::new(1.0, 0.627451, 0.478431, 1.0);

    /// `#20B2AA = rgb(32 178 170)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #20B2AA;"></div>
    pub const LIGHT_SEA_GREEN: Self = Self::new(0.12549, 0.698039, 0.666667, 1.0);

    /// `#87CEFA = rgb(135 206 250)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #87CEFA;"></div>
    pub const LIGHT_SKY_BLUE: Self = Self::new(0.529412, 0.807843, 0.980392, 1.0);

    /// `#778899 = rgb(119 136 153)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #778899;"></div>
    pub const LIGHT_SLATE_GRAY: Self = Self::new(0.466667, 0.533333, 0.6, 1.0);

    /// `#778899 = rgb(119 136 153)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #778899;"></div>
    pub const LIGHT_SLATE_GREY: Self = Self::new(0.466667, 0.533333, 0.6, 1.0);

    /// `#B0C4DE = rgb(176 196 222)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #B0C4DE;"></div>
    pub const LIGHT_STEEL_BLUE: Self = Self::new(0.690196, 0.768627, 0.870588, 1.0);

    /// `#FFFFE0 = rgb(255 255 224)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFFE0;"></div>
    pub const LIGHT_YELLOW: Self = Self::new(1.0, 1.0, 0.878431, 1.0);

    /// `#00FF00 = rgb(0 255 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00FF00;"></div>
    pub const LIME: Self = Self::new(0.0, 1.0, 0.0, 1.0);

    /// `#32CD32 = rgb(50 205 50)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #32CD32;"></div>
    pub const LIME_GREEN: Self = Self::new(0.196078, 0.803922, 0.196078, 1.0);

    /// `#FAF0E6 = rgb(250 240 230)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FAF0E6;"></div>
    pub const LINEN: Self = Self::new(0.980392, 0.941176, 0.901961, 1.0);

    /// `#FF00FF = rgb(255 0 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF00FF;"></div>
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    /// `#800000 = rgb(128 0 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #800000;"></div>
    pub const MAROON: Self = Self::new(0.501961, 0.0, 0.0, 1.0);

    /// `#66CDAA = rgb(102 205 170)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #66CDAA;"></div>
    pub const MEDIUM_AQUA_MARINE: Self = Self::new(0.4, 0.803922, 0.666667, 1.0);

    /// `#0000CD = rgb(0 0 205)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #0000CD;"></div>
    pub const MEDIUM_BLUE: Self = Self::new(0.0, 0.0, 0.803922, 1.0);

    /// `#BA55D3 = rgb(186 85 211)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #BA55D3;"></div>
    pub const MEDIUM_ORCHID: Self = Self::new(0.729412, 0.333333, 0.827451, 1.0);

    /// `#9370DB = rgb(147 112 219)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #9370DB;"></div>
    pub const MEDIUM_PURPLE: Self = Self::new(0.576471, 0.439216, 0.858824, 1.0);

    /// `#3CB371 = rgb(60 179 113)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #3CB371;"></div>
    pub const MEDIUM_SEA_GREEN: Self = Self::new(0.235294, 0.701961, 0.443137, 1.0);

    /// `#7B68EE = rgb(123 104 238)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #7B68EE;"></div>
    pub const MEDIUM_SLATE_BLUE: Self = Self::new(0.482353, 0.407843, 0.933333, 1.0);

    /// `#00FA9A = rgb(0 250 154)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00FA9A;"></div>
    pub const MEDIUM_SPRING_GREEN: Self = Self::new(0.0, 0.980392, 0.603922, 1.0);

    /// `#48D1CC = rgb(72 209 204)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #48D1CC;"></div>
    pub const MEDIUM_TURQUOISE: Self = Self::new(0.282353, 0.819608, 0.8, 1.0);

    /// `#C71585 = rgb(199 21 133)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #C71585;"></div>
    pub const MEDIUM_VIOLET_RED: Self = Self::new(0.780392, 0.0823529, 0.521569, 1.0);

    /// `#191970 = rgb(25 25 112)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #191970;"></div>
    pub const MIDNIGHT_BLUE: Self = Self::new(0.0980392, 0.0980392, 0.439216, 1.0);

    /// `#F5FFFA = rgb(245 255 250)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F5FFFA;"></div>
    pub const MINTCREAM: Self = Self::new(0.960784, 1.0, 0.980392, 1.0);

    /// `#FFE4E1 = rgb(255 228 225)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFE4E1;"></div>
    pub const MISTYROSE: Self = Self::new(1.0, 0.894118, 0.882353, 1.0);

    /// `#FFE4B5 = rgb(255 228 181)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFE4B5;"></div>
    pub const MOCCASIN: Self = Self::new(1.0, 0.894118, 0.709804, 1.0);

    /// `#FFDEAD = rgb(255 222 173)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFDEAD;"></div>
    pub const NAVAJO_WHITE: Self = Self::new(1.0, 0.870588, 0.678431, 1.0);

    /// `#000080 = rgb(0 0 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #000080;"></div>
    pub const NAVY: Self = Self::new(0.0, 0.0, 0.501961, 1.0);

    /// `#FDF5E6 = rgb(253 245 230)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FDF5E6;"></div>
    pub const OLDLACE: Self = Self::new(0.992157, 0.960784, 0.901961, 1.0);

    /// `#808000 = rgb(128 128 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #808000;"></div>
    pub const OLIVE: Self = Self::new(0.501961, 0.501961, 0.0, 1.0);

    /// `#6B8E23 = rgb(107 142 35)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #6B8E23;"></div>
    pub const OLIVEDRAB: Self = Self::new(0.419608, 0.556863, 0.137255, 1.0);

    /// `#FFA500 = rgb(255 165 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFA500;"></div>
    pub const ORANGE: Self = Self::new(1.0, 0.647059, 0.0, 1.0);

    /// `#FF4500 = rgb(255 69 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF4500;"></div>
    pub const ORANGERED: Self = Self::new(1.0, 0.270588, 0.0, 1.0);

    /// `#DA70D6 = rgb(218 112 214)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DA70D6;"></div>
    pub const ORCHID: Self = Self::new(0.854902, 0.439216, 0.839216, 1.0);

    /// `#EEE8AA = rgb(238 232 170)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #EEE8AA;"></div>
    pub const PALE_GOLDEN_ROD: Self = Self::new(0.933333, 0.909804, 0.666667, 1.0);

    /// `#98FB98 = rgb(152 251 152)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #98FB98;"></div>
    pub const PALE_GREEN: Self = Self::new(0.596078, 0.984314, 0.596078, 1.0);

    /// `#AFEEEE = rgb(175 238 238)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #AFEEEE;"></div>
    pub const PALE_TURQUOISE: Self = Self::new(0.686275, 0.933333, 0.933333, 1.0);

    /// `#DB7093 = rgb(219 112 147)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DB7093;"></div>
    pub const PALE_VIOLET_RED: Self = Self::new(0.858824, 0.439216, 0.576471, 1.0);

    /// `#FFEFD5 = rgb(255 239 213)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFEFD5;"></div>
    pub const PAPAYAWHIP: Self = Self::new(1.0, 0.937255, 0.835294, 1.0);

    /// `#FFDAB9 = rgb(255 218 185)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFDAB9;"></div>
    pub const PEACHPUFF: Self = Self::new(1.0, 0.854902, 0.72549, 1.0);

    /// `#CD853F = rgb(205 133 63)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #CD853F;"></div>
    pub const PERU: Self = Self::new(0.803922, 0.521569, 0.247059, 1.0);

    /// `#FFC0CB = rgb(255 192 203)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFC0CB;"></div>
    pub const PINK: Self = Self::new(1.0, 0.752941, 0.796078, 1.0);

    /// `#DDA0DD = rgb(221 160 221)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #DDA0DD;"></div>
    pub const PLUM: Self = Self::new(0.866667, 0.627451, 0.866667, 1.0);

    /// `#B0E0E6 = rgb(176 224 230)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #B0E0E6;"></div>
    pub const POWDER_BLUE: Self = Self::new(0.690196, 0.878431, 0.901961, 1.0);

    /// `#800080 = rgb(128 0 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #800080;"></div>
    pub const PURPLE: Self = Self::new(0.501961, 0.0, 0.501961, 1.0);

    /// `#663399 = rgb(102 51 153)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #663399;"></div>
    pub const REBECCA_PURPLE: Self = Self::new(0.4, 0.2, 0.6, 1.0);

    /// `#FF0000 = rgb(255 0 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF0000;"></div>
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);

    /// `#BC8F8F = rgb(188 143 143)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #BC8F8F;"></div>
    pub const ROSY_BROWN: Self = Self::new(0.737255, 0.560784, 0.560784, 1.0);

    /// `#4169E1 = rgb(65 105 225)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #4169E1;"></div>
    pub const ROYAL_BLUE: Self = Self::new(0.254902, 0.411765, 0.882353, 1.0);

    /// `#8B4513 = rgb(139 69 19)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #8B4513;"></div>
    pub const SADDLE_BROWN: Self = Self::new(0.545098, 0.270588, 0.0745098, 1.0);

    /// `#FA8072 = rgb(250 128 114)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FA8072;"></div>
    pub const SALMON: Self = Self::new(0.980392, 0.501961, 0.447059, 1.0);

    /// `#F4A460 = rgb(244 164 96)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F4A460;"></div>
    pub const SANDY_BROWN: Self = Self::new(0.956863, 0.643137, 0.376471, 1.0);

    /// `#2E8B57 = rgb(46 139 87)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #2E8B57;"></div>
    pub const SEA_GREEN: Self = Self::new(0.180392, 0.545098, 0.341176, 1.0);

    /// `#FFF5EE = rgb(255 245 238)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFF5EE;"></div>
    pub const SEA_SHELL: Self = Self::new(1.0, 0.960784, 0.933333, 1.0);

    /// `#A0522D = rgb(160 82 45)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #A0522D;"></div>
    pub const SIENNA: Self = Self::new(0.627451, 0.321569, 0.176471, 1.0);

    /// `#C0C0C0 = rgb(192 192 192)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #C0C0C0;"></div>
    pub const SILVER: Self = Self::new(0.752941, 0.752941, 0.752941, 1.0);

    /// `#87CEEB = rgb(135 206 235)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #87CEEB;"></div>
    pub const SKY_BLUE: Self = Self::new(0.529412, 0.807843, 0.921569, 1.0);

    /// `#6A5ACD = rgb(106 90 205)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #6A5ACD;"></div>
    pub const SLATE_BLUE: Self = Self::new(0.415686, 0.352941, 0.803922, 1.0);

    /// `#708090 = rgb(112 128 144)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #708090;"></div>
    pub const SLATE_GRAY: Self = Self::new(0.439216, 0.501961, 0.564706, 1.0);

    /// `#708090 = rgb(112 128 144)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #708090;"></div>
    pub const SLATE_GREY: Self = Self::new(0.439216, 0.501961, 0.564706, 1.0);

    /// `#FFFAFA = rgb(255 250 250)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFAFA;"></div>
    pub const SNOW: Self = Self::new(1.0, 0.980392, 0.980392, 1.0);

    /// `#00FF7F = rgb(0 255 127)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #00FF7F;"></div>
    pub const SPRING_GREEN: Self = Self::new(0.0, 1.0, 0.498039, 1.0);

    /// `#4682B4 = rgb(70 130 180)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #4682B4;"></div>
    pub const STEEL_BLUE: Self = Self::new(0.27451, 0.509804, 0.705882, 1.0);

    /// `#D2B48C = rgb(210 180 140)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #D2B48C;"></div>
    pub const TAN: Self = Self::new(0.823529, 0.705882, 0.54902, 1.0);

    /// `#008080 = rgb(0 128 128)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #008080;"></div>
    pub const TEAL: Self = Self::new(0.0, 0.501961, 0.501961, 1.0);

    /// `#D8BFD8 = rgb(216 191 216)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #D8BFD8;"></div>
    pub const THISTLE: Self = Self::new(0.847059, 0.74902, 0.847059, 1.0);

    /// `#FF6347 = rgb(255 99 71)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FF6347;"></div>
    pub const TOMATO: Self = Self::new(1.0, 0.388235, 0.278431, 1.0);

    /// `#40E0D0 = rgb(64 224 208)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #40E0D0;"></div>
    pub const TURQUOISE: Self = Self::new(0.25098, 0.878431, 0.815686, 1.0);

    /// `#EE82EE = rgb(238 130 238)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #EE82EE;"></div>
    pub const VIOLET: Self = Self::new(0.933333, 0.509804, 0.933333, 1.0);

    /// `#F5DEB3 = rgb(245 222 179)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F5DEB3;"></div>
    pub const WHEAT: Self = Self::new(0.960784, 0.870588, 0.701961, 1.0);

    /// `#FFFFFF = rgb(255 255 255)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFFFF;"></div>
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// `#F5F5F5 = rgb(245 245 245)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #F5F5F5;"></div>
    pub const WHITE_SMOKE: Self = Self::new(0.960784, 0.960784, 0.960784, 1.0);

    /// `#FFFF00 = rgb(255 255 0)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #FFFF00;"></div>
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);

    /// `#9ACD32 = rgb(154 205 50)`
    ///
    /// <div style="display: inline-block;width: 1rem;height: 1rem;border: 1px solid black;background: #9ACD32;"></div>
    pub const YELLOW_GREEN: Self = Self::new(0.603922, 0.803922, 0.196078, 1.0);
}
