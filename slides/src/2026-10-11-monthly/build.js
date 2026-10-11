// 「HGamey 近一个月进展」生成脚本。所有文字和数字在 content.json；这里只管版式。
// 刷新：python3 -I collect.py && node build.js   （输出覆盖 slides/2026-10-11-HGamey-近一个月进展.pptx）
const fs = require("fs");
const path = require("path");
const pptxgen = require("pptxgenjs");
const JSZip = require(require.resolve("jszip", { paths: [require.resolve("pptxgenjs")] }));

const HERE = __dirname;
const content = JSON.parse(fs.readFileSync(path.join(HERE, "content.json"), "utf8"));
const auto = content.auto || {};
const OUT = path.resolve(HERE, content.meta.out);

// ---------- 主题 ----------
const FONT = "Microsoft YaHei";
const THEME = {
  name: "HGamey Monthly",
  headFontFace: FONT,
  bodyFontFace: FONT,
  colors: {
    dk1: "0F2744", lt1: "FFFFFF", dk2: "5C6470", lt2: "EEF2F7",
    accent1: "1F4E79", accent2: "0A806F", accent3: "C2602E", accent4: "B23A3A",
    accent5: "D8DCE2", accent6: "8A94A6", hlink: "1F4E79", folHlink: "5C6470",
  },
};
const HEX = THEME.colors;

// ---------- 占位符替换 ----------
const vars = Object.assign({}, auto, {
  colleague_issue_n: (auto.colleague_issues || []).length,
  repo_n: (auto.our_pr_counts || []).filter((r) => r.merged > 0).length,
  pf_merged: ((auto.our_pr_counts || []).find((r) => r.repo === "BH3GEI/phonefarm") || {}).merged,
  gol_merged: ((auto.our_pr_counts || []).find((r) => r.repo === "HGamey/game_opt_loop") || {}).merged,
});
function f(s) {
  return String(s).replace(/\{(\w+)\}/g, (m, k) => (vars[k] !== undefined ? String(vars[k]) : m));
}
const BANNED = /\bgat(e|es|ed|ing)\b|门禁/i;

// ---------- 版式 ----------
const pres = new pptxgen();
pres.layout = "LAYOUT_WIDE"; // 13.333 x 7.5，与 09-24 版一致
pres.theme = { headFontFace: FONT, bodyFontFace: FONT };
pres.title = content.meta.title;
pres.author = content.meta.author;
const C = pres.SchemeColor;
const W = 13.333, M = 0.6, CW = W - 2 * M;

pres.defineSlideMaster({
  title: "TITLE",
  background: { color: C.text1 },
  objects: [
    { placeholder: { options: { name: "title", type: "title", x: 0.8, y: 2.1, w: 11.7, h: 1.2, fontSize: 44, bold: true, color: C.background1, valign: "bottom", margin: 0 }, text: "" } },
    { placeholder: { options: { name: "body", type: "body", x: 0.8, y: 3.5, w: 11.7, h: 1.0, fontSize: 20, color: C.background2, valign: "top", margin: 0 }, text: "" } },
  ],
});
pres.defineSlideMaster({
  title: "CONTENT",
  background: { color: C.background1 },
  margin: [0.5, 0.6, 0.6, 0.6],
  objects: [
    { placeholder: { options: { name: "title", type: "title", x: M, y: 0.35, w: CW, h: 0.8, fontSize: 28, bold: true, color: C.text1, valign: "middle", margin: 0 }, text: "" } },
    { text: { text: content.meta.footer, options: { x: M, y: 7.0, w: 8, h: 0.3, fontSize: 10, color: C.text2, margin: 0 } } },
  ],
  slideNumber: { x: 12.1, y: 7.0, w: 0.63, h: 0.3, fontSize: 10, color: C.text2, align: "right", margin: 0 },
});

