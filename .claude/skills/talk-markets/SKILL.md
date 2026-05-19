---
name: talk-markets
description: >-
  Turn marketing work into a polished, self-contained zh-CN HTML report aimed at
  a SPECIFIC stakeholder — the boss/CEO, a PM, sales/CS, press/reporters, a
  customer, investors/board, or a partner — then preview locally and publish to a
  gist for a durable shareable link. The same evidence is re-framed per audience
  (single persona, or multi-audience tabs over one evidence base). Use this
  whenever a marketing person needs to REPORT OUT rather than just produce an
  artifact: campaign recaps, monthly/QBR performance updates, "report to the
  boss / 给老板汇报", investor or board marketing slides-as-page, a press-facing
  brief, a customer-facing results page, a launch retro, or any "summarize what
  marketing did and tell <persona> about it". Its evidence GATE is the
  `/marketing` plugin: by default it first runs the `marketing:start` pipeline
  (/start) to gather a real, current evidence base, then frames that for the
  chosen reader; it only skips when `/marketing`-plugin outputs already exist.
  Triggers include /talk-markets,
  talk-markets, 营销汇报, 给老板/PM/客户/投资人汇报, marketing report, campaign
  recap, stakeholder update, QBR/board marketing page, press brief, customer
  results page. Reach for this even when the user only says "report this to my
  boss" or "make a page for the client" in a marketing context — it is the
  marketing-reporting front end to talk-html, not a generic page maker.
---

# talk-markets

Communicate marketing outcomes in HTML, aimed at the **person being reported
to** — not at the marketer who did the work. Local preview first, then
gist-publish for a durable link. **Output language is always Simplified
Chinese.**

> **Core idea.** `talk-html` makes a good page. `talk-markets` makes a page that
> *lands with a specific reader*. A monthly number that thrills the growth lead
> bores the CEO and scares the customer. This skill's whole job is choosing what
> that reader leads with, what gets cut, and what tone carries it — over
> evidence that is **real**, never invented.

## Relationship to talk-html (do not re-implement)

This skill is a thin marketing-reporting layer **on top of** the canonical
`talk-html`. All the page mechanics are inherited and **must not be copied or
rewritten** here:

- **Self-contained HTML rules**, editorial zh-CN typography, JS-free except the
  two sanctioned clipboard handlers (audit pill + 继续修改 bar), portability,
  reflow, no-emoji, < 200 KB — exactly as `~/.agents/skills/talk-html/SKILL.md`
  §3.
- **Publish / recall / record** use the canonical, pre-authorized helpers
  directly — never a vendored copy:
  - `bash ~/.agents/skills/talk-html/publish.sh "$HTML"` — gist publish, no
    permission prompt, no confirmation step, appends to the shared
    `~/.agents/talk-html/index.jsonl`.
  - `bash ~/.agents/skills/talk-html/recall.sh [substring]` — list/open past
    artifacts (talk-markets pages are indexed in the same place).
  - `bash ~/.agents/skills/talk-html/record-to-gif.sh …` — for any non-static
    dashboard/board motion (§3.1 of talk-html applies verbatim).
- **Audit pill + 继续修改 bar + `<!-- talk-html-meta -->` comment** structural
  slots come from `~/.agents/skills/talk-html/templates/skeleton.html`. Read it
  once for the slots. In the meta comment set `"template":"marketing-report"`.

If you find yourself re-writing publish logic, HTML boilerplate, or the pill —
stop, you are duplicating talk-html. This skill only owns the **audience
layer**: who the reader is, and how the same truth is shaped for them.

`templates/report.html` here is a marketing-report-flavored skeleton (KPI band +
audience-tab slot) layered on talk-html's structural slots — read it for the
shape, design the real page fresh.

## When to invoke

- A marketing person needs to **report out**: campaign recap, monthly /
  QBR / board performance update, launch retro, "tell the boss what we did".
- The output is for **someone the marketer answers to or serves**: CEO/founder,
  their manager/VP, a PM, sales/CS, the press, a customer, investors, a partner.
- The user says "report this to X", "make a page for the client / board",
  "给老板汇报", "营销月报", "season recap for the team".

