//! 结果浏览：排行榜、画面对比、真机效应量。数据来自 data/（refresh-data.sh 生成，编译时嵌入）。
//! 这里只出数字和图，不写「通过 / 不通过」之类的判词。

use crate::gen::{self, q_pub as q};
use serde_json::Value;
use std::fmt::Write;

include!(concat!(env!("OUT_DIR"), "/data_gen.rs"));

pub fn crop_bytes(name: &str) -> Option<&'static [u8]> {
    CROPS.iter().find(|(n, _)| *n == name).map(|(_, b)| *b)
}

fn num(v: &Value) -> Option<f64> {
    v.as_f64()
}

fn f(v: Option<f64>, d: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", d, x),
        _ => "—".into(),
    }
}

// ---------------- 排行榜 ----------------

#[derive(Clone, Default)]
pub struct LbRow {
    pub section: String,
    pub id: String,
    pub status: String,
    pub gmac: Option<f64>,
    pub npu_ms: Option<f64>,
    pub lat_kind: String,
    pub psnr: Option<f64>,
    pub margin: Option<f64>,
    pub dssim: Option<f64>,
    pub detail: Option<f64>,
    pub moving: Option<f64>,
    pub raw: Value,
}

pub const LB_COLS: [&str; 9] = ["编号", "状态", "GMAC", "NPU ms", "稳定块 PSNR", "距参照线 dB", "1−SSIM", "细节 log2", "在动 PSNR"];

pub struct Leaderboard {
    pub generated: String,
    pub sections: Vec<(String, String, Vec<LbRow>)>, // (key, protocol, rows)
}

pub fn load_leaderboard() -> Leaderboard {
    let d: Value = serde_json::from_str(LEADERBOARD_JSON).unwrap_or(Value::Null);
    let secs = d.get("sections").or_else(|| d.get("tasks")).cloned().unwrap_or(Value::Null);
    let mut out = Vec::new();
    if let Some(map) = secs.as_object() {
        for (key, sec) in map {
            let proto = sec.get("protocol").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let mut rows = Vec::new();
            for m in sec.get("models").and_then(|v| v.as_array()).cloned().unwrap_or_default() {
                let su = m.get("judge_same_ui").cloned().unwrap_or(Value::Null);
                let mv = m.get("judge_moving").cloned().unwrap_or(Value::Null);
                let cat = |k: &str| m.get("categories").and_then(|c| c.get(k)).and_then(|c| c.get("mean")).and_then(num);
                let mut psnr = su.get("psnr_med").and_then(num);
                if psnr.is_none() {
                    psnr = cat("interp.moving.psnr_db");
                }
                rows.push(LbRow {
                    section: key.clone(),
                    id: m.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    status: m.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    gmac: m.get("gmac").and_then(num),
                    npu_ms: m.get("npu_balanced_ms").and_then(num),
                    lat_kind: match m.get("latency_kind").and_then(|v| v.as_str()) {
                        Some("measured") => "实".into(),
                        Some("estimated") => "估".into(),
                        _ => "".into(),
                    },
                    psnr,
                    margin: su.get("m_psnr_med").and_then(num),
                    dssim: su.get("dssim_med").and_then(num),
                    detail: su.get("detail_med").and_then(num).or_else(|| cat("interp.detail_log2")),
                    moving: mv.get("psnr_med").and_then(num),
                    raw: m.clone(),
                });
            }
            out.push((key.clone(), proto, rows));
        }
    }
    Leaderboard { generated: d.get("generated").and_then(|v| v.as_str()).unwrap_or("").to_string(), sections: out }
}