let sectionNow = "";
function section(t) { pres.addSection({ title: t }); sectionNow = t; }
function slide(master, title, notes) {
  const s = pres.addSlide({ masterName: master, sectionTitle: sectionNow });
  if (title) s.addText(f(title), { placeholder: "title" });
  if (notes) s.addNotes(f(notes));
  return s;
}
function box(s, name, x, y, w, h, fill) {
  s.addShape(pres.shapes.RECTANGLE, { objectName: name, x, y, w, h, fill: { color: fill }, line: { type: "none" } });
}
function txt(s, name, text, o) {
  s.addText(typeof text === "string" ? f(text) : text, Object.assign({ objectName: name, isTextBox: true, margin: 0, valign: "top", color: C.text1, fontSize: 14 }, o));
}
function bullets(items, o = {}) {
  return items.map((t, i) => ({ text: f(t), options: Object.assign({ bullet: { indent: 14 }, paraSpaceAfter: 8, breakLine: i < items.length - 1 }, o) }));
}
function circleNum(s, name, n, x, y, d, fill) {
  s.addShape(pres.shapes.OVAL, { objectName: name, x, y, w: d, h: d, fill: { color: fill }, line: { type: "none" } });
  txt(s, name + "-n", String(n), { x, y, w: d, h: d, align: "center", valign: "middle", bold: true, color: C.background1, fontSize: 14 });
}
function ranges(nums) {
  const out = []; let a = null, b = null;
  for (const n of nums) {
    if (a === null) { a = b = n; continue; }
    if (n === b + 1) { b = n; continue; }
    out.push(a === b ? `#${a}` : `#${a}–#${b}`); a = b = n;
  }
  if (a !== null) out.push(a === b ? `#${a}` : `#${a}–#${b}`);
  return out.join(", ");
}


function table(s, name, data, o, hdrFill, sizes = {}) {
  const rows = data.map((r, i) => r.map((v, j) => {
    if (i === 0) return { text: f(v), options: { bold: true, color: C.background1, fill: { color: hdrFill || C.accent1 }, fontSize: sizes.head || 13, align: (o.align || [])[j] || "left" } };
    const opt = { color: C.text1, fill: { color: i % 2 ? C.background2 : C.background1 }, fontSize: sizes.body || 13, align: (o.align || [])[j] || "left", bold: (o.boldCols || []).includes(j) };
    if (o.colColor && o.colColor[j]) opt.color = o.colColor[j];
    return { text: f(v), options: opt };
  }));
  const t = Object.assign({ objectName: name, valign: "middle", margin: [0, 0.1, 0, 0.1], border: { type: "none" } }, o);
  delete t.align; delete t.boldCols; delete t.colColor;
  s.addTable(rows, t);
}

// ---------- 1 封面 ----------
section("开场");
{
  const c = content.title;
  const s = slide("TITLE", null, c.notes);
  s.addText(f(c.title), { placeholder: "title" });
  s.addText(f(c.subtitle), { placeholder: "body" });
  txt(s, "line", c.line, { x: 0.8, y: 5.6, w: 11.7, h: 0.4, fontSize: 14, color: C.accent6 });
}

// ---------- 2 结论 ----------
{
  const c = content.summary;
  const s = slide("CONTENT", c.title, c.notes);
  const n = c.cards.length, gap = 0.3, cw = (CW - gap * (n - 1)) / n, y = 1.45, h = 3.95;
  c.cards.forEach((k, i) => {
    const x = M + i * (cw + gap);
    box(s, `card${i}`, x, y, cw, h, C.background2);
    txt(s, `card${i}-big`, k.big, { x: x + 0.25, y: y + 0.25, w: cw - 0.5, h: 0.95, fontSize: 44, bold: true, color: C[k.tone], valign: "middle" });
    txt(s, `card${i}-label`, k.label, { x: x + 0.25, y: y + 1.25, w: cw - 0.5, h: 0.75, fontSize: 15, bold: true });
    txt(s, `card${i}-text`, k.text, { x: x + 0.25, y: y + 2.05, w: cw - 0.5, h: h - 2.25, fontSize: 13, color: C.text2, lineSpacingMultiple: 1.15 });
  });
  box(s, "bottom-bg", M, 5.75, CW, 0.95, C.accent1);
  txt(s, "bottom", c.bottom, { x: M + 0.3, y: 5.75, w: CW - 0.6, h: 0.95, fontSize: 16, bold: true, color: C.background1, valign: "middle" });
}

