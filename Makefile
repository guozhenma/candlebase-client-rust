.PHONY : run compile gen test clean

CARGO=cargo
CARGO_NIGHTLY=rustup run nightly cargo

run: gen compile
	RUST_BACKTRACE=1 ${CARGO} run -- --nocapture

run-nightly: gen compile-nightly
	RUST_BACKTRACE=1 ${CARGO_NIGHTLY} run -- --nocapture

compile:
	${CARGO} build --color=always --all --all-targets

compile-nightly:
	${CARGO_NIGHTLY} build --color=always --all --all-targets

release: gen
	${CARGO} build --release --color=always --all --all-targets && bin/release.sh

release-nightly: gen
	${CARGO_NIGHTLY} build --release --color=always --all --all-targets && bin/release.sh

gen:
	protocol-generator/bin/gen_protocol_code.sh

test:
	RUST_BACKTRACE=1 ${CARGO} test -- --nocapture

test-nightly:
	RUST_BACKTRACE=1 ${CARGO_NIGHTLY} test -- --nocapture

clean:
	protocol-generator/bin/clean_protocol_code.sh
	${CARGO} clean --target-dir=protocol-generator/target
	${CARGO} clean
