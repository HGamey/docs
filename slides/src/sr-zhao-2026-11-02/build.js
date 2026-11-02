const pptxgen = require("pptxgenjs");
// 生成：NODE_PATH=<含 pptxgenjs 的 node_modules> APPLY_THEME=<apply_theme.js> node build.js out.pptx
//       python3 fix_ea.py out.pptx "Microsoft YaHei"   # 主题里补上中文（东亚）字体
// 数字来源：experiments/2026-10-24-selfsr/STEP6_APPENDIX_CADENCE_SOAK.md（终版性能、功耗、长稳），
//          HGamey/experiments PR #13（FSR1/SGSR1/selfsr 离线 PSNR），研究 selfsr-genshin-ab-20261028 的两组排序盲评。
const { applyTheme } = require(process.env.APPLY_THEME || "./apply_theme.js");

const FONT = process.env.DECK_FONT || "Microsoft YaHei";
const THEME = {
  name: "HGamey Night",
  headFontFace: FONT,
  bodyFontFace: FONT,
  colors: {
    dk1: "1F1B2E", lt1: "FFFFFF", dk2: "5B5670", lt2: "F3F2F7",
    accent1: "F2A541", accent2: "2EC4B6", accent3: "8C86A8",
    accent4: "E4572E", accent5: "C9C6D6", accent6: "6C63FF",
    hlink: "2EC4B6", folHlink: "8C86A8",
  },
};
const HEX = THEME.colors;
const OUT = process.argv[2] || "deck.pptx";

const pres = new pptxgen();
pres.layout = "LAYOUT_16x9"; // 10 x 5.625
pres.theme = { headFontFace: FONT, bodyFontFace: FONT };
pres.title = "移动游戏超分与插帧：阶段结果";
pres.author = "HGamey";
const C = pres.SchemeColor;

pres.defineSlideMaster({
  title: "TITLE_DARK",
  background: { color: C.text1 },
  objects: [
    { placeholder: { options: { name: "title", type: "title", x: 0.6, y: 1.5, w: 8.8, h: 1.3, fontSize: 36, bold: true, color: C.background1, valign: "bottom", align: "left", margin: 0 }, text: "" } },
    { placeholder: { options: { name: "body", type: "body", x: 0.6, y: 3.0, w: 8.8, h: 1.2, fontSize: 16, color: C.accent5, valign: "top", align: "left", margin: 0 }, text: "" } },
  ],
});
pres.defineSlideMaster({
  title: "CONTENT",
  background: { color: C.background1 },
  margin: [0.5, 0.5, 0.5, 0.5],
  objects: [
    { placeholder: { options: { name: "title", type: "title", x: 0.5, y: 0.3, w: 9.0, h: 0.75, fontSize: 28, bold: true, color: C.text1, valign: "middle", align: "left", margin: 0 }, text: "" } },
    { text: { text: "HGamey · 2026-11-02", options: { x: 0.5, y: 5.2, w: 4, h: 0.3, fontSize: 10, color: C.accent3, margin: 0 } } },
  ],
  slideNumber: { x: 9.0, y: 5.2, w: 0.5, h: 0.3, fontSize: 10, color: HEX.accent3, align: "right" },
});

const T = (slide, text, opts) => slide.addText(text, { isTextBox: true, margin: 0, fontFace: FONT, ...opts });
const card = (slide, x, y, w, h, fill, name) => slide.addShape(pres.shapes.ROUNDED_RECTANGLE, {
  x, y, w, h, rectRadius: 0.08, fill: { color: fill }, line: { color: fill }, objectName: name,
});

// 1 封面
pres.addSection({ title: "开场" });
let s = pres.addSlide({ masterName: "TITLE_DARK", sectionTitle: "开场" });
s.addText("移动游戏超分与插帧：阶段结果", { placeholder: "title" });
s.addText("不改游戏，在系统层把低分辨率渲染变成高清、高帧率\nHGamey · 与东南大学赵普教授交流 · 2026-11-02", { placeholder: "body" });
s.addNotes("开场：介绍我们在做什么——在手机上不改游戏，用一个 Vulkan 层做超分和插帧；今天汇报阶段结果，并请教几个问题。");