pub fn sort_rows(rows: &mut [LbRow], col: usize, desc: bool) {
    let key = |r: &LbRow| -> Option<f64> {
        match col {
            2 => r.gmac,
            3 => r.npu_ms,
            4 => r.psnr,
            5 => r.margin,
            6 => r.dssim,
            7 => r.detail,
            8 => r.moving,
            _ => None,
        }
    };
    if col <= 1 {
        rows.sort_by(|a, b| if col == 0 { a.id.cmp(&b.id) } else { a.status.cmp(&b.status) });
        if desc {
            rows.reverse();
        }
        return;
    }
    rows.sort_by(|a, b| {
        let (x, y) = (key(a), key(b));
        match (x, y) {
            (Some(x), Some(y)) => {
                let o = x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal);
                if desc { o.reverse() } else { o }
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    });
}

pub fn row_cells(r: &LbRow) -> [String; 9] {
    let short: String = r.id.chars().take(30).collect();
    [
        short,
        r.status.clone(),
        f(r.gmac, 2),
        if r.npu_ms.is_some() { format!("{} {}", f(r.npu_ms, 2), r.lat_kind) } else { "—".into() },
        f(r.psnr, 2),
        f(r.margin, 2),
        f(r.dssim, 4),
        f(r.detail, 3),
        f(r.moving, 2),
    ]
}

/// 选中模型的明细（Splash 文本）：所有指标、按区域的均值±标准误、PSNR 与参照线对照条。
pub fn lb_detail(r: &LbRow, all: &[LbRow]) -> String {
    let mut s = String::from("width: Fill height: Fit flow: Down spacing: 10 ");
    s += &gen::bold_pub(&r.id, gen::INK, 12.0);
    let m = &r.raw;
    let txt = |k: &str| m.get(k).map(|v| if v.is_string() { v.as_str().unwrap().to_string() } else if v.is_null() { "—".into() } else { v.to_string() }).unwrap_or("—".into());
    let mut rows: Vec<Vec<String>> = vec![
        vec!["来源".into(), txt("source")],
        vec!["结构".into(), txt("arch")],
        vec!["训练数据".into(), txt("data")],
        vec!["参数量".into(), txt("params")],
        vec!["GMAC".into(), f(r.gmac, 3)],
        vec!["NPU 平衡档 ms".into(), format!("{} {}", f(r.npu_ms, 2), if r.lat_kind == "实" { "（真机实测）" } else if r.lat_kind == "估" { "（按 10-20 实测外推）" } else { "" })],
        vec!["评测目录".into(), txt("eval_dir")],
    ];
    for (lab, key) in [("同一瞬间、排除 UI", "judge_same_ui"), ("场景在动", "judge_moving")] {
        if let Some(j) = m.get(key).filter(|v| v.is_object()) {
            let g = |k: &str| j.get(k).and_then(num);
            let lim = match (g("psnr_med"), g("m_psnr_med")) { (Some(a), Some(b)) => Some(a - b), _ => None };
            rows.push(vec![format!("{lab}：样本数"), f(g("n"), 0)]);
            rows.push(vec![format!("{lab}：稳定块 PSNR 中位数 / 参照线"), format!("{} / {} dB（差 {}）", f(g("psnr_med"), 2), f(lim, 2), f(g("m_psnr_med"), 2))]);
            rows.push(vec![format!("{lab}：稳定块 1−SSIM 中位数"), format!("{}（距参照线 {}）", f(g("dssim_med"), 4), f(g("m_dssim_med"), 4))]);
            rows.push(vec![format!("{lab}：细节 log2 中位数"), format!("{}（距参照线 {}）", f(g("detail_med"), 3), f(g("m_detail_med"), 3))]);
        }
    }
    s += &gen::table_pub("指标", &["项", "值"], &[1.6, 3.0], &rows);
    // 区域
    if let Some(c) = m.get("categories").and_then(|v| v.as_object()) {
        let mut rr = Vec::new();
        for (k, v) in c {
            if k.ends_with(".tiles") || k.ends_with(".degraded_frac") {
                continue;
            }
            rr.push(vec![k.replace("region.", "").replace("interp.", ""), format!("{} ± {}", f(v.get("mean").and_then(num), 3), f(v.get("se").and_then(num), 3)), f(v.get("n").and_then(num), 0)]);
        }
        if !rr.is_empty() {
            s += &gen::table_pub("分区域（均值 ± 标准误）", &["指标", "值", "n"], &[2.0, 1.6, 0.5], &rr);
        }
    }
    // 同一节里所有模型的稳定块 PSNR，参考线 = 参照线（原生自噪声 − 1 dB）
    let mut items: Vec<(String, f64, bool)> = all.iter().filter_map(|x| x.psnr.map(|p| (x.id.chars().take(26).collect::<String>(), p, x.id == r.id))).collect();
    items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let lim = r.raw.get("judge_same_ui").and_then(|j| match (j.get("psnr_med").and_then(num), j.get("m_psnr_med").and_then(num)) { (Some(a), Some(b)) => Some(a - b), _ => None });
    let mut refs = Vec::new();
    if let Some(l) = lim {
        refs.push((l, format!("参照线 {:.1} dB（原生自噪声 − 1 dB）", l)));
    }
    refs.push((51.5, "原生对原生 51.5 dB（image-quality 中位数）".to_string()));
    s += &gen::refbars("本节所有模型：稳定块 PSNR（同一瞬间口径）", "dB", 30.0, 54.0, &items, &refs);
    // d3 散点：GMAC 对 PSNR
    let pts: Vec<String> = all.iter().filter_map(|x| match (x.gmac, x.psnr) { (Some(g), Some(p)) => Some(format!("[{:.3} {:.2}]", g, p)), _ => None }).collect();
    if !pts.is_empty() {
        let _ = write!(
            s,
            "RoundedView{{width: Fill height: Fit flow: Down spacing: 4 new_batch: true padding: 10 draw_bg.color: {} draw_bg.border_size: 1.0 draw_bg.border_color: {} draw_bg.border_radius: 4.0 {} d3.ScatterChart{{width: Fill height: 200 data: [{}] dot_color: {} dot_radius: 4.0}} {}}}",
            gen::PAPER, gen::LINE,
            gen::bold_pub("本节所有模型：计算量（横轴 GMAC）与稳定块 PSNR（纵轴 dB）", gen::INK, 10.5),
            pts.join(", "), gen::TEAL,
            gen::label_pub("makepad-d3 散点图；坐标轴文字目前不显示，数值见左侧表。", gen::MUTED, 9.0)
        );
    }
    s
}

// ---------------- 画面对比 ----------------

pub struct CropSet {
    pub items: Vec<Value>,
    pub meta: Value,
}

pub fn load_crops() -> CropSet {
    let d: Value = serde_json::from_str(CROPS_JSON).unwrap_or(Value::Null);
    CropSet { items: d.get("items").and_then(|v| v.as_array()).cloned().unwrap_or_default(), meta: d }
}

pub fn crop_title(it: &Value) -> String {
    format!(
        "帧 {} · ({}, {}) · {}",
        it.get("frame").and_then(|v| v.as_i64()).unwrap_or(0),
        it.get("x").and_then(|v| v.as_i64()).unwrap_or(0),
        it.get("y").and_then(|v| v.as_i64()).unwrap_or(0),
        it.get("why").and_then(|v| v.as_str()).unwrap_or("")
    )
}

/// 选中裁剪块的数值说明：每个候选的裁剪块 PSNR，参考线 = 原生自噪声与 iq-judge 参照线。
pub fn crop_chart(it: &Value, meta: &Value) -> String {
    let mut s = String::from("width: Fill height: Fit flow: Down spacing: 8 ");
    let items: Vec<(String, f64, bool)> = it
        .get("cands")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|c| Some((c.get("label")?.as_str()?.to_string(), c.get("psnr")?.as_f64()?, false))).collect())
        .unwrap_or_default();
    let r = meta.get("ref").cloned().unwrap_or(Value::Null);
    let mut refs = Vec::new();
    if let Some(v) = r.get("native_pairwise_stable_db_median").and_then(num) {
        refs.push((v, format!("原生对原生 {:.1} dB", v)));
    }
    if let Some(v) = r.get("same_instant_threshold_db").and_then(num) {
        refs.push((v, format!("iq-judge 参照线 {:.0} dB", v)));
    }
    s += &gen::refbars("这一块：候选对原生的 PSNR", "dB", 25.0, 56.0, &items, &refs);
    let note = r.get("note").and_then(|v| v.as_str()).unwrap_or("");
    s += &gen::label_pub(
        &format!(
            "裁剪 {}×{} 像素，差值图放大 {} 倍。{}来源：{}。",
            meta.get("crop_w").and_then(|v| v.as_i64()).unwrap_or(0),
            meta.get("crop_h").and_then(|v| v.as_i64()).unwrap_or(0),
            meta.get("diff_gain").and_then(|v| v.as_i64()).unwrap_or(0),
            note,
            meta.get("frames_src").and_then(|v| v.as_str()).unwrap_or("")
        ),
        gen::MUTED,
        9.0,
    );
    s
}

