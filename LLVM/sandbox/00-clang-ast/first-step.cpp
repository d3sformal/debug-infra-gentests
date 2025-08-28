#include <iostream>

namespace __framework {

    class Reporter {
        public:
        template<class T>
        static void report(T val, const char* info) {
            std::cout << "recorded value: " << val << " " << info << '\n';
        }
    };
}
    

int int_called_with_int_float(int i, float f) {
    return i * f;
}


float float_called_with_double_int(double d, int i) {
    return d * i;
}

int everything() {
    return int_called_with_int_float(0, 3.2f) + float_called_with_double_int(4.4, 32);
}

int main() {
    return everything();
}