// 2 一页结论
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "开场" });
s.addText("一页结论：更清楚、更流畅、GPU 更省", { placeholder: "title" });
const stats = [
  ["×3.95", "上屏帧率", "29.8 → 117.8 fps"],
  ["−51%", "GPU 活动时长", "游戏以 0.56× 分辨率渲染"],
  ["37.9 dB", "画质 PSNR 最高", "FSR 36.6 · GSR 31.7"],
  ["第 2", "人眼盲评（图片）", "仅次于原生全分辨率"],
];
stats.forEach(([big, label, sub], i) => {
  const x = 0.5 + i * 2.3;
  card(s, x, 1.4, 2.1, 2.6, C.background2, "stat-card-" + (i + 1));
  T(s, big, { x: x + 0.15, y: 1.6, w: 1.8, h: 0.9, fontSize: 28, bold: true, color: i === 0 ? C.accent1 : C.text1, objectName: "stat-big-" + (i + 1) });
  T(s, label, { x: x + 0.15, y: 2.6, w: 1.8, h: 0.45, fontSize: 15, bold: true, color: C.text1 });
  T(s, sub, { x: x + 0.15, y: 3.1, w: 1.8, h: 0.7, fontSize: 12, color: C.text2, valign: "top" });
});
T(s, "原神默认设置，骁龙旗舰 Android 手机（120 Hz 屏），冷启动 A/B 配对 6 对，全部指标 6/6 同向（p = 0.031）", { x: 0.5, y: 4.3, w: 9, h: 0.5, fontSize: 12, color: C.text2 });
s.addNotes("四个数：帧率接近 4 倍；GPU 工作时间减半；离线画质指标在三种放大方法里最高；人眼盲评里仅次于原生。");

// 3 做法
pres.addSection({ title: "方法" });
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "方法" });
s.addText("做法：一个挂在游戏上的 Vulkan 层", { placeholder: "title" });
const steps = [
  ["1", "低分渲染", "系统游戏模式让游戏以 0.56× 分辨率渲染（1505×681）"],
  ["2", "层内超分", "Catmull-Rom 放大 + RCAS 锐化，输出 2688×1216"],
  ["3", "层内插帧", "块匹配估计运动，在两帧真实帧之间合成一帧"],
  ["4", "120 Hz 上屏", "合成帧走独立子图层，约 118 fps"],
];
steps.forEach(([n, head, body], i) => {
  const x = 0.5 + i * 2.3;
  s.addShape(pres.shapes.OVAL, { x: x, y: 1.45, w: 0.55, h: 0.55, fill: { color: C.accent2 }, line: { color: C.accent2 }, objectName: "step-dot-" + n });
  T(s, n, { x: x, y: 1.45, w: 0.55, h: 0.55, fontSize: 16, bold: true, color: C.background1, align: "center", valign: "middle" });
  if (i < 3) s.addShape(pres.shapes.LINE, { x: x + 0.65, y: 1.725, w: 1.55, h: 0, line: { color: HEX.accent5, width: 1.5, endArrowType: "triangle" }, objectName: "step-arrow-" + n });
  T(s, head, { x: x, y: 2.15, w: 2.1, h: 0.45, fontSize: 17, bold: true, color: C.text1 });
  T(s, body, { x: x, y: 2.65, w: 2.0, h: 1.2, fontSize: 13, color: C.text2, valign: "top" });
});
card(s, 0.5, 4.05, 9.0, 0.85, C.background2, "method-note");
T(s, "不修改游戏、不改游戏内设置；层在运行时注入，结束后完整还原。需要 root。已在原神和 Unity《Megacity Metro》上验证。", { x: 0.7, y: 4.1, w: 8.6, h: 0.75, fontSize: 13, color: C.text1, valign: "middle" });
s.addNotes("关键点：省 GPU 来自低分渲染，清晰来自超分，流畅来自插帧。三步都在一个 Vulkan 层里完成，游戏无感。");