**The evidence gate: a talk-markets page may only report `/marketing`-plugin
output.** Before source-grounding, talk-markets by default invokes
`marketing:start` (the plugin's one-shot full pipeline) to autonomously gather
real, current data on the target and run every stage; that output is then
framed for the chosen reader (step 2.5). The individual `marketing` skills —
`campaign-plan`, `performance-report`, `seo-audit`, `competitive-brief`,
`brand-review`, `email-sequence`, `draft-content` — are the other admissible
producers. It skips the run only when `/marketing`-plugin outputs for the
target already exist (§2.5); a repo's own artifacts never substitute.

## Workflow

### 0. Preflight (cheap, non-blocking)

```bash
bash ~/.agents/skills/talk-html/check-canon.sh --quiet
```

The topic here is a marketing report, never talk-html itself, so this is just a
sanity line — note drift if printed, do not block on it.

### 1. Resolve the reporting context

Identify the *work being reported* and its boundary:

- Which marketing effort(s): a campaign, a month/quarter, a launch, a channel.
- The time window and the comparison baseline (vs last period / vs goal / vs
  plan). A report with no baseline is a number, not a story.
- Where the real evidence lives (next step).

Synthesize a `slug` (3–5 kebab-case words, e.g. `q2-paid-recap-ceo`) and a
one-sentence `prompt_summary` (≤ 200 chars).

### 2. Identify the audience(s) — the load-bearing step

Pick the persona(s) from `references/audiences.md`. Decide **single vs
multi-audience**:

- **Single persona** — one reader, one inverted-pyramid page tuned to them.
- **Multi-audience** — the *same evidence* genuinely serves ≥ 2 distinct
  readers (e.g. CEO wants moat + efficient growth; a customer wants proof +
  what it means for them). Give them **audience tabs / in-page anchors** that
  re-frame the copy over the **same** numbers — never one averaged page that
  lands with neither, and never a fabricated second audience to look thorough.
  This generalizes talk-html §3.2's two-audience rule to the marketing personas.

For each chosen persona, pull its row from `references/audiences.md`: *what they
care about · what to lead with · what to cut · tone · the one question they will
ask*. That row drives the page's first screen.

Compact persona map (full matrix + per-persona pitfalls in
`references/audiences.md` — read it before writing a page for any persona):

| 读者 | 最关心 | 首屏先讲 | 砍掉/下沉 | 语气 |
|---|---|---|---|---|
| 老板 / CEO·创始人 | 增长是否高效、是否在变好、要不要加注 | 一句结论 + 钱的方向（ROI/CAC/pipeline） | 渠道操作细节、工具名词 | 直接、有判断、给建议 |
| 直属上级 / 市场 VP | 目标达成度、风险、需要什么资源 | 对目标的红黄绿 + 缺口与所需支持 | 战术执行日志 | 务实、负责、要资源说清 |
| 产品 PM | 营销信号对产品的含义 | 用户/需求信号、转化卡点、共建项 | 创意稿、媒介排期 | 协作、以用户为中心 |
| 销售 / 客户成功 | 能不能用、给谁、怎么跟进 | 可用素材 + 线索质量 + 话术 | 品牌叙事、长期战略 | 赋能、可落地 |
| 媒体 / 记者 | 有什么新鲜事、可引用、可核实 | 新闻点一句话 + 可引用数据 + 事实出处 | 内部 KPI、未公开数字 | 客观、可核实、不浮夸 |
| 客户 / customer | 这对我意味着什么、可信吗 | 对客户的价值 + 可验证证据 + 下一步 | 内部指标、内部路径 | 以客户为中心、低风险、可审计 |
| 投资人 / 董事会 | 市场、护城河、可重复增长引擎 | 增长引擎与单位经济 + 可重复性 | 单次活动八卦 | 战略、数字诚实、有节奏 |
| 合作伙伴 / 渠道 | 共赢、各自动作、节奏 | 共同收益 + 双方接下来要做的事 | 内部预算、内部反思 | 互惠、清晰分工 |

### 2.5 Run `marketing:start` (default — build the real evidence base)

**By default, invoke `marketing:start` now**, before source-grounding. It is
the marketing plugin's one-shot full pipeline: given the step-1 target (a
campaign, site, repo, or org) it autonomously gathers real, current data and
runs every stage end-to-end — competitive brief, battlecard, gap messaging,
per-target deep-dives, monitoring — emitting a self-contained talk-html hub +
one HTML per stage + cited raw artifacts in `$CLAUDE_JOB_DIR`. That output is
the **primary real evidence** feeding §3 and §4, not a side deliverable.

