"""Local stdio MCP adapter; all writes go through the Rust server."""
import json
import sys
import urllib.request
import os
import subprocess

BASE = 'http://127.0.0.1:8787/api/notes'
def api(method='GET', suffix='', body=None):
    req = urllib.request.Request(BASE + suffix, data=json.dumps(body).encode() if body is not None else None,
                                 method=method, headers={'Content-Type': 'application/json'})
    with urllib.request.urlopen(req, timeout=10) as response:
        return json.load(response)

TOOLS = [
    ('discover_documents', 'Preview Markdown/text files under an explicit local folder without importing', {'root': {'type': 'string'}}),
    ('import_documents', 'Import selected relative paths from discovery; preserve existing imports', {'root': {'type': 'string'}, 'paths': {'type': 'array', 'items': {'type': 'string'}}}),
    ('list_notes', 'Read the local notes', {}),
    ('search_notes', 'Keyword search of local notes', {'query': {'type': 'string'}}),
    ('create_note', 'Create a Markdown note', {'title': {'type': 'string'}, 'body': {'type': 'string'}}),
    ('update_note', 'Replace title and Markdown body of a note', {'id': {'type': 'integer'}, 'title': {'type': 'string'}, 'body': {'type': 'string'}}),
    ('delete_note', 'Permanently delete a note and its incoming links', {'id': {'type': 'integer'}}),
]
def dispatch(msg):
    method = msg.get('method')
    if method == 'initialize':
        return {'protocolVersion': '2024-11-05', 'capabilities': {'tools': {}}, 'serverInfo': {'name': 'viki', 'version': '0.1.0'}}
    if method == 'ping': return {}
    if method == 'tools/list':
        return {'tools': [{'name': name, 'description': desc, 'inputSchema': {'type': 'object', 'properties': props, 'required': list(props), 'additionalProperties': False}} for name, desc, props in TOOLS]}
    if method == 'tools/call':
        try:
            params = msg['params']; name = params['name']; args = params.get('arguments', {})
            if os.environ.get('VIKI_LOCAL_BINARY'):
                reply = json.loads(subprocess.check_output([os.environ['VIKI_LOCAL_BINARY'], '--local-call'], input=json.dumps({'tool':name,'arguments':args}).encode(), timeout=60))
                if 'error' in reply: raise ValueError(reply['error'])
                return {'content':[{'type':'text','text':json.dumps(reply['result'])}]}
            if name == 'list_notes': result = api()
            elif name in ('discover_documents', 'import_documents'):
                endpoint = 'discover' if name == 'discover_documents' else 'import'
                req = urllib.request.Request('http://127.0.0.1:8787/api/'+endpoint, data=json.dumps(args).encode(), headers={'Content-Type':'application/json'}, method='POST')
                with urllib.request.urlopen(req, timeout=60) as response: result = json.load(response)
            elif name == 'search_notes': result = [n for n in api() if args['query'].lower() in (n['title']+' '+n['body']).lower()]
            elif name == 'create_note': result = api('POST', body=args)
            elif name in ('update_note', 'delete_note'):
                if not isinstance(args['id'], int) or args['id'] < 1: raise ValueError('Invalid note ID')
                result = api('PUT' if name == 'update_note' else 'DELETE', '/'+str(args['id']), args if name == 'update_note' else None)
            else: raise ValueError('Unknown tool')
            return {'content': [{'type': 'text', 'text': json.dumps(result)}]}
        except Exception as error:
            return {'isError': True, 'content': [{'type': 'text', 'text': str(error)}]}
    raise ValueError('Unknown method')

for line in sys.stdin:
    try:
        msg = json.loads(line)
        if 'id' not in msg: continue
        try: response = {'jsonrpc': '2.0', 'id': msg['id'], 'result': dispatch(msg)}
        except ValueError as error: response = {'jsonrpc': '2.0', 'id': msg['id'], 'error': {'code': -32601, 'message': str(error)}}
    except Exception:
        response = {'jsonrpc': '2.0', 'id': None, 'error': {'code': -32700, 'message': 'Parse error'}}
    print(json.dumps(response), flush=True)
