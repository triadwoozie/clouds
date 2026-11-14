#!/usr/bin/env python3
from http.server import HTTPServer, SimpleHTTPRequestHandler
import sys, os

class COOPHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

if __name__ == '__main__':
    web_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(web_dir)
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
    httpd = HTTPServer(('127.0.0.1', port), COOPHandler)
    print(f"Serving {web_dir} at http://127.0.0.1:{port}", flush=True)
    httpd.serve_forever()
