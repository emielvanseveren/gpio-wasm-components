.PHONY: build-host
build-host:
	@echo "Building host"
	@cargo build --manifest-path ./host/Cargo.toml --release
	
.PHONY: build-guest-rs
build-guest-rs:
	@echo "Building Rust guest"
	@cargo build --manifest-path ./guest-rs/Cargo.toml --target wasm32-wasi

	@echo "Making component"
	@wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o ./guest-rs/guest.component.wasm --adapt wasi_snapshot_preview1=./wasi_snapshot_preview1.reactor.wasm

	@echo "Generating WAT variant"
	@wasm-tools print ./guest-rs/guest.component.wasm > ./guest-rs/guest.component.wat

.PHONY: build-guest-js
build-guest-js:
	@echo "Building JavaScript guest and making component"
	@cd ./guest-js/ && npm run build

	@echo "Generating WAT variant"
	@wasm-tools print ./guest-js/guest.component.wasm > ./guest-js/guest.component.wat


.PHONY: run-large-host-implementation-bench
run-large-host-implementation-bench: build-host
	@echo "Running large host implementation bench"
	@cargo build --manifest-path ./bench/guest-large-host-implementation/Cargo.toml --target wasm32-wasi --release
	@wasm-tools component new ./target/wasm32-wasi/release/guest_large_host_implementation.wasm -o guest.large.host.implementation.component.wasm --adapt wasi_snapshot_preview1=./wasi_snapshot_preview1.reactor.wasm

	@echo "Run 1, 10, 100, 1000 instances simultaneously"
	@valgrind --massif-out-file=../profile/large-host-1.massif --trace-children=yes --stacks=yes --tool=massif ./target/release/host --impl sim --instances 1 ./guest.large.host.implementation.component.wasm
	@valgrind --massif-out-file=../profile/large-host-10.massif --trace-children=yes --stacks=yes --tool=massif ./target/release/host --impl sim --instances 10 ./guest.large.host.implementation.component.wasm
	@valgrind --massif-out-file=../profile/large-host-100.massif --trace-children=yes --stacks=yes --tool=massif ./target/release/host --impl sim --instances 100 ./guest.large.host.implementation.component.wasm
	@valgrind --massif-out-file=../profile/large-host-1000.massif --trace-children=yes --stacks=yes --max-threads=3000 --tool=massif ./target/release/host --impl sim --instances 1000 ./guest.large.host.implementation.component.wasm 

.PHONY: run-timing-bench
run-timing-bench: build-host build-guest-rs
	@echo "Running GPIO sim"
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source rust --instances 1 ./guest-rs/guest.component.wasm > ../profile/timings-rs-1.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source rust --instances 10 ./guest-rs/guest.component.wasm > ../profile/timings-rs-10.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source rust --instances 100 ./guest-rs/guest.component.wasm > ../profile/timings-rs-100.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source rust --instances 1000 ./guest-rs/guest.component.wasm > ../profile/timings-rs-1000.log

	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source javascript --instances 1 ./guest-js/guest.component.wasm > ../profile/timings-rs-1.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source javascript --instances 10 ./guest-js/guest.component.wasm > ../profile/timings-rs-10.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source javascript --instances 100 ./guest-js/guest.component.wasm > ../profile/timings-rs-100.log
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source javascript --instances 1000 ./guest-js/guest.component.wasm > ../profile/timings-rs-1000.log


.PHONY: run-sim-rs
run-sim-rs: build-host build-guest-rs
	@echo "Running GPIO sim RS"
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source rust --instances 1 ./guest-rs/guest.component.wasm


.PHONY: run-linux-rs
run-linux-rs: build-host build-guest-rs

	@echo "Running GPIO linux RS"
	@cargo run --manifest-path ./host/Cargo.toml -- --impl linux --source rust --instances 1 ./guest-rs/guest.component.wasm 


.PHONY: run-sim-js
run-sim-js: build-host build-guest-js
	@echo "Running GPIO sim JS"
	@cargo run --manifest-path ./host/Cargo.toml -- --impl sim --source javascript --instances 1 ./guest-js/guest.component.wasm

.PHONY: run-linux-js
run-linux-js: build-host build-guest-js

	@echo "Running GPIO linux JS"
	@cargo run --manifest-path ./host/Cargo.toml -- --impl linux --source javascript --instances 1 ./guest-js/guest.component.wasm 

