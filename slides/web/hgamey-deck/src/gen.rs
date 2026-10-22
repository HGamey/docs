//! 把 content/deck.json 里一页的内容块转成 Splash DSL 文本，交给 d3.Splash 求值。
//! 版式只在这里；数字和文字只在 deck.json 里。

use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize, Clone, Default)]
pub struct Deck {
    #[allow(dead_code)]
    pub meta: serde_json::Value,
    pub slides: Vec<Slide>,
}

#[derive(Deserialize, Clone, Default)]
pub struct Slide {
    #[allow(dead_code)]
    pub id: String,
    #[serde(default)]
    pub chapter: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub foot: String,
    #[serde(default)]
    pub layout: String,
    #[serde(default)]
    pub left: Vec<Block>,
    #[serde(default)]
    pub right: Vec<Block>,
}

#[derive(Deserialize, Clone, Default)]
pub struct Kpi {
    pub v: String,
    pub l: String,
    #[serde(default)]
    pub tag: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct Step {
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub state: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct ForestRow {
    pub label: String,
    pub est: f64,
    pub lo: f64,
    pub hi: f64,
    #[serde(default)]
    pub verdict: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct Block {
    pub t: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub pending: bool,
    #[serde(default)]
    pub what: String,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub items: serde_json::Value,
    #[serde(default)]
    pub head: Vec<String>,
    #[serde(default)]
    pub rows: serde_json::Value,
    #[serde(default)]
    pub w: Vec<f64>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub values: Vec<f64>,
    #[serde(default)]
    pub value_notes: Vec<String>,
    #[serde(default)]
    pub row_labels: Vec<String>,
    #[serde(default)]
    pub col_labels: Vec<String>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    /// chart3d：图表登记 id 与规格（见 chart3d.rs）
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub spec: serde_json::Value,
    #[serde(default)]
    pub height: Option<f64>,
}

// 配色：沿用 KDN deck 的中性底色，强调色换成 HGamey 的青绿 + 藏青。
pub const INK: &str = "#x202020";
pub const INK2: &str = "#x383A3C";
pub const MUTED: &str = "#x85827D";
pub const LINE: &str = "#xC4C4C2";
pub const LINE2: &str = "#xE2E0DA";
pub const PAPER: &str = "#xFFFFFF";
pub const PANEL: &str = "#xFAF8F4";
pub const NAVY: &str = "#x33435D";
pub const TEAL: &str = "#x2E7D74";
pub const TEAL_T: &str = "#xE3F0EE";
pub const ROSE: &str = "#xC76565";
pub const AMBER: &str = "#xA8752A";
pub const AMBER_T: &str = "#xF7EEDD";
pub const BEIGE: &str = "#x8A8174";
pub const BEIGE_T: &str = "#xF0EEEA";

const FS_BODY: f64 = 10.5;
const FS_SMALL: f64 = 9.0;
const FS_H: f64 = 11.5;

/// Splash 字符串字面量：去掉会破坏解析的字符。
fn q(s: &str) -> String {
    let s = s.replace('\\', "/").replace('"', "”").replace('\n', " ");
    format!("\"{}\"", s)
}

fn label(text: &str, color: &str, size: f64, fill: bool) -> String {
    format!(
        "Label{{{}text: {} draw_text.color: {} draw_text.text_style.font_size: {}}}",
        if fill { "width: Fill " } else { "" },
        q(text),
        color,
        size
    )
}

fn bold(text: &str, color: &str, size: f64, fill: bool) -> String {
    format!(
        "H4{{{}text: {} draw_text.color: {} draw_text.text_style.font_size: {}}}",
        if fill { "width: Fill " } else { "width: Fit " },
        q(text),
        color,
        size
    )
}

fn tag_colors(tag: &str) -> (&'static str, &'static str) {
    match tag {
        "实测" => (NAVY, "#xE6EAF0"),
        "推断" | "估算" | "外推" => (BEIGE, BEIGE_T),
        "离线" | "离线合成数据" => (TEAL, TEAL_T),
        "未验证" => (ROSE, "#xF5E5E3"),
        "待测" => (AMBER, AMBER_T),
        _ => (MUTED, BEIGE_T),
    }
}

fn chip(tag: &str) -> String {
    let (fg, bg) = tag_colors(tag);
    format!(
        "RoundedView{{width: Fit height: Fit new_batch: true padding: Inset{{left: 6 right: 6 top: 1 bottom: 1}} draw_bg.color: {bg} draw_bg.border_size: 1.0 draw_bg.border_color: {fg} draw_bg.border_radius: 3.0 {}}}",
        label(tag, fg, FS_SMALL, false)
    )
}

fn title_row(title: &str, tag: &str) -> String {
    let mut s = String::from("View{width: Fill height: Fit flow: Right spacing: 8 align: Align{y: 0.5} ");
    s += &bold(title, INK, FS_H, true);
    if !tag.is_empty() {
        s += &chip(tag);
    }
    s += "}";
    s
}

fn items_str(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}

fn fmt_num(v: f64) -> String {
    if v.fract().abs() < 1e-9 {
        format!("{}", v as i64)
    } else if (v * 10.0).fract().abs() < 1e-9 {
        format!("{:.1}", v)
    } else {
        format!("{:.2}", v)
    }
}

/// 用权重切分一行，实现按数值定位（Fill 的权重按长度给）。
fn seg(len: f64, inner: &str) -> String {
    let wgt = (len.max(0.0) * 1000.0).max(0.001);
    format!("View{{width: Fill{{weight: {:.3}}} height: Fill {inner}}}", wgt)
}

/// 有底色的一段（plain View 的 draw_bg 没有颜色实例，得用 SolidView）。
fn seg_solid(len: f64, color: &str) -> String {
    let wgt = (len.max(0.0) * 1000.0).max(0.001);
    format!("SolidView{{width: Fill{{weight: {:.3}}} height: Fill draw_bg.color: {color}}}", wgt)
}

fn block(b: &Block) -> String {
    match b.t.as_str() {
        "h" => bold(&b.text, INK, FS_H + 0.5, true),
        "p" => label(&b.text, INK2, FS_BODY, true),
        "note" => label(&b.text, MUTED, FS_SMALL + 0.5, true),
        "ul" | "ol" => {
            let mut s = String::from("View{width: Fill height: Fit flow: Down spacing: 3 ");
            for (i, it) in items_str(&b.items).iter().enumerate() {
                let mark = if b.t == "ol" { format!("{}.", i + 1) } else { "·".to_string() };
                let _ = write!(
                    s,
                    "View{{width: Fill height: Fit flow: Right spacing: 4 View{{width: 16 height: Fit {}}} {}}}",
                    label(&mark, TEAL, FS_BODY, false),
                    label(it, INK2, FS_BODY, true)
                );
            }
            s += "}";
            s
        }
        "kpis" => kpis(b),
        "table" => table(b),
        "flow" => flow(b),
        "pending" => pending(b),
        "bars" => {
            if b.pending {
                pending(b)
            } else if b.values.iter().any(|v| *v < 0.0) || !b.value_notes.is_empty() {
                hbars(b)
            } else {
                d3_bars(b)
            }
        }
        "forest" => {
            if b.pending {
                pending(b)
            } else {
                forest(b)
            }
        }
        "refbars" => {
            if b.pending {
                pending(b)
            } else {
                refbars_block(b)
            }
        }
        "chart3d" => chart3d_block(b),
        "heat" => heat(b),
        "grid3d" => {
            if b.pending {
                pending(b)
            } else {
                grid3d(b)
            }
        }
        _ => label(&format!("（未知内容类型：{}）", b.t), ROSE, FS_SMALL, true),
    }
}

fn kpis(b: &Block) -> String {
    let list: Vec<Kpi> = serde_json::from_value(b.items.clone()).unwrap_or_default();
    let mut s = String::from("View{width: Fill height: Fit flow: Right spacing: 8 ");
    for k in list {
        let _ = write!(
            s,
            "RoundedView{{width: Fill height: Fit flow: Down spacing: 2 new_batch: true padding: Inset{{left: 10 right: 10 top: 8 bottom: 8}} draw_bg.color: {PAPER} draw_bg.border_size: 1.0 draw_bg.border_color: {LINE} draw_bg.border_radius: 4.0 View{{width: Fill height: Fit flow: Right spacing: 6 align: Align{{y: 0.5}} {} {}}} {}}}",
            bold(&k.v, NAVY, 19.0, false),
            if k.tag.is_empty() { String::new() } else { chip(&k.tag) },
            label(&k.l, MUTED, FS_SMALL, true)
        );
    }
    s += "}";
    s
}

fn table(b: &Block) -> String {
    let rows: Vec<Vec<String>> = serde_json::from_value(b.rows.clone()).unwrap_or_default();
    let ncol = b.head.len().max(rows.first().map(|r| r.len()).unwrap_or(1));
    let w: Vec<f64> = (0..ncol).map(|i| *b.w.get(i).unwrap_or(&1.0)).collect();
    let mut s = format!(
        "RoundedView{{width: Fill height: Fit flow: Down new_batch: true padding: Inset{{left: 10 right: 10 top: 6 bottom: 6}} draw_bg.color: {PAPER} draw_bg.border_size: 1.0 draw_bg.border_color: {LINE} draw_bg.border_radius: 4.0 "
    );
    if !b.title.is_empty() {
        s += &title_row(&b.title, &b.tag);
    }
    let row = |cells: &[String], color: &str, size: f64| -> String {
        let mut r = String::from("View{width: Fill height: Fit flow: Right spacing: 10 padding: Inset{top: 3 bottom: 3} ");
        for (i, c) in cells.iter().enumerate() {
            let _ = write!(
                r,
                "View{{width: Fill{{weight: {:.1}}} height: Fit {}}}",
                w.get(i).unwrap_or(&1.0) * 100.0,
                label(c, color, size, true)
            );
        }
        r += "}";
        r
    };
    if !b.head.is_empty() {
        s += &row(&b.head, MUTED, FS_SMALL + 0.5);
        let _ = write!(s, "SolidView{{width: Fill height: 1 draw_bg.color: {LINE}}}");
    }
    for (i, r) in rows.iter().enumerate() {
        if i > 0 {
            let _ = write!(s, "SolidView{{width: Fill height: 1 draw_bg.color: {LINE2}}}");
        }
        s += &row(r, INK2, FS_BODY - 0.5);
    }
    s += "}";
    s
}

fn state_colors(state: &str) -> (&'static str, &'static str, &'static str) {
    // (边框, 底色, 状态文字)
    match state {
        "done" => (NAVY, PAPER, "已完成"),
        "wip" => (TEAL, TEAL_T, "进行中"),
        "pending" => (AMBER, AMBER_T, "待测"),
        _ => (BEIGE, BEIGE_T, "未开始"),
    }
}

fn flow(b: &Block) -> String {
    let steps: Vec<Step> = serde_json::from_value(b.items.clone()).unwrap_or_default();
    let vertical = steps.len() > 4;
    let mut s = if vertical {
        String::from("View{width: Fill height: Fit flow: Down spacing: 4 ")
    } else {
        String::from("View{width: Fill height: Fit flow: Right spacing: 4 align: Align{y: 0.5} ")
    };
    for (i, st) in steps.iter().enumerate() {
        if i > 0 {
            s += &if vertical {
                format!("View{{width: Fill height: Fit padding: Inset{{left: 18}} {}}}", label("↓", MUTED, FS_BODY, false))
            } else {
                label("→", MUTED, 13.0, false)
            };
        }
        let (fg, bg, st_txt) = state_colors(&st.state);
        let _ = write!(
            s,
            "RoundedView{{width: Fill height: Fit flow: Down spacing: 3 new_batch: true padding: Inset{{left: 10 right: 10 top: 8 bottom: 8}} draw_bg.color: {bg} draw_bg.border_size: 1.2 draw_bg.border_color: {fg} draw_bg.border_radius: 4.0 View{{width: Fill height: Fit flow: Right spacing: 6 align: Align{{y: 0.5}} {} {}}} {}}}",
            bold(&st.title, fg, FS_BODY + 0.5, true),
            label(st_txt, fg, FS_SMALL - 0.5, false),
            label(&st.text, INK2, FS_SMALL + 0.5, true)
        );
    }
    s += "}";
    s
}

fn pending(b: &Block) -> String {
    let mut s = format!(
        "RoundedView{{width: Fill height: Fit flow: Down spacing: 6 new_batch: true padding: 12 draw_bg.color: {AMBER_T} draw_bg.border_size: 1.5 draw_bg.border_color: {AMBER} draw_bg.border_radius: 4.0 "
    );
    s += &format!(
        "View{{width: Fill height: Fit flow: Right spacing: 8 align: Align{{y: 0.5}} {} {}}}",
        chip("待测"),
        bold(&b.title, INK, FS_H, true)
    );
    // 空图框：三条网格线，不画任何数据
    let _ = write!(
        s,
        "RoundedView{{width: Fill height: 64 flow: Down spacing: 0 new_batch: true padding: Inset{{left: 10 right: 10 top: 10 bottom: 6}} draw_bg.color: #xFFFDF8 draw_bg.border_size: 1.0 draw_bg.border_color: #xE3CFA8 draw_bg.border_radius: 3.0 \
         SolidView{{width: Fill height: 1 draw_bg.color: #xEADCC0}} View{{width: Fill height: 13}} \
         SolidView{{width: Fill height: 1 draw_bg.color: #xEADCC0}} View{{width: Fill height: Fit align: Align{{x: 0.5}} {}}} \
         SolidView{{width: Fill height: 1 draw_bg.color: #xEADCC0}}}}",
        label("暂无数据，不绘图", AMBER, FS_SMALL, false)
    );
    if !b.what.is_empty() {
        s += &format!(
            "View{{width: Fill height: Fit flow: Right spacing: 6 View{{width: 64 height: Fit {}}} {}}}",
            label("要放什么", MUTED, FS_SMALL, false),
            label(&b.what, INK2, FS_SMALL + 0.5, true)
        );
    }
    if !b.job.is_empty() {
        s += &format!(
            "View{{width: Fill height: Fit flow: Right spacing: 6 View{{width: 64 height: Fit {}}} {}}}",
            label("产出任务", MUTED, FS_SMALL, false),
            label(&b.job, INK2, FS_SMALL + 0.5, true)
        );
    }
    s += "}";
    s
}

fn chart_panel_open(b: &Block) -> String {
    let mut s = format!(
        "RoundedView{{width: Fill height: Fit flow: Down spacing: 6 new_batch: true padding: Inset{{left: 12 right: 12 top: 8 bottom: 8}} draw_bg.color: {PAPER} draw_bg.border_size: 1.0 draw_bg.border_color: {LINE} draw_bg.border_radius: 4.0 "
    );
    s += &title_row(&b.title, &b.tag);
    s
}

fn chart_panel_close(b: &Block) -> String {
    let mut s = String::new();
    if !b.note.is_empty() {
        s += &label(&b.note, MUTED, FS_SMALL, true);
    }
    s += "}";
    s
}

/// 正值柱状图：makepad-d3 的 d3.BarChart。轴文字目前在 d3 里不显示，所以数值和类别用 Label 写在柱下。
fn d3_bars(b: &Block) -> String {
    let mut s = chart_panel_open(b);
    let data: Vec<String> = b.values.iter().map(|v| fmt_num(*v)).collect();
    let labels: Vec<String> = b.labels.iter().map(|l| q(l)).collect();
    let n = b.values.len().max(1);
    let height = 150;
    let _ = write!(
        s,
        "d3.BarChart{{width: Fill height: {height} data: [{}] labels: [{}] bar_color: {TEAL} hover_color: {NAVY} grid_color: {LINE2} label_color: {MUTED} plot_margin: Inset{{left: 8 right: 8 top: 10 bottom: 8}}}}",
        data.join(" "),
        labels.join(" ")
    );
    let _ = write!(s, "View{{width: Fill height: Fit flow: Right spacing: 0 padding: Inset{{left: 8 right: 8}} ");
    for i in 0..n {
        let v = b.values.get(i).copied().unwrap_or(0.0);
        let l = b.labels.get(i).cloned().unwrap_or_default();
        let _ = write!(
            s,
            "View{{width: Fill height: Fit flow: Down align: Align{{x: 0.5}} {} {}}}",
            bold(&format!("{} {}", fmt_num(v), b.unit), NAVY, FS_BODY, false),
            label(&l, INK2, FS_SMALL, false)
        );
    }
    s += "}";
    s += &chart_panel_close(b);
    s
}

/// 有负值或带逐项说明的数值：横向条，零点居中，按数值定位。
fn hbars(b: &Block) -> String {
    let mut s = chart_panel_open(b);
    let mut lo = b.min.unwrap_or(0.0f64.min(b.values.iter().cloned().fold(f64::INFINITY, f64::min)));
    let mut hi = b.max.unwrap_or(0.0f64.max(b.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)));
    let m = lo.abs().max(hi.abs()) * 1.1;
    if b.min.is_none() {
        lo = if lo < 0.0 { -m } else { 0.0 };
    }
    if b.max.is_none() {
        hi = m;
    }
    for (i, v) in b.values.iter().enumerate() {
        let l = b.labels.get(i).cloned().unwrap_or_default();
        let note = b.value_notes.get(i).cloned().unwrap_or_default();
        let color = if *v < 0.0 { TEAL } else { ROSE };
        let (a, c) = if *v < 0.0 { (*v, 0.0) } else { (0.0, *v) };
        let track = format!(
            "View{{width: Fill height: 14 flow: Right {}{}{}}}",
            seg(a - lo, ""),
            seg_solid(c - a, color),
            seg(hi - c, "")
        );
        let zero = format!(
            "View{{width: Fill height: 14 flow: Right {} SolidView{{width: 1 height: Fill draw_bg.color: {MUTED}}} {}}}",
            seg(0.0 - lo, ""),
            seg(hi - 0.0, "")
        );
        let _ = write!(
            s,
            "View{{width: Fill height: Fit flow: Right spacing: 8 align: Align{{y: 0.5}} View{{width: 92 height: Fit {}}} View{{width: Fill height: 14 flow: Overlay {} {}}} View{{width: 60 height: Fit {}}} View{{width: 150 height: Fit {}}}}}",
            label(&l, INK2, FS_SMALL + 0.5, true),
            zero,
            track,
            bold(&format!("{}{}", fmt_num(*v), b.unit), color, FS_SMALL + 0.5, false),
            label(&note, MUTED, FS_SMALL, true)
        );
    }
    s += &chart_panel_close(b);
    s
}

/// 效应 + 95% 区间（森林图）：区间线 + 估计点 + 零线。
fn forest(b: &Block) -> String {
    let rows: Vec<ForestRow> = serde_json::from_value(b.rows.clone()).unwrap_or_default();
    let lo = b.min.unwrap_or(-1.0);
    let hi = b.max.unwrap_or(1.0);
    let mut s = chart_panel_open(b);
    // 刻度：三层叠放——左端值、零点处的「0」（按权重定位）、右端值
    let zero_lbl = format!("View{{width: Fill height: Fit flow: Right {} {} {}}}", seg(0.0 - lo, ""), label("0", MUTED, FS_SMALL, false), seg(hi - 0.0, ""));
    let _ = write!(
        s,
        "View{{width: Fill height: Fit flow: Right spacing: 8 View{{width: 150 height: Fit}} View{{width: Fill height: Fit flow: Overlay {} {} {}}} View{{width: 120 height: Fit}}}}",
        format!("View{{width: Fill height: Fit {}}}", label(&format!("{}{}", fmt_num(lo), b.unit), MUTED, FS_SMALL, false)),
        zero_lbl,
        format!("View{{width: Fill height: Fit align: Align{{x: 1.0}} {}}}", label(&format!("+{}{}", fmt_num(hi), b.unit), MUTED, FS_SMALL, false)),
    );
    for r in rows {
        let crosses = r.lo <= 0.0 && r.hi >= 0.0;
        let color = if crosses { AMBER } else if r.est > 0.0 { ROSE } else { TEAL };
        let line = format!(
            "View{{width: Fill height: 16 flow: Right {}{}{}}}",
            seg(r.lo - lo, ""),
            seg(r.hi - r.lo, &format!("align: Align{{y: 0.5}} SolidView{{width: Fill height: 3 draw_bg.color: {color}}}")),
            seg(hi - r.hi, "")
        );
        let dot = format!(
            "View{{width: Fill height: 16 flow: Right align: Align{{y: 0.5}} {} RoundedView{{width: 10 height: 10 draw_bg.color: {color} draw_bg.border_radius: 5.0}} {}}}",
            seg(r.est - lo, ""),
            seg(hi - r.est, "")
        );
        let zero = format!(
            "View{{width: Fill height: 16 flow: Right {} SolidView{{width: 1 height: Fill draw_bg.color: {MUTED}}} {}}}",
            seg(0.0 - lo, ""),
            seg(hi - 0.0, "")
        );
        let _ = write!(
            s,
            "View{{width: Fill height: Fit flow: Right spacing: 8 align: Align{{y: 0.5}} View{{width: 150 height: Fit flow: Down {} {}}} View{{width: Fill height: 16 flow: Overlay {zero} {line} {dot}}} View{{width: 120 height: Fit {}}}}}",
            label(&r.label, INK2, FS_SMALL + 0.5, true),
            label(&r.verdict, color, FS_SMALL, true),
            label(
                &format!("{}{} [{}, {}]", if r.est > 0.0 { "+" } else { "" }, fmt_num(r.est), fmt_num(r.lo), fmt_num(r.hi)),
                INK2,
                FS_SMALL,
                true
            )
        );
    }
    s += &chart_panel_close(b);
    s
}

/// 二维计数矩阵：makepad-d3 的 d3.Bar3D（可拖动旋转）；数值另列成表，避免只能看柱高。
fn grid3d(b: &Block) -> String {
    let rows: Vec<Vec<f64>> = serde_json::from_value(b.rows.clone()).unwrap_or_default();
    let mut s = chart_panel_open(b);
    let data: Vec<String> = rows
        .iter()
        .map(|r| format!("[{}]", r.iter().map(|v| fmt_num(*v)).collect::<Vec<_>>().join(" ")))
        .collect();
    let _ = write!(
        s,
        "View{{width: Fill height: Fit flow: Right spacing: 12 align: Align{{y: 0.5}} d3.Bar3D{{width: 300 height: 190 data: [{}] colormap: \"viridis\"}} ",
        data.join(", ")
    );
    // 数值表
    let mut t = String::from("View{width: Fill height: Fit flow: Down spacing: 2 ");
    let mut head = String::from("View{width: Fill height: Fit flow: Right View{width: 60 height: Fit} ");
    for c in &b.col_labels {
        let _ = write!(head, "View{{width: Fill height: Fit align: Align{{x: 0.5}} {}}}", label(c, MUTED, FS_SMALL, false));
    }
    head += "}";
    t += &head;
    for (i, r) in rows.iter().enumerate() {
        let mut line = format!(
            "View{{width: Fill height: Fit flow: Right View{{width: 60 height: Fit {}}} ",
            label(b.row_labels.get(i).map(|s| s.as_str()).unwrap_or(""), INK2, FS_SMALL + 0.5, false)
        );
        for v in r {
            let color = if *v > 0.0 { NAVY } else { LINE };
            let _ = write!(line, "View{{width: Fill height: Fit align: Align{{x: 0.5}} {}}}", bold(&fmt_num(*v), color, FS_BODY, false));
        }
        line += "}";
        t += &line;
    }
    t += &label("拖动左图可旋转。", MUTED, FS_SMALL, false);
    t += "}";
    s += &t;
    s += "}";
    s += &chart_panel_close(b);
    s
}

/// 三维数据图：规格登记到 chart3d，Splash 里只放一个按 id 取规格的 Chart3D。
fn chart3d_block(b: &Block) -> String {
    let spec: crate::chart3d::Spec = match serde_json::from_value(b.spec.clone()) {
        Ok(s) => s,
        Err(e) => return label(&format!("（图表规格解析失败：{} {e}）", b.id), ROSE, FS_SMALL, true),
    };
    crate::chart3d::register(&b.id, spec);
    let mut s = chart_panel_open(b);
    let _ = write!(s, "d3.Chart3D{{width: Fill height: {} chart: {}}}", b.height.unwrap_or(230.0), q(&b.id));
    s += &label("拖动图表可旋转视角。", MUTED, FS_SMALL - 0.5, true);
    s += &chart_panel_close(b);
    s
}

/// 分类矩阵（如「幅度 × 方向 → 检出 / 漏检」、覆盖矩阵）：着色单元格表。
/// rows: [[{"t": 文字, "c": teal|rose|amber|navy|grey}, ...], ...]
fn heat(b: &Block) -> String {
    #[derive(Deserialize, Default)]
    struct Cell {
        #[serde(default)]
        t: String,
        #[serde(default)]
        c: String,
    }
    let rows: Vec<Vec<Cell>> = serde_json::from_value(b.rows.clone()).unwrap_or_default();
    let mut s = chart_panel_open(b);
    let lw = 96;
    let mut head = format!("View{{width: Fill height: Fit flow: Right spacing: 4 View{{width: {lw} height: Fit}} ");
    for c in &b.col_labels {
        let _ = write!(head, "View{{width: Fill height: Fit align: Align{{x: 0.5}} {}}}", label(c, MUTED, FS_SMALL, false));
    }
    head += "}";
    s += &head;
    for (i, r) in rows.iter().enumerate() {
        let mut line = format!(
            "View{{width: Fill height: Fit flow: Right spacing: 4 align: Align{{y: 0.5}} View{{width: {lw} height: Fit {}}} ",
            label(b.row_labels.get(i).map(|s| s.as_str()).unwrap_or(""), INK2, FS_SMALL + 0.5, true)
        );
        for c in r {
            let (fg, bg) = match c.c.as_str() {
                "teal" => (TEAL, TEAL_T),
                "rose" => (ROSE, "#xF5E5E3"),
                "amber" => (AMBER, AMBER_T),
                "navy" => (NAVY, "#xE6EAF0"),
                _ => (MUTED, BEIGE_T),
            };
            let _ = write!(
                line,
                "RoundedView{{width: Fill height: 30 align: Align{{x: 0.5 y: 0.5}} new_batch: true draw_bg.color: {bg} draw_bg.border_size: 1.0 draw_bg.border_color: {fg} draw_bg.border_radius: 3.0 {}}}",
                label(&c.t, fg, FS_SMALL, false)
            );
        }
        line += "}";
        s += &line;
    }
    s += &chart_panel_close(b);
    s
}

/// 亮色 Splash 文本 → 暗色：按色值逐个替换（版式不变）。只用于幻灯片正文；结果浏览保持亮色面板。
pub fn to_dark(src: &str) -> String {
    const MAP: &[(&str, &str)] = &[
        ("202020", "E9E7E2"),
        ("383A3C", "D2D0CB"),
        ("85827D", "A39F98"),
        ("C4C4C2", "4A5058"),
        ("E2E0DA", "3A3F45"),
        ("FFFFFF", "23282E"),
        ("FAF8F4", "20252B"),
        ("33435D", "8EA4CC"),
        ("2E7D74", "4FB0A3"),
        ("E3F0EE", "1F3A36"),
        ("C76565", "E38C8C"),
        ("A8752A", "DDA75A"),
        ("F7EEDD", "3A2F1E"),
        ("8A8174", "B3AA9C"),
        ("F0EEEA", "2E3236"),
        ("E6EAF0", "263044"),
        ("F5E5E3", "3D2626"),
        ("FFFDF8", "2A2620"),
        ("E3CFA8", "6B5A3A"),
        ("EADCC0", "5A4C32"),
    ];
    let mut out = String::with_capacity(src.len());
    let b = src.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' && i + 8 <= b.len() && b[i + 1] == b'x' && b[i + 2..i + 8].iter().all(|c| c.is_ascii_hexdigit()) {
            let hex = &src[i + 2..i + 8];
            if let Some((_, d)) = MAP.iter().find(|(l, _)| l.eq_ignore_ascii_case(hex)) {
                out.push_str("#x");
                out.push_str(d);
                i += 8;
                continue;
            }
        }
        let ch = src[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn column(blocks: &[Block], weight: f64, bg: &str) -> String {
    let mut s = format!(
        "RoundedView{{width: Fill{{weight: {weight:.0}}} height: Fill new_batch: true draw_bg.color: {bg} draw_bg.border_size: 1.0 draw_bg.border_color: {LINE} draw_bg.border_radius: 6.0 \
         ScrollYView{{width: Fill height: Fill flow: Down spacing: 10 padding: Inset{{left: 14 right: 14 top: 12 bottom: 12}} "
    );
    for b in blocks {
        s += &block(b);
    }
    s += "}}";
    s
}

/// 一页正文的 Splash 文本（标题、页脚在外壳里）。
pub fn slide_body(sl: &Slide) -> String {
    let mut s = String::from("width: Fill height: Fill flow: Right spacing: 16 ");
    if sl.layout == "full" || sl.right.is_empty() {
        s += &column(&sl.left, 100.0, "#xFFFFFFF0");
    } else {
        s += &column(&sl.left, 130.0, "#xFFFFFFF0");
        s += &column(&sl.right, 100.0, "#xFAF8F4F2");
    }
    s
}

// ---------------- 给 explorer 和新内容类型用的公开小件 ----------------

pub fn q_pub(s: &str) -> String {
    q(s)
}
pub fn bold_pub(t: &str, c: &str, size: f64) -> String {
    bold(t, c, size, true)
}
pub fn label_pub(t: &str, c: &str, size: f64) -> String {
    label(t, c, size, true)
}
pub fn pending_pub(title: &str, what: &str, job: &str) -> String {
    pending(&Block { t: "pending".into(), title: title.into(), what: what.into(), job: job.into(), pending: true, ..Default::default() })
}
pub fn table_pub(title: &str, head: &[&str], w: &[f64], rows: &[Vec<String>]) -> String {
    table(&Block {
        t: "table".into(),
        title: title.into(),
        head: head.iter().map(|s| s.to_string()).collect(),
        w: w.to_vec(),
        rows: serde_json::to_value(rows).unwrap_or_default(),
        ..Default::default()
    })
}
pub fn forest_pub(title: &str, unit: &str, min: f64, max: f64, rows: &[ForestRow]) -> String {
    let lo = rows.iter().map(|r| r.lo).fold(min, f64::min);
    let hi = rows.iter().map(|r| r.hi).fold(max, f64::max);
    let m = lo.abs().max(hi.abs());
    forest(&Block {
        t: "forest".into(),
        title: title.into(),
        unit: unit.into(),
        min: Some(-m),
        max: Some(m),
        rows: serde_json::to_value(rows.iter().map(|r| serde_json::json!({"label": r.label, "est": r.est, "lo": r.lo, "hi": r.hi, "verdict": r.verdict})).collect::<Vec<_>>()).unwrap_or_default(),
        ..Default::default()
    })
}

const REF_COLORS: [&str; 3] = [AMBER, ROSE, NAVY];

/// 横向数值条 + 参考线（如原生自噪声、参照线）。items: (名称, 值, 是否高亮)。
pub fn refbars(title: &str, unit: &str, min: f64, max: f64, items: &[(String, f64, bool)], refs: &[(f64, String)]) -> String {
    let mut s = chart_panel_open(&Block { title: title.into(), ..Default::default() });
    let lo = min.min(items.iter().map(|i| i.1).fold(f64::INFINITY, f64::min));
    let hi = max.max(items.iter().map(|i| i.1).fold(f64::NEG_INFINITY, f64::max));
    let lines = |h: u32| -> String {
        let mut o = String::new();
        for (k, (v, _)) in refs.iter().enumerate() {
            let c = REF_COLORS[k % REF_COLORS.len()];
            let _ = write!(
                o,
                "View{{width: Fill height: {h} flow: Right {} SolidView{{width: 2 height: Fill draw_bg.color: {c}}} {}}}",
                seg(v.clamp(lo, hi) - lo, ""),
                seg(hi - v.clamp(lo, hi), "")
            );
        }
        o
    };
    // 刻度
    let _ = write!(
        s,
        "View{{width: Fill height: Fit flow: Right spacing: 8 View{{width: 170 height: Fit}} View{{width: Fill height: Fit flow: Right View{{width: Fill height: Fit {}}} View{{width: Fit height: Fit {}}}}} View{{width: 64 height: Fit}}}}",
        label(&format!("{}{}", fmt_num(lo), unit), MUTED, FS_SMALL, false),
        label(&format!("{}{}", fmt_num(hi), unit), MUTED, FS_SMALL, false)
    );
    for (name, v, hl) in items {
        let color = if *hl { NAVY } else { TEAL };
        let bar = format!(
            "View{{width: Fill height: 12 flow: Right align: Align{{y: 0.5}} {} {}}}",
            seg_solid(v.clamp(lo, hi) - lo, color),
            seg(hi - v.clamp(lo, hi), "")
        );
        let _ = write!(
            s,
            "View{{width: Fill height: Fit flow: Right spacing: 8 align: Align{{y: 0.5}} View{{width: 170 height: Fit {}}} View{{width: Fill height: 14 flow: Overlay align: Align{{y: 0.5}} {bar} {}}} View{{width: 64 height: Fit {}}}}}",
            label(name, if *hl { NAVY } else { INK2 }, FS_SMALL + 0.5, true),
            lines(14),
            bold(&format!("{:.2}", v), color, FS_SMALL + 0.5, false)
        );
    }
    // 图例
    let mut lg = String::from("View{width: Fill height: Fit flow: Right spacing: 14 padding: Inset{top: 4} ");
    for (k, (_, l)) in refs.iter().enumerate() {
        let c = REF_COLORS[k % REF_COLORS.len()];
        let _ = write!(lg, "View{{width: Fit height: Fit flow: Right spacing: 4 align: Align{{y: 0.5}} SolidView{{width: 14 height: 2 draw_bg.color: {c}}} {}}}", label(l, INK2, FS_SMALL, false));
    }
    lg += "}";
    s += &lg;
    s += "}";
    s
}

#[derive(Deserialize, Clone, Default)]
pub struct RefItem {
    pub label: String,
    pub v: f64,
}

fn refbars_block(b: &Block) -> String {
    let items: Vec<RefItem> = serde_json::from_value(b.items.clone()).unwrap_or_default();
    let refs: Vec<RefItem> = serde_json::from_value(b.rows.clone()).unwrap_or_default();
    let mut s = refbars(
        &b.title,
        &b.unit,
        b.min.unwrap_or(0.0),
        b.max.unwrap_or(1.0),
        &items.iter().map(|i| (i.label.clone(), i.v, false)).collect::<Vec<_>>(),
        &refs.iter().map(|r| (r.v, r.label.clone())).collect::<Vec<_>>(),
    );
    if !b.note.is_empty() {
        s += &label(&b.note, MUTED, FS_SMALL, true);
    }
    s
}
