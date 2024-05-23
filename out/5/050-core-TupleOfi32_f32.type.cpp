extern "C" {
	// std::pair<int, float>::new(Primitive, Primitive) generated
	// ("std::pair<int, float>::new", vec![(pred!(const, ["arg", "arg_1"], ["int", "float"]), _)]),
	std::pair<int, float>* std_pairLint__floatG_new_const_int_float(int arg, float arg_1) {
			std::pair<int, float>* ret = new std::pair<int, float>(arg, arg_1);
			return ret;
	}

	// std::pair<int, float>::get_0() generated
	// ("std::pair<int, float>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLint__floatG_get_0_const(const std::pair<int, float>* instance, int* ocvrs_return) {
			int ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, float>::get_1() generated
	// ("std::pair<int, float>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLint__floatG_get_1_const(const std::pair<int, float>* instance, float* ocvrs_return) {
			float ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<int, float>::delete() generated
	// ("std::pair<int, float>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLint__floatG_delete(std::pair<int, float>* instance) {
			delete instance;
	}

}

