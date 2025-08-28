When instrumenting an application with DiSL, one is required to create a JAR with the instrumentation logic. The source code from which the JAR is created may look like this, based on the examples available in DiSL:
```java
import ch.usi.dag.disl.annotation.After;  
import ch.usi.dag.disl.annotation.AfterThrowing;  
import ch.usi.dag.disl.annotation.AfterReturning;  
import ch.usi.dag.disl.annotation.Before;  
import ch.usi.dag.disl.marker.BodyMarker;  
  
public class DiSLClass {  
    @Before(marker = MethodInvocationMarker.class, scope = "Main.main")  
    public static void beforeInvocation() {  
       System.out.println("disl: before invocation");  
    }  
  
    @After(marker = MethodInvocationMarker.class, scope = "Main.main")  
    public static void afterInvocation() {  
       System.out.println("disl: after invocation");  
    }  
  
    @After(marker = BodyMarker.class, scope = "Main.test*")  
    public static void afterMethod() {  
       System.out.println("disl: after method Main.test*");  
    }  
  
    @AfterThrowing(marker = BodyMarker.class, scope = "Main.test*")  
    public static void afterThrowing() {  
       System.out.println("disl: after throwing Main.test*");  
    }  
  
    @AfterReturning(marker = BodyMarker.class, scope = "Main.test*")  
    public static void afterReturning() {  
       System.out.println("disl: after returning Main.test*");  
    }  
}
```

Additionally, the sources may extend on what DiSL provides.

```
import java.util.LinkedList;  
import java.util.List;  
  
import org.objectweb.asm.tree.AbstractInsnNode;  
import org.objectweb.asm.tree.InsnList;  
import org.objectweb.asm.tree.MethodInsnNode;  
import org.objectweb.asm.tree.MethodNode;  
  
import ch.usi.dag.disl.marker.AbstractMarker;  
import ch.usi.dag.disl.marker.AbstractDWRMarker;  
    
public class MethodInvocationMarker extends AbstractDWRMarker {  
  
    public List<MarkedRegion> markWithDefaultWeavingReg(MethodNode method) {  
       List<MarkedRegion> regions = new LinkedList<MarkedRegion>();  
  
       // traverse all instructions  
       InsnList instructions = method.instructions;  
  
       for (AbstractInsnNode instruction : instructions.toArray()) {  
          // check for method invocation instructions  
          if (instruction instanceof MethodInsnNode) {  
             // add region containing one instruction (method invocation)  
             regions.add(new MarkedRegion(instruction, instruction));  
          }  
       }  
       return regions;  
    }  
}
```
We can access values of the instrumented application based on their type: arguments, locals, and fields.
```
int i = di.getMethodArgumentValue(1, int.class);
```

```
int a = di.getLocalVariableValue(0, int.class);
```

```
final Class <?> instType = dc.getInstanceFieldValue (  
    dc.getThis (), TargetClass.class, "instType", Class.class  
);
```