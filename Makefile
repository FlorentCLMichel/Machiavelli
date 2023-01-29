server: release
	./target/release/server

client: release
	./target/release/client

clippy: 
	cargo clippy --offline

debug: 
	cargo build --offline

test: 
	cargo test --offline

run: release
	./target/release/machiavelli

release: 
	cargo build --offline --release

doc:
	cargo doc --offline

open_doc: doc
	xdg-open ./target/doc/machiavelli/index.html

clean: 
	trash target; trash Cargo.lock; trash *.sav
