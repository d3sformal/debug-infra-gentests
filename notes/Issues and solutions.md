# Input
- Application JAR 
	- Required by DiSL
- Application source code
	- Required for test execution
	- Possibly required by DiSL and ShadowVM for type imports
- Method location
	- (package, class, name, arguments)
	- Required by DiSL
- Target instrumentation values 
	- (argument slots/field identifiers/variable identifiers)
	- Required by DiSL, ShadowVM, and test generation

- [ ] Define a way to pass the locations of methods and identifiers to the tool

# Use of identifiers in tests
Tests in their naive form should set the values of different arguments/fields to those collected during instrumentation. We need to track the values with respect to their original identifier, so that the identifier can be assigned a proper value during test generation. 

![[value-extraction.png]]

We assign an internal ID to each identifier. This ID is an integer, which makes integration with DiSL and ShadowVM easier. Instead of using the identifier specifics for each identifier type, such as:
- Slot number for arguments
- Package name, class name, and field name for fields
we use the internal ID to unify the value collection process across these different identifier types. The mapping needs to be used by different processes, due to ShadowVM/DiSL running as separate processes from the main tool. A naive approach is to serialize the identifier mapping (the way internal IDs correspond to ExportableValue objects) and deserialize it in another process for further use later.

![[processes.png]]

# Generating tests
All generated tests should import packages and types from the provided app's source code. 
- [ ] Create a mechanism that ensures used types are imported 

The techniques and complexity of generated tests should differ to allow for comparison of various methods.
## Naive
Generate tests that execute the target method with given parameter values. For each combination of parameter values in the value cross product, generate a single test. 

This can be made further complicated by setting up the field values of the tested class with the collected values. 

## Deterministic reduction
Instead of running the whole method with different collected values, create tests out of the method's body. In the most naive form, that could be just copying the method's body and assigning collected values to the variables inside of the body. 

## LLM 
- [ ] Experiment with using various LLM suited towards creating tests
- [ ] Try to ensure generated tests are executable and that they "terminate" eventually

# Executing tests
The selected testing framework should have an API that allows for launching tests directly from the tool. This way, we could easily collect and show the results of the execution.
- [ ] Explore junit API options