.PHONY: help clean build all

help:
	@echo "Available targets:"
	@echo "  make day_N_example  - Run day N with example input"
	@echo "  make day_N_input		 - Run day N with puzzle input"
	@echo "  make build          - Build all days"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make all_examples   - Run all days with example inputs"
	@echo "  make all_inputs     - Run all days with puzzle inputs"
	@echo ""
	@echo "Example: make day_1_example"

build:
	cargo build --release

clean:
	cargo clean

define DAY_TARGETS
day_$(1)_example:
	@cargo run --release --bin day_$(1) day_$(1)/example.txt

day_$(1)_input:
	@cargo run --release --bin day_$(1) day_$(1)/input.txt
endef

$(foreach day,$(shell seq 1 11),$(eval $(call DAY_TARGETS,$(day))))
