#   -------------------------------------------------------------
#   Nasqueron Datasources
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

PREFIX=/usr/local

CARGO=${HOME}/.cargo/bin/cargo
INSTALL=install
RM=rm -rf

#   -------------------------------------------------------------
#   Main targets
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

all: build

build: target/release

test:
	RUST_TEST_THREADS=1 ${CARGO} test

clean:
	${RM} target

clean-all:
	${CARGO} clean

install: ${PREFIX}/bin/fantoir-datasource ${PREFIX}/bin/language-subtag-registry-datasource ${PREFIX}/bin/rfc-datasource

#   -------------------------------------------------------------
#   Build
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

target/release:
	${CARGO} build --release

#   -------------------------------------------------------------
#   Install
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

${PREFIX}/bin/fantoir-datasource:
	${INSTALL} target/release/fantoir-datasource ${PREFIX}/bin/

${PREFIX}/bin/language-subtag-registry-datasource:
	${INSTALL} target/release/language-subtag-registry-datasource ${PREFIX}/bin/

${PREFIX}/bin/rfc-datasource:
	${INSTALL} target/release/rfc-datasource ${PREFIX}/bin/
