# agentMarket

终端**营销仪表盘**（Rust + [Ratatui](https://ratatui.rs)），把 knowledge-work
`marketing` 插件（`marketing@knowledge-work-plugins` v1.2.0，通过
`.claude/settings.json` 在项目级启用）的能力领域可视化呈现。界面为中文。

## 运行

```sh
cargo run --release
```

## 按键

| 按键              | 操作                     |
| ----------------- | ------------------------ |
| `↑`/`k`           | 上一个频道               |
| `↓`/`j`           | 下一个频道               |
| `Tab`/`Shift+Tab` | 循环切换频道             |
| `r`               | 刷新（演示用效果序列）   |
| `q`/`Esc`/`Ctrl-C`| 退出                     |

## 频道 → 插件 skill

每个面板把一个营销能力领域映射到支撑真实工作流的插件 skill：

| 频道       | Skill                                |
| ---------- | ------------------------------------ |
| 营销活动   | `campaign-plan`                      |
| 内容创作   | `draft-content` / `content-creation` |
| SEO 优化   | `seo-audit`                          |
| 效果分析   | `performance-report`                 |
| 品牌审查   | `brand-review`                       |
| 竞品分析   | `competitive-brief`                  |
| 邮件序列   | `email-sequence`                     |

界面指标仅作示意；把每个面板接到对应 skill 即可获得实时数据。

## 测试

```sh
cargo test      # 无头 TestBackend 渲染 + 行为测试
cargo clippy    # lint
```
