extern "C" {
	// std::pair<int, int>::new(Primitive, Primitive) generated
	// ("std::pair<int, int>::new", vec![(pred!(const, ["arg", "arg_1"], ["int", "int"]), _)]),
	std::pair<int, int>* std_pairLint__intG_new_const_int_int(int arg, int arg_1) {
			std::pair<int, int>* ret = new std::pair<int, int>(arg, arg_1);
			return ret;
	}

	// std::pair<int, int>::get_0() generated
	// ("std::pair<int, int>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLint__intG_get_0_const(const std::pair<int, int>* instance, int* ocvrs_return) {
			int ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, int>::get_1() generated
	// ("std::pair<int, int>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLint__intG_get_1_const(const std::pair<int, int>* instance, int* ocvrs_return) {
			int ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, int>::delete() generated
	// ("std::pair<int, int>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLint__intG_delete(std::pair<int, int>* instance) {
			delete instance;
	}

}

