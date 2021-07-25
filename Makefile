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
	if [ -d target ]; then trash target && echo "Deleted the target directory"; fi
	if [ -f Cargo.lock ]; then trash Cargo.lock && echo "Deleted Cargo.lock"; fi
	if [ -f *.sav ]; then trash *.sav && echo "Deleted *sav files"; fi
