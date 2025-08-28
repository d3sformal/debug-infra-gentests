We put stress on performance when collecting the results of the dynamic analysis. The point of the tool is to debug large and complex applications and these are expected to have a long running time already. Having a significant slow-down on top of that is not desirable.

The dynamic analysis should be done with an existing framework and the framework should be extended/modified if required to fit your needs. Each supported language will have its own framework for dynamic analysis, here we focus on Java. In order to support different analysis tools for different languages, I should create a wrapper or an common interface for the analysis tools.

The selected framework must be abstract enough to hide any issues with the JVM. Instrumenting bytecode in a more low-level way will be complicated since I don't know the nuances of JVM.
## Potential candidates
- JVM TI
- RoadRunner
- ASM
- DiSL
### JVM TI
Pros:
- Allows to control execution (e.g., take different branches, set values of variables), which could lead to better state exploration/coverage if used
Cons:
- Very noticeable impact on performance when instrumenting function calls (TODO: verify)
### RoadRunner
Pros:
- Supports concurrency
- Good for prototyping
	- Instrumentations/tools written with RR are only tens of lines long
- Good level of abstraction
- Should work fine on large applications
Cons:
- The performance might be worse than other frameworks
### ASM
Pros:
- Good performance
- Used in large projects and compilers => tested
Cons:
- Lower level of abstraction compared to other frameworks
### DiSL
Pros:
- Builds on top of ASM
	- Good performance
	- Higher level of abstraction
- AOP approach to instrumentation
- JVM nuances are handled by the framework
Cons:
- No binary distribution
- No active development anymore
## Choosing DiSL
Choosing DiSL should be the safest bet. The performance should be good and the abstraction and usability should be high enough to allow for quick prototyping and ease of use.

MFF's lecture on Performance Evaluation covers DiSL a little bit and the professors in the department contributed to its development, so they should be able to help with technical issues.
## Working with DiSL
The framework does not ship as a binary package and has to be built locally. This is not an issue on the usual `x86_64` systems with GCC. However, on my system with Apple Silicon M1 (`aarch64`), GCC cannot be installed and used easily.

Apple "forces" the use of Clang, which works well for the most part. One can still install GCC through the Homebrew package manager, however, the `aarch64` cask does not work seamlessly and there are issues with glib. As a result, the Ant build of DiSL fails when calling `ld` to create a dynamic library for the DiSL agent. 

This issue could either be solved by cross compiling to `x86_64`, which could then run on Apple Silicon in a translation layer, or by adding support for `aarch64` and compiling with Clang (the preferred compiler).

An easy fix was found when reproducing the steps of the build manually. Adding `aarch64` as a target for the compiler and compiling the problematic dynamic library with `clang` instead of `ld` with increased verbosity allowed me to find what is required to run the linker successfully. Adding an explicit path to the standard library as one of the linker's arguments solved the issue.

DiSL is executed via a Python script. The user simply needs to create a Java file that specifies the instrumentation and the Python script uses that and the given application to run an instrumented execution. Results are printed to stdout and stderr.
