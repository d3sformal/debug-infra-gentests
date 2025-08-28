#!/usr/bin/env bash
set -euo pipefail
OUTDIR=${1:-}
if [[ -z "$OUTDIR" ]]; then
  echo "Usage: $0 <output-directory>" >&2
  exit 1
fi
mkdir -p "$OUTDIR/stub-tests"
RESULTS_FILE="$OUTDIR/generated-tests.lst"
cat >"$OUTDIR/stub-tests/StubTest.java" <<'JAVA'
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
public class StubTest {
  @BeforeEach void setup(){ Object o = new Object(); assertNotNull(o); }
  @Test void testExample(){ int a = 1; int b = a + 1; assertTrue(b > a); }
}
JAVA
printf "%s\n" "$OUTDIR/stub-tests/StubTest.java" >"$RESULTS_FILE"
echo "Wrote stub results to $RESULTS_FILE"

