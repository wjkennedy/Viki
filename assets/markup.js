// Extend inline tokens only: Markdown protects code, escapes and raw HTML blocks.
const escapeHtml = s => s.replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
marked.use({extensions:[{
 name:'wiki',level:'inline',start:s=>s.indexOf('[['),
 tokenizer(s){const m=/^\[\[([^\]\n]+)\]\]/.exec(s);if(m)return{type:'wiki',raw:m[0],value:m[1]};},
 renderer(t){const [target,...label]=t.value.split('|');return '<a href="#" data-wiki="'+escapeHtml(target)+'">'+escapeHtml(label.length?label.join('|'):target)+'</a>';}
},{name:'wikiExternal',level:'inline',
 tokenizer(s){const m=/^\[(https?:\/\/[^\s\]]+)(?:\s+([^\]]+))?\]/.exec(s);if(m)return{type:'wikiExternal',raw:m[0],url:m[1],label:m[2]||m[1]};},
 renderer(t){return '<a href="'+escapeHtml(t.url)+'">'+escapeHtml(t.label)+'</a>';}
}]});
function renderNote(source, note, all, navigate, create){
 const root=document.createElement('div');
 root.innerHTML=DOMPurify.sanitize(marked.parse(source),{FORBID_TAGS:['style','form','input','button'],FORBID_ATTR:['style']});
 const ids=new Map();
 root.querySelectorAll('h1,h2,h3,h4,h5,h6').forEach(h=>{let base=h.textContent.trim().replace(/\s+/g,'_');let count=ids.get(base)||0;ids.set(base,count+1);h.id=base+(count?'_'+(count+1):'');});
 root.querySelectorAll('a[data-wiki]').forEach(a=>{
  const [title,...parts]=a.dataset.wiki.split('#');const fragment=parts.join('#');
  const matches=title.trim()?all.filter(n=>n.title.trim()===title.trim()):[note];
  a.onclick=e=>{e.preventDefault();if(matches.length===1){navigate(matches[0]);if(fragment)document.getElementById(fragment.replace(/\s+/g,'_'))?.scrollIntoView();}else if(!matches.length)create(title.trim());};
  if(!matches.length){a.title='Missing page — click to create';a.classList.add('missing');}
  if(matches.length>1){a.title='Ambiguous title: rename duplicate notes';a.setAttribute('aria-disabled','true');}
 });
 return root;
}
