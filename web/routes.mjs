const repository = 'https://github.com/flockcap/cantrip';
export function route(request, scripts) {
  const url = new URL(request.url);
  const headers = { 'x-content-type-options':'nosniff', 'cache-control':'no-store', 'referrer-policy':'no-referrer' };
  if (!['GET','HEAD'].includes(request.method)) return new Response('method not allowed\n', { status:405, headers:{...headers,allow:'GET, HEAD'} });
  if (url.pathname === '/install' || url.pathname === '/install.ps1') {
    const ps = url.pathname.endsWith('.ps1') || ['powershell','pwsh'].includes(url.searchParams.get('shell')) || /powershell/i.test(request.headers.get('user-agent') || '');
    return new Response(request.method === 'HEAD' ? null : ps ? scripts.powershell : scripts.shell, { headers:{...headers,'content-type':'text/plain; charset=utf-8', vary:'User-Agent'} });
  }
  if (url.pathname === '/health') return Response.json({ product:'cantrip', status:'ok' }, { headers });
  return new Response(null, { status:302, headers:{...headers,location:repository} });
}
