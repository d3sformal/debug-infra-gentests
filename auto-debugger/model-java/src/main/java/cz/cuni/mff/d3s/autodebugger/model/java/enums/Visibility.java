package cz.cuni.mff.d3s.autodebugger.model.java.enums;

import lombok.Getter;

import java.util.List;

/**
 * Java visibility modifiers enumeration.
 * Represents the four levels of access control in Java with predefined
 * collections for common visibility filtering operations.
 */
@Getter
public enum Visibility {
    PUBLIC,
    PROTECTED,
    PACKAGE,
    PRIVATE;

    private static final List<Visibility> all = List.of(PUBLIC, PROTECTED, PACKAGE, PRIVATE);
    private static final List<Visibility> none = List.of();
}