// 4 平台
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "方法" });
s.addText("研究平台：人与 AI agent 在真机上协作", { placeholder: "title" });
const boxes = [
  ["人", "网页提任务、给指导、做盲评"],
  ["外部 AI agent", "命令行认领任务、设计实验、写报告"],
  ["节点服务", "任务排队、设备锁、结果与证据"],
  ["真机", "Android / HarmonyOS，USB 直连"],
];
boxes.forEach(([head, body], i) => {
  const y = 1.3 + i * 0.9;
  card(s, 0.5, y, 4.2, 0.75, i === 2 ? C.text1 : C.background2, "platform-box-" + (i + 1));
  T(s, head, { x: 0.7, y: y + 0.05, w: 1.6, h: 0.65, fontSize: 15, bold: true, color: i === 2 ? C.background1 : C.text1, valign: "middle" });
  T(s, body, { x: 2.3, y: y + 0.05, w: 2.3, h: 0.65, fontSize: 12, color: i === 2 ? C.accent5 : C.text2, valign: "middle" });
});
T(s, "平台保证结论可信", { x: 5.2, y: 1.3, w: 4.3, h: 0.45, fontSize: 18, bold: true, color: C.text1 });
T(s, [
  { text: "每臂冷启动，A/B 交替配对，精确符号检验", options: { bullet: true, breakLine: true } },
  { text: "同一台手机同一时刻只有一个写者", options: { bullet: true, breakLine: true } },
  { text: "画质由人盲评，作答后才揭晓来源", options: { bullet: true, breakLine: true } },
  { text: "每个数字都附任务号，可一条命令复现", options: { bullet: true } },
], { x: 5.2, y: 1.85, w: 4.3, h: 2.6, fontSize: 14, color: C.text2, paraSpaceAfter: 10, valign: "top" });
s.addNotes("这些结果不是手工测的：AI agent 在平台上设计和执行实验，人只在需要主观判断时介入（盲评）。");

// 5 画质对比
pres.addSection({ title: "结果" });
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "结果" });
s.addText("画质：和业界基线比，自研超分最接近原生", { placeholder: "title" });
s.addChart(pres.charts.BAR, [{
  name: "PSNR (dB)",
  labels: ["高通 GSR（SGSR1）", "AMD FSR1 默认", "AMD FSR1 同强度", "双线性", "自研超分"],
  values: [31.72, 35.83, 36.62, 36.96, 37.89],
}], {
  x: 0.5, y: 1.25, w: 5.6, h: 3.7, barDir: "bar",
  chartColors: [HEX.accent5, HEX.accent5, HEX.accent5, HEX.accent5, HEX.accent1],
  showValue: true, dataLabelPosition: "outEnd", dataLabelFormatCode: "0.0", dataLabelFontSize: 11, dataLabelColor: HEX.dk1, dataLabelFontFace: "+mn-lt",
  valAxisMinVal: 30, valAxisMaxVal: 39, valAxisHidden: true, valGridLine: { style: "none" },
  catAxisLabelColor: HEX.dk2, catAxisLabelFontSize: 11, catAxisLabelFontFace: "+mn-lt", catGridLine: { style: "none" },
  showLegend: false, showTitle: true, title: "与原生比的 PSNR（dB，越高越好）", titleFontSize: 12, titleColor: HEX.dk2, titleFontFace: "+mn-lt",
});
const notes = [
  ["自研超分", "无锯齿、无描边，观感最接近原生", C.accent1],
  ["AMD FSR1", "边缘最锐，但文字和图标外有一圈描边", C.text1],
  ["高通 GSR", "斜线阶梯锯齿，平坦区域有条纹", C.text1],
];
notes.forEach(([head, body, color], i) => {
  const y = 1.35 + i * 1.15;
  T(s, head, { x: 6.4, y, w: 3.1, h: 0.4, fontSize: 15, bold: true, color });
  T(s, body, { x: 6.4, y: y + 0.42, w: 3.1, h: 0.6, fontSize: 12, color: C.text2, valign: "top" });
});
T(s, "同一帧：原生 2141×969 缩到 1505×681 再放大回来，离线计算", { x: 6.4, y: 4.6, w: 3.1, h: 0.45, fontSize: 10, color: C.accent3 });
s.addNotes("双线性 PSNR 高是因为它糊而不出错；人眼看它最差。FSR 有两档：默认锐化，以及和我们相同锐化强度。");