// ---------- 3 harness 是什么 ----------
section("我们这边");
{
  const c = content.harness;
  const s = slide("CONTENT", c.title, c.notes);
  box(s, "ask-bg", M, 1.35, CW, 0.75, C.text1);
  txt(s, "ask", "用户只说一句：" + c.ask, { x: M + 0.3, y: 1.35, w: CW - 0.6, h: 0.75, fontSize: 18, bold: true, color: C.background1, valign: "middle" });
  const n = c.steps.length, gap = 0.12, sw = (CW - gap * (n - 1)) / n, y = 2.35;
  c.steps.forEach((st, i) => {
    const x = M + i * (sw + gap);
    s.addShape(i === 0 ? pres.shapes.PENTAGON : pres.shapes.CHEVRON, { objectName: `step${i}`, x, y, w: sw, h: 0.7, fill: { color: i === 4 ? C.accent2 : C.accent1 }, line: { type: "none" } });
    txt(s, `step${i}-h`, st.h, { x: x + 0.15, y, w: sw - 0.3, h: 0.7, fontSize: 15, bold: true, color: C.background1, align: "center", valign: "middle" });
    txt(s, `step${i}-b`, st.b, { x: x + 0.05, y: y + 0.85, w: sw - 0.1, h: 1.1, fontSize: 12, color: C.text2, align: "center" });
  });
  const rw = (CW - 0.3 * 2) / 3;
  c.repos.forEach((r, i) => {
    const x = M + i * (rw + 0.3);
    box(s, `repo${i}`, x, 4.45, rw, 1.15, C.background2);
    txt(s, `repo${i}-h`, r.h, { x: x + 0.25, y: 4.55, w: rw - 0.5, h: 0.4, fontSize: 15, bold: true, color: C.accent1 });
    txt(s, `repo${i}-b`, r.b, { x: x + 0.25, y: 4.98, w: rw - 0.5, h: 0.55, fontSize: 12, color: C.text1 });
  });
  txt(s, "rule", c.rule, { x: M, y: 5.85, w: CW, h: 0.8, fontSize: 15, bold: true, color: C.accent2, valign: "middle" });
}

// ---------- 4 三层 ----------
{
  const c = content.layers;
  const s = slide("CONTENT", c.title, c.notes);
  const gap = 0.3, cw = (CW - gap * 2) / 3, y = 1.35, h = 5.4;
  c.cols.forEach((col, i) => {
    const x = M + i * (cw + gap);
    box(s, `l${i}-h-bg`, x, y, cw, 0.6, C.accent1);
    txt(s, `l${i}-h`, `${col.tag} ${col.head}`, { x: x + 0.25, y, w: cw - 0.5, h: 0.6, fontSize: 18, bold: true, color: C.background1, valign: "middle" });
    box(s, `l${i}-bg`, x, y + 0.6, cw, h - 0.6, C.background2);
    txt(s, `l${i}-how`, col.how, { x: x + 0.25, y: y + 0.75, w: cw - 0.5, h: 0.6, fontSize: 12, color: C.text2 });
    txt(s, `l${i}-big`, col.big, { x: x + 0.25, y: y + 1.4, w: cw - 0.5, h: 0.65, fontSize: 32, bold: true, color: C.accent2, valign: "middle" });
    txt(s, `l${i}-bigl`, col.bigl, { x: x + 0.25, y: y + 2.05, w: cw - 0.5, h: 0.4, fontSize: 12, bold: true });
    txt(s, `l${i}-done`, bullets(col.done, { fontSize: 12.5, paraSpaceAfter: 6 }), { x: x + 0.25, y: y + 2.6, w: cw - 0.5, h: 2.25 });
    txt(s, `l${i}-r`, col.refs, { x: x + 0.25, y: y + h - 0.45, w: cw - 0.5, h: 0.3, fontSize: 11, bold: true, color: C.accent1 });
  });
}