*Why default:* talk-markets reports marketing outcomes; without freshly
gathered, sourced data it can only re-skin whatever was lying around — exactly
how a stakeholder page ends up thin or padded. Gathering first *is* the honesty
bar in practice. Hand `marketing:start` the step-1 target; the step-2 audience
decision is unchanged — it frames this evidence, never whether it is gathered.

*The gate:* every number and claim on the page must have exited the
`/marketing` plugin (`marketing:start` or a `marketing:*` skill). A repo's
`judge.json`, a recorded proof run, generic CSVs/dashboards, or user-pasted
numbers are at most *inputs the pipeline consumes* — never the report's
evidence on their own.

*Opt out* (skip 2.5 → §3; note the evidence path in one line) on **one**
condition: valid `/marketing`-plugin outputs for this target already exist (a
prior `marketing:*` run still available, or the user hands over actual
marketing-plugin outputs) — report those instead of re-running. If the user
says "skip /start / 别跑 start" but no such outputs exist, talk-markets cannot
honestly produce a marketing report: say so (zh-CN) and stop — never fabricate
or pass non-marketing artifacts off as marketing evidence.

### 3. Source-ground in REAL marketing evidence (honesty is load-bearing)

Inherit talk-html §3.0/§3.1 verbatim, with one marketing-specific sharpening:
**numbers in a stakeholder report are claims about the business — never invent
them.** Before writing, find the real source:

- **The only admissible evidence is `/marketing`-plugin output:** the
  `marketing:start` hub, per-stage HTML, and cited artifacts in
  `$CLAUDE_JOB_DIR` from step 2.5 — or, on the §2.5 opt-out, a prior
  `marketing:*` run's outputs. Treat their cited sources as the report's
  sources.
- Real analytics exports, CSVs, dashboards, screenshots, or user-pasted files
  are **inputs you feed the `/marketing` pipeline**, not the report's evidence
  by themselves — route them through a `marketing:*` skill so the page still
  stands on plugin output.
- If a dashboard/board is part of the story and it is **non-static** (it
  *runs*, it has live state), talk-html §3.1 applies: record a real run with
  `record-to-gif.sh` and embed the GIF — do not draw a fake chart.
- If a number the persona needs does not exist yet, you have two honest moves,
  never a third (fabrication):
  1. produce it with the right `marketing` skill (e.g. run
     `performance-report`) and report *that*, or
  2. state plainly on the page, in Chinese, that the metric is unavailable and
     show what you do have.

Name the source of every headline number inline or in a collapsed
「数据来源」`<details>`. A stakeholder page that gets caught with an invented
KPI is worse than no page.

### 4. Structure the page per audience (inverted pyramid, re-framed)

Apply talk-html §3.2's inverted pyramid, with the **value sentence and proof
chain rewritten in the persona's terms** (from the matrix). Shape ≈ 6 blocks:

