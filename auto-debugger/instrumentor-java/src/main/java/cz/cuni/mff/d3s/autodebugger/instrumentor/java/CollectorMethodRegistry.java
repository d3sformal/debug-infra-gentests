package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;

import java.util.Map;

public class CollectorMethodRegistry {
    private static final Map<String, String> collectorMethods = Map.of(
            "byte", "collectByte",
            "char", "collectChar",
            "short", "collectShort",
            "int", "collectInt",
            "long", "collectLong",
            "float", "collectFloat",
            "double", "collectDouble",
            "boolean", "collectBoolean",
            "java.lang.String", "collectString"
    );

    public static String getCollectorMethodName(JavaValueIdentifier value) {
        return collectorMethods.getOrDefault(value.getType(), "collectObject");
    }
}
