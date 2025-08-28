## Running DiSL
DiSL is made out of multiple projects and running an instrumentation requires executing these projects as separate processes.
This is done in the `bin/disl.py` script, which sets up the correct configuration for the desired instrumentation. The script offers a plethora of options for configuring where the processes communicate and where they output data.

There are three processes that DiSL can run: 
- DiSL application, 
- DiSL instrumentation server  
- Shadow VM server launcher

By default, the script starts a DiSL instrumentation server VM and a client application
VM that will be instrumented using DiSL. For instrumentations using remote analysis
execution in Shadow VM, the Shadow VM server needs to be started as well, which needs
to be specified explicitly.

The user provides two Java applications (`*.jar`). One is a build of the instrumentation, which is written as a separate application that uses the DiSL libraries. The other is the build of the application that we want to instrument. This build can be untouched, since all the instrumentation logic is done in the instrumentation build. 

## Integrating DiSL into the project
Initially, we thought it would make sense to run DiSL as a library call instead of launching it via an external script. By doing this, we would get the result of the instrumentation as an in-memory object, instead of having to scrape it from the output of the tool (a file).

However, due to the need to spawn multiple processes and having them communicate via an already established means set by DiSL, library calls from a single application would not suffice.
### Extracting runtime values
As of now, we have multiple different concurrently-running processes that perform the instrumentation. Their communication looks like this:
![[processes.png]]
The DiSL instrumentation queries the values of the instrumented app and is able to perform any logic on top of them. In order to get the values from the DiSL instrumentation JAR, we need some sort of an IPC mechanism. The simplest way is to store the values in a file and read them later on in the auto-debugger process. By choosing this approach, we run into several issues, mainly with types and their recognition. 

We can still make this work by going for a more standardized way of storing the data, which is serialization. 
#### Serializing values
After we collect the necessary values in the instrumentation, we dump them into a file using existing means of serialization. These files are then loaded back in auto-debugger and the values are reconstructed into runtime variables that can be used for further steps of the auto-debugger pipeline.

We use the `java.io.ObjectOutputStream` to write values to files, creating one file per value. The full generated instrumentation code looks as follows:
```java
@Before(marker = BodyMarker.class, scope = "test")  
public static void generatedMethod0(DynamicContext di) {  
    int generatedVariable0 = di.getLocalVariableValue(0, int.class);  
    try {  
       FileOutputStream fileOut = new FileOutputStream("variable.ser");  
       ObjectOutputStream out = new ObjectOutputStream(fileOut);  
       out.writeObject(generatedVariable0);  
       out.close();  
       fileOut.close();  
    } catch (IOException e) {  
       e.printStackTrace();  
    }  
}
```

#### GRPC

#### ShadowVM
A built-in way of sending values from the instrumentation to another process is through the ShadowObject API.

ShadowVM serves as a system for creating dynamic analyses on top of the DiSL instrumentation. While the DiSL instrumentation can provides us with means of observing the instrumented application, ShadowVM allows us to use the observed values in a custom analyzer. 

##### ShadowVM Example
In order to illustrate the purpose of ShadowVM and the ShadowObject API, let's look at the following code based on the `dispatch` example in the DiSL repository.

The instrumented (target) application's code is listed below. Running the main function is enough to trigger the instrumentation, so the body of the function is kept simple.
```java
public class Main {  
    
    public static void main(String[] args) {  
    
       System.out.println("[Instrumentee process] PID: " + ProcessHandle.current().pid());  
  
       System.out.println("app: main");  
    }  
}
```

Below is the instrumentation code. The logic of the instrumentation might be more complex, e.g., listing values of local variables and parameters. However, here it's kept simple. The instrumentation just performs a call to a custom analysis logic exposed by `CodeExecutedRE`.
```java
public class DiSLClass {  
  
    @After(marker = BodyMarker.class, scope = "Main.main")  
    public static void testing() {  
  
       System.out.println("[Instrumentation process] PID: " + ProcessHandle.current().pid());  
  
       CodeExecutedRE.testingBasic(true, (byte) 125, 's', (short) 50000, 100000, 10000000000L);
    }  
}
```

In `CodeExecutedRE`, we showcase the possibility of transferring obtained values from one process to another. Using the `REDispatch` class, we send several primitive values to the receiving process. The same applies to objects, these are however omitted from this simple example and showcased in later sections.
```java
public class CodeExecutedRE {

private static short tbId = REDispatch.registerMethod(  
       "CodeExecuted.testingBasic");
       
	public static void testingBasic(final boolean b, final byte by, final char c, final short s, final int i,  
	         final long l) {  
	    REDispatch.analysisStart(tbId);  
	  
	    System.out.println("[Sending process] PID: " + ProcessHandle.current().pid());  
	  
	    REDispatch.sendBoolean(b);  
	    REDispatch.sendByte(by);  
	    REDispatch.sendChar(c);  
	    REDispatch.sendShort(s);  
	    REDispatch.sendInt(i);  
	    REDispatch.sendLong(l);  
	  
	    REDispatch.analysisEnd();  
	}
}
```

The receiving class extend the `RemoteAnalysis` class provided by DiSL. Here, we can receive primitive values sent to us from the instance of the custom `CodeExecutedRE` class. The primitive values are received in the same form. These values can now be used for further manipulation in an outside process, while the instrumented process remains isolated.
```java
public class CodeExecuted extends RemoteAnalysis {
	public void testingBasic(final boolean b, final byte by, final char c, final short s, final int i, final long l) {  
	
	    System.out.println("[Receiving process] PID: " + ProcessHandle.current().pid());  
	  
	    if(b != true) {  
	       throw new RuntimeException("Incorect transfer of boolean");  
	    }  
	  
	    if(by != (byte) 125) {  
	       throw new RuntimeException("Incorect transfer of byte");  
	    }  
	  
	    if(c != 's') {  
	       throw new RuntimeException("Incorect transfer of char");  
	    }  
	  
	    if(s != (short) 50000) {  
	       throw new RuntimeException("Incorect transfer of short");  
	    }  
	  
	    if(i != 100000) {  
	       throw new RuntimeException("Incorect transfer of int");  
	    }  
	  
	    if(l != 10000000000L) {  
	       throw new RuntimeException("Incorect transfer of long");  
	    }
	}
}
```

Upon correct execution with the proper ShadowVM option (`-cse`), we arrive at the following output.
```
     [exec] [Instrumentee process] PID: 28737
     [exec] app: main
     [exec] [Instrumentation process] PID: 28737
     [exec] [Sending process] PID: 28737
     [exec] [Receiving process] PID: 28732
```
As we can see, the instrumentation logic in `DiSLClass` and the logic that sends the values in `CodeExecutedRE` are woven into the instrumented (target) application, thus sharing the same PID. The values are received by an outside process, where we can analyze and process the values further.

*TODO: Show ShadowObject API on objects*
