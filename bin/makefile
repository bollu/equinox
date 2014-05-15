all: build run

build:
	tup upd
	@echo "---------"

tests:
	rustc   ../src/main.rs -L ../lib/ --test -o equinox-tests
	
	./equinox-tests
	@echo "---------"

bench:
	rustc ../src/main.rs -L ../lib/   --bench -o equinox-tests 
run:
	@echo 
	
	@./equinox
	
	@echo
	@echo "---------"
	@echo

run-debug:
	@echo 
	
	@RUST_LOG=debug ./equinox
	
	@echo
	@echo "---------"
	@echo