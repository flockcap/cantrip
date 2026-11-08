use crate::{
    suggestion::{visible, Danger, Suggestion},
    theme::Theme,
};
use anyhow::{bail, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Terminal, TerminalOptions, Viewport,
};
use std::{
    io::{self, IsTerminal, Write},
    sync::LazyLock,
    time::Duration,
};
use unicode_width::UnicodeWidthStr;

pub enum Action {
    Run,
    Edit,
    Alternative,
    Copy,
    Regenerate,
    Quit,
}
struct Screen {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}
impl Screen {
    fn new(height: u16) -> Result<Self> {
        enable_raw_mode()?;
        let terminal = Terminal::with_options(
            CrosstermBackend::new(io::stdout()),
            TerminalOptions {
                viewport: Viewport::Inline(height),
            },
        );
        match terminal {
            Ok(terminal) => Ok(Self { terminal }),
            Err(error) => {
                let _ = disable_raw_mode();
                Err(error.into())
            }
        }
    }
}
impl Drop for Screen {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = self.terminal.show_cursor();
    }
}

pub struct Card<'a> {
    pub suggestion: &'a Suggestion,
    pub intent: &'a str,
    pub agent: &'a str,
    pub theme: Theme,
    pub dry_run: bool,
    pub status: &'a str,
}

pub fn card(card: &Card<'_>) -> Result<Action> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        print_plain(card);
        if card.dry_run {
            return Ok(Action::Quit);
        }
        bail!("confirmation needs a terminal; use --json, --dry-run, or --yolo");
    }
    let (columns, height) = crossterm::terminal::size()?;
    let rows = command_rows(
        card.suggestion
            .raw
            .as_deref()
            .unwrap_or(&card.suggestion.command),
        columns.min(100).saturating_sub(10) as usize,
    )
    .min(8);
    let desired = 14 + rows as u16;
    let mut screen = Screen::new(height.saturating_sub(2).clamp(8, desired))?;
    let mut scroll = 0u16;
    loop {
        screen.terminal.draw(|frame| draw(frame, card, scroll))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return Ok(Action::Quit);
                }
                match key.code {
                    KeyCode::Enter if card.suggestion.executable() && !card.dry_run => {
                        return Ok(Action::Run)
                    }
                    KeyCode::Char('e') => return Ok(Action::Edit),
                    KeyCode::Char('a') => return Ok(Action::Alternative),
                    KeyCode::Char('c') => return Ok(Action::Copy),
                    KeyCode::Char('r') => return Ok(Action::Regenerate),
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter if card.dry_run => {
                        return Ok(Action::Quit)
                    }
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Quit),
                    KeyCode::Down | KeyCode::Char('j') => {
                        scroll = scroll.saturating_add(1).min(5000)
                    }
                    KeyCode::Up | KeyCode::Char('k') => scroll = scroll.saturating_sub(1),
                    KeyCode::Home => scroll = 0,
                    _ => {}
                }
            }
        }
    }
}

pub fn draw(frame: &mut ratatui::Frame<'_>, card: &Card<'_>, scroll: u16) {
    let t = card.theme;
    let whole = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(t.hard)), whole);
    let width = whole.width.min(100);
    let area = Rect::new(
        whole.x + whole.width.saturating_sub(width) / 2,
        whole.y,
        width,
        whole.height,
    );
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(t.border))
        .style(Style::default().bg(t.bg).fg(t.fg))
        .padding(Padding::new(3, 3, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.height < 5 || inner.width < 15 {
        return;
    }
    let parts = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Min(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .split(inner);
    let title = Layout::horizontal([Constraint::Min(10), Constraint::Length(17)]).split(parts[0]);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("✦ ", Style::default().fg(t.orange)),
            Span::styled(
                "cantrip",
                Style::default().fg(t.aqua).add_modifier(Modifier::BOLD),
            ),
        ])),
        title[0],
    );
    frame.render_widget(
        Paragraph::new("◌  by flock")
            .right_aligned()
            .style(Style::default().fg(t.gray)),
        title[1],
    );
    let wish = visible(card.intent).replace('\n', " ");
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("wish  ", Style::default().fg(t.gray)),
            Span::styled(wish, Style::default().fg(t.muted)),
        ])),
        parts[1],
    );
    let content = Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).split(parts[2]);
    let command = if let Some(raw) = &card.suggestion.raw {
        Text::raw(visible(raw))
    } else {
        highlight(&card.suggestion.command, t)
    };
    frame.render_widget(
        Paragraph::new(command)
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0)),
        content[0],
    );
    frame.render_widget(
        Paragraph::new(visible(&card.suggestion.explanation))
            .style(Style::default().fg(t.muted))
            .wrap(Wrap { trim: false }),
        content[1],
    );
    let color = match card.suggestion.danger {
        Danger::Low => t.green,
        Danger::Medium => t.yellow,
        Danger::High => t.red,
    };
    let badge = format!(" {} ", card.suggestion.danger.label().to_uppercase());
    let warning = if card.dry_run {
        "dry run · nothing will execute".to_owned()
    } else if !card.status.is_empty() {
        card.status.to_owned()
    } else {
        card.suggestion.warnings.join(" · ")
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                badge,
                Style::default()
                    .fg(t.hard)
                    .bg(color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}  ·  {}", visible(card.agent), visible(&warning)),
                Style::default().fg(t.gray),
            ),
        ]))
        .wrap(Wrap { trim: false }),
        parts[3],
    );
    let enter = if card.dry_run {
        "enter close"
    } else {
        "enter run"
    };
    frame.render_widget(Paragraph::new(format!("{enter} · e edit · a alternatives · c copy\nr regenerate · q quit                              ↑↓ scroll")).style(Style::default().fg(t.gray)).wrap(Wrap { trim: false }), parts[4]);
}

