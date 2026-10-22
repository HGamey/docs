//! HGamey 进展网页幻灯片。外壳（标题栏、底栏、总览、三维背景）在这里；
//! 每页正文由 gen.rs 从 content/deck.json 生成 Splash 文本，交给 d3.Splash 求值。

pub use makepad_widgets;
use makepad_widgets::*;

mod bg;
mod chart3d;
mod explorer;
mod gen;

use bg::DeckBg;
use gen::Deck;

app_main!(App);

/// 内容文件在编译时嵌入；改数字只改 content/deck.json，重新构建即可。
const DECK_JSON: &str = include_str!("../content/deck.json");
const OV_SLOTS: usize = 24;
const LB_SLOTS: usize = 30;
const SP_SLOTS: usize = 16;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let NavButton = ButtonFlat{
        width: Fit
        height: 30
        padding: Inset{left: 14 right: 14}
        draw_text.color: #x383A3C
        draw_text.color_hover: #x2E7D74
        draw_text.color_down: #x2E7D74
        draw_text.color_focus: #x383A3C
        draw_bg.color: #x80808014
        draw_bg.color_focus: #x80808014
        draw_bg.color_hover: #x2E7D7426
        draw_bg.color_down: #x2E7D7440
        draw_bg.border_color: #x8C8C8C55
        draw_bg.border_color_hover: #x2E7D7499
        draw_bg.border_color_down: #x2E7D7499
        draw_bg.border_color_focus: #x8C8C8C55
        draw_text.text_style.font_size: 11
    }
    // 暗色主题的底栏按钮（按钮文字色无法在运行时安全改写，故亮 / 暗各一套底栏，按主题切换可见性）
    let NavButtonDark = ButtonFlat{
        width: Fit
        height: 30
        padding: Inset{left: 14 right: 14}
        draw_text.color: #xD2D0CB
        draw_text.color_hover: #x7FD0C4
        draw_text.color_down: #x7FD0C4
        draw_text.color_focus: #xD2D0CB
        draw_text.text_style.font_size: 11
        draw_bg.color: #xFFFFFF0A
        draw_bg.color_focus: #xFFFFFF0A
        draw_bg.color_hover: #x4FB0A333
        draw_bg.color_down: #x4FB0A355
        draw_bg.border_color: #x5A606888
        draw_bg.border_color_hover: #x4FB0A399
        draw_bg.border_color_down: #x4FB0A399
        draw_bg.border_color_focus: #x5A606888
    }
    let TabBtn = ButtonFlat{
        width: Fit
        height: 30
        padding: Inset{left: 12 right: 12}
        draw_text.color: #x33435D
        draw_text.color_hover: #x33435D
        draw_text.color_down: #x33435D
        draw_text.color_focus: #x33435D
        draw_bg.color_hover: #xE3ECEA
        draw_bg.color_down: #xD5E3E0
        draw_text.text_style.font_size: 10.5
    }
    let SortBtn = ButtonFlat{
        width: Fit
        height: 26
        padding: Inset{left: 8 right: 8}
        draw_text.color: #x2E7D74
        draw_text.color_hover: #x2E7D74
        draw_text.color_down: #x2E7D74
        draw_text.color_focus: #x2E7D74
        draw_bg.color_hover: #xE3ECEA
        draw_bg.color_down: #xD5E3E0
        draw_text.text_style.font_size: 9.5
    }
    let SpotBtn = ButtonFlat{
        width: Fill
        height: 30
        visible: false
        draw_text.color: #x383A3C
        draw_text.color_hover: #x383A3C
        draw_text.color_down: #x383A3C
        draw_text.color_focus: #x383A3C
        draw_bg.color_hover: #xE3ECEA
        draw_bg.color_down: #xD5E3E0
        draw_text.text_style.font_size: 9.5
    }
    let OvCard = ButtonFlat{
        width: Fill
        height: 58
        visible: false
        draw_text.color: #x202020
        draw_text.color_hover: #x202020
        draw_text.color_down: #x202020
        draw_text.color_focus: #x202020
        draw_bg.color_hover: #xE3ECEA
        draw_bg.color_down: #xD5E3E0
        draw_text.text_style.font_size: 10
    }
    let LbCell = Label{
        text: ""
        draw_text.color: #x383A3C
        draw_text.text_style.font_size: 9.5
    }
    let LbRow = View{
        width: Fill
        height: 24
        flow: Overlay
        visible: false
        sel := ButtonFlat{width: Fill height: Fill text: ""}
        cells := View{width: Fill height: Fill flow: Right spacing: 6 align: Align{y: 0.5} padding: Inset{left: 6 right: 6}
            c0 := LbCell{width: Fill{weight: 300}} c1 := LbCell{width: Fill{weight: 90}} c2 := LbCell{width: Fill{weight: 70}} c3 := LbCell{width: Fill{weight: 90}} c4 := LbCell{width: Fill{weight: 100}} c5 := LbCell{width: Fill{weight: 90}} c6 := LbCell{width: Fill{weight: 90}} c7 := LbCell{width: Fill{weight: 90}} c8 := LbCell{width: Fill{weight: 90}}
        }
    }
    let CropBox = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 2
        im := Image{width: Fill height: 150 fit: ImageFit.Smallest}
        t := Label{text: "" draw_text.color: #x85827D draw_text.text_style.font_size: 9}
    }
    let Panel = RoundedView{
        width: Fill
        height: Fill
        new_batch: true
        draw_bg.color: #xFFFFFFF0
        draw_bg.border_size: 1.0
        draw_bg.border_color: #xC4C4C2
        draw_bg.border_radius: 6.0
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(1600, 940)
                window.title: "HGamey 进展"
                pass.clear_color: #xF4F3EF
                body +: {
                    flow: Overlay
                    width: Fill
                    height: Fill

                    bg := DeckBg{}

                    View{
                        width: Fill
                        height: Fill
                        flow: Down

                        deck_view := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 10
                            padding: Inset{left: 40 right: 40 top: 20 bottom: 6}

                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 4
                                chapter := Label{text: "" draw_text.color: #x2E7D74 draw_text.text_style.font_size: 10}
                                title := H2{width: Fill text: "" draw_text.color: #x202020 draw_text.text_style.font_size: 20}
                                subtitle := Label{width: Fill text: "" draw_text.color: #x383A3C draw_text.text_style.font_size: 11.5}
                            }
                            sep1 := SolidView{width: Fill height: 1 draw_bg.color: #x8C8C8C66}

                            slide := d3.Splash{width: Fill height: Fill}

                            sep2 := SolidView{width: Fill height: 1 draw_bg.color: #x8C8C8C66}
                            foot := Label{width: Fill text: "" draw_text.color: #x85827D draw_text.text_style.font_size: 9}
                        }

                        explorer := SolidView{
                            visible: false
                            draw_bg.color: #xF4F3EF
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 8
                            padding: Inset{left: 32 right: 32 top: 16 bottom: 6}

                            View{width: Fill height: Fit flow: Right spacing: 6 align: Align{y: 0.5}
                                H2{width: Fit text: "结果浏览" draw_text.color: #x202020 draw_text.text_style.font_size: 18}
                                View{width: 20 height: 1}
                                tab_lb := TabBtn{text: "模型排行榜"}
                                tab_img := TabBtn{text: "画面对比"}
                                tab_eff := TabBtn{text: "真机效应量"}
                                View{width: Fill height: 1}
                                data_note := Label{text: "" draw_text.color: #x85827D draw_text.text_style.font_size: 9}
                            }
                            SolidView{width: Fill height: 1 draw_bg.color: #xC4C4C2}

                            lb_panel := View{width: Fill height: Fill flow: Right spacing: 14
                                Panel{width: Fill{weight: 150}
                                    ScrollYView{width: Fill height: Fill flow: Down spacing: 2 padding: 12
                                        View{width: Fill height: Fit flow: Right spacing: 6 align: Align{y: 0.5}
                                            Label{text: "分组" draw_text.color: #x85827D draw_text.text_style.font_size: 9.5}
                                            sec0 := TabBtn{visible: false} sec1 := TabBtn{visible: false} sec2 := TabBtn{visible: false} sec3 := TabBtn{visible: false}
                                            View{width: Fill height: 1}
                                            lb_info := Label{text: "" draw_text.color: #x85827D draw_text.text_style.font_size: 9}
                                        }
                                        View{width: Fill height: Fit flow: Right spacing: 2 align: Align{y: 0.5}
                                            Label{text: "排序（再点一次反向）" draw_text.color: #x85827D draw_text.text_style.font_size: 9.5}
                                            sort0 := SortBtn{text: "编号"} sort1 := SortBtn{text: "状态"} sort2 := SortBtn{text: "GMAC"} sort3 := SortBtn{text: "NPU ms"} sort4 := SortBtn{text: "稳定块 PSNR"} sort5 := SortBtn{text: "距参照线"} sort6 := SortBtn{text: "1−SSIM"} sort7 := SortBtn{text: "细节"} sort8 := SortBtn{text: "在动 PSNR"}
                                        }
                                        lbh := LbRow{visible: true}
                                        SolidView{width: Fill height: 1 draw_bg.color: #xC4C4C2}
                                    lb0 := LbRow{}
                                    lb1 := LbRow{}
                                    lb2 := LbRow{}
                                    lb3 := LbRow{}
                                    lb4 := LbRow{}
                                    lb5 := LbRow{}
                                    lb6 := LbRow{}
                                    lb7 := LbRow{}
                                    lb8 := LbRow{}
                                    lb9 := LbRow{}
                                    lb10 := LbRow{}
                                    lb11 := LbRow{}
                                    lb12 := LbRow{}
                                    lb13 := LbRow{}
                                    lb14 := LbRow{}
                                    lb15 := LbRow{}
                                    lb16 := LbRow{}
                                    lb17 := LbRow{}
                                    lb18 := LbRow{}
                                    lb19 := LbRow{}
                                    lb20 := LbRow{}
                                    lb21 := LbRow{}
                                    lb22 := LbRow{}
                                    lb23 := LbRow{}
                                    lb24 := LbRow{}
                                    lb25 := LbRow{}
                                    lb26 := LbRow{}
                                    lb27 := LbRow{}
                                    lb28 := LbRow{}
                                    lb29 := LbRow{}
                                    }
                                }
                                Panel{width: Fill{weight: 100}
                                    ScrollYView{width: Fill height: Fill flow: Down padding: 12
                                        lb_detail := d3.Splash{width: Fill height: Fit}
                                    }
                                }
                            }

                            img_panel := View{visible: false width: Fill height: Fill flow: Right spacing: 14
                                Panel{width: 300
                                    ScrollYView{width: Fill height: Fill flow: Down spacing: 4 padding: 12
                                        Label{width: Fill text: "选择裁剪位置（原神 4 帧，2144×976）" draw_text.color: #x85827D draw_text.text_style.font_size: 9.5}
                                    sp0 := SpotBtn{}
                                    sp1 := SpotBtn{}
                                    sp2 := SpotBtn{}
                                    sp3 := SpotBtn{}
                                    sp4 := SpotBtn{}
                                    sp5 := SpotBtn{}
                                    sp6 := SpotBtn{}
                                    sp7 := SpotBtn{}
                                    sp8 := SpotBtn{}
                                    sp9 := SpotBtn{}
                                    sp10 := SpotBtn{}
                                    sp11 := SpotBtn{}
                                    sp12 := SpotBtn{}
                                    sp13 := SpotBtn{}
                                    sp14 := SpotBtn{}
                                    sp15 := SpotBtn{}
                                    }
                                }
                                Panel{width: Fill
                                    ScrollYView{width: Fill height: Fill flow: Down spacing: 10 padding: 14
                                        crop_title := Label{width: Fill text: "" draw_text.color: #x202020 draw_text.text_style.font_size: 12}
                                    cand0 := View{width: Fill height: Fit flow: Down spacing: 4 visible: false
                                        cl0 := Label{width: Fill text: "" draw_text.color: #x202020 draw_text.text_style.font_size: 10.5}
                                        View{width: Fill height: Fit flow: Right spacing: 8
                                            n0 := CropBox{}
                                            cb0 := CropBox{}
                                            db0 := CropBox{}
                                        }
                                    }
                                    cand1 := View{width: Fill height: Fit flow: Down spacing: 4 visible: false
                                        cl1 := Label{width: Fill text: "" draw_text.color: #x202020 draw_text.text_style.font_size: 10.5}
                                        View{width: Fill height: Fit flow: Right spacing: 8
                                            n1 := CropBox{}
                                            cb1 := CropBox{}
                                            db1 := CropBox{}
                                        }
                                    }
                                    cand2 := View{width: Fill height: Fit flow: Down spacing: 4 visible: false
                                        cl2 := Label{width: Fill text: "" draw_text.color: #x202020 draw_text.text_style.font_size: 10.5}
                                        View{width: Fill height: Fit flow: Right spacing: 8
                                            n2 := CropBox{}
                                            cb2 := CropBox{}
                                            db2 := CropBox{}
                                        }
                                    }
                                        crop_chart := d3.Splash{width: Fill height: Fit}
                                    }
                                }
                            }

                            eff_panel := View{visible: false width: Fill height: Fill
                                Panel{
                                    ScrollYView{width: Fill height: Fill flow: Down padding: 14
                                        eff_body := d3.Splash{width: Fill height: Fit}
                                    }
                                }
                            }
                        }

                        navbar := SolidView{
                            width: Fill
                            height: 44
                            flow: Right
                            spacing: 10
                            align: Align{y: 0.5}
                            padding: Inset{left: 16 right: 16}
                            new_batch: true
                            draw_bg.color: #xF8F8F6
                            prev := NavButton{text: "‹ 上一页"}
                            bar_chapter := Label{width: Fill text: "" draw_text.color: #x85827D draw_text.text_style.font_size: 10}
                            pager := Label{text: "" draw_text.color: #x383A3C draw_text.text_style.font_size: 10.5}
                            theme_btn := NavButton{text: "暗色"}
                            overview_btn := NavButton{text: "总览"}
                            explorer_btn := NavButton{text: "结果浏览"}
                            next := NavButton{text: "下一页 ›"}
                        }
                        navbar_d := SolidView{
                            visible: false
                            width: Fill
                            height: 44
                            flow: Right
                            spacing: 10
                            align: Align{y: 0.5}
                            padding: Inset{left: 16 right: 16}
                            new_batch: true
                            draw_bg.color: #x1B1F23
                            prev_d := NavButtonDark{text: "‹ 上一页"}
                            bar_chapter_d := Label{width: Fill text: "" draw_text.color: #xA39F98 draw_text.text_style.font_size: 10}
                            pager_d := Label{text: "" draw_text.color: #xD2D0CB draw_text.text_style.font_size: 10.5}
                            theme_btn_d := NavButtonDark{text: "亮色"}
                            overview_btn_d := NavButtonDark{text: "总览"}
                            explorer_btn_d := NavButtonDark{text: "结果浏览"}
                            next_d := NavButtonDark{text: "下一页 ›"}
                        }
                    }

                    overview := SolidView{
                        visible: false
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 10
                        padding: Inset{left: 40 right: 40 top: 28 bottom: 28}
                        new_batch: true
                        draw_bg.color: #xF4F3EFF4
                        View{width: Fill height: Fit flow: Right align: Align{y: 0.5}
                            H2{width: Fill text: "全部页面" draw_text.color: #x202020 draw_text.text_style.font_size: 18}
                            ov_close := NavButton{text: "返回（O）"}
                        }
                        View{width: Fill height: Fit flow: Right spacing: 10 ov0 := OvCard{} ov1 := OvCard{} ov2 := OvCard{} ov3 := OvCard{}}
                        View{width: Fill height: Fit flow: Right spacing: 10 ov4 := OvCard{} ov5 := OvCard{} ov6 := OvCard{} ov7 := OvCard{}}
                        View{width: Fill height: Fit flow: Right spacing: 10 ov8 := OvCard{} ov9 := OvCard{} ov10 := OvCard{} ov11 := OvCard{}}
                        View{width: Fill height: Fit flow: Right spacing: 10 ov12 := OvCard{} ov13 := OvCard{} ov14 := OvCard{} ov15 := OvCard{}}
                        View{width: Fill height: Fit flow: Right spacing: 10 ov16 := OvCard{} ov17 := OvCard{} ov18 := OvCard{} ov19 := OvCard{}}
                        View{width: Fill height: Fit flow: Right spacing: 10 ov20 := OvCard{} ov21 := OvCard{} ov22 := OvCard{} ov23 := OvCard{}}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    deck: Deck,
    #[rust]
    current: usize,
    #[rust]
    loaded: bool,
    #[rust]
    overview_open: bool,
    #[rust]
    explorer_open: bool,
    #[rust]
    swipe_start: Option<DVec2>,
    #[rust]
    lb: Option<explorer::Leaderboard>,
    #[rust]
    lb_section: usize,
    #[rust]
    lb_sort: (usize, bool),
    #[rust]
    lb_rows: Vec<explorer::LbRow>,
    #[rust]
    lb_sel: usize,
    #[rust]
    crops: Option<explorer::CropSet>,
    #[rust]
    crop_sel: usize,
    #[rust]
    tab: usize,
    /// 四个 Splash 区（幻灯片正文、排行榜明细、裁剪图表、效应量）最近一次的内容
    #[rust]
    bodies: [String; 4],
    #[rust]
    refresh_timer: Timer,
    #[rust]
    refresh_left: u32,
    /// 待重算的 Splash 区（位掩码）；只重算刚设置过内容的区，不碰其他（含隐藏的结果浏览区）
    #[rust]
    refresh_mask: u8,
    #[rust]
    dark: bool,
}

fn id1(name: &str) -> [LiveId; 1] {
    [LiveId::from_str(name)]
}

impl App {
    fn load(&mut self, cx: &mut Cx) {
        if self.loaded {
            return;
        }
        self.loaded = true;
        self.deck = match serde_json::from_str::<Deck>(DECK_JSON) {
            Ok(d) => d,
            Err(e) => {
                error!("deck.json 解析失败: {e}");
                Deck::default()
            }
        };
        for i in 0..OV_SLOTS {
            let b = self.ui.button(cx, &id1(&format!("ov{i}")));
            if let Some(sl) = self.deck.slides.get(i) {
                b.set_text(cx, &format!("{} · {}", i + 1, short(&sl.chapter, &sl.title)));
                b.set_visible(cx, true);
            } else {
                b.set_visible(cx, false);
            }
        }
        // 结果浏览的数据
        let lb = explorer::load_leaderboard();
        for i in 0..4 {
            let b = self.ui.button(cx, &id1(&format!("sec{i}")));
            if let Some((k, _, rows)) = lb.sections.get(i) {
                b.set_text(cx, &format!("{k}（{}）", rows.len()));
                b.set_visible(cx, true);
            } else {
                b.set_visible(cx, false);
            }
        }
        self.lb = Some(lb);
        self.lb_sort = (4, true);
        let hdr = explorer::LB_COLS;
        for (j, h) in hdr.iter().enumerate() {
            self.ui.label(cx, &[live_id!(lbh), live_id!(cells), LiveId::from_str(&format!("c{j}"))]).set_text(cx, h);
        }
        self.ui.label(cx, ids!(data_note)).set_text(cx, &explorer::manifest_note());
        self.crops = Some(explorer::load_crops());
        let titles: Vec<String> = self.crops.as_ref().unwrap().items.iter().map(explorer::crop_title).collect();
        for i in 0..SP_SLOTS {
            let b = self.ui.button(cx, &id1(&format!("sp{i}")));
            if let Some(t) = titles.get(i) {
                b.set_text(cx, t);
                b.set_visible(cx, true);
            } else {
                b.set_visible(cx, false);
            }
        }
        let eff = explorer::effects_body();
        self.splash(cx, 3, eff);
        self.select_section(cx, 0);
        self.select_crop(cx, 0);
        let dark = matches!(cx.os_type(), OsType::Web(p) if p.search.contains("theme=dark"));
        self.dark = dark;
        self.show(cx, 0);
        if dark {
            self.apply_theme(cx, true);
        }
    }

    /// 设置 Splash 区内容，并安排两次延时重算：首次求值时字体或字形图集可能还没就绪，
    /// 部分文字会画成方块；重算一次就正常（网页端实测）。
    fn splash(&mut self, cx: &mut Cx, which: usize, body: String) {
        self.splash_raw(cx, which, &body);
        self.bodies[which] = body;
        self.refresh_mask |= 1 << which;
        self.refresh_left = 2;
        self.refresh_timer = cx.start_timeout(0.5);
    }

    fn splash_raw(&mut self, cx: &mut Cx, which: usize, body: &str) {
        let id = match which {
            0 => ids!(slide),
            1 => ids!(lb_detail),
            2 => ids!(crop_chart),
            _ => ids!(eff_body),
        };
        self.ui.widget(cx, id).set_text(cx, body);
    }

    fn refresh_splash(&mut self, cx: &mut Cx) {
        // 末尾加/去一个空格让内容不同，迫使 Splash 重新求值
        let pad = if self.refresh_left % 2 == 0 { "" } else { " " };
        for which in 0..4 {
            if self.refresh_mask & (1 << which) != 0 && !self.bodies[which].is_empty() {
                let b = format!("{}{}", self.bodies[which], pad);
                self.splash_raw(cx, which, &b);
            }
        }
        self.ui.redraw(cx);
    }

    fn select_section(&mut self, cx: &mut Cx, i: usize) {
        let Some(lb) = &self.lb else { return };
        let Some((k, proto, rows)) = lb.sections.get(i) else { return };
        self.lb_section = i;
        let mut rows = rows.clone();
        explorer::sort_rows(&mut rows, self.lb_sort.0, self.lb_sort.1);
        self.ui.label(cx, ids!(lb_info)).set_text(cx, &format!("{k} · 协议 {proto} · 生成 {}", lb.generated));
        self.lb_rows = rows;
        self.lb_sel = self.lb_sel.min(self.lb_rows.len().saturating_sub(1));
        self.fill_lb(cx);
    }

    fn fill_lb(&mut self, cx: &mut Cx) {
        for i in 0..LB_SLOTS {
            let row = self.ui.view(cx, &id1(&format!("lb{i}")));
            if let Some(r) = self.lb_rows.get(i) {
                let cells = explorer::row_cells(r);
                for (j, c) in cells.iter().enumerate() {
                    let t = if j == 0 && i == self.lb_sel { format!("▶ {c}") } else { c.clone() };
                    self.ui
                        .label(cx, &[LiveId::from_str(&format!("lb{i}")), live_id!(cells), LiveId::from_str(&format!("c{j}"))])
                        .set_text(cx, &t);
                }
                row.set_visible(cx, true);
            } else {
                row.set_visible(cx, false);
            }
        }
        if let Some(r) = self.lb_rows.get(self.lb_sel) {
            let body = explorer::lb_detail(r, &self.lb_rows);
            self.splash(cx, 1, body);
        }
        self.ui.redraw(cx);
    }

    fn select_crop(&mut self, cx: &mut Cx, i: usize) {
        let Some(cs) = &self.crops else { return };
        let Some(it) = cs.items.get(i).cloned() else { return };
        self.crop_sel = i;
        self.ui.label(cx, ids!(crop_title)).set_text(cx, &explorer::crop_title(&it));
        let native = it.get("native").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let cands = it.get("cands").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        for k in 0..3 {
            let row = self.ui.view(cx, &id1(&format!("cand{k}")));
            if let Some(c) = cands.get(k) {
                let lab = c.get("label").and_then(|v| v.as_str()).unwrap_or("");
                let ps = c.get("psnr").and_then(|v| v.as_f64()).unwrap_or(f64::NAN);
                self.ui.label(cx, &id1(&format!("cl{k}"))).set_text(cx, &format!("{lab}　裁剪块 PSNR {ps:.2} dB"));
                for (bx, file, cap) in [
                    (format!("n{k}"), native.clone(), "原生".to_string()),
                    (format!("cb{k}"), c.get("img").and_then(|v| v.as_str()).unwrap_or("").to_string(), lab.to_string()),
                    (format!("db{k}"), c.get("diff").and_then(|v| v.as_str()).unwrap_or("").to_string(), "|候选 − 原生| × 8".to_string()),
                ] {
                    if let Some(bytes) = explorer::crop_bytes(&file) {
                        let _ = self.ui.image(cx, &[LiveId::from_str(&bx), live_id!(im)]).load_png_from_data(cx, bytes);
                    }
                    self.ui.label(cx, &[LiveId::from_str(&bx), live_id!(t)]).set_text(cx, &cap);
                }
                row.set_visible(cx, true);
            } else {
                row.set_visible(cx, false);
            }
        }
        let body = explorer::crop_chart(&it, &cs.meta);
        for k in 0..SP_SLOTS {
            if let Some(t) = cs.items.get(k).map(explorer::crop_title) {
                let mark = if k == i { format!("▶ {t}") } else { t };
                self.ui.button(cx, &id1(&format!("sp{k}"))).set_text(cx, &mark);
            }
        }
        self.splash(cx, 2, body);
        self.ui.redraw(cx);
    }

    fn set_tab(&mut self, cx: &mut Cx, t: usize) {
        self.tab = t;
        self.ui.view(cx, ids!(lb_panel)).set_visible(cx, t == 0);
        self.ui.view(cx, ids!(img_panel)).set_visible(cx, t == 1);
        self.ui.view(cx, ids!(eff_panel)).set_visible(cx, t == 2);
        self.ui.redraw(cx);
    }

    fn set_explorer(&mut self, cx: &mut Cx, open: bool) {
        self.explorer_open = open;
        self.ui.view(cx, ids!(explorer)).set_visible(cx, open);
        self.ui.view(cx, ids!(deck_view)).set_visible(cx, !open);
        for id in [ids!(explorer_btn), ids!(explorer_btn_d)] {
            self.ui.button(cx, id).set_text(cx, if open { "回到幻灯片" } else { "结果浏览" });
        }
        if open {
            self.set_tab(cx, self.tab);
        }
        self.ui.redraw(cx);
    }

    fn show(&mut self, cx: &mut Cx, i: usize) {
        let n = self.deck.slides.len();
        if n == 0 {
            self.ui.label(cx, ids!(title)).set_text(cx, "内容文件为空或解析失败（content/deck.json）");
            return;
        }
        let i = i.min(n - 1);
        self.current = i;
        let sl = self.deck.slides[i].clone();
        self.ui.label(cx, ids!(chapter)).set_text(cx, &sl.chapter);
        self.ui.label(cx, ids!(title)).set_text(cx, &sl.title);
        self.ui.label(cx, ids!(subtitle)).set_text(cx, &sl.subtitle);
        self.ui.label(cx, ids!(foot)).set_text(cx, &sl.foot);
        for id in [ids!(bar_chapter), ids!(bar_chapter_d)] {
            self.ui.label(cx, id).set_text(cx, &sl.chapter);
        }
        for id in [ids!(pager), ids!(pager_d)] {
            self.ui.label(cx, id).set_text(cx, &format!("{} / {}", i + 1, n));
        }
        let body = gen::slide_body(&sl);
        let body = if self.dark { gen::to_dark(&body) } else { body };
        self.splash(cx, 0, body);
        if let Some(mut bg) = self.ui.widget(cx, ids!(bg)).borrow_mut::<DeckBg>() {
            bg.set_goal(cx, i);
        }
        self.ui.redraw(cx);
    }

    fn go(&mut self, cx: &mut Cx, delta: isize) {
        if self.explorer_open {
            self.set_explorer(cx, false);
        }
        let n = self.deck.slides.len() as isize;
        let next = (self.current as isize + delta).clamp(0, (n - 1).max(0));
        if next as usize != self.current {
            self.show(cx, next as usize);
        }
    }

    fn set_overview(&mut self, cx: &mut Cx, open: bool) {
        self.overview_open = open;
        self.ui.view(cx, ids!(overview)).set_visible(cx, open);
        self.ui.redraw(cx);
    }
}

fn hex4(c: u32) -> Vec4f {
    vec4f(((c >> 16) & 255) as f32 / 255.0, ((c >> 8) & 255) as f32 / 255.0, (c & 255) as f32 / 255.0, 1.0)
}

impl App {
    /// 亮 / 暗主题：外壳文字用 Label::set_text_color 换色（script_apply 改已绘制的文字实例会在
    /// draw_vars.rs 越界 panic，不用）；分隔线、底栏、按钮用对亮暗都可读的半透明 / 中间色，不随主题切换；
    /// 幻灯片正文按色值映射重生成；三维背景与数据图读 chart3d::DARK。结果浏览与总览是独立的亮色底板。
    fn apply_theme(&mut self, cx: &mut Cx, dark: bool) {
        self.dark = dark;
        chart3d::DARK.store(dark, std::sync::atomic::Ordering::Relaxed);
        let (ink, ink2, muted, teal) = if dark { (0xE9E7E2u32, 0xD2D0CB, 0xA39F98, 0x4FB0A3) } else { (0x202020, 0x383A3C, 0x85827D, 0x2E7D74) };
        self.ui.label(cx, ids!(chapter)).set_text_color(cx, hex4(teal));
        self.ui.label(cx, ids!(title)).set_text_color(cx, hex4(ink));
        self.ui.label(cx, ids!(subtitle)).set_text_color(cx, hex4(ink2));
        self.ui.label(cx, ids!(foot)).set_text_color(cx, hex4(muted));
        self.ui.view(cx, ids!(navbar)).set_visible(cx, !dark);
        self.ui.view(cx, ids!(navbar_d)).set_visible(cx, dark);
        if !self.deck.slides.is_empty() {
            let cur = self.current;
            self.show(cx, cur);
        }
        self.ui.redraw(cx);
    }
}

fn short(chapter: &str, title: &str) -> String {
    let t: String = title.chars().take(22).collect();
    let more = if title.chars().count() > 22 { "…" } else { "" };
    if chapter.is_empty() {
        format!("{t}{more}")
    } else {
        format!("{chapter}｜{t}{more}")
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.load(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let clicked = |s: &Self, cx: &mut Cx, a: &[LiveId; 1], b: &[LiveId; 1]| s.ui.button(cx, a).clicked(actions) || s.ui.button(cx, b).clicked(actions);
        if clicked(self, cx, ids!(prev), ids!(prev_d)) {
            self.go(cx, -1);
        }
        if clicked(self, cx, ids!(next), ids!(next_d)) {
            self.go(cx, 1);
        }
        if clicked(self, cx, ids!(overview_btn), ids!(overview_btn_d)) {
            let open = !self.overview_open;
            self.set_overview(cx, open);
        }
        if clicked(self, cx, ids!(explorer_btn), ids!(explorer_btn_d)) {
            let open = !self.explorer_open;
            self.set_explorer(cx, open);
        }
        if clicked(self, cx, ids!(theme_btn), ids!(theme_btn_d)) {
            let d = !self.dark;
            self.apply_theme(cx, d);
        }
        if self.ui.button(cx, ids!(ov_close)).clicked(actions) {
            self.set_overview(cx, false);
        }
        for (t, id) in [(0usize, ids!(tab_lb)), (1, ids!(tab_img)), (2, ids!(tab_eff))] {
            if self.ui.button(cx, id).clicked(actions) {
                self.set_tab(cx, t);
            }
        }
        for i in 0..OV_SLOTS.min(self.deck.slides.len()) {
            if self.ui.button(cx, &id1(&format!("ov{i}"))).clicked(actions) {
                self.set_overview(cx, false);
                self.set_explorer(cx, false);
                self.show(cx, i);
            }
        }
        for i in 0..4 {
            if self.ui.button(cx, &id1(&format!("sec{i}"))).clicked(actions) {
                self.lb_sel = 0;
                self.select_section(cx, i);
            }
        }
        for j in 0..explorer::LB_COLS.len() {
            if self.ui.button(cx, &id1(&format!("sort{j}"))).clicked(actions) {
                let desc = if self.lb_sort.0 == j { !self.lb_sort.1 } else { j >= 2 && j != 6 && j != 3 && j != 2 };
                self.lb_sort = (j, desc);
                self.lb_sel = 0;
                self.select_section(cx, self.lb_section);
            }
        }
        for i in 0..LB_SLOTS.min(self.lb_rows.len()) {
            if self.ui.button(cx, &[LiveId::from_str(&format!("lb{i}")), live_id!(sel)]).clicked(actions) {
                self.lb_sel = i;
                self.fill_lb(cx);
            }
        }
        let n_crops = self.crops.as_ref().map(|c| c.items.len()).unwrap_or(0);
        for i in 0..SP_SLOTS.min(n_crops) {
            if self.ui.button(cx, &id1(&format!("sp{i}"))).clicked(actions) {
                self.select_crop(cx, i);
            }
        }
    }
}

fn sandbox_mods(vm: &mut ScriptVm) {
    let _ = bg::script_mod(vm);
    let _ = chart3d::script_mod(vm);
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_d3::script_mod(vm);
        bg::script_mod(vm);
        chart3d::script_mod(vm);
        // d3.Splash 在独立的沙箱 VM 里求值正文：把三维图（及其用到的 DrawFace 着色器）也注册进去
        makepad_d3::splash::host::register_splash_mod(sandbox_mods);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if !self.loaded {
            if let Event::Draw(_) = event {
                self.load(cx);
            }
        }
        if self.refresh_timer.is_event(event).is_some() {
            self.refresh_splash(cx);
            self.refresh_left = self.refresh_left.saturating_sub(1);
            if self.refresh_left > 0 {
                self.refresh_timer = cx.start_timeout(1.5);
            } else {
                self.refresh_mask = 0;
            }
        }
        match event {
            Event::KeyDown(ke) => match ke.key_code {
                KeyCode::ArrowRight | KeyCode::PageDown | KeyCode::Space => self.go(cx, 1),
                KeyCode::ArrowLeft | KeyCode::PageUp => self.go(cx, -1),
                KeyCode::Home => {
                    self.go(cx, -(self.current as isize));
                }
                KeyCode::End => {
                    let n = self.deck.slides.len() as isize;
                    self.go(cx, n);
                }
                KeyCode::KeyO => {
                    let open = !self.overview_open;
                    self.set_overview(cx, open);
                }
                KeyCode::KeyT => {
                    let d = !self.dark;
                    self.apply_theme(cx, d);
                }
                KeyCode::KeyE => {
                    let open = !self.explorer_open;
                    self.set_explorer(cx, open);
                }
                KeyCode::Escape => {
                    self.set_overview(cx, false);
                    self.set_explorer(cx, false);
                }
                _ => {}
            },
            Event::TouchUpdate(te) => {
                for t in &te.touches {
                    match t.state {
                        makepad_widgets::makepad_platform::event::finger::TouchState::Start => self.swipe_start = Some(t.abs),
                        makepad_widgets::makepad_platform::event::finger::TouchState::Stop => {
                            if let Some(s) = self.swipe_start.take() {
                                let dx = t.abs.x - s.x;
                                let dy = t.abs.y - s.y;
                                if !self.explorer_open && dx.abs() > 70.0 && dx.abs() > dy.abs() * 1.5 {
                                    self.go(cx, if dx < 0.0 { 1 } else { -1 });
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
