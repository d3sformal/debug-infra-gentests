1. Add instrumentation to given method (can be done before runtime, search given procedures for given variables and track their loads and stores - do the same for all fields inside the current class and global variables)
2. (Alt: do it at runtime by checking the call stack: if one of the given procedures is on the call stack, start tracking all the given variables. Be careful to log them properly based on scope)
3. Create the cross product of the tracked values of all variables. The cross product will represent all possible states. We know that at least one of the states will fail UTs
4. Break a given method down based on data flow. The whenever a value is assigned to one of the given variables, a block is created. The blocks end at the end of the procedure. TODO: Can they end sooner?
5. Generate UTs based on the semantics of the code (via Codex or similar), each test requiring the 
6. Run the UTs with the possible 

Issues: 
- Scopes (a given variable might be shadowed by a loop or a branch)
- Everything should ideally be done on the level of bytecode, but I need to figure out if that makes sense 