// ---------------- 效应量 ----------------

pub fn effects_body() -> String {
    let d: Value = serde_json::from_str(EFFECTS_JSON).unwrap_or(Value::Null);
    let mut s = String::from("width: Fill height: Fit flow: Down spacing: 12 ");
    let res = d.get("results").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    if res.is_empty() {
        s += &gen::pending_pub("真机判定结果", "每个实验的效应、95% 区间、p 和对数。", "refresh-data.sh 找到真机 verdict.json 后自动出现（演练运行不计入）。");
    }
    for r in &res {
        let title = format!(
            "{} · {} / {} · {}（{} 对）",
            r.get("source").and_then(|v| v.as_str()).unwrap_or(""),
            r.get("run").and_then(|v| v.as_str()).unwrap_or(""),
            r.get("case").and_then(|v| v.as_str()).unwrap_or(""),
            r.get("compare").and_then(|v| v.as_str()).unwrap_or(""),
            r.get("n_pairs").map(|v| v.to_string()).unwrap_or("?".into())
        );
        let ms = r.get("metrics").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let mut rows = Vec::new();
        for m in &ms {
            let g = |k: &str| m.get(k).and_then(num);
            rows.push(vec![
                m.get("metric_zh").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                f(g("a_mean"), 3),
                f(g("b_mean"), 3),
                format!("{} ({}%)", f(g("diff"), 3), f(g("diff_pct"), 2)),
                format!("[{}, {}]", f(g("ci_lo"), 3), f(g("ci_hi"), 3)),
                f(g("p"), 4),
                m.get("n").map(|v| v.to_string()).unwrap_or("—".into()),
            ]);
        }
        s += &gen::table_pub(&title, &["指标", "A 均值", "B 均值", "B−A（%）", "95% 区间", "p", "n"], &[1.6, 0.9, 0.9, 1.3, 1.5, 0.8, 0.4], &rows);
        // 每个指标一条森林图（按相对变化 %，便于放在同一坐标里）
        let fr: Vec<gen::ForestRow> = ms
            .iter()
            .filter_map(|m| {
                let a = m.get("a_mean")?.as_f64()?;
                if a == 0.0 {
                    return None;
                }
                Some(gen::ForestRow {
                    label: m.get("metric_zh")?.as_str()?.to_string(),
                    est: m.get("diff")?.as_f64()? / a * 100.0,
                    lo: m.get("ci_lo")?.as_f64()? / a * 100.0,
                    hi: m.get("ci_hi")?.as_f64()? / a * 100.0,
                    verdict: format!("p = {}", f(m.get("p").and_then(num), 4)),
                })
            })
            .collect();
        s += &gen::forest_pub("B−A 相对变化与 95% 区间（%）", "%", -15.0, 15.0, &fr);
    }
    let mde = d.get("mde_power").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    if !mde.is_empty() {
        let rows: Vec<Vec<String>> = mde
            .iter()
            .map(|m| {
                let g = |k: &str| m.get(k).and_then(num);
                vec![
                    f(g("pairs"), 0),
                    f(g("windows"), 0),
                    f(g("base_w"), 3),
                    f(g("pair_diff_sd_w"), 3),
                    if g("mde80_pct").is_some() { format!("{}% / {} W", f(g("mde80_pct"), 1), f(g("mde80_w"), 3)) } else { "≤20% 内未达 80% 检出".into() },
                    f(g("false_pos_rate_at_0"), 2),
                    f(m.get("sampler").and_then(|s| s.get("hz_median")).and_then(num), 2),
                ]
            })
            .collect();
        s += &gen::table_pub(
            "功耗最小可分辨效应（两臂相同的考题上注入已知差值回放）",
            &["对数", "窗口数", "基线 W", "配对差标准差 W", "80% 检出所需效应", "零效应误报率", "采样 Hz"],
            &[0.6, 0.7, 0.8, 1.1, 1.6, 1.0, 0.7],
            &rows,
        );
    }
    let sk = d.get("skipped_dry_runs").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    s += &gen::label_pub(&format!("未计入的演练 / 模拟设备运行：{} 个。数据生成时间见 manifest。", sk), gen::MUTED, 9.0);
    s
}

pub fn manifest_note() -> String {
    let d: Value = serde_json::from_str(MANIFEST_JSON).unwrap_or(Value::Null);
    format!("数据生成：{}；来源文件 {} 个（sha256 见 data/manifest.json）", d.get("generated").and_then(|v| v.as_str()).unwrap_or("—"), d.get("sources").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0))
}

#[allow(dead_code)]
fn _unused() -> String {
    q("")
}
