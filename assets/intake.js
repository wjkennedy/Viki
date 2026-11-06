// Discovery never writes. Only explicitly selected candidates are imported.
const intake=document.createElement('details');
intake.innerHTML='<summary>Import local documents</summary><div class="content"><p>Scan a research folder for .md, .markdown and .txt files. Original files remain unchanged.</p><form id="discover-form"><label>Folder <input id="corpus-root" required placeholder="/path/to/research"></label> <button>Discover</button></form><p id="intake-status" role="status"></p><div id="candidates"></div><button id="import-selected" hidden>Import selected</button></div>';
document.querySelector('main').prepend(intake);
let discoveredRoot='',candidates=[];
async function intakeRequest(endpoint,body){let r=await fetch('/api/'+endpoint,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});let data=await r.json();if(!r.ok)throw Error(data.error||'Intake failed');return data}
$('discover-form').onsubmit=async e=>{e.preventDefault();$('intake-status').textContent='Scanning…';$('import-selected').hidden=true;$('candidates').replaceChildren();try{const data=await intakeRequest('discover',{root:$('corpus-root').value});discoveredRoot=data.root;candidates=data.files;renderTree();$('intake-status').textContent=data.files.length+' candidates (maximum 500). '+data.warnings.join(' ');$('import-selected').hidden=!data.files.some(f=>!f.imported)}catch(err){$('intake-status').textContent=err.message}};
$('import-selected').onclick=async()=>{const paths=Array.from($('candidates').querySelectorAll('input[data-file]:checked')).map(x=>x.value);if(!paths.length){$('intake-status').textContent='Select documents to import.';return}$('import-selected').disabled=true;try{const result=await intakeRequest('import',{root:discoveredRoot,paths});await load();$('intake-status').textContent=result.imported.length+' imported; '+result.skipped.length+' skipped. '+result.warnings.join(' ');for(let checkbox of $('candidates').querySelectorAll('input[data-file]:checked')){checkbox.checked=false;checkbox.disabled=true}syncFolders()}catch(err){$('intake-status').textContent=err.message}finally{$('import-selected').disabled=false}};

function renderTree(){
 const root={dirs:new Map(),files:[]};
 for(const file of candidates){const parts=file.path.split('/');let node=root;for(const part of parts.slice(0,-1)){if(!node.dirs.has(part))node.dirs.set(part,{dirs:new Map(),files:[]});node=node.dirs.get(part)}node.files.push(file)}
 function draw(node,parent){
  for(const [name,child] of [...node.dirs].sort(([a],[b])=>a.localeCompare(b))){
   const folder=el('details'),summary=el('summary'),check=document.createElement('input'),children=el('div','','content');
   folder.open=true;check.type='checkbox';check.dataset.folder='true';check.setAttribute('aria-label','Select folder '+name);
   check.onclick=e=>e.stopPropagation();check.onchange=()=>{for(const c of children.querySelectorAll('input[data-file]:not(:disabled)'))c.checked=check.checked;syncFolders()};
   summary.append(check,document.createTextNode(' '+name+'/'));folder.append(summary,children);draw(child,children);parent.append(folder);
  }
  for(const file of node.files){
   const row=el('details'),summary=el('summary'),check=document.createElement('input');check.type='checkbox';check.dataset.file='true';check.value=file.path;check.disabled=file.imported;check.setAttribute('aria-label','Import '+file.path);check.onclick=e=>e.stopPropagation();check.onchange=syncFolders;
   summary.append(check,document.createTextNode(' '+file.path.split('/').at(-1)+' · '+file.bytes+' bytes'+(file.imported?' · already imported':'')));row.append(summary,el('pre',file.preview,'content'));parent.append(row);
  }
 }
 $('candidates').replaceChildren();draw(root,$('candidates'));syncFolders();
}
function syncFolders(){
 for(const folder of $('candidates').querySelectorAll('input[data-folder]')){
  const files=[...folder.closest('details').querySelectorAll('input[data-file]:not(:disabled)')],selected=files.filter(c=>c.checked).length;
  folder.disabled=!files.length;folder.checked=files.length>0&&selected===files.length;folder.indeterminate=selected>0&&selected<files.length;
 }
}
