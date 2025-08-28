package cz.cuni.mff.d3s.autodebugger.model.java.parsing;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class JavaMethodSignatureParserTest {

    @Test
    void givenIncompleteParentheses_whenParsingSignature_thenDetectsMethodNameOnly() {
        MethodSignature ms = JavaMethodSignatureParser.parseMethodSignature("Class.method(int");

        // Let's see what the parser actually returns
        System.err.println("=== PARSER OUTPUT ===");
        System.err.println("State: " + ms.getState());
        System.err.println("ClassName: '" + ms.getClassName() + "'");
        System.err.println("MethodName: '" + ms.getMethodName() + "'");
        System.err.println("ParameterTypes: " + ms.getParameterTypes());
        System.err.println("=== END OUTPUT ===");

        // The parser should detect this as METHOD_NAME_ONLY due to incomplete parentheses
        assertEquals(SignatureState.METHOD_NAME_ONLY, ms.getState());
    }
    
    @Test
    void givenValidFullMethodSignature_whenParsing_thenReturnsCorrectMethodSignature() {
        MethodSignature ms = JavaMethodSignatureParser.parseMethodSignature("com.example.Class.method(int,String)");
        
        assertEquals(SignatureState.FULL_METHOD, ms.getState());
        assertEquals("com.example.Class", ms.getClassName());
        assertEquals("method", ms.getMethodName());
        assertEquals(2, ms.getParameterTypes().size());
    }
}
