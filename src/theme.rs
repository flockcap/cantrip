use ratatui::style::Color;
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub bg: Color,
    pub hard: Color,
    pub surface: Color,
    pub border: Color,
    pub fg: Color,
    pub muted: Color,
    pub gray: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub purple: Color,
    pub aqua: Color,
    pub orange: Color,
    /// text drawn on top of colored badges
    pub ink: Color,
    /// draw the rounded box around the card
    pub framed: bool,
}
impl Theme {
    pub fn named(name: &str) -> Option<Self> {
        let mut t = Self {
            bg: rgb(0x282828),
            hard: rgb(0x1d2021),
            surface: rgb(0x3c3836),
            border: rgb(0x504945),
            fg: rgb(0xebdbb2),
            muted: rgb(0xa89984),
            gray: rgb(0x928374),
            red: rgb(0xfb4934),
            green: rgb(0xb8bb26),
            yellow: rgb(0xfabd2f),
            blue: rgb(0x83a598),
            purple: rgb(0xd3869b),
            aqua: rgb(0x8ec07c),
            orange: rgb(0xfe8019),
            ink: rgb(0x1d2021),
            framed: true,
        };
        match name {
            "gruvbox" => {}
            "rose-pine" => {
                t.bg = rgb(0x191724);
                t.hard = rgb(0x191724);
                t.ink = rgb(0x191724);
                t.surface = rgb(0x26233a);
                t.border = rgb(0x524f67);
                t.fg = rgb(0xe0def4);
                t.muted = rgb(0x908caa);
                t.gray = rgb(0x6e6a86);
                t.aqua = rgb(0x9ccfd8);
                t.orange = rgb(0xebbcba);
                t.green = rgb(0x9ccfd8);
                t.yellow = rgb(0xf6c177);
                t.red = rgb(0xeb6f92);
                t.blue = rgb(0x31748f);
                t.purple = rgb(0xc4a7e7);
            }
            "flock" => {
                t.bg = t.hard;
                t.aqua = t.fg;
                t.green = t.fg;
                t.blue = t.muted;
                t.purple = t.muted;
            }
            "glass" => {
                // transparent: uses the terminal's own (translucent) background
                t.bg = Color::Reset;
                t.hard = Color::Reset;
                t.surface = rgb(0x232326);
                t.border = rgb(0x3a3a3e);
                t.fg = rgb(0xe4e4e6);
                t.muted = rgb(0xbdbdc2);
                t.gray = rgb(0x76767c);
                t.red = rgb(0xe07a7a);
                t.green = rgb(0x8cc896);
                t.yellow = rgb(0xe1c88c);
                t.blue = rgb(0x8fa8c8);
                t.purple = rgb(0xc09ad0);
                t.aqua = rgb(0x88c4c0);
                t.orange = rgb(0xe0a87a);
                t.ink = rgb(0x101011);
            }
            _ => return None,
        }
        Some(t)
    }
}
const fn rgb(n: u32) -> Color {
    Color::Rgb((n >> 16) as u8, (n >> 8) as u8, n as u8)
}
