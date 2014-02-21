all: build run

build:
	tup upd
	@echo "---------"

run:
	@echo 
	
	@bin/equinox
	
	@echo
	@echo "---------"
	@echo