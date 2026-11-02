//! 背景三维场景：等轴视角的方块阵列，CPU 投影、按深度排序，每个可见面画成一个平行四边形。
//! 纯装饰，高度不代表任何数据。一个小人沿 8 字路线在方块顶面上行走（摆臂摆腿、随方块起伏上下），
//! 相机平移跟随它；翻页时相机转一个角度、人物沿路线快走一段、波形相位移动，作为过场。

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    set_type_default() do #(DrawFace::script_shader(vm)){
        ..mod.draw.DrawQuad
        pixel: fn() {
            let q = self.pos * self.rect_size - self.org
            let det = self.ea.x * self.eb.y - self.ea.y * self.eb.x
            let u = (q.x * self.eb.y - q.y * self.eb.x) / det
            let v = (self.ea.x * q.y - self.ea.y * q.x) / det
            let da = min(u, 1.0 - u) * abs(det) / max(length(self.eb), 0.001)
            let db = min(v, 1.0 - v) * abs(det) / max(length(self.ea), 0.001)
            let d = min(da, db)
            let cov = clamp(d + 0.5, 0.0, 1.0)
            let edge = clamp(1.2 - d, 0.0, 1.0) * self.line.w
            let c = mix(self.fill.xyz, self.line.xyz, edge)
            let a = cov * max(self.fill.w, edge)
            return vec4(c * a, a)
        }
    }

    mod.widgets.DeckBgBase = #(DeckBg::register_widget(vm))
    mod.widgets.DeckBg = set_type_default() do mod.widgets.DeckBgBase{
        width: Fill
        height: Fill
    }
}

#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawFace {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub org: Vec2f,
    #[live]
    pub ea: Vec2f,
    #[live]
    pub eb: Vec2f,
    #[live]
    pub fill: Vec4f,
    #[live]
    pub line: Vec4f,
}

#[derive(Script, ScriptHook, Widget)]
pub struct DeckBg {
    #[uid]
    uid: WidgetUid,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_face: DrawFace,
    #[rust]
    next_frame: NextFrame,
    #[rust]
    area: Area,
    #[rust]
    time: f64,
    #[rust]
    slide: f64,
    #[rust]
    pub goal: f64,
    #[rust]
    last_pos: Option<(f32, f32)>,
    #[rust]
    walk_dist: f32,
    #[rust]
    walker_y: f32,
}

const NX: i32 = 11;
const NZ: i32 = 8;

fn rgba(hex: u32, a: f32) -> Vec4f {
    vec4f(
        ((hex >> 16) & 255) as f32 / 255.0,
        ((hex >> 8) & 255) as f32 / 255.0,
        (hex & 255) as f32 / 255.0,
        a,
    )
}

fn shade(c: Vec4f, k: f32) -> Vec4f {
    vec4f(c.x * k, c.y * k, c.z * k, c.w)
}

/// 正交相机：偏航 + 俯仰 + 平移（跟随行走的人物）。
#[derive(Clone, Copy)]
struct Cam {
    sy: f32,
    cy: f32,
    sp: f32,
    cp: f32,
    scale: f32,
    center: Vec2f,
    pan: Vec3f,
}

impl Cam {
    fn proj(&self, p: Vec3f) -> (Vec2f, f32) {
        let x = p.x - self.pan.x;
        let y = p.y - self.pan.y;
        let z = p.z - self.pan.z;
        let xr = x * self.cy - z * self.sy;
        let zr = x * self.sy + z * self.cy;
        (
            vec2f(self.center.x + xr * self.scale, self.center.y + (-y * self.cp + zr * self.sp) * self.scale),
            zr * self.cp + y * self.sp,
        )
    }
    /// 指向观察者的方向（世界坐标）
    fn view(&self) -> Vec3f {
        vec3f(self.sy * self.cp, self.sp, self.cy * self.cp)
    }
}

/// 任意朝向的长方体：中心 + 三个半轴向量（世界坐标）。
struct Solid {
    c: Vec3f,
    ax: [Vec3f; 3],
    col: Vec4f,
    line: Vec4f,
    depth: f32,
}

fn v3(x: f32, y: f32, z: f32) -> Vec3f {
    vec3f(x, y, z)
}
fn add(a: Vec3f, b: Vec3f) -> Vec3f {
    v3(a.x + b.x, a.y + b.y, a.z + b.z)
}
fn mul(a: Vec3f, k: f32) -> Vec3f {
    v3(a.x * k, a.y * k, a.z * k)
}
fn dot(a: Vec3f, b: Vec3f) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
/// 先绕 x 轴转 pitch（摆臂/摆腿），再绕 y 轴转 yaw（朝向）。
fn rot(v: Vec3f, yaw: f32, pitch: f32) -> Vec3f {
    let (sa, ca) = pitch.sin_cos();
    let v = v3(v.x, v.y * ca - v.z * sa, v.y * sa + v.z * ca);
    let (sb, cb) = yaw.sin_cos();
    v3(v.x * cb + v.z * sb, v.y, -v.x * sb + v.z * cb)
}

