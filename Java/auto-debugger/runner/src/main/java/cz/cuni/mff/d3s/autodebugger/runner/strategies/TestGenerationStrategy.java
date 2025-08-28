package cz.cuni.mff.d3s.autodebugger.runner.strategies;

/**
 * Represents a test generation strategy with metadata and configuration.
 * Each strategy defines a specific approach to generating unit tests from runtime traces.
 */
public class TestGenerationStrategy {
    private final String id;
    private final String displayName;
    private final String description;
    private final boolean isDefault;

    /**
     * Creates a new test generation strategy.
     *
     * @param id The unique identifier for this strategy
     * @param displayName The human-readable display name
     * @param description The description of this strategy
     * @param isDefault Whether this strategy should be selected by default
     */
    public TestGenerationStrategy(String id, String displayName, String description, boolean isDefault) {
        this.id = id;
        this.displayName = displayName;
        this.description = description;
        this.isDefault = isDefault;
    }

    /**
     * Creates a new test generation strategy using enum.
     *
     * @param name The unique identifier for this strategy
     * @param isDefault Whether this strategy should be selected by default
     */
    public TestGenerationStrategy(TestGenerationStrategyName name, boolean isDefault) {
        this.id = name.name().toLowerCase().replace('_', '-');
        this.displayName = name.getDisplayName();
        this.description = name.getDescription();
        this.isDefault = isDefault;
    }

    /**
     * Gets the unique identifier for this strategy.
     *
     * @return The strategy ID
     */
    public String getId() {
        return id;
    }

    /**
     * Gets the display name for this strategy.
     *
     * @return The display name
     */
    public String getDisplayName() {
        return displayName;
    }

    /**
     * Gets the description for this strategy.
     *
     * @return The description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Checks if this strategy is the default strategy.
     *
     * @return true if this is the default strategy, false otherwise
     */
    public boolean isDefault() {
        return isDefault;
    }

    @Override
    public String toString() {
        return displayName;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        TestGenerationStrategy that = (TestGenerationStrategy) obj;
        return id.equals(that.id);
    }

    @Override
    public int hashCode() {
        return id.hashCode();
    }
}
