package cz.cuni.mff.d3s.autodebugger.model.common.technique;

import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Singular;
import lombok.ToString;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for a selected test generation/analysis technique.
 * Encapsulates the technique identifier and any optional parameters or secrets
 * without proliferating ad-hoc parameters across component boundaries.
 */
@Getter
@Builder
@EqualsAndHashCode
@ToString
public class TestTechniqueConfig {

    /**
     * Technique identifier (e.g., "trace-based-basic", "ai-assisted").
     */
    private final String id;

    /**
     * Arbitrary string parameters for the technique (e.g., toggles, thresholds).
     */
    @Singular("param")
    private final Map<String, String> parameters;

    /**
     * Optional API key or short-lived secret for the technique.
     * For production, consider replacing with a proper secret provider.
     */
    private final String apiKey;

    /**
     * Returns an immutable view of parameters.
     */
    public Map<String, String> getParameters() {
        return parameters == null ? Collections.emptyMap() : Collections.unmodifiableMap(parameters);
    }

    /**
     * Convenience accessor with default value.
     */
    public String getParameterOrDefault(String key, String defaultValue) {
        Map<String, String> map = parameters == null ? new HashMap<>() : parameters;
        return map.getOrDefault(key, defaultValue);
    }

    public static TestTechniqueConfig ofId(String id) {
        return TestTechniqueConfig.builder().id(id).build();
    }
}

