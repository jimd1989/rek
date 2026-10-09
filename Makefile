.POSIX:
PREFIX = /usr/local

.SUFFIXES:
.PHONY: all install uninstall
all:
	mkdir -p dist
	chicken-csc -O3 -lfa2 src/main.scm -o dist/rek
install:
	mkdir -p $(PREFIX)/bin
	cp dist/rek $(PREFIX)/bin
uninstall:
	rm $(PREFIX)/bin/rek