// 6 盲评
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "结果" });
s.addText("人眼盲评：不知道来源，按好坏排序", { placeholder: "title" });
const rank = (x, head, items) => {
  T(s, head, { x, y: 1.3, w: 4.2, h: 0.45, fontSize: 17, bold: true, color: C.text1 });
  items.forEach(([name, gap, hi], i) => {
    const y = 1.9 + i * 0.75;
    card(s, x, y, 4.2, 0.6, hi ? C.accent1 : C.background2, head + "-rank-" + (i + 1));
    T(s, String(i + 1), { x: x + 0.2, y, w: 0.4, h: 0.6, fontSize: 16, bold: true, color: C.text1, valign: "middle" });
    T(s, name, { x: x + 0.65, y, w: 2.3, h: 0.6, fontSize: 14, bold: !!hi, color: C.text1, valign: "middle" });
    if (gap) T(s, gap, { x: x + 2.95, y, w: 1.1, h: 0.6, fontSize: 11, color: C.text2, align: "right", valign: "middle" });
  });
};
rank(0.5, "图片：同一帧四种处理", [
  ["原生全分辨率", "差距小 ↓", false],
  ["自研超分", "差距中 ↓", true],
  ["高通 GSR（SGSR1）", "差距中 ↓", false],
  ["双线性放大", "", false],
]);
rank(5.3, "视频：同一场景四种管线", [
  ["超分 + 插帧", "差距大 ↓", true],
  ["其余三种管线", "", false],
]);
T(s, "超分 + 插帧约 118 fps；其余三种（原生、双线性、仅超分）都在约 30 fps，彼此差距中到小。流畅度差异一眼可辨", { x: 5.3, y: 3.5, w: 4.2, h: 0.8, fontSize: 12, color: C.text2, valign: "top" });
s.addNotes("盲评在平台上做：人只看到 A/B/C/D，排完序才揭晓来源。图片上自研超分仅次于原生，视频上超分加插帧第一，差距'大'。");

// 7 性能与代价
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "结果" });
s.addText("性能与代价：帧率近 4 倍，功耗也升高", { placeholder: "title" });
const hdr = { bold: true, color: HEX.lt1, fill: { color: HEX.dk1 }, fontSize: 13 };
const row = (a, b, c, d, good) => [
  { text: a, options: { fontSize: 13, color: HEX.dk1 } },
  { text: b, options: { fontSize: 13, color: HEX.dk2, align: "right" } },
  { text: c, options: { fontSize: 13, color: HEX.dk2, align: "right" } },
  { text: d, options: { fontSize: 13, bold: true, color: good ? "B7791F" : HEX.accent4, align: "right" } },
];
s.addTable([
  [{ text: "指标", options: hdr }, { text: "原生", options: { ...hdr, align: "right" } }, { text: "超分 + 插帧", options: { ...hdr, align: "right" } }, { text: "变化", options: { ...hdr, align: "right" } }],
  row("上屏帧率", "29.8 fps", "117.8 fps", "+295%", true),
  row("游戏渲染帧率", "—", "—", "+99.5%", true),
  row("GPU 活动时长", "—", "—", "−51%", true),
  row("帧时 p95", "—", "—", "−46%", true),
  row("画面锐度", "—", "—", "+236%", true),
  row("整机功耗", "2.94 W", "7.82 W", "+166%", false),
], { x: 0.5, y: 1.3, w: 5.8, colW: [1.9, 1.2, 1.4, 1.3], rowH: 0.42, border: { type: "solid", pt: 0.5, color: HEX.lt2 }, fill: { color: HEX.lt1 }, fontFace: FONT, valign: "middle" });
card(s, 6.6, 1.3, 2.9, 2.95, C.background2, "cost-note");
T(s, "怎么看功耗", { x: 6.8, y: 1.45, w: 2.5, h: 0.45, fontSize: 15, bold: true, color: C.text1 });
T(s, "原生游戏在默认设置下限 30 帧。功耗增加，主要因为上屏帧数变成了 4 倍；折算到每帧，功耗反而下降。", { x: 6.8, y: 1.95, w: 2.5, h: 2.2, fontSize: 12, color: C.text2, valign: "top" });
T(s, "6 对冷启动配对，全部指标 6/6 同向（p = 0.031）", { x: 0.5, y: 4.4, w: 5.8, h: 0.4, fontSize: 10, color: C.accent3 });
s.addNotes("每帧功耗：原生 2.94/29.8 ≈ 0.099 W，超分插帧 7.82/117.8 ≈ 0.066 W。代价是绝对功耗和温度上升，下一页讲。");

