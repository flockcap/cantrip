import shell from './install.sh';
import powershell from './install.ps1';
import { route } from './routes.mjs';
export default { fetch(request) { return route(request, { shell, powershell }); } };
