package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;

import static org.junit.jupiter.api.Assertions.*;

class CollectorTemplateCompletenessTest {

    private static String readResource(String path) throws Exception {
        try (InputStream in = CollectorTemplateCompletenessTest.class.getResourceAsStream(path)) {
            assertNotNull(in, "Resource should exist: " + path);
            try (BufferedReader br = new BufferedReader(new InputStreamReader(in, StandardCharsets.UTF_8))) {
                StringBuilder sb = new StringBuilder();
                String line;
                while ((line = br.readLine()) != null) { sb.append(line).append('\n'); }
                return sb.toString();
            }
        }
    }

    @Test
    void collectorRE_contains_all_registers_and_methods() throws Exception {
        String re = readResource("/templates/java/disl-analysis/CollectorRE.java");
        // registerMethod ids
        assertTrue(re.contains("registerMethod(\"Collector.startEvent\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectByte\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectChar\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectShort\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectInt\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectLong\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectFloat\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectDouble\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectBoolean\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectString\")"));
        assertTrue(re.contains("registerMethod(\"Collector.collectObject\")"));
        // method bodies include appropriate send* calls
        assertTrue(re.contains("sendByte("));
        assertTrue(re.contains("sendChar("));
        assertTrue(re.contains("sendShort("));
        assertTrue(re.contains("sendInt("));
        assertTrue(re.contains("sendLong("));
        assertTrue(re.contains("sendFloat("));
        assertTrue(re.contains("sendDouble("));
        assertTrue(re.contains("sendBoolean("));
        assertTrue(re.contains("sendObject("));
    }

    @Test
    void collector_contains_all_corresponding_methods() throws Exception {
        String c = readResource("/templates/java/disl-analysis/Collector.jt");
        assertTrue(c.contains("public void collectByte("));
        assertTrue(c.contains("public void collectChar("));
        assertTrue(c.contains("public void collectShort("));
        assertTrue(c.contains("public void collectInt("));
        assertTrue(c.contains("public void collectLong("));
        assertTrue(c.contains("public void collectFloat("));
        assertTrue(c.contains("public void collectDouble("));
        assertTrue(c.contains("public void collectBoolean("));
        assertTrue(c.contains("public void collectString("));
        assertTrue(c.contains("public void collectObject("));
    }
}

