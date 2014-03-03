all: build run

build:
	tup upd
	@echo "---------"

run:
	@echo 
	
	@RUST_LOG=debug ./bin/equinox
	
	@echo
	@echo "---------"
	@echo