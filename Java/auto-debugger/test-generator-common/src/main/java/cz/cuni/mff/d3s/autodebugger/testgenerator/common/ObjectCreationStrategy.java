package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

/**
 * Strategies for creating complex objects in generated tests.
 */
public enum ObjectCreationStrategy {
    /**
     * Use simple constructors and setters only.
     */
    SIMPLE,
    
    /**
     * Use builder patterns when available.
     */
    BUILDER_PATTERN,
    
    /**
     * Use factory methods when available.
     */
    FACTORY_METHODS,
    
    /**
     * Use test data builders or object mothers.
     */
    TEST_DATA_BUILDERS,
    
    /**
     * Use mocking frameworks for complex dependencies.
     */
    MOCKING,
    
    /**
     * Automatically detect the best strategy based on available constructors and methods.
     */
    AUTO_DETECT
}
