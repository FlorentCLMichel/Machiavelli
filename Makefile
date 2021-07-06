debug: 
	cargo rustc --offline --bin machiavelli -- -C prefer-dynamic

test: 
	cargo test --offline

release: 
	cargo rustc --offline --release --bin machiavelli

clean: 
	trash target
	trash Cargo.lock