// ---------- 5 upop ----------
{
  const c = content.upop;
  const s = slide("CONTENT", c.title, c.notes);
  const lw = 7.6;
  txt(s, "u-head", c.head, { x: M, y: 1.35, w: lw, h: 0.4, fontSize: 17, bold: true });
  txt(s, "u-cond", c.cond, { x: M, y: 1.8, w: lw, h: 0.6, fontSize: 12, color: C.text2 });
  table(s, "u-table", c.rows, { x: M, y: 2.5, w: lw, colW: [1.5, 1.6, 1.7, 1.2, 1.6], rowH: 0.5, align: ["left", "center", "center", "center", "center"], boldCols: [3] }, C.accent1, { head: 13, body: 14 });
  box(s, "u-v-bg", M, 4.7, lw, 0.7, C.accent2);
  txt(s, "u-v", c.verdict, { x: M + 0.25, y: 4.7, w: lw - 0.5, h: 0.7, fontSize: 15, bold: true, color: C.background1, valign: "middle" });
  txt(s, "u-cav", c.caveat, { x: M, y: 5.6, w: lw, h: 0.8, fontSize: 12, color: C.text2 });
  const rx = M + lw + 0.4, rw = W - M - rx;
  box(s, "q-bg", rx, 1.35, rw, 5.05, C.background2);
  txt(s, "q-t", "画质", { x: rx + 0.3, y: 1.55, w: rw - 0.6, h: 0.4, fontSize: 16, bold: true });
  txt(s, "q-big", c.quality_big, { x: rx + 0.3, y: 2.1, w: rw - 0.6, h: 0.9, fontSize: 40, bold: true, color: C.accent2, valign: "middle" });
  txt(s, "q-l", c.quality_l, { x: rx + 0.3, y: 3.05, w: rw - 0.6, h: 0.5, fontSize: 13 });
  txt(s, "q-cmp", c.quality_cmp, { x: rx + 0.3, y: 3.7, w: rw - 0.6, h: 0.5, fontSize: 18, bold: true, color: C.accent3 });
  txt(s, "q-c", c.quality_cond, { x: rx + 0.3, y: 4.35, w: rw - 0.6, h: 0.8, fontSize: 12, color: C.text2 });
  txt(s, "q-note", "数值越小越接近原生；我们的算子比游戏自带放大接近 3–4 倍", { x: rx + 0.3, y: 5.3, w: rw - 0.6, h: 0.8, fontSize: 12, color: C.text1 });
}

// ---------- 6 系统参数 + 已知答案 ----------
{
  const c = content.sysparam;
  const s = slide("CONTENT", c.title, c.notes);
  const lw = 6.4;
  txt(s, "sp-h", c.head, { x: M, y: 1.35, w: lw, h: 0.4, fontSize: 17, bold: true });
  table(s, "sp-table", c.rows, { x: M, y: 1.9, w: lw, colW: [2.2, 3.1, 1.1], rowH: 0.55, align: ["left", "left", "center"], boldCols: [2], colColor: { 2: C.accent4 } }, C.accent4, { head: 13, body: 13 });
  txt(s, "sp-foot", c.foot, { x: M, y: 3.6, w: lw, h: 0.6, fontSize: 12, color: C.text2 });
  box(s, "lo-bg", M, 4.45, lw, 2.0, C.background2);
  txt(s, "lo-h", c.loadop_head, { x: M + 0.25, y: 4.6, w: lw - 0.5, h: 0.45, fontSize: 16, bold: true, color: C.accent3 });
  txt(s, "lo-b", c.loadop, { x: M + 0.25, y: 5.1, w: lw - 0.5, h: 1.2, fontSize: 13 });
  const rx = M + lw + 0.4, rw = W - M - rx;
  txt(s, "k-h", c.known_head, { x: rx, y: 1.35, w: rw, h: 0.4, fontSize: 17, bold: true });
  c.known.forEach((k, i) => {
    const y = 1.95 + i * 2.3;
    box(s, `k${i}-bg`, rx, y, rw, 2.1, C.background2);
    txt(s, `k${i}-big`, k.big, { x: rx + 0.3, y: y + 0.2, w: rw - 0.6, h: 0.85, fontSize: 40, bold: true, color: C.accent2, valign: "middle" });
    txt(s, `k${i}-h`, k.h, { x: rx + 0.3, y: y + 1.1, w: rw - 0.6, h: 0.4, fontSize: 15, bold: true });
    txt(s, `k${i}-b`, k.b, { x: rx + 0.3, y: y + 1.5, w: rw - 0.6, h: 0.45, fontSize: 13, color: C.text2 });
  });
}

