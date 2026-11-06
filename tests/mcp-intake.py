"""Exercise actual stdio MCP and Rust intake in an isolated corpus/store."""
import json, os, pathlib, subprocess, tempfile
repo=pathlib.Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(prefix='viki-mcp-') as work:
    root=pathlib.Path(work)/'corpus';(root/'research').mkdir(parents=True)
    (root/'research'/'cooling.md').write_text('# Thermal management\n\nLiquid cooling removes waste heat from the processor.\n')
    (root/'research'/'storage.md').write_text('# Local storage\n\nAn embedded database keeps research notes on disk.\n')
    env=dict(os.environ,VIKI_LOCAL_BINARY=str(repo/'target/debug/viki'))
    p=subprocess.Popen(['/opt/homebrew/bin/python3',str(repo/'scripts/mcp.py')],cwd=work,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
    def call(name,args):
        p.stdin.write(json.dumps({'jsonrpc':'2.0','id':1,'method':'tools/call','params':{'name':name,'arguments':args}})+'\n');p.stdin.flush()
        result=json.loads(p.stdout.readline())['result'];assert not result.get('isError'),result
        return json.loads(result['content'][0]['text'])
    try:
        discovery=call('discover_documents',{'root':str(root)})
        assert len(discovery['files'])==2
        args={'root':str(root),'paths':[f['path'] for f in discovery['files']]}
        assert len(call('import_documents',args)['imported'])==2
        assert len(call('import_documents',args)['imported'])==0
        exact=call('search_notes',{'query':'liquid cooling'})
        paraphrase=call('search_notes',{'query':'how do we prevent the chip from overheating?'})
        assert len(exact)==1 and len(paraphrase)==0
        print('MCP: discovered 2, imported 2, repeat import 0; exact retrieval 1, paraphrase retrieval 0. Semantic retrieval is not implemented in MCP.')
    finally:
        p.stdin.close();p.wait(timeout=5)
