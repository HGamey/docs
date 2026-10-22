// Vercel Routing Middleware：整站密码保护。只有密码，没有用户名；用自己的登录页代替浏览器默认弹窗。
// 密码取环境变量 SITE_PASSWORD；登录成功后写入 HttpOnly cookie（密码的 SHA-256 摘要），有效 30 天。
export const config = { matcher: '/:path*' };

const COOKIE = 'hgamey_auth';
const LOGIN_PATH = '/__login';

async function digest(text) {
  const buf = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(`hgamey-deck:${text}`));
  return [...new Uint8Array(buf)].map(b => b.toString(16).padStart(2, '0')).join('');
}

function readCookie(request, name) {
  const raw = request.headers.get('cookie') || '';
  const hit = raw.split(/;\s*/).find(c => c.startsWith(`${name}=`));
  return hit ? decodeURIComponent(hit.slice(name.length + 1)) : '';
}

function safeNext(value) {
  return typeof value === 'string' && value.startsWith('/') && !value.startsWith('//') ? value : '/';
}

function loginPage(next, failed) {
  const html = `<!doctype html>
<html lang="zh-CN">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>HGamey 进展</title>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;500;700&display=swap">
<style>
  :root { --bg: #F8F8F6; --paper: #FFFFFF; --ink: #202020; --ink2: #383A3C; --muted: #92918E; --line: #C4C4C2; --rose: #C76565; --rose-t: #F5E5E3; --navy: #33435D; --blue: #6191AE; }
  * { box-sizing: border-box; }
  html, body { margin: 0; height: 100%; background: var(--bg); color: var(--ink); font-family: "Noto Sans SC", "PingFang SC", "Microsoft YaHei UI", sans-serif; }
  body { display: grid; place-items: center; padding: 16px; }
  main { width: min(420px, 100%); background: var(--paper); border: 1px solid var(--line); border-radius: 6px; padding: 36px 32px 28px; }
  h1 { margin: 0; font-size: 22px; font-weight: 700; }
  .sub { margin: 6px 0 0; padding-bottom: 14px; border-bottom: 1px solid var(--line); color: var(--ink2); font-size: 13px; }
  label { display: block; margin: 26px 0 10px; font-size: 13px; color: var(--muted); }
  .field { position: relative; display: flex; align-items: center; border-bottom: 2px solid var(--ink2); transition: border-color .15s; }
  .field:focus-within { border-color: var(--navy); }
  .field.bad { border-color: var(--rose); }
  .field input { all: unset; flex: 1; min-width: 0; padding: 8px 2px 10px; font-size: 20px; letter-spacing: .18em; color: var(--ink); caret-color: var(--navy); font-family: inherit; }
  .field input::placeholder { color: #C9C7C2; letter-spacing: .05em; font-size: 15px; }
  .eye { all: unset; cursor: pointer; padding: 6px 4px; color: var(--muted); font-size: 12px; }
  .eye:hover, .eye:focus-visible { color: var(--navy); }
  .err { min-height: 20px; margin: 10px 0 0; color: var(--rose); font-size: 13px; }
  .go { all: unset; box-sizing: border-box; display: block; width: 100%; margin-top: 18px; padding: 11px 0; text-align: center; cursor: pointer;
        background: var(--navy); color: #fff; border-radius: 4px; font-size: 15px; font-weight: 500; letter-spacing: .1em; }
  .go:hover { background: #283650; }
  .go:focus-visible { outline: 2px solid var(--blue); outline-offset: 2px; }
  .foot { margin: 22px 0 0; color: var(--muted); font-size: 12px; }
</style>
</head>
<body>
<main>
  <h1>HGamey 进展</h1>
  <p class="sub">手机游戏优化：测量链、NPU 超分、插帧、系统级路线</p>
  <form method="post" action="${LOGIN_PATH}" autocomplete="off">
    <input type="hidden" name="next" value="${next.replace(/"/g, '&quot;')}">
    <label for="pw">访问密码</label>
    <div class="field${failed ? ' bad' : ''}">
      <input id="pw" name="password" type="password" placeholder="输入密码" autofocus required>
      <button class="eye" type="button" aria-label="显示密码" onclick="const i=document.getElementById('pw');const s=i.type==='password';i.type=s?'text':'password';this.textContent=s?'隐藏':'显示';">显示</button>
    </div>
    <p class="err" role="alert">${failed ? '密码不正确' : ''}</p>
    <button class="go" type="submit">进入</button>
  </form>
  <p class="foot">内部资料，请勿转发。</p>
</main>
</body>
</html>`;
  return new Response(html, { status: failed ? 401 : 200, headers: { 'Content-Type': 'text/html; charset=utf-8', 'Cache-Control': 'no-store' } });
}

export default async function middleware(request) {
  const expected = process.env.SITE_PASSWORD;
  if (!expected) return new Response('未配置密码', { status: 500 });
  const token = await digest(expected);
  const url = new URL(request.url);

  if (url.pathname === LOGIN_PATH) {
    if (request.method === 'POST') {
      const form = await request.formData();
      const next = safeNext(form.get('next'));
      if (form.get('password') === expected) {
        return new Response(null, {
          status: 303,
          headers: { Location: next, 'Set-Cookie': `${COOKIE}=${token}; Path=/; Max-Age=2592000; HttpOnly; Secure; SameSite=Lax`, 'Cache-Control': 'no-store' },
        });
      }
      return loginPage(next, true);
    }
    return loginPage(safeNext(url.searchParams.get('next')), false);
  }

  if (readCookie(request, COOKIE) === token) return;

  // 未登录：页面请求显示登录页（地址不变，登录后回到原页面），其他资源返回 401
  const wantsHtml = (request.headers.get('accept') || '').includes('text/html');
  if (wantsHtml) return loginPage(url.pathname + url.search, false);
  return new Response('需要登录', { status: 401, headers: { 'Cache-Control': 'no-store' } });
}
