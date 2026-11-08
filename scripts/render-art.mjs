// Seeded, code-native brand furniture. Run: node scripts/render-art.mjs
import { createServer } from 'node:http';
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
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
if(type==='logo'){const im=new Image();im.src='data:image/svg+xml;base64,'+btoa('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="512" height="512" color="#ebdbb2">\n  <defs><mask id="fm-gap"><rect x="0" y="0" width="64" height="64" fill="#fff"/><g fill="#000"><circle cx="27" cy="34" r="12.2"/><circle cx="41" cy="30" r="13.2"/><circle cx="43" cy="42" r="11.2"/><circle cx="30" cy="44" r="10.2"/></g></mask></defs>\n  <g fill="#ebdbb2" opacity=".55" mask="url(#fm-gap)"><circle cx="15" cy="27" r="7"/><circle cx="23" cy="24" r="8"/><circle cx="24" cy="33" r="6.5"/><circle cx="16" cy="35" r="6"/></g>\n  <g fill="#ebdbb2"><circle cx="27" cy="34" r="10"/><circle cx="41" cy="30" r="11"/><circle cx="43" cy="42" r="9"/><circle cx="30" cy="44" r="8"/></g>\n  <circle cx="47" cy="26" r="3.4" fill="var(--accent, #fe8019)"/>\n</svg>\n');await im.decode();ctx.drawImage(im,0,0,512,512);return}
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
}draw();
</script></body></html>`;
const server=createServer((req,res)=>{
 if(req.url.startsWith('/render')){res.setHeader('content-type','text/html');res.end(html);return;}
 try{const p=join(root,decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(root+'/'))throw Error();res.end(readFileSync(p))}catch{res.statusCode=404;res.end()}
});
await new Promise(r=>server.listen(0,'127.0.0.1',r));
const profile=mkdtempSync(join(tmpdir(),'cantrip-art-'));
try{
 for(const [type,w,h,out] of [['logo',512,512,'assets/logo.png'],['banner',1600,420,'assets/banner.png'],['social',1280,640,'assets/social-preview.png']]){
  await promisify(execFile)(process.env.CHROME||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',['--headless=new','--disable-gpu','--hide-scrollbars','--no-sandbox','--force-device-scale-factor=1',
   '--user-data-dir='+profile,'--window-size='+w+','+h,'--virtual-time-budget=5000','--screenshot='+resolve(out),'http://127.0.0.1:'+server.address().port+'/render?type='+type],{maxBuffer:1<<20});
  console.log(out+' '+w+'×'+h);
 }
}finally{server.close();rmSync(profile,{recursive:true,force:true})}
