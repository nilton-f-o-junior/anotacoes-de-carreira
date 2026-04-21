use std::io;

use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

// ─── ASCII Digits ─────────────────────────────────────────────────────────────

fn digit_rows(ch: char) -> [&'static str; 5] {
    match ch {
        '0' => ["  ###  ", " #   # ", " #   # ", " #   # ", "  ###  "],
        '1' => ["   #   ", "  ##   ", "   #   ", "   #   ", " ##### "],
        '2' => [" ##### ", "     # ", " ##### ", " #     ", " ##### "],
        '3' => [" ##### ", "     # ", " ##### ", "     # ", " ##### "],
        '4' => [" #   # ", " #   # ", " ##### ", "     # ", "     # "],
        '5' => [" ##### ", " #     ", " ##### ", "     # ", " ##### "],
        '6' => [" ##### ", " #     ", " ##### ", " #   # ", " ##### "],
        '7' => [" ##### ", "     # ", "    #  ", "   #   ", "   #   "],
        '8' => [" ##### ", " #   # ", " ##### ", " #   # ", " ##### "],
        '9' => [" ##### ", " #   # ", " ##### ", "     # ", " ##### "],
        ':' => ["       ", "   #   ", "       ", "   #   ", "       "],
        _ => ["     ", "     ", "     ", "     ", "     "],
    }
}

fn ascii_time_lines(mins: u32, secs: u32) -> Vec<String> {
    let s = format!("{:02}:{:02}", mins, secs);
    let mut rows = vec![String::new(); 5];
    for ch in s.chars() {
        for (i, line) in digit_rows(ch).iter().enumerate() {
            rows[i].push_str(line);
            rows[i].push(' ');
        }
    }
    rows
}

// ─── App State ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Work,
    Break,
    WaitBreak,
    WaitWork,
}

struct App {
    phase: Phase,
    cycle: u32,
    work_secs: u32,
    break_secs: u32,
    remaining: u32,
    paused: bool,
    last_tick: Instant,
}

impl App {
    fn new(work_mins: u32, break_mins: u32) -> Self {
        Self {
            phase: Phase::Work,
            cycle: 1,
            work_secs: work_mins * 60,
            break_secs: break_mins * 60,
            remaining: work_mins * 60,
            paused: false,
            last_tick: Instant::now(),
        }
    }

    fn total_secs(&self) -> u32 {
        match self.phase {
            Phase::Work => self.work_secs,
            Phase::Break => self.break_secs,
            Phase::WaitBreak | Phase::WaitWork => 1,
        }
    }

    fn elapsed(&self) -> u32 {
        self.total_secs().saturating_sub(self.remaining)
    }

