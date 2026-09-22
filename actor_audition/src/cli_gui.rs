use crate::queue_manager::Queues;
use crate::settings::Settings;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct Stats {
    finished: Arc<Mutex<VecDeque<FinishedTask>>>,
    total_processed: Arc<Mutex<u64>>,
}

#[derive(Clone)]
pub struct FinishedTask {
    name: String,
    duration: Duration,
    since_request: Duration,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            finished: Arc::new(Mutex::new(VecDeque::new())),
            total_processed: Arc::new(Mutex::new(0)),
        }
    }

    pub fn record(&self, name: String, since_request: Duration, duration: Duration) {
        let mut f = self.finished.lock().unwrap();
        if f.len() >= 15 {
            f.pop_front();
        }
        f.push_back(FinishedTask {
            name,
            duration,
            since_request,
        });
        *self.total_processed.lock().unwrap() += 1;
    }
}

pub async fn run_gui(queues: Queues, stats: Stats, settings: Settings) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(6),
                    Constraint::Min(8),
                    Constraint::Length(1),
                ])
                .split(area);

            let total = *stats.total_processed.lock().unwrap();
            let delay_ms = settings.delay().as_millis();
            let header = Paragraph::new(format!(
                "total processed: {}   |   delay: {}ms   (↑ slower / ↓ faster)",
                total, delay_ms
            ))
            .block(Block::default().borders(Borders::ALL).title("Status"));
            frame.render_widget(header, chunks[0]);

            let p = queues.priority_queue.lock().unwrap().len();
            let n = queues.normal_queue.lock().unwrap().len();
            let queue_lines = vec![
                Line::from(vec![
                    Span::styled(
                        "PRIORITY  ",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!("{} waiting", p)),
                ]),
                Line::from(vec![
                    Span::styled(
                        "NORMAL    ",
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!("{} waiting", n)),
                ]),
            ];
            let queue_block = Paragraph::new(queue_lines)
                .block(Block::default().borders(Borders::ALL).title("Queues"));
            frame.render_widget(queue_block, chunks[1]);

            let history = stats.finished.lock().unwrap();
            let items: Vec<ListItem> = history
                .iter()
                .rev()
                .enumerate()
                .map(|(i, t)| {
                    let style = if i < 3 {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    ListItem::new(format!(
                        "{:<8}  ran {:>4}ms   from request to finish {:>4}ms",
                        t.name,
                        t.duration.as_millis(),
                        t.since_request.as_millis()
                    ))
                    .style(style)
                })
                .collect();
            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Last 15 finished (newest first)"),
            );
            frame.render_widget(list, chunks[2]);

            let hint =
                Paragraph::new("press q to quit").style(Style::default().fg(Color::DarkGray));
            frame.render_widget(hint, chunks[3]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => settings.slower(),
                    KeyCode::Down => settings.faster(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
