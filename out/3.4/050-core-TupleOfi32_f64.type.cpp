extern "C" {
	// std::pair<int, double>::new(Primitive, Primitive) generated
	// ("std::pair<int, double>::new", vec![(pred!(const, ["arg", "arg_1"], ["int", "double"]), _)]),
	std::pair<int, double>* std_pairLint__doubleG_new_const_int_double(int arg, double arg_1) {
			std::pair<int, double>* ret = new std::pair<int, double>(arg, arg_1);
			return ret;
	}

	// std::pair<int, double>::get_0() generated
	// ("std::pair<int, double>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLint__doubleG_get_0_const(const std::pair<int, double>* instance, int* ocvrs_return) {
			int ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, double>::get_1() generated
	// ("std::pair<int, double>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLint__doubleG_get_1_const(const std::pair<int, double>* instance, double* ocvrs_return) {
			double ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, double>::delete() generated
	// ("std::pair<int, double>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLint__doubleG_delete(std::pair<int, double>* instance) {
			delete instance;
	}

}

