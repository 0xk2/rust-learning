run1:
	# rustc ./no_module_code/1_helloworld.rs -o ./bin/1_helloworld
	# ./bin/1_helloworld
	# rustc ./no_module_code/1_debug.rs -o ./bin/1_debug
	# ./bin/1_debug
	rustc ./no_module_code/1_formatter.rs -o ./bin/1_formatter
	./bin/1_formatter