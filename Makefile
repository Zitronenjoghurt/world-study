.PHONY: trace gen-country

trace:
	cargo run --release --features tracy

gen-country:
	cd tools && cargo run --bin generate-country-data