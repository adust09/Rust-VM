all:
	test production

test:
	cargo test

production:
	cargo build --release
	strip target/release/iridium
	mv target/debug/iridium /usr/local/bin/
	chmod ugo+x /usr/local/bin/

dev:
	cargo build
	sudo mv target/debug/iridium /usr/local/bin/
	chmod ugo+x /usr/local/bin/
