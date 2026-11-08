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
        };
        match name {
            "gruvbox" => {}
            "rose-pine" => {
                t.bg = rgb(0x191724);
                t.hard = rgb(0x191724);
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
            _ => return None,
        }
        Some(t)
    }
}
const fn rgb(n: u32) -> Color {
    Color::Rgb((n >> 16) as u8, (n >> 8) as u8, n as u8)
}
