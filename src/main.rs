//! agentMarket — a terminal marketing dashboard.
//!
//! Surfaces the capability areas of the knowledge-work `marketing` plugin
//! (campaigns, content, SEO, performance, brand, competitors, email) as a
//! navigable TUI. Data shown here is illustrative; each panel names the
//! plugin skill that powers the real workflow.

use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Padding, Paragraph, Sparkline, Wrap,
    },
    Frame,
};

const PLUGIN: &str = "marketing@knowledge-work-plugins v1.2.0";
const ACCENT: Color = Color::Rgb(255, 122, 0);

/// One capability area of the marketing plugin.
struct Section {
    name: &'static str,
    skill: &'static str,
    status: &'static str,
    detail: &'static str,
}

const SECTIONS: &[Section] = &[
    Section {
        name: "Campaigns",
        skill: "campaign-plan",
        status: "3 active",
        detail: "Plan multi-channel campaigns end to end: objectives, audience, \
                 channel mix, timeline, and budget. Turns a brief into a structured \
                 plan with measurable goals and a launch checklist.",
    },
    Section {
        name: "Content",
        skill: "draft-content / content-creation",
        status: "7 drafts",
        detail: "Draft blog posts, social, email, landing pages, and press \
                 releases with channel-specific formatting. Generates headline \
                 and subject-line options and adapts tone to the target audience.",
    },
    Section {
        name: "SEO",
        skill: "seo-audit",
        status: "2 audits queued",
        detail: "Audit a site or page for on-page SEO: titles, meta, headings, \
                 internal linking, and content gaps. Produces a prioritized list \
                 of fixes ranked by impact and effort.",
    },
    Section {
        name: "Performance",
        skill: "performance-report",
        status: "+12% WoW",
        detail: "Pull channel performance into one report: reach, engagement, \
                 conversion, and spend efficiency. Highlights what is working \
                 and where to reallocate.",
    },
    Section {
        name: "Brand",
        skill: "brand-review",
        status: "voice 8.4/10",
        detail: "Review any asset against brand voice and guidelines. Flags \
                 off-tone phrasing and inconsistent terminology, then rewrites \
                 to match the approved voice.",
    },
    Section {
        name: "Competitors",
        skill: "competitive-brief",
        status: "5 tracked",
        detail: "Build a competitive brief: positioning, messaging deltas, and \
                 the gaps you do not cover yet. Diff-focused — what changed and \
                 what to do about it.",
    },
    Section {
        name: "Email",
        skill: "email-sequence",
        status: "2 sequences live",
        detail: "Design lifecycle email sequences: onboarding, nurture, \
                 re-engagement. Maps triggers to messages with timing and \
                 a clear call to action per step.",
    },
];

struct App {
    list: ListState,
    perf: Vec<u64>,
    running: bool,
}

impl App {
    fn new() -> Self {
        let mut list = ListState::default();
        list.select(Some(0));
        Self {
            list,
            perf: vec![3, 5, 4, 6, 8, 7, 9, 8, 11, 10, 12, 14, 13, 15],
            running: true,
        }
    }

    fn selected(&self) -> usize {
        self.list.selected().unwrap_or(0)
    }

    fn step(&mut self, delta: isize) {
        let n = SECTIONS.len() as isize;
        let next = (self.selected() as isize + delta).rem_euclid(n);
        self.list.select(Some(next as usize));
    }

    /// Demo-only: nudge the performance series so `r` feels alive.
    fn refresh(&mut self) {
        if let Some(&last) = self.perf.last() {
            let next = (last as i64 + ((self.selected() as i64 * 7 + 3) % 9) - 3).clamp(1, 20);
            self.perf.remove(0);
            self.perf.push(next as u64);
        }
    }