// 8 问题
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "结果" });
s.addText("遇到的问题", { placeholder: "title" });
const issues = [
  ["系统合成节律", "已解决", C.accent2, "约一半冷启动卡在 90 fps。根因是系统合成器等待游戏帧的机制；关闭该等待后 6/6 稳定在约 118 fps。"],
  ["高温节流", "待解决", C.accent4, "30 分钟连续运行，电池升到 56 °C 以上时降到约 74 fps，回落到 55 °C 以下即恢复。持续满帧需要外部散热。"],
  ["换游戏、换手机", "进行中", C.accent1, "Unity《Megacity Metro》已验证。另一品牌手机上层能加载，性能还没测准。"],
];
issues.forEach(([head, tag, color, body], i) => {
  const x = 0.5 + i * 3.05;
  card(s, x, 1.3, 2.85, 2.7, C.background2, "issue-card-" + (i + 1));
  s.addShape(pres.shapes.ROUNDED_RECTANGLE, { x: x + 0.2, y: 1.5, w: 0.95, h: 0.38, rectRadius: 0.08, fill: { color }, line: { color }, objectName: "issue-tag-" + (i + 1) });
  T(s, tag, { x: x + 0.2, y: 1.5, w: 0.95, h: 0.38, fontSize: 12, bold: true, color: C.background1, align: "center", valign: "middle" });
  T(s, head, { x: x + 0.2, y: 2.05, w: 2.45, h: 0.5, fontSize: 16, bold: true, color: C.text1 });
  T(s, body, { x: x + 0.2, y: 2.6, w: 2.45, h: 1.95, fontSize: 12, color: C.text2, valign: "top" });
});
s.addNotes("节律双峰与温度无关，冷启动随机定型；高温节流随温度可逆，两者机制不同。");

// 9 下一步
pres.addSection({ title: "下一步" });
s = pres.addSlide({ masterName: "CONTENT", sectionTitle: "下一步" });
s.addText("下一步", { placeholder: "title" });
const next = [
  ["高低清配对数据集", "采集方案（原理、软硬件、流程）正在编写"],
  ["学习型超分", "用配对数据训练轻量模型，和现有着色器对比"],
  ["基线真机测量", "FSR / GSR 在手机上的帧率与功耗"],
  ["内部 Demo 环境", "两台 PC 加一台 root 手机，部署手册已完成"],
];
next.forEach(([head, body], i) => {
  const y = 1.3 + i * 0.9;
  s.addShape(pres.shapes.OVAL, { x: 0.5, y: y + 0.1, w: 0.5, h: 0.5, fill: { color: C.text1 }, line: { color: C.text1 }, objectName: "next-dot-" + (i + 1) });
  T(s, String(i + 1), { x: 0.5, y: y + 0.1, w: 0.5, h: 0.5, fontSize: 14, bold: true, color: C.background1, align: "center", valign: "middle" });
  T(s, head, { x: 1.2, y, w: 3.0, h: 0.7, fontSize: 16, bold: true, color: C.text1, valign: "middle" });
  T(s, body, { x: 4.2, y, w: 5.3, h: 0.7, fontSize: 13, color: C.text2, valign: "middle" });
});
s.addNotes("数据集是学习型超分的前提，也是想听赵老师意见的重点。");

// 10 请教
s = pres.addSlide({ masterName: "TITLE_DARK", sectionTitle: "下一步" });
s.addText("想请教赵老师", { placeholder: "title" });
T(s, [
  { text: "移动端学习型超分：模型规模与时延的合理区间？", options: { bullet: true, breakLine: true } },
  { text: "插帧的时域稳定性：HUD、遮挡区域怎么处理更好？", options: { bullet: true, breakLine: true } },
  { text: "高低清配对数据：引擎内双分辨率渲染与截图配对，哪种更可信？", options: { bullet: true, breakLine: true } },
  { text: "CG 开源框架：合作和开源的切入点在哪里？", options: { bullet: true } },
], { x: 0.6, y: 3.0, w: 8.8, h: 2.0, fontSize: 15, color: C.background1, paraSpaceAfter: 8, valign: "top" });
s.addNotes("留出主要时间讨论这四个问题。");

pres.writeFile({ fileName: OUT }).then(() => applyTheme(OUT, THEME)).then(() => console.log("wrote", OUT));
