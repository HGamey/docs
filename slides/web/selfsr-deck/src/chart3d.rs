//! 数据图（三维）：与背景场景同一套画法——CPU 正交投影、按深度排序、每个可见面画成平行四边形（bg.rs 的 DrawFace）。
//! 支持两类：
//! - `bars`：类别 × 系列的立体柱（单系列即普通柱状图；`col_norm` 时每列独立归一化，用于单位不同的指标对照）；
//!   可选对数轴、水平参照面（参照线）、柱顶数值。
//! - `scatter`：三维散点（带落地垂线与地面投影点），可选画出 (x 越小、y 越大) 的 Pareto 前沿。
//! 规格写在 content/deck.json 的 `chart3d` 块里，gen.rs 生成 Splash 时按 id 登记，这里按 id 取。
//! 拖动可旋转视角；进入页面时柱子从 0 长到数值（约 0.6 s），之后静止，不占帧。

use crate::bg::DrawFace;
use makepad_widgets::*;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.Chart3DBase = #(Chart3D::register_widget(vm))
    mod.widgets.Chart3D = set_type_default() do mod.widgets.Chart3DBase{
        width: Fill
        height: 240
        draw_text +: {
            text_style: theme.font_regular{font_size: 8.5}
        }
    }
    // Splash 正文只导入 mod.prelude.widgets；其中的 d3 是 mod.d3 的引用，挂在 d3 下即可在正文里写 d3.Chart3D
    mod.d3.Chart3D = mod.widgets.Chart3D
}

