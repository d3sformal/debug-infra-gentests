## Not started
### Trivial
- [ ] Pick an UI framework: https://github.com/JetBrains/jewel?tab=readme-ov-file
- [ ] Make DISL_PATH from Gradle accessible from the code
- [ ] Remove all generated sources before building (via Gradle task)
- [ ] Move serialization into separate packages, leave them in the project (do not delete them)
- [ ] Configure Gradle to compile test resources into separate JARs
- [ ] Use jvm options to pass in classpath to ShadowVM
### Tests
- [ ] Create examples from the dacapo benchmark
- [ ] Test effects of field writes
### DiSL Integration
- [ ] Represent Trace as an interface that allows to query values and method that were called at a certain point in time - if there's nothing to return, handle it as the Optional monad
- [ ] DiSL scope annotation - see how to inject scope based on a value of a variable
- [ ] Allow the user to specify field names using wildcards
- Work with different types of variables
	- [ ] Objects
- Generating tests based on values - all parameter values used for running tests
	- [ ] Static methods
	- [ ] Instance methods
		- [ ] Setup an object based on field values etc.
- Running generated tests
	- [ ] Use junit API to run tests from Java
### Writing
- [ ] Write a section on Gradle
	- [ ] Spring selection - yes/no
	- [ ] UI framework selection
- Write down a section of encountered problems
	- [ ] Modeling (metaclasses etc.)
	- [ ] Exporting values with non-primitive types
		- [ ] External types specific to just the target app
	- [ ] Tech debt
	- [ ] Development testing  
	- [ ] Access to sources
		- [ ] Makes things easier, for example getting fields from a class makes use of imports
- Chapter 1
	- [ ] Create and show an example for "For example, bugs may arise only under specific conditions, such as particular combinations of input values or sequences of interactions between various system parts, making it challenging to pinpoint and replicate the exact scenario that triggers the error." in paragraph 2
	- [ ] Add references for runtime monitoring overhead issues
	- [ ] Add references for issues with multithreaded/concurrent debugging
- Chapter 2
	- [ ] Address review comments 
	- [ ] Pridat priklady
		- az budu delat evaluation na nejakem konkretnim prikladu, tak muzu ukazat, jak jsou slozite codebases nachylne na chybu
		- nebo pripadne zminit chybu jako CrowdStrike
- Chapter 3
	- [ ] Address review comments 
- Chapter 4
	- [ ] Add references
		- asm4 paper
			- https://www.baeldung.com/java-asm
			- https://github.com/Baeldung/test-history-8/blob/6c341beb1aae58ab53dc1da5ef2e771036cf0117/libraries-bytecode/src/main/java/com/baeldung/asm/CustomClassWriter.java#L91
		- https://svenruppert.com/2025/04/11/open-hearted-bytecode-java-instrumentation-api/
		- The ROADRUNNER Dynamic Analysis Framework for Concurrent Programs
			- Atomizer: A dynamic atomicity checker for multithreaded programs.
				- ^ example provided on compactness of RoadRunner programs
		- DiSL: A Domain-Specific Language for Bytecode Instrumentation
		- ShadowVM: Robust and Comprehensive Dynamic Program Analysis for the Java Platform
	- [ ] Address review comments 
- Chapter 5
	- [ ] V sekci 5.3 popsat jen základní myšlenku generování testů v naivní variantě
	- [ ] Udělat další sekci/kapitolu, ve které popisu pokročilejší nápady
		- Idea je seznámit čtenáře s celou pipeline a až pak ukázat lepší postupy
## Done
### Trivial
- [x] create a repo
- [x] move notes to the repo
- [x] Mock all parts of the project
- [x] rename packages to cz.cuni.mff
- [x] Create a step-by-step list for all the required features of the project (backlog)
### Tests
- [x] Add UTs for code generation
### DiSL Integration
- [x] Explore ShadowVM for ShadowObject API and RemoteAnalysis
- [x] Map variable names to slot indices
- [x] Method parameters
	- At the time of the method call
- [x] Return values
- [x] Field writes
	- Don't care about the line numbers of the writes, only the possible values 
- [x] Static field writes
	- Don't care about the line numbers of the writes, only the possible values 
- [x] Sending values from `Instrumentation` to `auto-debugger`
- Enhance the instrumentation 
	- [x] Run instrumentation before and after a method call
	- Work with different types of variables
		- [x] Primitives
		- [x] Strings
	- [x] Take values of fields of an object
- [x] Generating tests based on values - call the instrumented method with a value
- [x] Take the return value of the method
### Writing
- [x] design and document the possible skeleton of the project
- [x] Create a PNG folder in notes for individual diagrams
- [x] Take a template (from somebody or from the bachelor thesis)
- [x] Send a link with invite 
- [x] Use official template instead of the alternative one
- Chapter 1
	- [x] Give the outline of the paper, define goals
- Chapter 2
	- [x] Introduce the domain
- Chapter 3
	- [x] Add deterministic techniques
	- [x] Describe LLM/AI usage in generating tests
	- [x] Add runtime values usage section 
- Chapter 4
	- [x] find more generic information for Java Instrumentation API, add more text before the example
		- [x] explain the motivation of the example, put it into a section/paragraph
	- [x] get the paper of ASM, add more text before the example (motivation, capabilities)
		- [x] explain the motivation of the example, put it into a section/paragraph
	- [x] get the paper of DiSL, add more text before the example
		- [x] explain the motivation of the example, put it into a section/paragraph
	- [x] get the paper of ShadowVM, add more text before the example
		- [x] explain the motivation of the example, put it into a section/paragraph
	- [x] add RoadRunner section
	- [x] get the paper of RoadRunner, add more generic text and explanation
	- [x] add example of RoadRunner, put it into a section/paragraph


Use of AI in test generation (possibility):
```
In summary, AI-driven test generation methods tend to produce more _human-friendly_ tests and can leverage learning to target complex input patterns, whereas traditional methods provide more _systematic guarantees_ of coverage or correctness but often at the expense of readability or requiring manual effort to interpret results. The best outcomes in research often come from **hybrid approaches** – using AI to guide or post-process traditional techniques. For instance, an LLM might be used to suggest assertions for tests generated by a coverage tool, or symbolic execution results might be used to prompt an LLM with specific scenarios to write tests for. The landscape is moving towards integrating these techniques to harness their complementary strengths.
```



