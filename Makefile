FORMAT=elf64

test/%.s: test/%.snek src/main.rs
	cargo run -- $< $@

test/%.run: test/%.s runtime/start.rs
	nasm -f $(FORMAT) $< -o runtime/our_code.o
	ar rcs runtime/libour_code.a runtime/our_code.o
	rustc -L runtime/ runtime/start.rs -o $@

clean:
	rm -f test/*.s test/*.run runtime/*.o runtime/*.a

# This will build and run every .snek file in the test/ folder
test: $(patsubst test/%.snek,test/%.run,$(wildcard test/*.snek))
	@echo "Running all discovered tests..."
	@for f in test/*.run; do \
		echo "--- Running $$f ---"; \
		./$$f || echo "Test failed as expected"; \
	done

.PHONY: clean test