/// 全局主题（亮 / 暗），图表和 gen.rs 的配色都读它。
pub static DARK: AtomicBool = AtomicBool::new(false);
pub fn dark() -> bool {
    DARK.load(Ordering::Relaxed)
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct RefLine {
    pub v: f64,
    #[serde(default)]
    pub label: String,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Pt {
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub z: f64,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub c: String,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Spec {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub cats: Vec<String>,
    #[serde(default)]
    pub series: Vec<String>,
    /// [系列][类别]
    #[serde(default)]
    pub values: Vec<Vec<f64>>,
    /// 柱顶文字（与 values 同形）；不给则按数值格式化
    #[serde(default)]
    pub texts: Vec<Vec<String>>,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub log: bool,
    #[serde(default)]
    pub ymin: Option<f64>,
    #[serde(default)]
    pub ymax: Option<f64>,
    #[serde(default)]
    pub refs: Vec<RefLine>,
    /// 每个系列的颜色名（teal / navy / rose / amber / beige / grey）
    #[serde(default)]
    pub colors: Vec<String>,
    /// 单系列时按类别上色
    #[serde(default)]
    pub cat_colors: Vec<String>,
    #[serde(default)]
    pub col_norm: bool,
    #[serde(default)]
    pub show_values: Option<bool>,
    /// 类别标签每隔几个显示一个（直方图）
    #[serde(default)]
    pub label_every: usize,
    #[serde(default)]
    pub points: Vec<Pt>,
    #[serde(default)]
    pub x_label: String,
    #[serde(default)]
    pub y_label: String,
    #[serde(default)]
    pub z_label: String,
    #[serde(default)]
    pub x_log: bool,
    #[serde(default)]
    pub z_log: bool,
    #[serde(default)]
    pub front: bool,
    #[serde(default)]
    pub yaw: Option<f32>,
    #[serde(default)]
    pub pitch: Option<f32>,
    /// 世界坐标里柱高相对类别间距的比例（默认按类别数自动）
    #[serde(default)]
    pub depth: Option<f32>,
}

static SPECS: Mutex<Vec<(String, Spec)>> = Mutex::new(Vec::new());

pub fn register(id: &str, spec: Spec) {
    if let Ok(mut g) = SPECS.lock() {
        if let Some(e) = g.iter_mut().find(|(k, _)| k == id) {
            e.1 = spec;
        } else {
            g.push((id.to_string(), spec));
        }
    }
}

fn lookup(id: &str) -> Option<Spec> {
    SPECS.lock().ok()?.iter().find(|(k, _)| k == id).map(|(_, s)| s.clone())
}

// ---------------- 配色 ----------------

fn hexc(hex: u32, a: f32) -> Vec4f {
    vec4f(((hex >> 16) & 255) as f32 / 255.0, ((hex >> 8) & 255) as f32 / 255.0, (hex & 255) as f32 / 255.0, a)
}

/// 颜色名 → (亮, 暗)。与 gen.rs 的强调色一致；暗色版本提高明度，保证在深底上对比足够。
pub fn named(name: &str) -> u32 {
    let d = dark();
    let (l, k) = match name {
        "navy" => (0x33435D, 0x8EA4CC),
        "rose" => (0xC76565, 0xE38C8C),
        "amber" => (0xA8752A, 0xDDA75A),
        "beige" => (0x8A8174, 0xB3AA9C),
        "grey" => (0xA5A19A, 0x6F6B66),
        _ => (0x2E7D74, 0x4FB0A3),
    };
    if d {
        k
    } else {
        l
    }
}

struct Pal {
    ink: Vec4f,
    muted: Vec4f,
    grid: Vec4f,
    wall: Vec4f,
    floor: Vec4f,
    edge_dark: bool,
}

fn pal() -> Pal {
    if dark() {
        Pal {
            ink: hexc(0xE9E7E2, 1.0),
            muted: hexc(0xA39F98, 1.0),
            grid: hexc(0x4A5058, 1.0),
            wall: hexc(0x262B31, 0.92),
            floor: hexc(0x2E343B, 1.0),
            edge_dark: false,
        }
    } else {
        Pal {
            ink: hexc(0x202020, 1.0),
            muted: hexc(0x76736E, 1.0),
            grid: hexc(0xD3CFC7, 1.0),
            wall: hexc(0xF6F4F0, 0.95),
            floor: hexc(0xE8E5DF, 1.0),
            edge_dark: true,
        }
    }
}

// ---------------- 几何 ----------------

#[derive(Clone, Copy)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}
fn v3(x: f32, y: f32, z: f32) -> V3 {
    V3 { x, y, z }
}
impl V3 {
    fn add(self, o: V3) -> V3 {
        v3(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    fn mul(self, k: f32) -> V3 {
        v3(self.x * k, self.y * k, self.z * k)
    }
    fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

#[derive(Clone, Copy)]
struct Cam {
    sy: f32,
    cy: f32,
    sp: f32,
    cp: f32,
    scale: f32,
    off: Vec2f,
}
impl Cam {
    fn raw(&self, p: V3) -> (Vec2f, f32) {
        let xr = p.x * self.cy - p.z * self.sy;
        let zr = p.x * self.sy + p.z * self.cy;
        (vec2f(xr, -p.y * self.cp + zr * self.sp), zr * self.cp + p.y * self.sp)
    }
    fn proj(&self, p: V3) -> Vec2f {
        let (q, _) = self.raw(p);
        vec2f(self.off.x + q.x * self.scale, self.off.y + q.y * self.scale)
    }
    fn depth(&self, p: V3) -> f32 {
        self.raw(p).1
    }
    fn view(&self) -> V3 {
        v3(self.sy * self.cp, self.sp, self.cy * self.cp)
    }
}

enum Prim {
    /// 中心、三个半轴、填充色、描边色
    Box { c: V3, ax: [V3; 3], fill: Vec4f, line: Vec4f },
    /// 屏幕空间线段（世界坐标端点），宽度 px
    Line { a: V3, b: V3, w: f32, col: Vec4f },
}

struct Item {
    depth: f32,
    prim: Prim,
}

struct Label3 {
    p: V3,
    text: String,
    /// 0 = 左对齐，0.5 = 居中，1 = 右对齐
    ax: f32,
    /// 0 = 顶在锚点，1 = 底在锚点，0.5 = 垂直居中
    ay: f32,
    dx: f32,
    dy: f32,
    col: Vec4f,
    size: f32,
}

fn fmt_v(v: f64) -> String {
    let a = v.abs();
    let s = if a >= 100.0 || (v.fract().abs() < 1e-9 && a < 1e6) {
        return format!("{:.0}", v);
    } else if a >= 10.0 {
        format!("{:.1}", v)
    } else if a >= 1.0 {
        format!("{:.2}", v)
    } else {
        format!("{:.3}", v)
    };
    // 去掉小数尾部多余的 0
    let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if s == "-0" { "0".into() } else { s }
}

fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    if !(hi > lo) {
        return vec![lo];
    }
    let raw = (hi - lo) / n.max(1) as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * mag).find(|s| *s >= raw).unwrap_or(10.0 * mag);
    let mut t = (lo / step).ceil() * step;
    let mut out = Vec::new();
    while t <= hi + step * 1e-6 {
        out.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    out
}

fn log_ticks(lo: f64, hi: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut e = lo.log10().floor() as i32;
    // 跨度 ≥ 3 个数量级时只标 10 的整次幂，避免刻度挤在一起
    let mults: &[f64] = if hi / lo >= 999.0 { &[1.0] } else { &[1.0, 2.0, 5.0] };
    while 10f64.powi(e) <= hi * 1.0001 {
        for &m in mults {
            let v = m * 10f64.powi(e);
            if v >= lo * 0.9999 && v <= hi * 1.0001 {
                out.push(v);
            }
        }
        e += 1;
    }
    out
}

struct Axis {
    lo: f64,
    hi: f64,
    log: bool,
}
impl Axis {
    fn new(vals: impl Iterator<Item = f64>, min: Option<f64>, max: Option<f64>, log: bool, zero: bool) -> Axis {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for v in vals {
            if v.is_finite() && (!log || v > 0.0) {
                lo = lo.min(v);
                hi = hi.max(v);
            }
        }
        if !lo.is_finite() {
            lo = if log { 1.0 } else { 0.0 };
            hi = lo + 1.0;
        }
        if log {
            lo = min.unwrap_or(10f64.powf(lo.log10().floor()));
            hi = max.unwrap_or(10f64.powf(hi.log10().ceil()));
        } else {
            let span = (hi - lo).max(hi.abs() * 0.05).max(1e-9);
            lo = min.unwrap_or(if zero && lo >= 0.0 { 0.0 } else { lo - span * 0.08 });
            hi = max.unwrap_or(hi + span * 0.08);
        }
        if hi <= lo {
            hi = lo + 1.0;
        }
        Axis { lo, hi, log }
    }
    fn t(&self, v: f64) -> f32 {
        let r = if self.log {
            (v.max(1e-12).log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10())
        } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        r.clamp(0.0, 1.0) as f32
    }
    fn ticks(&self) -> Vec<f64> {
        if self.log {
            log_ticks(self.lo, self.hi)
        } else {
            nice_ticks(self.lo, self.hi, 4)
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct Chart3D {
    #[uid]
    uid: WidgetUid,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_face: DrawFace,
    #[live]
    draw_text: DrawText,
    /// deck.json 里 chart3d 块的登记 id
    #[live]
    chart: String,
    #[rust]
    area: Area,
    #[rust]
    yaw_off: f32,
    #[rust]
    pitch_off: f32,
    #[rust]
    grow: f32,
    #[rust]
    t0: Option<f64>,
    #[rust]
    next_frame: NextFrame,
    #[rust]
    drag: Option<DVec2>,
}

impl Widget for Chart3D {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Event::NextFrame(ne) = event {
            if ne.set.contains(&self.next_frame) {
                let t0 = *self.t0.get_or_insert(ne.time);
                let k = ((ne.time - t0) / 0.6).clamp(0.0, 1.0) as f32;
                self.grow = 1.0 - (1.0 - k).powi(3);
                self.area.redraw(cx);
                if k < 1.0 {
                    self.next_frame = cx.new_next_frame();
                }
            }
        }
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => cx.set_cursor(MouseCursor::Grab),
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                self.drag = Some(fe.abs);
                cx.set_cursor(MouseCursor::Grabbing);
            }
            Hit::FingerMove(fe) => {
                if let Some(p) = self.drag {
                    let d = fe.abs - p;
                    // 旋转范围收窄：大角度时类别轴近乎竖直，标签会叠在一起
                    self.yaw_off = (self.yaw_off + d.x as f32 * 0.005).clamp(-0.45, 0.45);
                    self.pitch_off = (self.pitch_off + d.y as f32 * 0.004).clamp(-0.25, 0.35);
                    self.drag = Some(fe.abs);
                    self.area.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.drag = None;
                cx.set_cursor(MouseCursor::Grab);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        if self.t0.is_none() && self.grow == 0.0 {
            self.next_frame = cx.new_next_frame();
        }
        if let Some(spec) = lookup(&self.chart) {
            self.draw_chart(cx, rect, &spec);
        }
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl Chart3D {
    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect, spec: &Spec) {
        let w = rect.size.x as f32;
        let h = rect.size.y as f32;
        if w < 40.0 || h < 40.0 {
            return;
        }
        let pal = pal();
        let mut items: Vec<Item> = Vec::new();
        let mut labels: Vec<Label3> = Vec::new();
        let scatter = spec.kind == "scatter";
        let g = self.grow.max(0.0);

        // ---- 世界坐标里的盒子尺寸 ----
        let (nx, nz) = if scatter { (4.2f32, 2.6f32) } else { (spec.cats.len().max(1) as f32, spec.series.len().max(1) as f32) };
        let hgt = if scatter { 2.4 } else { spec.depth.unwrap_or((nx.max(nz) * 0.42).clamp(1.6, 5.0)) };
        let yaw = spec.yaw.unwrap_or(if scatter { 0.55 } else if nz > 1.5 { 0.50 } else { 0.30 }) + self.yaw_off;
        let pitch = (spec.pitch.unwrap_or(0.42) + self.pitch_off).clamp(0.08, 1.1);
        let (sy, cy) = yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        let mut cam = Cam { sy, cy, sp, cp, scale: 1.0, off: vec2f(0.0, 0.0) };
        let view = cam.view();
        // 墙放在背离观察者的一侧
        let xs = if view.x > 0.0 { 0.0 } else { nx };
        let zb = if view.z > 0.0 { 0.0 } else { nz };
        let zf = nz - zb;
        let xe = nx - xs;

        // ---- 数值轴 ----
        let yax = if scatter {
            Axis::new(spec.points.iter().map(|p| p.y).chain(spec.refs.iter().map(|r| r.v)), spec.ymin, spec.ymax, spec.log, false)
        } else {
            Axis::new(spec.values.iter().flatten().cloned().chain(spec.refs.iter().map(|r| r.v)), spec.ymin, spec.ymax, spec.log, true)
        };
        let xax = Axis::new(spec.points.iter().map(|p| p.x), None, None, spec.x_log, false);
        let zax = Axis::new(spec.points.iter().map(|p| p.z), None, None, spec.z_log, true);

        // ---- 拟合：用盒子八个角 + 标签余量算缩放 ----
        let corners = [
            v3(0.0, 0.0, 0.0),
            v3(nx, 0.0, 0.0),
            v3(0.0, 0.0, nz),
            v3(nx, 0.0, nz),
            v3(0.0, hgt, 0.0),
            v3(nx, hgt, 0.0),
            v3(0.0, hgt, nz),
            v3(nx, hgt, nz),
        ];
        let mut mn = vec2f(f32::MAX, f32::MAX);
        let mut mx = vec2f(f32::MIN, f32::MIN);
        for c in corners {
            let (q, _) = cam.raw(c);
            mn.x = mn.x.min(q.x);
            mn.y = mn.y.min(q.y);
            mx.x = mx.x.max(q.x);
            mx.y = mx.y.max(q.y);
        }
        let has_series_lbl = !scatter && spec.series.len() > 1;
        let (ml, mr, mt, mb) = (52.0f32, if has_series_lbl || scatter { 96.0 } else { 18.0 }, 20.0f32, if scatter { 34.0 } else { 44.0 });
        let sc = ((w - ml - mr) / (mx.x - mn.x).max(1e-3)).min((h - mt - mb) / (mx.y - mn.y).max(1e-3));
        cam.scale = sc;
        let used_w = (mx.x - mn.x) * sc;
        let left = rect.pos.x as f32 + ml + ((w - ml - mr) - used_w) * 0.5;
        cam.off = vec2f(left - mn.x * sc, rect.pos.y as f32 + mt - mn.y * sc);

        // ---- 地面、墙、网格 ----
        let thick = 0.03;
        items.push(Item {
            depth: -1e6,
            prim: Prim::Box { c: v3(nx * 0.5, -thick, nz * 0.5), ax: [v3(nx * 0.5, 0.0, 0.0), v3(0.0, thick, 0.0), v3(0.0, 0.0, nz * 0.5)], fill: pal.floor, line: pal.grid },
        });
        items.push(Item {
            depth: -1e6 + 1.0,
            prim: Prim::Box { c: v3(nx * 0.5, hgt * 0.5, zb), ax: [v3(nx * 0.5, 0.0, 0.0), v3(0.0, hgt * 0.5, 0.0), v3(0.0, 0.0, 0.001)], fill: pal.wall, line: pal.grid },
        });
        items.push(Item {
            depth: -1e6 + 2.0,
            prim: Prim::Box { c: v3(xs, hgt * 0.5, nz * 0.5), ax: [v3(0.001, 0.0, 0.0), v3(0.0, hgt * 0.5, 0.0), v3(0.0, 0.0, nz * 0.5)], fill: pal.wall, line: pal.grid },
        });
        let yticks = if spec.col_norm { vec![] } else { yax.ticks() };
        for t in &yticks {
            let y = yax.t(*t) * hgt;
            items.push(Item { depth: -1e5, prim: Prim::Line { a: v3(0.0, y, zb), b: v3(nx, y, zb), w: 1.0, col: pal.grid } });
            items.push(Item { depth: -1e5, prim: Prim::Line { a: v3(xs, y, 0.0), b: v3(xs, y, nz), w: 1.0, col: pal.grid } });
            labels.push(Label3 { p: v3(xs, y, zf), text: fmt_v(*t), ax: 1.0, ay: 0.5, dx: -6.0, dy: 0.0, col: pal.muted, size: 8.0 });
        }
        if !spec.y_label.is_empty() {
            labels.push(Label3 { p: v3(xs, hgt, zf), text: spec.y_label.clone(), ax: 0.5, ay: 1.0, dx: 0.0, dy: -8.0, col: pal.muted, size: 8.5 });
        } else if !spec.unit.is_empty() && !spec.col_norm {
            labels.push(Label3 { p: v3(xs, hgt, zf), text: spec.unit.clone(), ax: 0.5, ay: 1.0, dx: 0.0, dy: -8.0, col: pal.muted, size: 8.5 });
        }
        // 参照面：半透明薄板 + 两面墙上的实线 + 标注
        for (k, r) in spec.refs.iter().enumerate() {
            let y = yax.t(r.v) * hgt;
            let c = named(["amber", "rose", "navy"][k % 3]);
            items.push(Item {
                depth: -5e4,
                prim: Prim::Box { c: v3(nx * 0.5, y, nz * 0.5), ax: [v3(nx * 0.5, 0.0, 0.0), v3(0.0, 0.002, 0.0), v3(0.0, 0.0, nz * 0.5)], fill: hexc(c, 0.16), line: hexc(c, 0.0) },
            });
            items.push(Item { depth: -4e4, prim: Prim::Line { a: v3(0.0, y, zb), b: v3(nx, y, zb), w: 2.0, col: hexc(c, 1.0) } });
            items.push(Item { depth: -4e4, prim: Prim::Line { a: v3(xs, y, 0.0), b: v3(xs, y, nz), w: 2.0, col: hexc(c, 1.0) } });
            items.push(Item { depth: 1e5, prim: Prim::Line { a: v3(xe, y, 0.0), b: v3(xe, y, nz), w: 1.5, col: hexc(c, 0.9) } });
            labels.push(Label3 { p: v3(xe, y, zf), text: format!("{} {}", r.label, fmt_v(r.v)), ax: 0.0, ay: 0.5, dx: 6.0, dy: 0.0, col: hexc(c, 1.0), size: 8.5 });
        }

        if scatter {
            self.scatter(spec, &cam, &xax, &yax, &zax, nx, nz, hgt, zf, xe, g, &pal, &mut items, &mut labels);
        } else {
            self.bars(spec, &cam, &yax, nx, nz, hgt, zf, xe, g, &pal, &mut items, &mut labels);
        }

        // ---- 画：按深度从远到近 ----
        items.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));
        let light = v3(-0.35, 0.85, 0.40);
        for it in &items {
            match &it.prim {
                Prim::Box { c, ax, fill, line } => {
                    for k in 0..3 {
                        for sgn in [1.0f32, -1.0] {
                            let n = ax[k].mul(sgn);
                            if n.dot(view) <= 1e-6 {
                                continue;
                            }
                            let a1 = ax[(k + 1) % 3];
                            let a2 = ax[(k + 2) % 3];
                            let o = c.add(n).add(a1.mul(-1.0)).add(a2.mul(-1.0));
                            let po = cam.proj(o);
                            let pa = cam.proj(o.add(a1.mul(2.0)));
                            let pb = cam.proj(o.add(a2.mul(2.0)));
                            let len = n.dot(n).sqrt().max(1e-6);
                            let lit = 0.70 + 0.30 * (n.dot(light) / len).max(0.0);
                            let f = vec4f(fill.x * lit, fill.y * lit, fill.z * lit, fill.w);
                            face(&mut self.draw_face, cx, po, pa - po, pb - po, f, *line);
                        }
                    }
                }
                Prim::Line { a, b, w, col } => {
                    let pa = cam.proj(*a);
                    let pb = cam.proj(*b);
                    let d = pb - pa;
                    let l = (d.x * d.x + d.y * d.y).sqrt().max(1e-3);
                    let nrm = vec2f(-d.y / l * w, d.x / l * w);
                    face(&mut self.draw_face, cx, pa - nrm * 0.5, d, nrm, *col, vec4f(0.0, 0.0, 0.0, 0.0));
                }
            }
        }
        // ---- 文字在最上层 ----
        for lb in &labels {
            let p = cam.proj(lb.p);
            self.draw_text.text_style.font_size = lb.size;
            self.draw_text.color = lb.col;
            let lay = self.draw_text.layout(cx, 0.0, 0.0, None, false, Align::default(), &lb.text);
            let tw = lay.size_in_lpxs.width;
            let th = lay.size_in_lpxs.height;
            let x = p.x + lb.dx - tw * lb.ax;
            let y = p.y + lb.dy - th * lb.ay;
            self.draw_text.draw_abs(cx, dvec2(x as f64, y as f64), &lb.text);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn bars(&self, spec: &Spec, cam: &Cam, yax: &Axis, nx: f32, nz: f32, hgt: f32, zf: f32, xe: f32, g: f32, pal: &Pal, items: &mut Vec<Item>, labels: &mut Vec<Label3>) {
        let ncat = spec.cats.len();
        let nser = spec.series.len().max(1);
        let many = ncat > 12;
        let show_values = spec.show_values.unwrap_or(!many);
        let bw = if many { 0.42 } else { 0.32 };
        let bd = if nser > 1 { 0.30 } else { 0.32 };
        // 每列独立归一化时的范围
        let col_rng: Vec<(f64, f64)> = (0..ncat)
            .map(|c| {
                let vs: Vec<f64> = (0..nser).filter_map(|s| spec.values.get(s).and_then(|r| r.get(c)).cloned()).filter(|v| v.is_finite()).collect();
                let lo = vs.iter().cloned().fold(f64::INFINITY, f64::min);
                let hi = vs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                (lo, hi)
            })
            .collect();
        for s in 0..nser {
            // 第一个系列在最前排
            let zc = if zf > 0.0 { nz - 0.5 - s as f32 } else { 0.5 + s as f32 };
            let scol = spec.colors.get(s).map(|c| named(c)).unwrap_or(named(["teal", "navy", "amber", "rose", "beige"][s % 5]));
            for c in 0..ncat {
                let Some(v) = spec.values.get(s).and_then(|r| r.get(c)).cloned() else { continue };
                if !v.is_finite() {
                    continue;
                }
                let t = if spec.col_norm {
                    let (lo, hi) = col_rng[c];
                    if hi > lo {
                        0.12 + 0.88 * ((v - lo) / (hi - lo)) as f32
                    } else {
                        0.6
                    }
                } else {
                    yax.t(v)
                };
                let top = (t * hgt * g).max(0.004);
                let col = if nser == 1 { spec.cat_colors.get(c).map(|n| named(n)).unwrap_or(scol) } else { scol };
                let center = v3(c as f32 + 0.5, top * 0.5, zc);
                let edge = if pal.edge_dark { hexc(0x202020, 0.35) } else { hexc(0xFFFFFF, 0.22) };
                items.push(Item { depth: cam.depth(center), prim: Prim::Box { c: center, ax: [v3(bw, 0.0, 0.0), v3(0.0, top * 0.5, 0.0), v3(0.0, 0.0, bd)], fill: hexc(col, 1.0), line: edge } });
                if show_values && g > 0.95 {
                    let txt = spec
                        .texts
                        .get(s)
                        .and_then(|r| r.get(c))
                        .cloned()
                        .unwrap_or_else(|| if spec.col_norm || spec.unit.is_empty() { fmt_v(v) } else { format!("{}{}", fmt_v(v), spec.unit) });
                    labels.push(Label3 { p: v3(c as f32 + 0.5, top, zc), text: txt, ax: 0.5, ay: 1.0, dx: 0.0, dy: -3.0, col: pal.ink, size: 8.5 });
                }
            }
        }
        let every = if spec.label_every > 0 { spec.label_every } else if many { (ncat / 8).max(1) } else { 1 };
        // 相邻类别标签在屏幕上的间距；按每字约 9 px 估算标签宽度，放不下时奇数项下移一行
        let gap = if ncat > 1 {
            let a = cam.proj(v3(0.5, 0.0, zf));
            let b = cam.proj(v3(1.5, 0.0, zf));
            ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt() * every as f32
        } else {
            f32::MAX
        };
        let widest = spec.cats.iter().map(|c| c.chars().map(|ch| if ch.is_ascii() { 5.0 } else { 9.0 }).sum::<f32>()).fold(0.0, f32::max);
        // 类别轴在屏幕上明显倾斜（相邻类别竖直落差 ≥ 13 px）时，标签右对齐放在轴线左下方，逐个下移，不会互相覆盖；
        // 否则居中放在柱下，放不下时奇偶交错两行。
        let (steep, step_dy) = if ncat > 1 {
            let a = cam.proj(v3(0.5, 0.0, zf));
            let b = cam.proj(v3(1.5, 0.0, zf));
            ((b.y - a.y).abs() * every as f32 >= 13.0, b.y - a.y)
        } else {
            (false, 0.0)
        };
        let stagger = !steep && widest > gap * 0.92;
        for (c, name) in spec.cats.iter().enumerate() {
            if c % every != 0 && c + 1 != ncat {
                continue;
            }
            if steep {
                let (ax, dx) = if step_dy > 0.0 { (1.0, -10.0) } else { (0.0, 10.0) };
                labels.push(Label3 { p: v3(c as f32 + 0.5, 0.0, zf + 0.3), text: name.clone(), ax, ay: 0.0, dx, dy: 2.0, col: pal.ink, size: 8.5 });
            } else {
                let dy = if stagger && (c / every) % 2 == 1 { 19.0 } else { 6.0 };
                labels.push(Label3 { p: v3(c as f32 + 0.5, 0.0, zf), text: name.clone(), ax: 0.5, ay: 0.0, dx: 0.0, dy, col: pal.ink, size: 8.5 });
            }
        }
        if nser > 1 {
            for (s, name) in spec.series.iter().enumerate() {
                let zc = if zf > 0.0 { nz - 0.5 - s as f32 } else { 0.5 + s as f32 };
                let scol = spec.colors.get(s).map(|c| named(c)).unwrap_or(named(["teal", "navy", "amber", "rose", "beige"][s % 5]));
                labels.push(Label3 { p: v3(xe, 0.0, zc), text: name.clone(), ax: 0.0, ay: 0.5, dx: 8.0, dy: 0.0, col: hexc(scol, 1.0), size: 8.5 });
            }
        }
        if !spec.x_label.is_empty() {
            labels.push(Label3 { p: v3(nx * 0.5, 0.0, zf), text: spec.x_label.clone(), ax: 0.5, ay: 0.0, dx: 0.0, dy: if stagger { 34.0 } else { 21.0 }, col: pal.muted, size: 8.0 });
        }
        let _ = hgt;
    }

    #[allow(clippy::too_many_arguments)]
    fn scatter(&self, spec: &Spec, cam: &Cam, xax: &Axis, yax: &Axis, zax: &Axis, nx: f32, nz: f32, hgt: f32, zf: f32, xe: f32, g: f32, pal: &Pal, items: &mut Vec<Item>, labels: &mut Vec<Label3>) {
        let pos = |p: &Pt| v3(xax.t(p.x) * nx, yax.t(p.y) * hgt, zax.t(p.z) * nz);
        // 地面网格（x、z 刻度）
        for t in xax.ticks() {
            let x = xax.t(t) * nx;
            items.push(Item { depth: -9e4, prim: Prim::Line { a: v3(x, 0.0, 0.0), b: v3(x, 0.0, nz), w: 1.0, col: pal.grid } });
            labels.push(Label3 { p: v3(x, 0.0, zf), text: fmt_v(t), ax: 0.5, ay: 0.0, dx: 0.0, dy: 5.0, col: pal.muted, size: 8.0 });
        }
        for t in zax.ticks() {
            let z = zax.t(t) * nz;
            items.push(Item { depth: -9e4, prim: Prim::Line { a: v3(0.0, 0.0, z), b: v3(nx, 0.0, z), w: 1.0, col: pal.grid } });
            labels.push(Label3 { p: v3(xe, 0.0, z), text: fmt_v(t), ax: 0.0, ay: 0.5, dx: 6.0, dy: 0.0, col: pal.muted, size: 8.0 });
        }
        if !spec.x_label.is_empty() {
            labels.push(Label3 { p: v3(nx * 0.5, 0.0, zf), text: spec.x_label.clone(), ax: 0.5, ay: 0.0, dx: 0.0, dy: 18.0, col: pal.muted, size: 8.0 });
        }
        if !spec.z_label.is_empty() {
            labels.push(Label3 { p: v3(xe, 0.0, nz * 0.5), text: spec.z_label.clone(), ax: 0.0, ay: 0.5, dx: 34.0, dy: 14.0, col: pal.muted, size: 8.0 });
        }
        // Pareto 前沿：x 越小越好、y 越大越好
        let mut front: Vec<usize> = Vec::new();
        if spec.front {
            let mut idx: Vec<usize> = (0..spec.points.len()).collect();
            idx.sort_by(|a, b| spec.points[*a].x.partial_cmp(&spec.points[*b].x).unwrap_or(std::cmp::Ordering::Equal));
            let mut best = f64::NEG_INFINITY;
            for i in idx {
                if spec.points[i].y > best {
                    best = spec.points[i].y;
                    front.push(i);
                }
            }
        }
        let r = 0.075f32;
        for (i, p) in spec.points.iter().enumerate() {
            let on_front = front.contains(&i);
            let mut q = pos(p);
            q.y *= g;
            let cname = if !p.c.is_empty() { p.c.as_str() } else if on_front { "rose" } else { "teal" };
            let col = named(cname);
            // 落地垂线 + 地面投影
            items.push(Item { depth: cam.depth(v3(q.x, 0.0, q.z)) - 0.01, prim: Prim::Line { a: v3(q.x, 0.0, q.z), b: q, w: 1.0, col: hexc(col, 0.45) } });
            items.push(Item {
                depth: -8e4,
                prim: Prim::Box { c: v3(q.x, 0.001, q.z), ax: [v3(r * 0.8, 0.0, 0.0), v3(0.0, 0.0005, 0.0), v3(0.0, 0.0, r * 0.8)], fill: hexc(col, 0.30), line: hexc(col, 0.0) },
            });
            let rr = if on_front { r * 1.25 } else { r };
            let edge = if pal.edge_dark { hexc(0x202020, 0.4) } else { hexc(0xFFFFFF, 0.3) };
            items.push(Item { depth: cam.depth(q), prim: Prim::Box { c: q, ax: [v3(rr, 0.0, 0.0), v3(0.0, rr, 0.0), v3(0.0, 0.0, rr)], fill: hexc(col, 1.0), line: edge } });
            if !p.label.is_empty() && g > 0.95 {
                labels.push(Label3 { p: q, text: p.label.clone(), ax: 0.0, ay: 1.0, dx: 6.0, dy: -3.0, col: if on_front { hexc(named("rose"), 1.0) } else { pal.ink }, size: 8.0 });
            }
        }
        for w in front.windows(2) {
            let mut a = pos(&spec.points[w[0]]);
            let mut b = pos(&spec.points[w[1]]);
            a.y *= g;
            b.y *= g;
            let mid = v3((a.x + b.x) * 0.5, (a.y + b.y) * 0.5, (a.z + b.z) * 0.5);
            items.push(Item { depth: cam.depth(mid), prim: Prim::Line { a, b, w: 2.0, col: hexc(named("rose"), 0.85) } });
        }
    }
}

fn face(df: &mut DrawFace, cx: &mut Cx2d, o: Vec2f, ea: Vec2f, eb: Vec2f, fill: Vec4f, line: Vec4f) {
    let pts = [o, o + ea, o + eb, o + ea + eb];
    let mut mn = pts[0];
    let mut mx = pts[0];
    for p in &pts[1..] {
        mn.x = mn.x.min(p.x);
        mn.y = mn.y.min(p.y);
        mx.x = mx.x.max(p.x);
        mx.y = mx.y.max(p.y);
    }
    let pad = 2.0;
    let r = Rect { pos: dvec2((mn.x - pad) as f64, (mn.y - pad) as f64), size: dvec2((mx.x - mn.x + pad * 2.0) as f64, (mx.y - mn.y + pad * 2.0) as f64) };
    df.org = vec2f(o.x - r.pos.x as f32, o.y - r.pos.y as f32);
    df.ea = ea;
    df.eb = eb;
    df.fill = fill;
    df.line = line;
    df.draw_abs(cx, r);
}
