import http.server
import socketserver

class VerboseHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        print(f"Request: {self.client_address[0]} - {self.requestline}")
        super().log_message(format, *args)

PORT = 3000
with socketserver.TCPServer(("", PORT), VerboseHandler) as httpd:
    print(f"Serving HTTP on 0.0.0.0 port {PORT} (http://0.0.0.0:{PORT}/)...")
    httpd.serve_forever()
