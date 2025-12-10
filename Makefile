DAY=$(shell date +%d)

update-tasks:
	@./update.sh ${DAY}

create-cpp:
	@mkdir day${DAY}
	@touch day${DAY}/testinput
	@touch day${DAY}/input
	@touch day${DAY}/notes.md
	@mkdir day${DAY}/part1
	@mkdir day${DAY}/part2
ifeq ($(TITLE),)
	@cp template.cpp day${DAY}/part1/
else
	@cp template.cpp day${DAY}/part1/${TITLE}.cpp
endif

create-rust:
	cargo new day${DAY}
	@mkdir day${DAY}/src/bin
	@cp template.rs day${DAY}/src/bin/part1.rs
	@rm day${DAY}/src/main.rs
	@touch day${DAY}/src/testinput
	@touch day${DAY}/src/input
	@touch day${DAY}/notes.md

rust-run-1:
	@cd day${DAY} && cargo run --bin part1 --quiet

rust-run-2:
	@cd day${DAY} && cargo run --bin part2 --quiet

rust-run:
	@echo "Part 1"
	@cd day${DAY} && cargo run --bin part1 --quiet
	@echo "Part 2"
	@cd day${DAY} && cargo run --bin part2 --quiet

2-cpp:
	@cp day${DAY}/part1/*.cpp day${DAY}/part2/

2-rust:
	@cp day${DAY}/src/bin/part1.rs day${DAY}/src/bin/part2.rs

push:
	@git pull
	@git add .
ifeq ($(PART),)
	@git commit -am "Completed Day ${DAY}"
else
	@git commit -am "Completed day ${DAY} part $(PART)"
endif
	@git push
