#include <vector>

#ifndef TESTHEADERRRR
#define TESTHEADERRRR

inline int headerFunc() { return 14; }

double adder(double x);

inline int vec_size() { return std::vector<char>{'a', 'b'}.size(); }

#endif