1. **结论一句话** — the one sentence *this persona* would repeat to their own
   boss. (CEO: "付费渠道 Q2 ROI 转正，建议加注 X"; customer: "你的这项指标改善
   了 N%，下面是怎么验证的"。)
2. **背景/问题** — the gap or question the persona recognizes, in their words.
3. **结果与证据** — the real numbers (§3) with a proof caption that says what
   they *rule out*, not just what they show. Embed recorded motion if the
   evidence is a live board (§3.1).
4. **要点拆解** — the causal chain as discrete, checkable claims.
5. **建议 / 下一步** — what you want *from this persona* (a decision, budget,
   a hand-off, a quote approval) — explicit, not implied.
6. **诚实边界 / 数据来源** — caveats, method, sources. Preserved but collapsed
   in a final `<details>`, never on the first screen.

Multi-audience: each tab/anchor is its own 1–5 over the **same** block 3
evidence; block 6 is shared.

### 5. Write the HTML

Save to `~/.agents/talk-html/<slug>-YYYYMMDD-HHMMSS.html` (same index/recall
home as talk-html). Follow talk-html §3 hard requirements verbatim — zh-CN
editorial typography (Noto Serif SC carries the body), self-contained, SVG
diagrams, mobile reflow, no emoji, JS-free except the two sanctioned clipboard
handlers. Use `templates/report.html` for the KPI-band + audience-tab shape;
design the real visual identity fresh per report.

Stakeholder-report specifics on top of talk-html:

- **Numbers legible to a non-analyst.** Round, label, show direction (▲/▼ + vs
  what). One hero KPI band on the first screen; deep tables sink into
  `<details>`.
- **No hype adjectives** ("革命性 / 颠覆性 / explosive") — the proof is the
  claim. The persona's manager will read this; write like it.
- **Period + baseline always visible** near the headline ("2026 Q2 · vs Q1 ·
  vs 计划").

### 6. Meta, audit pill, 继续修改 bar

Use the canonical slots from `~/.agents/skills/talk-html/templates/skeleton.html`
exactly as talk-html §4–§5.1 specifies. Resolve `session_id` / `origin_prompt`
with talk-html §4's snippet. In the `<!-- talk-html-meta -->` comment set
`"template":"marketing-report"` and put the persona(s) in `prompt_summary`
(e.g. `Q2 付费复盘 → CEO+客户双受众`). The 继续修改 bar's copied prompt is
talk-html's verbatim (it relocates via `recall.sh <slug>`); do not invent a
talk-markets-specific resume path.

### 7. Preview, then publish (default — no confirmation, no permission prompt)

```bash
open "$HTML_PATH"
bash ~/.agents/skills/talk-html/publish.sh "$HTML_PATH"           # secret gist (link-only)
bash ~/.agents/skills/talk-html/publish.sh "$HTML_PATH" --public  # public (press/customer pages)
```

talk-html §6–§7 govern this verbatim: opening the preview is **not** a gate —
do not stop to ask "should I publish?". Publish automatically. Skip publishing
**only** on an explicit opt-out ("别发 / 本地就行 / no gist"); then print
`local: file://<path>` and the manual publish command, and stop.

A press- or customer-facing page is the case where `--public` is usually right
(it will be linked from outside) — still default to secret if unsure and say so.

### 8. Print URLs

Output exactly these four lines, in this order (talk-html §8):

```
local:    file://<path>
gist:     <gist page URL>
raw:      <raw.githubusercontent URL>
rendered: <htmlpreview.github.io URL>
```

The **rendered** URL is what the user forwards to the stakeholder.

## Quality bar — do not violate

Everything in `~/.agents/skills/talk-html/SKILL.md` "Quality bar" applies (real
editorial design, no emoji unless asked, no clipping, SVG diagrams, < 200 KB,
three-path session traceability, gist parity, non-static→recorded, the 继续修改
bar, the §3.2 audience discipline). On top of that, talk-markets adds:

1. **The first screen passes the "could the persona repeat it?" test.** If the
   chosen reader could not say the headline sentence to *their* boss in one
   breath, rewrite it. The persona row in `references/audiences.md` is the rubric.
2. **Every headline number is real and sourced.** No invented or
   "representative" KPIs. Each is traceable to a `marketing`-skill output, a
   real export, or a recorded run; sources named inline or in 「数据来源」. A
   fabricated metric is an automatic fail.
3. **You asked for something.** A stakeholder report with no decision, resource
   ask, hand-off, or next step is a status dump, not a report. Block 5 is
   mandatory.
4. **Right reader, not the author.** No internal job dirs, host paths, tool
   plumbing, channel-ops minutiae, or "what I did" log on a CEO/customer/press
   page — those sink into `<details>` or are cut. Honest caveats stay, collapsed
   last.
5. **Multi-audience splits are real.** Two tabs only when two real readers exist
   for this report; same evidence under each; never a padded second audience.

## Failure modes

| Failure | Recovery |
|---|---|
| `marketing:start` fails / unavailable / target unfetchable | Do NOT substitute non-marketing artifacts. State on the page (zh-CN) that the `/marketing`-plugin evidence run failed and the report cannot be sourced from it; retry a narrower `marketing:*` skill if one fits. Never fabricate. |
| No real numbers available | Run the right `marketing:*` skill to produce them; if it yields nothing, state on the page (zh-CN) that the metric is unavailable. Show only `/marketing`-plugin output — never fabricate or backfill with non-marketing artifacts. |
| Unsure which persona | Ask the user once: "report to whom?" — the persona changes the whole first screen. If truly unanswered, default to 直属上级/VP (the most common marketing reporting line) and say so on the page. |
| Dashboard is live/non-static | talk-html §3.1: `record-to-gif.sh` a real run, embed the GIF. Do not draw a fake chart. |
| `gh` missing / not authed | talk-html failure modes apply — keep local file, print local path + manual `publish.sh`. |
| User said "don't publish" | Honor it: print `local: file://<path>` + the manual publish command, stop. |
| Tempted to copy publish.sh / pill / boilerplate | Stop — that is talk-html's, reused by reference. This skill only owns the audience layer. |
