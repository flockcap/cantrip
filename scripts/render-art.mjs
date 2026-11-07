// Seeded, code-native brand furniture. Run: node scripts/render-art.mjs
import { createServer } from 'node:http';
import { readFileSync, writeFileSync } from 'node:fs';
import puppeteer from 'puppeteer-core';
import { Script } from 'node:vm';
import { resolve, join, extname } from 'node:path';
const root = resolve('.');
const mark = readFileSync('assets/flock-mark.svg', 'utf8').replace('aria-hidden="true"', 'color="#ebdbb2"');
const logo = mark.replace('viewBox="6 9 50 50"', 'viewBox="0 0 64 64" width="512" height="512"').replace('currentColor', '#ebdbb2').replace('currentColor', '#ebdbb2');
writeFileSync('assets/logo.svg', logo);
const html = `<!doctype html><html><head><meta charset="utf-8"><style>
@font-face{font-family:Etoile;src:url(/assets/fonts/IosevkaEtoile-SemiBold.woff2)}
@font-face{font-family:Term;src:url(/assets/fonts/IosevkaTerm-Regular.woff2)}
@font-face{font-family:Aile;src:url(/assets/fonts/IosevkaAile-Regular.woff2)}
*{box-sizing:border-box}html,body{margin:0;background:#1d2021;overflow:hidden}canvas{display:block}
</style></head><body><canvas id="art"></canvas><script>
const c=document.querySelector('canvas'),ctx=c.getContext('2d');
const type=new URLSearchParams(location.search).get('type')||'social';
c.width=type==='banner'?1600:type==='logo'?512:1280;c.height=type==='banner'?420:type==='logo'?512:640;
const C={bg:'#1d2021',surface:'#282828',fg:'#ebdbb2',muted:'#a89984',faint:'#665c54',line:'#3c3836',orange:'#fe8019',aqua:'#8ec07c',green:'#b8bb26'};
function text(s,x,y,font,color=C.fg){ctx.font=font;ctx.fillStyle=color;ctx.fillText(s,x,y)}
function line(x,y,w,color=C.line){ctx.fillStyle=color;ctx.fillRect(x,y,w,1)}
function star(x,y,r){ctx.strokeStyle=C.orange;ctx.lineWidth=2;ctx.beginPath();ctx.moveTo(x-r,y);ctx.lineTo(x+r,y);ctx.moveTo(x,y-r);ctx.lineTo(x,y+r);ctx.stroke()}
function fleece(x,y,size){ctx.save();ctx.translate(x,y);ctx.scale(size/50,size/50);ctx.translate(-6,-9);ctx.fillStyle=C.fg;for(const [x,y,r]of [[27,34,10],[41,30,11],[43,42,9],[30,44,8]]){ctx.beginPath();ctx.arc(x,y,r,0,Math.PI*2);ctx.fill()}ctx.fillStyle=C.orange;ctx.beginPath();ctx.arc(47,26,3.4,0,Math.PI*2);ctx.fill();ctx.restore()}
function dither(cx,cy,r){
  let seed=0xcab7;const rand=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296};
  const cell=3,w=Math.ceil(c.width/cell),h=Math.ceil(c.height/cell),v=new Float32Array(w*h);
  for(let y=0;y<h;y++)for(let x=0;x<w;x++){
    const dx=x*cell-cx,dy=y*cell-cy,dist=Math.hypot(dx,dy),angle=Math.atan2(dy,dx);
    const ring=Math.exp(-Math.pow((dist-r*(0.7+0.11*Math.sin(3*angle)))/(r*.24),2));
    const light=.15+.38*(1+Math.cos(angle+1.8))/2;
    v[y*w+x]=ring*light+(rand()-.5)*.04;
  }
  ctx.fillStyle=C.muted;
  for(let y=0;y<h;y++){const flip=y%2===1;for(let k=0;k<w;k++){const x=flip?w-1-k:k,i=y*w+x,d=flip?-1:1,on=v[i]>=.5?1:0,e=v[i]-on;if(on)ctx.fillRect(x*cell,y*cell,2,2);if(x+d>=0&&x+d<w)v[i+d]+=e*7/16;if(y+1<h){if(x-d>=0&&x-d<w)v[i+w-d]+=e*3/16;v[i+w]+=e*5/16;if(x+d>=0&&x+d<w)v[i+w+d]+=e/16}}}
}
async function draw(){await Promise.all([document.fonts.load('600 130px Etoile'),document.fonts.load('400 24px Term'),document.fonts.load('400 32px Aile')]);
ctx.fillStyle=C.bg;ctx.fillRect(0,0,c.width,c.height);
if(type==='logo'){const im=new Image();im.src='/assets/logo.svg';await im.decode();ctx.drawImage(im,0,0,512,512);return}
if(type==='banner'){
 dither(1390,210,175);line(48,48,1504);line(48,371,1504);
 fleece(63,113,95);text('flock / terminal tools',196,86,'400 19px Term',C.muted);
 text('cantrip',192,235,'600 138px Etoile');text('Speak plainly. Get the incantation.',198,296,'400 31px Aile',C.muted);
 text('01',49,352,'400 17px Term',C.faint);text('KEYLESS  /  QUIET BY DESIGN',1111,352,'400 17px Term',C.muted);star(1277,122,14);return;
}
// Social: a single quiet, legible frame; actual UI colors, brand typography.
dither(1120,140,225);
fleece(54,44,40);text('flock / terminal tools',110,76,'400 18px Term',C.muted);
text('cantrip',53,218,'600 116px Etoile');
text('Speak plainly.',58,281,'400 32px Aile');text('Get the incantation.',58,320,'400 32px Aile');
line(58,372,420);text('YOUR AGENT. YOUR LOGIN.',58,416,'400 18px Term',C.aqua);
text('No API key. No new subscription.',58,453,'400 24px Aile',C.muted);
const x=558,y=234,w=662,h=277;
ctx.fillStyle=C.surface;ctx.fillRect(x,y,w,h);ctx.strokeStyle='#504945';ctx.lineWidth=1;ctx.strokeRect(x+.5,y+.5,w,h);
text('✦',x+27,y+39,'400 24px Term',C.orange);text('cantrip',x+56,y+39,'400 22px Term',C.aqua);text('◌ by flock',x+w-151,y+39,'400 17px Term',C.muted);
text('wish  find the biggest files here',x+28,y+88,'400 18px Term',C.muted);
text('›',x+28,y+138,'400 24px Term',C.orange);text('du',x+51,y+138,'400 24px Term',C.green);text('-ah . | sort -rh | head -20',x+87,y+138,'400 24px Term');
text('The twenty largest files and directories.',x+28,y+182,'400 18px Term',C.muted);
ctx.fillStyle=C.green;ctx.fillRect(x+28,y+210,41,24);text('LOW',x+33,y+228,'400 14px Term',C.bg);text('enter run · e edit · c copy · q quit',x+86,y+229,'400 16px Term',C.muted);
line(58,557,1162);text('claude / codex / gemini / opencode',58,597,'400 18px Term',C.muted);text('flock.holdings',1011,597,'400 18px Term',C.muted);
}draw().then(()=>{window.__artReady=true}).catch(e=>{window.__artError=String(e)});
</script></body></html>`;
new Script(html.match(/<script>([\s\S]*)<\/script>/)[1]);
const server=createServer((req,res)=>{
 if(req.url.startsWith('/render')){res.setHeader('content-type','text/html');res.end(html);return;}
 try{const p=join(root,decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(root+'/'))throw Error();res.setHeader('Content-Type', {'.svg':'image/svg+xml','.woff2':'font/woff2','.png':'image/png'}[extname(p)]||'application/octet-stream');res.end(readFileSync(p))}catch{res.statusCode=404;res.end()}
});
await new Promise(r=>server.listen(0,'127.0.0.1',r));
const browser = await puppeteer.launch({ executablePath: process.env.CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', headless: true, args: ['--no-sandbox'] });
try {
 for (const [type,w,h,out] of [['logo',512,512,'assets/logo.png'],['banner',1600,420,'assets/banner.png'],['social',1280,640,'assets/social-preview.png']]) {
  const page = await browser.newPage();
  await page.setViewport({ width:w, height:h, deviceScaleFactor:1 });
  await page.goto('http://127.0.0.1:'+server.address().port+'/render?type='+type, { waitUntil:'networkidle0' });
  await page.waitForFunction('window.__artReady || window.__artError', { timeout:20000 });
  const error = await page.evaluate(()=>window.__artError);
  if (error) throw Error(error);
  const colors = await page.evaluate(()=>{
   const data = document.querySelector('canvas').getContext('2d').getImageData(0,0,document.querySelector('canvas').width,document.querySelector('canvas').height).data;
   let ink=0; for(let i=0;i<data.length;i+=4) if(data[i]>100) ink++; return ink;
  });
  if (colors<1000) throw Error(type+' rendered blank');
  await page.screenshot({ path:resolve(out), type:'png' });
  await page.close();
  console.log(out+' '+w+'×'+h+' · '+colors+' ink pixels');
 }
} finally { await browser.close();server.close(); }
