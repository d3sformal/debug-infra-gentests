package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Collections;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class for JavaMethodIdentifier utility methods added for test generation support.
 */
class JavaMethodIdentifierUtilityMethodsTest {

    private JavaMethodIdentifier methodWithPackage;
    private JavaMethodIdentifier methodWithoutPackage;
    private JavaMethodIdentifier methodWithComplexParams;
    private JavaMethodIdentifier methodWithoutParams;

    @BeforeEach
    void setUp() {
        // Create method with package
        JavaPackageIdentifier packageIdentifier = new JavaPackageIdentifier("com.example.service");
        JavaClassIdentifier classWithPackage = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .packageIdentifier(packageIdentifier)
                .className("UserService")
                .build()
        );
        
        methodWithPackage = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(classWithPackage)
                .methodName("authenticate")
                .returnType("boolean")
                .parameterTypes(Arrays.asList("java.lang.String", "java.lang.String"))
                .build()
        );

        // Create method without package (default package)
        JavaClassIdentifier classWithoutPackage = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                .className("Calculator")
                .build()
        );
        
        methodWithoutPackage = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(classWithoutPackage)
                .methodName("add")
                .returnType("int")
                .parameterTypes(Arrays.asList("int", "int"))
                .build()
        );

        // Create method with complex parameter types
        methodWithComplexParams = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(classWithPackage)
                .methodName("processData")
                .returnType("java.util.List<java.lang.String>")
                .parameterTypes(Arrays.asList("java.util.Map<java.lang.String,java.lang.Object>", "int[]", "boolean"))
                .build()
        );

        // Create method without parameters
        methodWithoutParams = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(classWithPackage)
                .methodName("initialize")
                .returnType("void")
                .parameterTypes(Collections.emptyList())
                .build()
        );
    }

    @Test
    void givenMethodIdentifiers_whenGettingPackageName_thenReturnsExpected() {
        assertEquals("com.example.service", methodWithPackage.getPackageName());
        assertEquals("", methodWithoutPackage.getPackageName());
        assertEquals("com.example.service", methodWithComplexParams.getPackageName());
        assertEquals("com.example.service", methodWithoutParams.getPackageName());
    }

    @Test
    void givenMethodIdentifiers_whenGettingClassName_thenReturnsExpected() {
        assertEquals("UserService", methodWithPackage.getClassName());
        assertEquals("Calculator", methodWithoutPackage.getClassName());
        assertEquals("UserService", methodWithComplexParams.getClassName());
        assertEquals("UserService", methodWithoutParams.getClassName());
    }

    @Test
    void givenMethodIdentifiers_whenGettingFqcn_thenReturnsExpected() {
        assertEquals("com.example.service.UserService", methodWithPackage.getFullyQualifiedClassName());
        assertEquals("Calculator", methodWithoutPackage.getFullyQualifiedClassName());
        assertEquals("com.example.service.UserService", methodWithComplexParams.getFullyQualifiedClassName());
        assertEquals("com.example.service.UserService", methodWithoutParams.getFullyQualifiedClassName());
    }

    @Test
    void givenMethodIdentifiers_whenGettingSimpleSignature_thenReturnsExpected() {
        assertEquals("authenticate(String, String)", methodWithPackage.getSimpleSignature());
        assertEquals("add(int, int)", methodWithoutPackage.getSimpleSignature());
        assertEquals("processData(Map, int[], boolean)", methodWithComplexParams.getSimpleSignature());
        assertEquals("initialize()", methodWithoutParams.getSimpleSignature());
    }

    @Test
    void givenMethodIdentifiers_whenGettingFullyQualifiedSignature_thenReturnsExpected() {
        assertEquals("com.example.service.UserService.authenticate(String, String)", 
                    methodWithPackage.getFullyQualifiedSignature());
        assertEquals("Calculator.add(int, int)", 
                    methodWithoutPackage.getFullyQualifiedSignature());
        assertEquals("com.example.service.UserService.processData(Map, int[], boolean)", 
                    methodWithComplexParams.getFullyQualifiedSignature());
        assertEquals("com.example.service.UserService.initialize()", 
                    methodWithoutParams.getFullyQualifiedSignature());
    }

    @Test
    void givenMethodIdentifiers_whenGettingTestFriendlySignature_thenReturnsExpected() {
        assertEquals("UserService.authenticate(String, String)", 
                    methodWithPackage.getTestFriendlySignature());
        assertEquals("Calculator.add(int, int)", 
                    methodWithoutPackage.getTestFriendlySignature());
        assertEquals("UserService.processData(Map, int[], boolean)", 
                    methodWithComplexParams.getTestFriendlySignature());
        assertEquals("UserService.initialize()", 
                    methodWithoutParams.getTestFriendlySignature());
    }

    @Test
    void givenNullOwnerClassIdentifier_whenConstructingMethodIdentifier_thenHandlesGracefully() {
        JavaMethodIdentifier methodWithNullOwner = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(null)
                .methodName("testMethod")
                .returnType("void")
                .parameterTypes(Collections.emptyList())
                .build()
        );

        assertEquals("", methodWithNullOwner.getPackageName());
        assertEquals("UnknownClass", methodWithNullOwner.getClassName());
        assertEquals("UnknownClass", methodWithNullOwner.getFullyQualifiedClassName());
        assertEquals("testMethod()", methodWithNullOwner.getSimpleSignature());
        assertEquals("UnknownClass.testMethod()", methodWithNullOwner.getFullyQualifiedSignature());
        assertEquals("UnknownClass.testMethod()", methodWithNullOwner.getTestFriendlySignature());
    }
}