// ---------- 7 闭环与判定 ----------
{
  const c = content.loop;
  const s = slide("CONTENT", c.title, c.notes);
  const lw = 6.9;
  txt(s, "lp-h", c.left_head, { x: M, y: 1.35, w: lw, h: 0.4, fontSize: 17, bold: true });
  table(s, "lp-table", c.rows, { x: M, y: 1.9, w: lw, colW: [1.9, lw - 1.9], rowH: 0.72, boldCols: [0] }, C.accent2, { head: 13, body: 13 });
  const rx = M + lw + 0.4, rw = W - M - rx;
  box(s, "r-h-bg", rx, 1.35, rw, 0.55, C.accent1);
  txt(s, "r-h", c.right_head, { x: rx + 0.25, y: 1.35, w: rw - 0.5, h: 0.55, fontSize: 16, bold: true, color: C.background1, valign: "middle" });
  box(s, "r-bg", rx, 1.9, rw, 4.32, C.background2);
  txt(s, "r-b", bullets(c.right, { fontSize: 13.5, paraSpaceAfter: 10 }), { x: rx + 0.25, y: 2.1, w: rw - 0.5, h: 4.0 });
}

// ---------- 8 不自欺 ----------
{
  const c = content.honesty;
  const s = slide("CONTENT", c.title, c.notes);
  const gap = 0.4, cw = (CW - gap) / 2, y = 1.4, h = 4.1;
  [[c.left_head, c.left, C.accent1], [c.right_head, c.right, C.accent3]].forEach(([head, items, col], i) => {
    const x = M + i * (cw + gap);
    box(s, `col${i}-h-bg`, x, y, cw, 0.6, col);
    txt(s, `col${i}-h`, head, { x: x + 0.25, y, w: cw - 0.5, h: 0.6, fontSize: 16, bold: true, color: C.background1, valign: "middle" });
    box(s, `col${i}-bg`, x, y + 0.6, cw, h - 0.6, C.background2);
    txt(s, `col${i}-b`, bullets(items, { fontSize: 14 }), { x: x + 0.25, y: y + 0.85, w: cw - 0.5, h: h - 1.1 });
  });
  txt(s, "bottom", c.bottom, { x: M, y: 5.75, w: CW, h: 0.95, fontSize: 15, bold: true, color: C.accent1, valign: "middle" });
}

// ---------- 9 工程推进 ----------
{
  const c = content.eng;
  const s = slide("CONTENT", c.title, c.notes);
  const gap = 0.3, cw = (CW - gap * 2) / 3, ch = 2.5;
  c.items.forEach((it, i) => {
    const x = M + (i % 3) * (cw + gap), y = 1.4 + Math.floor(i / 3) * (ch + gap);
    box(s, `t${i}`, x, y, cw, ch, C.background2);
    circleNum(s, `t${i}-c`, i + 1, x + 0.25, y + 0.25, 0.45, i >= 3 ? C.accent3 : C.accent1);
    txt(s, `t${i}-h`, it.head, { x: x + 0.85, y: y + 0.25, w: cw - 1.1, h: 0.45, fontSize: 17, bold: true, valign: "middle" });
    txt(s, `t${i}-b`, it.body, { x: x + 0.25, y: y + 0.88, w: cw - 0.5, h: 1.15, fontSize: 13, color: C.text2, lineSpacingMultiple: 1.1 });
    txt(s, `t${i}-r`, it.refs, { x: x + 0.25, y: y + ch - 0.45, w: cw - 0.5, h: 0.3, fontSize: 11, color: C.accent1, bold: true });
  });
}

