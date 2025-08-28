#!/usr/bin/env python3
"""
Mock DiSL script that simulates timeout scenario.
This script runs indefinitely to test timeout handling.
"""

import sys
import time

def main():
    # Print startup message
    print("[DiSL] Mock DiSL script started", file=sys.stderr)
    print(f"[DiSL] Arguments: {sys.argv}", file=sys.stderr)
    
    print("[DiSL] Analysis started...")
    print("[DiSL] This process will run indefinitely to test timeout...")
    
    # Run indefinitely to simulate timeout
    while True:
        time.sleep(1)
        print("[DiSL] Still running...")

if __name__ == "__main__":
    main()
