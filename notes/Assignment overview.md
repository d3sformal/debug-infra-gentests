Generating tests based on the result of the dynamic analysis is the end goal of the project. Trying different ways to generate them (e.g., randomized like in the Rust compiler, through LLM like Copilot) would be interesting.
## Steps in debugging
*(copied from project proposal)*
1. Developer specifies, either manually or with the help of a tool, the set of application procedures and components whose execution should be tracked, and determines also the names of program variables whose runtime values should be recorded (e.g., because they are likely relevant to the root cause of the error in question). Tracking execution of a specific procedure means recording every call of the procedure, together with actual runtime values of its arguments.
2. Fully automated dynamic analysis is used to record all the necessary information during a run of the subject application. The necessary information would include, besides the values of arguments for each call of every tracked procedure, also relevant parts of the internal state of the program (component) — specifically, runtime values of global variables and contents of global data structures. Existing dynamic analysis frameworks will be used for this purpose.
3. A set of test cases is generated automatically for the respective tracked application components and procedures, using information recorded by the dynamic analysis. Specifically, at least one test case has to be generated for each call of every tracked application procedure, because the developer does not know (at the beginning of a debugging session) which argument values trigger the runtime error in question.
4. With the help of tools, developers will execute the generated tests and discover some patterns of differences between passing and failing tests, e.g. in terms of procedure call arguments and input program states, to determine the likely root cause of the runtime error in question.
5. In order to confirm or refute a particular hypothesis about the root cause, the developer will thoroughly inspect the given execution trace where the runtime error has been observed. We plan to provide tool support that will enable developers to move forward and backward over the recorded execution trace and inspect specific executions of tracked procedures (including runtime values of program variables). Executions of the individual generated test cases will be used for this purpose. Stepping over the execution trace will be possible only at the granularity of tracked procedures and components, in the sense that other procedures will be ”invisible”. More precisely, developers will be able to inspect a projection of the full execution trace that shows just procedures specified in the first step, which saves them a lot of time. The respective tool support will allow the developers, for example, to quickly jump to another call (execution) of a specific procedure with different runtime argument values and an input program state.
## Goals
*(copied from project proposal)*
### G1
Design efficient and precise techniques for automatic inference of the names of application procedures and variables that should be tracked (step 1). Since fully automated inference may not be possible in many cases, a practical variant of this goal is to support the following process: first the developers annotate just a small subset of program variables, and then our fully automated technique identifies other variables and procedures into which may flow the runtime values of variables explicitly annotated by developers.
### G2
Extend and optimize the selected dynamic analysis frameworks (e.g., RoadRunner and SharpDetect) to en- able gathering of all the necessary information about runtime events and state during execution of a subject program with sufficient performance (step 2). In particular, we would like to minimize the performance overhead associated with observing and recording information during the run of dynamic analysis.
### G3
Design an efficient procedure that generates test cases for application components and procedures based on information recorded by the dynamic analysis (step 3). The generated set of tests should be as small as possible, while ensuring sufficient coverage of program behavior, to achieve practical (short) execution time of tests within a debugging session.
### G4
Implement tools that enable developers to inspect the recorded execution trace and step over it both in the forward and backward direction (step 5).
## Implementation plan
I need to make a draft/skeleton/design for G1, G2, G3 before continuing on with experimenting with different ways of doing G3.
G4 is not needed initially, but it will be the most useful/visible part of the project.
The functional parts of the project should be mocked or implemented naively.

Multithreading support can be left out of the implementation, but it would be nice to keep it in mind during the design phase. The tool will be used to debug large complex programs, which usually utilize multithreading. If I find a way to support multithreading easily (e.g., I use an analysis framework that deals with multithreading), I should try it and expand on it later.

The goal is to experiment and explore different approaches, not to create a functional and usable tool. The result will be a prototype.

## Workflow
*Input*:
- a large application
	- source code or bytecode, decide on what's more convenient
- a list of procedures/functions/methods
	- this can be in a form of a text file, something very simple
Steps:
1. find given procedures and instrument them
2. launch the program with the instrumentation and collect the result
	- the result could be in logs or an in-memory object if we use the analysis tool's API directly
3. generate tests based on the result of (2)
4. run the tests with different inputs based on the values collected in (2)
Output:
- a set of executable tests that capture the possible states of the application
- results of the generated tests (passed/failed)

More details and insights in [[Detailed workflow]]

## Technologies
I should target applications written in Java. The stack in which I build the debugger in does not matter much, but having it Java as well would be nice, since most instrumentation frameworks for Java are written in Java.

The motivation behind debugging Java applications is that Manta analysis for an entire application runs a long time and we only want to analyze a certain method or part of the application.

While debugging Java is the end goal, the tool should be designed in a way that allows for adding support of multiple languages (e.g., via pluggable analyzers and generators). The design should be modular and I should come up with some standardized data formats for data exchanges. The process of automated debugging is abstract enough to work on (nearly) every language.

Technology selection insights in [[Framework selection]]