#! /usr/bin/env python
import posixpath
import argparse
import urllib
from urlparse import urlparse
import os

from SimpleHTTPServer import SimpleHTTPRequestHandler
from BaseHTTPServer import HTTPServer

SimpleHTTPRequestHandler.extensions_map['.wasm'] = 'application/wasm'

class RootedHTTPServer(HTTPServer):
    def __init__(self, base_path, *args, **kwargs):
        HTTPServer.__init__(self, *args, **kwargs)
        self.RequestHandlerClass.base_path = base_path


class RootedHTTPRequestHandler(SimpleHTTPRequestHandler):
    def translate_path(self, path):
        path = posixpath.normpath(urllib.unquote(urlparse(path).path))
        words = path.split('/')
        words = filter(None, words)
        path = self.base_path
        for word in words:
            drive, word = os.path.splitdrive(word)
            head, word = os.path.split(word)
            if word in (os.curdir, os.pardir):
                continue
            path = os.path.join(path, word)
        return path


def serve(HandlerClass=RootedHTTPRequestHandler, ServerClass=RootedHTTPServer):
    default_path = os.path.abspath(os.path.dirname(os.path.abspath(__file__)) + "/dist")

    parser = argparse.ArgumentParser()
    parser.add_argument('--port', '-p', default=9000, type=int)
    parser.add_argument('--dir', '-d', default=default_path, type=str)
    args = parser.parse_args()

    server_address = ('', args.port)

    httpd = ServerClass(args.dir, server_address, HandlerClass)

    sa = httpd.socket.getsockname()
    print "Serving HTTP on", sa[0], "port", sa[1], "path", args.dir, "..."
    httpd.serve_forever()


if __name__ == '__main__':
    serve()