    fn on_key(&mut self, code: KeyCode, mods: KeyModifiers) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char('c') if mods.contains(KeyModifiers::CONTROL) => self.running = false,
            KeyCode::Down | KeyCode::Char('j') => self.step(1),
            KeyCode::Up | KeyCode::Char('k') => self.step(-1),
            KeyCode::Tab => self.step(1),
            KeyCode::BackTab => self.step(-1),
            KeyCode::Char('r') => self.refresh(),
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    while app.running {
        terminal.draw(|f| ui(f, &mut app))?;
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press {
                    app.on_key(k.code, k.modifiers);
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(f.area());

    render_header(f, header);

    let [nav, panel] =
        Layout::horizontal([Constraint::Length(26), Constraint::Min(0)]).areas(body);
    render_nav(f, nav, app);
    render_panel(f, panel, app);

    render_footer(f, footer);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        Span::styled(" agentMarket ", Style::new().fg(Color::Black).bg(ACCENT).bold()),
        Span::raw("  Marketing Dashboard"),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(ACCENT));
    f.render_widget(Paragraph::new(title).block(block), area);
}

fn render_nav(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = SECTIONS
        .iter()
        .map(|s| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:<13}", s.name)),
                Span::styled(s.status, Style::new().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Channels ")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(Style::new().fg(ACCENT).add_modifier(Modifier::BOLD))
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, area, &mut app.list);
}

fn render_panel(f: &mut Frame, area: Rect, app: &App) {
    let s = &SECTIONS[app.selected()];
    let is_perf = s.name == "Performance";

    let [head, spark_area, detail] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(if is_perf { 9 } else { 0 }),
        Constraint::Min(0),
    ])
    .areas(area);

    let head_text = vec![
        Line::from(Span::styled(
            s.name,
            Style::new().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("skill  ", Style::new().fg(Color::DarkGray)),
            Span::raw(s.skill),
        ]),
        Line::from(vec![
            Span::styled("status ", Style::new().fg(Color::DarkGray)),
            Span::styled(s.status, Style::new().fg(Color::Green)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(head_text).block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        ),
        head,
    );

    if is_perf {
        let spark = Sparkline::default()
            .block(
                Block::default()
                    .title(" Engagement (14w) ")
                    .borders(Borders::ALL),
            )
            .data(app.perf.iter().copied())
            .style(Style::new().fg(ACCENT));
        f.render_widget(spark, spark_area);
    }

    let detail_p = Paragraph::new(s.detail)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" What this does ")
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1)),
        );
    f.render_widget(detail_p, detail);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let keys = "↑↓/jk move  ·  tab cycle  ·  r refresh  ·  q quit";
    let line = Line::from(vec![
        Span::styled(format!(" {keys} "), Style::new().fg(Color::DarkGray)),
        Span::raw("   "),
        Span::styled(PLUGIN, Style::new().fg(ACCENT)),
    ]);
    f.render_widget(
        Paragraph::new(line).alignment(Alignment::Left),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn screen(app: &mut App, w: u16, h: u16) -> String {
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        t.draw(|f| ui(f, app)).unwrap();
        t.backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect()
    }

    #[test]
    fn renders_header_and_footer() {
        let mut app = App::new();
        let s = screen(&mut app, 100, 30);
        assert!(s.contains("agentMarket"), "header missing");
        assert!(s.contains("Marketing Dashboard"), "title missing");
        assert!(s.contains(PLUGIN), "plugin status missing");
        assert!(s.contains("Campaigns"), "nav missing");
    }

    #[test]
    fn navigation_wraps_both_ways() {
        let mut app = App::new();
        assert_eq!(app.selected(), 0);
        app.step(-1);
        assert_eq!(app.selected(), SECTIONS.len() - 1, "up from 0 wraps to last");
        app.step(1);
        assert_eq!(app.selected(), 0, "down from last wraps to 0");
    }

    #[test]
    fn performance_section_renders_sparkline() {
        let mut app = App::new();
        let perf = SECTIONS.iter().position(|s| s.name == "Performance").unwrap();
        app.list.select(Some(perf));
        let s = screen(&mut app, 100, 30);
        assert!(s.contains("Engagement"), "sparkline block missing");
    }

    #[test]
    fn quit_key_stops_running() {
        let mut app = App::new();
        app.on_key(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(!app.running);
    }

    #[test]
    fn refresh_keeps_series_length() {
        let mut app = App::new();
        let n = app.perf.len();
        app.refresh();
        assert_eq!(app.perf.len(), n, "refresh must not change series length");
    }
}
