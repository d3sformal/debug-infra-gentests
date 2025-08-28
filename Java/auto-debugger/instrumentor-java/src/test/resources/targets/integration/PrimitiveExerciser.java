package com.example.target;

public class PrimitiveExerciser {
    public static void main(String[] args) {
        // This call provides the ground truth for our test.
        exerciseAllPrimitives(
            10,                  // int
            -20L,                // long
            (short) 30,          // short
            (byte) 40,           // byte
            50.5f,               // float
            60.6,                // double
            'A',                 // char
            true                 // boolean
        );
    }

    public static void exerciseAllPrimitives(int i, long l, short s, byte b, float f, double d, char c, boolean z) {
        // The body of this method is the instrumentation target.
        // It doesn't need to do anything; its invocation is what matters.
        System.out.println("Target method executed.");
        System.out.println("int: " + i);
        System.out.println("long: " + l);
        System.out.println("short: " + s);
        System.out.println("byte: " + b);
        System.out.println("float: " + f);
        System.out.println("double: " + d);
        System.out.println("char: " + c);
        System.out.println("boolean: " + z);
    }
}
