run1:
	# rustc ./no_module_code/1_helloworld.rs -o ./bin/1_helloworld
	# ./bin/1_helloworld
	# rustc ./no_module_code/1_debug.rs -o ./bin/1_debug
	# ./bin/1_debug
	rustc ./no_module_code/1_formatter.rs -o ./bin/1_formatter
	./bin/1_formatter
run2:
	# rustc ./no_module_code/2_primitives.rs -o ./bin/2_primitives
	# ./bin/2_primitives
	# rustc ./no_module_code/2_tuples.rs -o ./bin/2_tuples
	# ./bin/2_tuples
	rustc ./no_module_code/2_arrays_slices.rs -o ./bin/2_arrays_slices
	./bin/2_arrays_slices
run3:
	rustc ./no_module_code/3_custom_types.rs -o ./bin/3_custom_types
	./bin/3_custom_types
run4:
	rustc ./no_module_code/4_variable_binding.rs -o ./bin/4_variable_binding
	./bin/4_variable_binding