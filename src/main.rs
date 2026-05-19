//! agentMarket — 终端营销仪表盘。
//!
//! 把 knowledge-work `marketing` 插件的能力领域（营销活动、内容创作、SEO、
//! 效果分析、品牌审查、竞品分析、邮件序列）呈现为可导航的 TUI。此处展示的
//! 数据仅作示意；每个面板都标注了支撑真实工作流的插件 skill。

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
use unicode_width::UnicodeWidthStr;

const PLUGIN: &str = "marketing@knowledge-work-plugins v1.2.0";
const ACCENT: Color = Color::Rgb(255, 122, 0);

/// 营销插件的一个能力领域。
struct Section {
    name: &'static str,
    skill: &'static str,
    status: &'static str,
    detail: &'static str,
    /// 是否在面板中渲染走势图（互动度迷你图）。
    chart: bool,
}

const SECTIONS: &[Section] = &[
    Section {
        name: "营销活动",
        skill: "campaign-plan",
        status: "3 个进行中",
        detail: "端到端规划多渠道营销活动：目标、受众、渠道组合、时间线与预算。\
                 把一份简报转化为带可衡量目标和上线清单的结构化方案。",
        chart: false,
    },
    Section {
        name: "内容创作",
        skill: "draft-content / content-creation",
        status: "7 篇草稿",
        detail: "撰写博客、社媒、邮件、落地页与新闻稿，并按渠道适配排版。生成\
                 多个标题与邮件主题选项，并按目标受众调整语气。",
        chart: false,
    },
    Section {
        name: "SEO 优化",
        skill: "seo-audit",
        status: "2 个待审计",
        detail: "审计站点或页面的页面内 SEO：标题、meta、标题层级、内链与内容\
                 缺口。产出一份按影响和投入排序的优先修复清单。",
        chart: false,
    },
    Section {
        name: "效果分析",
        skill: "performance-report",
        status: "周环比 +12%",
        detail: "把各渠道表现汇总成一份报告：触达、互动、转化与花费效率。指出\
                 哪些有效，以及该把预算重新分配到哪里。",
        chart: true,
    },
    Section {
        name: "品牌审查",
        skill: "brand-review",
        status: "语调 8.4/10",
        detail: "对照品牌语调与规范审查任意素材。标出跑调的措辞和不一致的术语，\
                 再改写为符合既定语调的版本。",
        chart: false,
    },
    Section {
        name: "竞品分析",
        skill: "competitive-brief",
        status: "5 个跟踪中",
        detail: "构建竞品简报：定位、信息差异，以及你尚未覆盖的缺口。以 diff 为\
                 核心——什么变了，以及该怎么应对。",
        chart: false,
    },
    Section {
        name: "邮件序列",
        skill: "email-sequence",
        status: "2 条序列运行中",
        detail: "设计生命周期邮件序列：新用户引导、培育、再激活。把触发条件映射\
                 到每一封邮件，含时机与每步明确的行动号召。",
        chart: false,
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

    /// 仅用于演示：轻推效果数据序列，让 `r` 刷新有反馈感。
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

/// 按终端显示列宽在右侧补空格（CJK 字符占 2 列），保证导航列对齐。
fn pad_display(s: &str, width: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
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
        Layout::horizontal([Constraint::Length(28), Constraint::Min(0)]).areas(body);
    render_nav(f, nav, app);
    render_panel(f, panel, app);

    render_footer(f, footer);
}

fn render_header(f: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        Span::styled(" agentMarket ", Style::new().fg(Color::Black).bg(ACCENT).bold()),
        Span::raw("  营销仪表盘"),
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
                Span::raw(pad_display(s.name, 12)),
                Span::styled(s.status, Style::new().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" 频道 ")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(Style::new().fg(ACCENT).add_modifier(Modifier::BOLD))
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, area, &mut app.list);
}

fn render_panel(f: &mut Frame, area: Rect, app: &App) {
    let s = &SECTIONS[app.selected()];

    let [head, spark_area, detail] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(if s.chart { 9 } else { 0 }),
        Constraint::Min(0),
    ])
    .areas(area);

    let head_text = vec![
        Line::from(Span::styled(
            s.name,
            Style::new().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("技能  ", Style::new().fg(Color::DarkGray)),
            Span::raw(s.skill),
        ]),
        Line::from(vec![
            Span::styled("状态  ", Style::new().fg(Color::DarkGray)),
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

    if s.chart {
        let spark = Sparkline::default()
            .block(
                Block::default()
                    .title(" 互动度 · 近 14 周 ")
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
                .title(" 功能说明 ")
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1)),
        );
    f.render_widget(detail_p, detail);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let keys = "↑↓/jk 切换  ·  tab 循环  ·  r 刷新  ·  q 退出";
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

    /// Scrape the rendered buffer back to text. ratatui stores a wide (CJK)
    /// grapheme in one cell and fills the next cell with a literal space, so a
    /// naive symbol join splits multi-byte text. Skip the filler cell after any
    /// width-2 grapheme to reconstruct what the user actually sees.
    fn screen(app: &mut App, w: u16, h: u16) -> String {
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        t.draw(|f| ui(f, app)).unwrap();
        let cells: Vec<String> = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol().to_string())
            .collect();
        let mut out = String::new();
        let mut skip = false;
        for sym in &cells {
            if skip {
                skip = false;
                continue;
            }
            out.push_str(sym);
            if UnicodeWidthStr::width(sym.as_str()) == 2 {
                skip = true;
            }
        }
        out
    }

    #[test]
    fn renders_header_and_footer() {
        let mut app = App::new();
        let s = screen(&mut app, 100, 30);
        assert!(s.contains("agentMarket"), "缺少头部品牌");
        assert!(s.contains("营销仪表盘"), "缺少标题");
        assert!(s.contains(PLUGIN), "缺少插件状态行");
        assert!(s.contains("营销活动"), "缺少导航项");
    }

    #[test]
    fn navigation_wraps_both_ways() {
        let mut app = App::new();
        assert_eq!(app.selected(), 0);
        app.step(-1);
        assert_eq!(app.selected(), SECTIONS.len() - 1, "从 0 向上应绕回最后一项");
        app.step(1);
        assert_eq!(app.selected(), 0, "从最后一项向下应绕回 0");
    }

    #[test]
    fn performance_section_renders_sparkline() {
        let mut app = App::new();
        let perf = SECTIONS.iter().position(|s| s.chart).unwrap();
        app.list.select(Some(perf));
        let s = screen(&mut app, 100, 30);
        assert!(s.contains("互动度"), "缺少走势图区块");
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
        assert_eq!(app.perf.len(), n, "刷新不得改变序列长度");
    }

    #[test]
    fn nav_names_pad_to_display_width() {
        // CJK 名称按显示列宽补齐，ASCII 名称同理对齐。
        assert_eq!(pad_display("营销活动", 12), "营销活动    ");
        assert_eq!(pad_display("SEO 优化", 12), "SEO 优化    ");
    }
}
