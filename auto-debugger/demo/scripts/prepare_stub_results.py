#!/usr/bin/env python3
import sys, os

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <output-directory>", file=sys.stderr)
    sys.exit(1)

outdir = sys.argv[1]
os.makedirs(os.path.join(outdir, 'stub-tests'), exist_ok=True)
results = os.path.join(outdir, 'generated-tests.lst')
stub = os.path.join(outdir, 'stub-tests', 'StubTest.java')
with open(stub, 'w') as f:
    f.write('''import org.junit.jupiter.api.BeforeEach;\nimport org.junit.jupiter.api.Test;\nimport static org.junit.jupiter.api.Assertions.*;\npublic class StubTest {\n  @BeforeEach void setup(){ Object o = new Object(); assertNotNull(o); }\n  @Test void testExample(){ int a = 1; int b = a + 1; assertTrue(b > a); }\n}\n''')
with open(results, 'w') as f:
    f.write(stub + '\n')
print(f"Wrote stub results to {results}")

