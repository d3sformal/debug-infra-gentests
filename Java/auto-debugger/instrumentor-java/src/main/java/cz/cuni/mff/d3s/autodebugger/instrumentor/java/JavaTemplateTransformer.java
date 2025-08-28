package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import org.javatuples.Pair;

public class JavaTemplateTransformer {
    private final String formatOpener;
    private final String formatCloser;

    public JavaTemplateTransformer(String templateFormat) {
        if (!templateFormat.contains("%s")) {
            throw new IllegalArgumentException("Template format must contain %s");
        }
        var parts = templateFormat.split("%s");
        if (parts.length != 2) {
            throw new IllegalArgumentException("Template format must contain exactly one %s");
        }
        this.formatOpener = parts[0];
        this.formatCloser = parts[1];
    }

    public String transform(String template, Pair<String, ?>... values) {
        if (values.length == 0) {
            return template;
        }
        int openerIndex = template.indexOf(formatOpener);
        while (openerIndex != -1) {
            int closerIndex = template.indexOf(formatCloser, openerIndex);
            if (closerIndex == -1) {
                throw new IllegalArgumentException("Template format is invalid");
            }
            String prefix = template.substring(0, openerIndex);
            String suffix = template.substring(closerIndex + formatCloser.length());
            String key = template.substring(openerIndex + formatOpener.length(), closerIndex);
            for (Pair<String, ?> value : values) {
                if (value.getValue0().equals(key)) {
                    template = prefix + value.getValue1() + suffix;
                    closerIndex = prefix.length() + value.getValue1().toString().length();
                    break;
                }
            }
            openerIndex = template.indexOf(formatOpener, closerIndex);
        }
        return template;
    }
}
