# agentMarket

A terminal **marketing dashboard** (Rust + [Ratatui](https://ratatui.rs)) that
surfaces the capability areas of the knowledge-work `marketing` plugin
(`marketing@knowledge-work-plugins` v1.2.0), installed at the project level via
`.claude/settings.json`.

## Run

```sh
cargo run --release
```

## Keys

| Key            | Action                          |
| -------------- | ------------------------------- |
| `↑`/`k`         | previous channel                |
| `↓`/`j`         | next channel                    |
| `Tab`/`Shift+Tab` | cycle channels               |
| `r`            | refresh (demo perf series)      |
| `q`/`Esc`/`Ctrl-C` | quit                        |

## Channels → plugin skills

Each panel maps a marketing capability area to the plugin skill that powers
the real workflow:

| Channel      | Skill                          |
| ------------ | ------------------------------ |
| Campaigns    | `campaign-plan`                |
| Content      | `draft-content` / `content-creation` |
| SEO          | `seo-audit`                    |
| Performance  | `performance-report`           |
| Brand        | `brand-review`                 |
| Competitors  | `competitive-brief`            |
| Email        | `email-sequence`               |

Metrics shown are illustrative; wire each panel to its skill for live data.

## Test

```sh
cargo test      # headless TestBackend render + behavior tests
cargo clippy    # lint
```
