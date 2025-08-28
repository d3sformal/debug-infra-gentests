#!/usr/bin/env python3
"""
Mock DiSL script that simulates successful execution.
This script mimics the behavior of disl.py for testing purposes.
"""

import sys
import time

def main():
    # Print the arguments received (for debugging)
    print("[DiSL] Mock DiSL script started", file=sys.stderr)
    print(f"[DiSL] Arguments: {sys.argv}", file=sys.stderr)
    
    # Simulate DiSL startup messages
    print("[DiSL] Analysis started...")
    print("[DiSL] Initializing instrumentation...")
    
    # Simulate some processing time
    time.sleep(0.1)
    
    # Simulate successful analysis output
    print("[DiSL] Loading target application...")
    print("[DiSL] Applying instrumentation...")
    print("[DiSL] Executing instrumented application...")
    print("[DiSL] Collecting runtime data...")
    print("[DiSL] Analysis finished successfully.")
    
    # Exit with success code
    sys.exit(0)

if __name__ == "__main__":
    main()