    fn tick(&mut self) {
        if self.paused {
            return;
        }
        if self.last_tick.elapsed() < Duration::from_secs(1) {
            return;
        }
        self.last_tick = Instant::now();

        match self.phase {
            Phase::Work | Phase::Break => {
                if self.remaining > 0 {
                    self.remaining -= 1;
                } else {
                    beep();
                    match self.phase {
                        Phase::Work => {
                            self.phase = Phase::WaitBreak;
                            self.remaining = 0;
                        }
                        Phase::Break => {
                            self.phase = Phase::WaitWork;
                            self.remaining = 0;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::WaitBreak => {
                self.phase = Phase::Break;
                self.remaining = self.break_secs;
                self.last_tick = Instant::now();
            }
            Phase::WaitWork => {
                self.cycle += 1;
                self.phase = Phase::Work;
                self.remaining = self.work_secs;
                self.last_tick = Instant::now();
            }
            _ => {}
        }
    }

    fn toggle_pause(&mut self) {
        if matches!(self.phase, Phase::Work | Phase::Break) {
            self.paused = !self.paused;
            if !self.paused {
                self.last_tick = Instant::now();
            }
        }
    }
}

fn beep() {
    print!("\x07");
}

// ─── UI Rendering ─────────────────────────────────────────────────────────────

fn color_for(app: &App) -> Color {
    match app.phase {
        Phase::Work | Phase::WaitBreak => {
            if app.remaining <= 60 {
                Color::Red
            } else {
                Color::Green
            }
        }
        Phase::Break | Phase::WaitWork => Color::Cyan,
    }
}

fn make_block_bar(ratio: f64, width: usize, color: Color) -> Line<'static> {
    let filled = (ratio * width as f64).round() as usize;
    let filled = filled.min(width);
    let bar: String = "▓".repeat(filled) + &"░".repeat(width - filled);
    let pct = (ratio * 100.0) as u32;
    let colored = Span::styled(bar, Style::default().fg(color).add_modifier(Modifier::BOLD));
    let label = Span::raw(format!("  {:3}%", pct));
    Line::from(vec![colored, label])
}

fn render(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &App) -> io::Result<()> {
    terminal.draw(|f| {
        let size = f.size();

        // Altura fixa: nunca muda independente do estado (evita pulo ao pausar)
        // 1 header + 1 spacer + 5 clock + 1 spacer + 1 bar + 1 spacer + 1 hint = 11
        let content_height = 11u16;

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(content_height),
                Constraint::Fill(1),
            ])
            .split(size);

        let color = color_for(app);
        let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
        let dim = Style::default().fg(Color::DarkGray);

        // ── Header ──
        let (label, icon) = match app.phase {
            Phase::Work => ("FOCO", "▶"),
            Phase::Break => ("PAUSA", "▶"),
            Phase::WaitBreak => ("FOCO", "✔"),
            Phase::WaitWork => ("PAUSA", "✔"),
        };
        let (h_icon, h_label, h_style) = if app.paused {
            (
                "⏸",
                "PAUSADO",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (icon, label, style)
        };
        let header = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {}  [ {} ]  ", h_icon, h_label), h_style),
            Span::styled("│  ", dim),
            Span::styled(format!("Ciclo #{}", app.cycle), dim),
        ]))
        .alignment(Alignment::Center);

        let content_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // header
                Constraint::Length(1), // spacer
                Constraint::Length(5), // clock
                Constraint::Length(1), // spacer
                Constraint::Length(1), // bar
                Constraint::Length(1), // spacer
                Constraint::Length(1), // hint/status
            ])
            .split(outer[1]);

        f.render_widget(header, content_area[0]);

        // ── Coluna central compartilhada por clock + barra ──
        // Cada dígito ASCII tem 7 chars + 1 espaço = 8 cols; MM:SS = 5 chars → 5×8 = 40
        // mais o padding "  " do início = 42 cols visuais para o clock
        // A barra usa a mesma largura (42) + "  xxx%" (6) = 48 cols totais
        // Centralizamos esse bloco de 48 cols na tela
        let block_width: u16 = 48;
        let center_constraints = [
            Constraint::Fill(1),
            Constraint::Length(block_width),
            Constraint::Fill(1),
        ];

        // ── ASCII Clock ──
        let (mins, secs) = (app.remaining / 60, app.remaining % 60);
        let clock_color = if app.paused { Color::Yellow } else { color };
        let clock_style = Style::default()
            .fg(clock_color)
            .add_modifier(Modifier::BOLD);
        let clock_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(center_constraints)
            .split(content_area[2]);
        let clock_lines: Vec<Line> = ascii_time_lines(mins, secs)
            .into_iter()
            .map(|l| Line::from(Span::styled(l, clock_style)))
            .collect();
        let clock = Paragraph::new(clock_lines);
        f.render_widget(clock, clock_row[1]);

        // ── Barra: mesmo container central que o clock ──
        let total = app.total_secs().max(1);
        let ratio = (app.elapsed() as f64 / total as f64).clamp(0.0, 1.0);
        let bar_width = 42usize;
        let bar_color = if app.paused { Color::Yellow } else { color };
        let bar_line = make_block_bar(ratio, bar_width, bar_color);
        let bar_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(center_constraints)
            .split(content_area[4]);
        let bar_para = Paragraph::new(bar_line);
        f.render_widget(bar_para, bar_row[1]);

        // ── Hint / status (altura fixa — sempre ocupa 1 linha) ──
        let hint_line = match app.phase {
            Phase::WaitBreak => Line::from(Span::styled(
                "✔  Foco concluído! Pressione ESPAÇO para iniciar a pausa...",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Phase::WaitWork => Line::from(Span::styled(
                "✔  Pausa concluída! Pressione ESPAÇO para voltar ao trabalho...",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            _ if app.paused => Line::from(Span::styled(
                "[P] Retomar   |   [Ctrl+C] Sair",
                Style::default().fg(Color::Yellow),
            )),
            _ => Line::from(Span::styled(
                "[P] Pausar   |   [Ctrl+C] Sair",
                Style::default().fg(Color::DarkGray),
            )),
        };
        let hint_widget = Paragraph::new(hint_line).alignment(Alignment::Center);
        f.render_widget(hint_widget, content_area[6]);
    })?;
    Ok(())
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let work = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(25u32);
    let break_ = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5u32);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(work, break_);
    let tick_rate = Duration::from_millis(100);

    loop {
        render(&mut terminal, &app)?;
        app.tick();

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('c') => break,
                    KeyCode::Char('p') | KeyCode::Char('P') => app.toggle_pause(),
                    KeyCode::Char(' ') | KeyCode::Enter => app.advance(),
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("\n  \x1b[1;31mPomodoro encerrado. Até a próxima! 🍅\x1b[0m\n");
    Ok(())
}