/// 方块阵列在 (i, j) 处的高度；人物踩在方块顶面上，所以也用它。
fn wave_h(x: f32, z: f32, t: f32, s: f32) -> f32 {
    let ph = s * 0.9;
    let wave = ((x * 0.55 + t * 0.35 + ph).sin() * 0.5 + 0.5) * ((z * 0.45 - t * 0.22 + ph * 0.7).cos() * 0.5 + 0.5);
    0.12 + wave * 1.3
}

/// 人物沿方块顶面走的路线（8 字形），u 为路线参数。
fn path(u: f32) -> (f32, f32) {
    (3.6 * u.sin(), 2.4 * (2.0 * u).sin())
}

impl DeckBg {
    pub fn set_goal(&mut self, cx: &mut Cx, goal: usize) {
        self.goal = goal as f64;
        self.area.redraw(cx);
    }

    fn draw_scene(&mut self, cx: &mut Cx2d, rect: Rect) {
        let w = rect.size.x as f32;
        let h = rect.size.y as f32;
        if w < 10.0 || h < 10.0 {
            return;
        }
        let t = self.time as f32;
        let s = self.slide as f32;
        let dark = crate::chart3d::dark();
        // 底色（亮 / 暗主题）
        let bgc = if dark { rgba(0x16191C, 1.0) } else { rgba(0xF4F3EF, 1.0) };
        let p0 = vec2f(rect.pos.x as f32, rect.pos.y as f32);
        self.face(cx, p0, vec2f(w, 0.0), vec2f(0.0, h), bgc, rgba(0, 0.0));

        // ---- 人物位置：时间推动慢走，翻页时沿路线快走一段 ----
        let u = t * 0.09 + s * 0.85;
        let (wx, wz) = path(u);
        let (nx, nz) = path(u + 0.01);
        let yaw_w = (nx - wx).atan2(nz - wz);
        if let Some((lx, lz)) = self.last_pos {
            let d = ((wx - lx).powi(2) + (wz - lz).powi(2)).sqrt();
            if d < 1.0 {
                self.walk_dist += d;
            }
        }
        self.last_pos = Some((wx, wz));
        // 脚下方块（取最近的格子），高度平滑跟随，看起来像上下台阶
        let gi = (wx + (NX - 1) as f32 * 0.5).round().clamp(0.0, (NX - 1) as f32);
        let gj = (wz + (NZ - 1) as f32 * 0.5).round().clamp(0.0, (NZ - 1) as f32);
        let gx = gi - (NX - 1) as f32 * 0.5;
        let gz = gj - (NZ - 1) as f32 * 0.5;
        let ground = wave_h(gx, gz, t, s);
        self.walker_y = if self.walker_y == 0.0 { ground } else { self.walker_y + (ground - self.walker_y) * 0.18 };
        let wy = self.walker_y;

        // ---- 相机：偏航随页码转，同时平移跟随人物 ----
        let yaw = 0.62 + s * 0.21 + (t * 0.05).sin() * 0.05;
        let pitch: f32 = 0.56;
        let (sy, cy) = yaw.sin_cos();
        let (sp, cp) = pitch.sin_cos();
        let cam = Cam {
            sy,
            cy,
            sp,
            cp,
            scale: (w.min(h * 1.6)) / 15.5,
            center: vec2f(rect.pos.x as f32 + w * 0.70, rect.pos.y as f32 + h * 0.66),
            pan: v3(wx * 0.30, wy * 0.15, wz * 0.30),
        };

        let palette = if dark { [0x283037u32, 0x232A30, 0x2E2B27, 0x2B272B, 0x262E2A] } else { [0xE7EEF0u32, 0xDDE7EA, 0xEDE6DA, 0xE9E2E6, 0xE3ECE6] };
        let accent = if dark { [0x3F8F85u32, 0x5D6F8F, 0xA85C5C] } else { [0x2E7D74u32, 0x33435D, 0xC76565] };
        let line = if dark { rgba(0x8A94A0, 0.30) } else { rgba(0x5A6470, 0.55) };

        let mut solids: Vec<Solid> = Vec::new();
        let hs = 0.39f32;
        for i in 0..NX {
            for j in 0..NZ {
                let x = i as f32 - (NX - 1) as f32 * 0.5;
                let z = j as f32 - (NZ - 1) as f32 * 0.5;
                let hgt = wave_h(x, z, t, s);
                let k = ((i * 7 + j * 13) as usize) % palette.len();
                let mut col = rgba(palette[k], 0.92);
                let pick = ((i + j * 3 + (s.round() as i32) * 5) % 17) == 0;
                if pick {
                    col = rgba(accent[((i + j) as usize) % accent.len()], 0.80);
                }
                let c = v3(x, hgt * 0.5, z);
                let depth = cam.proj(c).1;
                solids.push(Solid { c, ax: [v3(hs, 0.0, 0.0), v3(0.0, hgt * 0.5, 0.0), v3(0.0, 0.0, hs)], col, line, depth });
            }
        }

        // ---- 人物：躯干、头、双臂、双腿，按步伐相位摆动 ----
        let phase = self.walk_dist * 6.5;
        let swing = phase.sin() * 0.65;
        let bob = phase.sin().abs() * 0.035;
        let base = v3(wx, wy, wz);
        let pline = if dark { rgba(0xD0D4D8, 0.5) } else { rgba(0x202020, 0.65) };
        let mut part = |solids: &mut Vec<Solid>, pivot: Vec3f, off: Vec3f, half: Vec3f, pitch: f32, col: u32| {
            let c = add(add(base, rot(pivot, yaw_w, 0.0)), rot(off, yaw_w, pitch));
            let ax = [rot(v3(half.x, 0.0, 0.0), yaw_w, pitch), rot(v3(0.0, half.y, 0.0), yaw_w, pitch), rot(v3(0.0, 0.0, half.z), yaw_w, pitch)];
            // 人物各部件的深度整体抬高一点，避免被脚下方块盖住
            let depth = cam.proj(c).1 + 0.6;
            solids.push(Solid { c, ax, col: rgba(col, 1.0), line: pline, depth });
        };
        let hip = 0.46 + bob;
        part(&mut solids, v3(-0.09, hip, 0.0), v3(0.0, -0.23, 0.0), v3(0.065, 0.23, 0.07), swing, 0x33435D);
        part(&mut solids, v3(0.09, hip, 0.0), v3(0.0, -0.23, 0.0), v3(0.065, 0.23, 0.07), -swing, 0x33435D);
        part(&mut solids, v3(0.0, hip + 0.21, 0.0), v3(0.0, 0.0, 0.0), v3(0.17, 0.22, 0.10), 0.0, 0x2E7D74);
        part(&mut solids, v3(-0.23, hip + 0.40, 0.0), v3(0.0, -0.18, 0.0), v3(0.05, 0.18, 0.05), -swing * 0.8, 0x2E7D74);
        part(&mut solids, v3(0.23, hip + 0.40, 0.0), v3(0.0, -0.18, 0.0), v3(0.05, 0.18, 0.05), swing * 0.8, 0x2E7D74);
        part(&mut solids, v3(0.0, hip + 0.58, 0.0), v3(0.0, 0.0, 0.0), v3(0.13, 0.13, 0.13), 0.0, 0xEDE6DA);
        // 影子：脚下一块薄片
        let sc = v3(wx, wy + 0.004, wz);
        solids.push(Solid {
            c: sc,
            ax: [v3(0.26, 0.0, 0.0), v3(0.0, 0.002, 0.0), v3(0.0, 0.0, 0.26)],
            col: rgba(0x202020, 0.16),
            line: rgba(0x202020, 0.0),
            depth: cam.proj(sc).1 + 0.55,
        });

        solids.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));
        let view = cam.view();
        let light = v3(-0.35, 0.85, 0.40);
        for so in &solids {
            for k in 0..3 {
                for sgn in [1.0f32, -1.0] {
                    let n = mul(so.ax[k], sgn);
                    if dot(n, view) <= 1e-5 {
                        continue;
                    }
                    let a1 = so.ax[(k + 1) % 3];
                    let a2 = so.ax[(k + 2) % 3];
                    let o = add(add(add(so.c, n), mul(a1, -1.0)), mul(a2, -1.0));
                    let (po, _) = cam.proj(o);
                    let (pa, _) = cam.proj(add(o, mul(a1, 2.0)));
                    let (pb, _) = cam.proj(add(o, mul(a2, 2.0)));
                    let len = dot(n, n).sqrt().max(1e-6);
                    let lit = 0.72 + 0.28 * (dot(n, light) / len).max(0.0);
                    self.face(cx, po, pa - po, pb - po, shade(so.col, lit), so.line);
                }
            }
        }
    }

    fn face(&mut self, cx: &mut Cx2d, o: Vec2f, ea: Vec2f, eb: Vec2f, fill: Vec4f, line: Vec4f) {
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
        let r = Rect {
            pos: dvec2((mn.x - pad) as f64, (mn.y - pad) as f64),
            size: dvec2((mx.x - mn.x + pad * 2.0) as f64, (mx.y - mn.y + pad * 2.0) as f64),
        };
        self.draw_face.org = vec2f(o.x - r.pos.x as f32, o.y - r.pos.y as f32);
        self.draw_face.ea = ea;
        self.draw_face.eb = eb;
        self.draw_face.fill = fill;
        self.draw_face.line = line;
        self.draw_face.draw_abs(cx, r);
    }
}

impl Widget for DeckBg {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Event::NextFrame(ne) = event {
            if ne.set.contains(&self.next_frame) {
                self.time = ne.time;
                self.slide = self.slide * 0.9 + self.goal * 0.1;
                if (self.slide - self.goal).abs() < 0.0005 {
                    self.slide = self.goal;
                }
                self.area.redraw(cx);
                self.next_frame = cx.new_next_frame();
            }
        }
        if matches!(event, Event::Startup) {
            self.next_frame = cx.new_next_frame();
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        self.draw_scene(cx, rect);
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}