// ---------- 10 正在做 ----------
section("正在做与下一步");
{
  const c = content.now;
  const s = slide("CONTENT", c.title, c.notes);
  const n = c.agents.length, gap = 0.2, cw = (CW - gap * (n - 1)) / n, y = 1.35, h = 3.3;
  c.agents.forEach((a, i) => {
    const x = M + i * (cw + gap);
    box(s, `a${i}`, x, y, cw, h, C.background2);
    txt(s, `a${i}-n`, a.name, { x: x + 0.2, y: y + 0.2, w: cw - 0.4, h: 0.45, fontSize: 20, bold: true, color: C.accent1, valign: "middle" });
    txt(s, `a${i}-t`, a.task, { x: x + 0.2, y: y + 0.75, w: cw - 0.4, h: 0.9, fontSize: 13, bold: true });
    txt(s, `a${i}-s`, a.status, { x: x + 0.2, y: y + 1.75, w: cw - 0.4, h: 1.45, fontSize: 12, color: C.accent2 });
  });
  box(s, "inc-bg", M, 4.95, CW, 1.55, C.background2);
  txt(s, "inc-h", c.incident_head, { x: M + 0.25, y: 4.95, w: 1.5, h: 1.55, fontSize: 15, bold: true, color: C.accent3, valign: "middle" });
  txt(s, "inc", c.incident, { x: M + 1.8, y: 4.95, w: CW - 2.05, h: 1.55, fontSize: 12.5, valign: "middle", lineSpacingMultiple: 1.1 });
}

// ---------- 11 下一步 ----------
{
  const c = content.next;
  const s = slide("CONTENT", c.title, c.notes);
  const gap = 0.3, cw = (CW - gap) / 2, ch = 2.45;
  c.items.forEach((it, i) => {
    const x = M + (i % 2) * (cw + gap), y = 1.4 + Math.floor(i / 2) * (ch + gap);
    box(s, `n${i}`, x, y, cw, ch, C.background2);
    circleNum(s, `n${i}-c`, i + 1, x + 0.3, y + 0.3, 0.5, C.accent1);
    txt(s, `n${i}-h`, it.head, { x: x + 1.0, y: y + 0.3, w: cw - 2.6, h: 0.5, fontSize: 19, bold: true, valign: "middle" });
    const st = it.status;
    s.addShape(pres.shapes.ROUNDED_RECTANGLE, { objectName: `n${i}-st-bg`, x: x + cw - 1.45, y: y + 0.35, w: 1.15, h: 0.4, rectRadius: 0.2, fill: { color: st === "进行中" ? C.accent2 : C.accent6 }, line: { type: "none" } });
    txt(s, `n${i}-st`, st, { x: x + cw - 1.45, y: y + 0.35, w: 1.15, h: 0.4, fontSize: 12, bold: true, color: C.background1, align: "center", valign: "middle" });
    txt(s, `n${i}-b`, it.body, { x: x + 0.3, y: y + 1.0, w: cw - 0.6, h: ch - 1.2, fontSize: 14, color: C.text1, lineSpacingMultiple: 1.1 });
  });
}

// ---------- 12 同事 ----------
section("同事与协调");
{
  const c = content.colleague;
  const s = slide("CONTENT", c.title, c.notes);
  const lw = 7.0;
  const n = c.stats.length, gap = 0.2, sw = (lw - gap * (n - 1)) / n;
  c.stats.forEach((st, i) => {
    const x = M + i * (sw + gap);
    box(s, `cs${i}`, x, 1.35, sw, 1.1, C.background2);
    txt(s, `cs${i}-big`, st.big, { x: x + 0.15, y: 1.4, w: sw - 0.3, h: 0.6, fontSize: 28, bold: true, color: C.accent3, valign: "middle" });
    txt(s, `cs${i}-l`, st.label, { x: x + 0.15, y: 2.0, w: sw - 0.3, h: 0.35, fontSize: 12, color: C.text2 });
  });
  table(s, "themes", [["方向", "PR"]].concat(c.themes.map((t) => [t.head, t.prs])), { x: M, y: 2.65, w: lw, colW: [2.6, lw - 2.6], rowH: 0.55, boldCols: [0] }, C.accent3, { head: 12, body: 12 });
  txt(s, "foot", c.footnote, { x: M, y: 6.05, w: lw, h: 0.7, fontSize: 11.5, color: C.text2 });
  const rx = M + lw + 0.4, rw = W - M - rx;
  box(s, "m-bg", rx, 1.35, rw, 5.4, C.background2);
  txt(s, "m-h", c.meet_head, { x: rx + 0.25, y: 1.5, w: rw - 0.5, h: 0.45, fontSize: 15, bold: true, color: C.accent1 });
  txt(s, "m-b", bullets(c.meet, { fontSize: 13, paraSpaceAfter: 12 }), { x: rx + 0.25, y: 2.1, w: rw - 0.5, h: 4.4 });
}

