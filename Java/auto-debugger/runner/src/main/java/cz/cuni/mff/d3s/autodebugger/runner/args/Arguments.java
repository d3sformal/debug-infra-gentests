package cz.cuni.mff.d3s.autodebugger.runner.args;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import picocli.CommandLine;
import java.util.List;

/**
 * Command-line arguments container for the auto-debugger application.
 * Uses PicoCLI annotations to define command-line options with validation,
 * default values, and help text for user-friendly CLI experience.
 */
public class Arguments {
    @CommandLine.Option(names = { "-j", "--jar" }, paramLabel = "JAR", description = "Path to the application JAR file", required = true)
    public String applicationJarPath;

    @CommandLine.Option(names = { "-c", "--classpath" }, paramLabel = "CLASSPATH", description = "Additional classpath entries (separated by ':')", split = ":")
    public List<String> classpath;

    @CommandLine.Option(names = { "-a", "--args" }, paramLabel = "ARGS", description = "Runtime arguments for the target application (separated by ' ')", split = " ")
    public List<String> runtimeArguments;

    @CommandLine.Option(names = { "-s", "--source" }, paramLabel = "SOURCE", description = "Path to the target application's source code", required = true)
    public String sourceCodePath;

    @CommandLine.Option(names = { "-o", "--output-dir" }, paramLabel = "OUTPUT", description = "Output directory for generated artifacts (results, identifiers)")
    public String outputDirectory;

    @CommandLine.Option(names = { "-d", "--disl-home" }, paramLabel = "DISL_HOME", description = "Path to the DiSL project (required for DiSL-based analysis)")
    public String dislHomePath;

    @CommandLine.Option(names = { "-m", "--method" }, paramLabel = "METHOD", description = "Target method reference (e.g., org.example.Main.main(String[]))", required = true)
    public String targetMethodReference;

    @CommandLine.Option(names = { "-p", "--parameters" }, paramLabel = "PARAMETERS", description = "Target method parameters (format: type:name or slot:type)", split = ",")
    public List<String> targetParameters;

    @CommandLine.Option(names = { "-f", "--fields" }, paramLabel = "FIELDS", description = "Target class fields (format: type:name)", split = ",")
    public List<String> targetFields;

    @CommandLine.Option(names = { "-l", "--language" }, paramLabel = "LANGUAGE",
                        description = "Programming language. Supported: ${COMPLETION-CANDIDATES}",
                        defaultValue = "java", converter = TargetLanguageConverter.class)
    public TargetLanguage language;

    @CommandLine.Option(names = { "-t", "--test-strategy" }, paramLabel = "STRATEGY",
                        description = "Test generation strategy (e.g., trace-based-basic, ai-assisted)",
                        defaultValue = "trace-based-basic")
    public String testGenerationStrategy;

    @CommandLine.Option(names = { "-r", "--trace-mode" }, paramLabel = "TRACE_MODE",
                        description = "Trace collection mode: naive or temporal",
                        defaultValue = "naive")
    public String traceMode;

    @CommandLine.Option(names = { "-k", "--api-key" }, paramLabel = "API_KEY",
                        description = "API key for LLM services (can also be set via ANTHROPIC_API_KEY environment variable)")
    public String apiKey;

    @CommandLine.Option(names = { "-h", "--help" }, usageHelp = true, description = "display a help message")
    private boolean helpRequested = false;

    /**
     * Custom converter for TargetLanguage enum to work with picocli.
     */
    public static class TargetLanguageConverter implements CommandLine.ITypeConverter<TargetLanguage> {
        @Override
        public TargetLanguage convert(String value) {
            return TargetLanguage.fromIdentifier(value);
        }
    }
}