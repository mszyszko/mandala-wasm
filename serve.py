#!/usr/bin/python3

import http.server
import socketserver

PORT = 8000
DIRECTORY = "build"


class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)
    extensions_map = {
        '': 'application/octet-stream',
        '.manifest': 'text/cache-manifest',
        '.html': 'text/html',
        '.png': 'image/png',
        '.jpg': 'image/jpg',
        '.svg':	'image/svg+xml',
        '.css':	'text/css',
        '.js': 'application/x-javascript',
        '.wasm': 'application/wasm',
        '.json': 'application/json',
        '.xml': 'application/xml',
    }


class MyServer(socketserver.TCPServer):
    allow_reuse_address = True


httpd = MyServer(("localhost", PORT), HttpRequestHandler)

try:
    print(f"serving at http://localhost:{PORT}")
    httpd.serve_forever()
except KeyboardInterrupt:
    pass