// ---------- 13 协调 ----------
{
  const c = content.coordinate;
  const s = slide("CONTENT", c.title, c.notes);
  c.items.forEach((it, i) => {
    const y = 1.45 + i * 1.3;
    box(s, `k${i}-bg`, M, y, CW, 1.1, i % 2 ? C.background1 : C.background2);
    circleNum(s, `k${i}`, i + 1, M + 0.25, y + 0.28, 0.55, C.accent3);
    txt(s, `k${i}-h`, it.head, { x: M + 1.05, y, w: 1.9, h: 1.1, fontSize: 18, bold: true, valign: "middle" });
    txt(s, `k${i}-b`, it.body, { x: M + 3.0, y, w: CW - 3.2, h: 1.1, fontSize: 14, valign: "middle" });
  });
}

// ---------- 14 附录 ----------
section("附录");
{
  const c = content.appendix_ours;
  const s = slide("CONTENT", c.title, c.notes);
  const data = [["仓库", "合并", "其中文档", "编号"]].concat((auto.our_pr_counts || []).map((r) => [r.repo, String(r.merged), String(r.docs), ranges(r.numbers || [])]));
  table(s, "ours", data, { x: M, y: 1.4, w: CW, colW: [2.8, 0.9, 1.1, CW - 4.8], rowH: 0.5, align: ["left", "center", "center", "left"], boldCols: [0] }, C.accent1, { head: 13, body: 12 });
  txt(s, "sum", `合计 ${vars.our_total} 个（其中文档 ${vars.our_docs} 个）；HGamey/phonefarm 上另有 ${vars.sync_prs} 个镜像同步 PR 不计。按合并日期：${Object.entries(vars.our_by_day || {}).map(([d, n]) => `${d} ${n} 个`).join("、")}（太平洋时间；09-25 至 10-10 国庆无合并）。数据截至 ${vars.updated_at} PDT。`, { x: M, y: 5.9, w: CW, h: 0.8, fontSize: 12, color: C.text2 });
}

// ---------- 写文件 + 主题 ----------
async function applyTheme(file) {
  const zip = await JSZip.loadAsync(fs.readFileSync(file));
  const part = "ppt/theme/theme1.xml";
  const SLOTS = ["dk1", "lt1", "dk2", "lt2", "accent1", "accent2", "accent3", "accent4", "accent5", "accent6", "hlink", "folHlink"];
  const scheme = `<a:clrScheme name="${THEME.name}">` + SLOTS.map((k) => `<a:${k}><a:srgbClr val="${HEX[k]}"/></a:${k}>`).join("") + "</a:clrScheme>";
  let xml = await zip.file(part).async("string");
  xml = xml.replace(/<a:clrScheme\b[\s\S]*?<\/a:clrScheme>/, scheme)
    .replace(/(<a:(?:theme|fontScheme)\b[^>]*?\bname=")[^"]*"/g, (_, h) => `${h}${THEME.name}"`)
    .replace(/<a:ea typeface="[^"]*"\s*\/>/g, `<a:ea typeface="${FONT}"/>`);
  if (!xml.includes(scheme)) throw new Error("theme1.xml 里没有 clrScheme");
  zip.file(part, xml);
  for (const name of Object.keys(zip.files)) {
    if (!name.endsWith(".xml")) continue;
    const s = await zip.file(name).async("string");
    const bad = s.match(/<a:srgbClr val="((?![0-9A-Fa-f]{6}")[^"]*)"/);
    if (bad) throw new Error(`${name}: srgbClr val="${bad[1]}"，hex-only 选项里用了 scheme 色`);
    if (BANNED.test((s.match(/<a:t>[^<]*<\/a:t>/g) || []).join(" "))) throw new Error(`${name}: 出现了禁用词（GATE/门禁）`);
  }
  fs.writeFileSync(file, await zip.generateAsync({ type: "nodebuffer", compression: "DEFLATE" }));
}

(async () => {
  await pres.writeFile({ fileName: OUT });
  await applyTheme(OUT);
  console.log(`写出 ${OUT}（${(fs.statSync(OUT).size / 1024).toFixed(0)} KB）`);
})().catch((e) => { console.error(e); process.exit(1); });