pub fn highlight(command: &str, t: Theme) -> Text<'static> {
    static TOKENS: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r#"'[^']*'|"(?:\\.|[^"\\])*"|[|&;><()]+|\s+|[^\s|&;><()]+"#).unwrap()
    });
    let mut lines = Vec::new();
    for line in visible(command).lines() {
        let mut first = true;
        let mut spans = vec![Span::styled("› ", Style::default().fg(t.orange))];
        for m in TOKENS.find_iter(line) {
            let word = m.as_str();
            let color = if word.trim().is_empty() {
                t.fg
            } else if word.starts_with(['\'', '"', '$']) {
                t.aqua
            } else if word.starts_with('-') {
                t.yellow
            } else if word.starts_with(['|', '&', ';', '>', '<', '(', ')']) {
                first = true;
                t.orange
            } else if first {
                first = false;
                t.green
            } else {
                t.fg
            };
            spans.push(Span::styled(
                word.to_owned(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}

pub fn confirm_high(suggestion: &Suggestion, theme: Theme) -> Result<bool> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        bail!("high danger needs an interactive confirmation, or explicit --yolo --force");
    }
    let mut screen = Screen::new(14)?;
    let mut typed = String::new();
    loop {
        screen.terminal.draw(|frame| {
            let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.red)).style(Style::default().bg(theme.hard).fg(theme.red)).padding(Padding::uniform(2));
            let area = frame.area(); let inner = block.inner(area);
            frame.render_widget(block, area);
            let text = format!("HIGH DANGER\n\n{}\n\n{}\n\nDefault: No. Type run + enter to execute; esc cancels.\n> {}", visible(&suggestion.command), suggestion.warnings.join("\n"), typed);
            frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Enter => return Ok(typed == "run"),
                KeyCode::Esc => return Ok(false),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(false)
                }
                KeyCode::Char(c) if typed.len() < 10 => typed.push(c),
                KeyCode::Backspace => {
                    typed.pop();
                }
                _ => {}
            }
        }
    }
}

pub fn print_plain(card: &Card<'_>) {
    println!(
        "\n✦ cantrip · {} · {}\n",
        card.agent,
        card.suggestion.danger.label()
    );
    println!(
        "{}\n\n{}",
        visible(
            card.suggestion
                .raw
                .as_deref()
                .unwrap_or(&card.suggestion.command)
        ),
        visible(&card.suggestion.explanation)
    );
    for warning in &card.suggestion.warnings {
        println!("! {}", visible(warning));
    }
}

pub fn wish() -> Result<String> {
    if io::stdout().is_terminal() {
        print!("\x1b[38;2;254;128;25m✦\x1b[0m \x1b[38;2;142;192;124mcantrip\x1b[0m  speak plainly.\n\n  wish › ");
        io::stdout().flush()?;
    }
    let mut text = String::new();
    io::stdin().read_line(&mut text)?;
    if text.trim().is_empty() {
        bail!("a wish is needed; try: cantrip list the largest files here");
    }
    Ok(text.trim().into())
}

pub fn command_rows(command: &str, width: usize) -> usize {
    command
        .lines()
        .map(|s| (s.width() + 2).div_ceil(width.max(1)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    #[test]
    fn card_renders_command_brand_and_keys_at_multiple_sizes() {
        for (width, height) in [(100, 22), (60, 18), (30, 14)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            let suggestion = Suggestion {
                command: "find . -name '*.png'".into(),
                explanation: "Find the images.".into(),
                danger: Danger::Low,
                alternatives: vec![],
                warnings: vec![],
                raw: None,
            };
            terminal
                .draw(|f| {
                    draw(
                        f,
                        &Card {
                            suggestion: &suggestion,
                            intent: "find pngs",
                            agent: "claude",
                            theme: Theme::named("gruvbox").unwrap(),
                            dry_run: false,
                            status: "",
                        },
                        0,
                    )
                })
                .unwrap();
            let buffer = terminal.backend().buffer();
            let text: String = buffer.content.iter().map(|c| c.symbol()).collect();
            assert!(text.contains("cantrip"));
            if width > 30 {
                assert!(text.contains("find .") && text.contains("enter run"));
            }
        }
    }
}
