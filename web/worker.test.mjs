import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { route } from './routes.mjs';
const scripts = { shell:readFileSync(new URL('install.sh',import.meta.url),'utf8'), powershell:readFileSync(new URL('install.ps1',import.meta.url),'utf8') };
test('root redirects to the canonical public repository',()=>{
 const r=route(new Request('https://cantrip.fyi/'),scripts);
 assert.equal(r.status,302);assert.equal(r.headers.get('location'),'https://github.com/flockcap/cantrip');
});
test('one installer URL negotiates POSIX and PowerShell; explicit suffix works',async()=>{
 const sh=route(new Request('https://cantrip.fyi/install',{headers:{'User-Agent':'curl/8'}}),scripts);
 assert.match(await sh.text(),/^#!\/bin\/sh/);
 for(const request of [new Request('https://cantrip.fyi/install',{headers:{'User-Agent':'Mozilla/5.0 PowerShell/7.5'}}),new Request('https://cantrip.fyi/install.ps1'),new Request('https://cantrip.fyi/install?shell=powershell')]){
  const r=route(request,scripts);assert.match(await r.text(),/Invoke-RestMethod/);assert.equal(r.headers.get('vary'),'User-Agent');
 }
});
test('installers never cache old versions, HEAD is empty, POST is rejected',async()=>{
 const r=route(new Request('https://cantrip.fyi/install',{method:'HEAD'}),scripts);
 assert.equal(await r.text(),'');assert.equal(r.headers.get('cache-control'),'no-store');
 assert.equal(route(new Request('https://cantrip.fyi/install',{method:'POST'}),scripts).status,405);
 assert.match(scripts.shell,/releases\/latest\/download/);assert.match(scripts.powershell,/releases\/latest\/download/);
});
