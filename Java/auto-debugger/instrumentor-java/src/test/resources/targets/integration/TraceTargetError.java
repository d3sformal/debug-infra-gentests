package com.example.target;

public class TraceTargetError {
    public static void main(String[] args) {
        // Call processData once, then throw an exception
        processData("error_call", 500);
        throw new RuntimeException("Intentional test error");
    }
    
    // This is the method we will instrument.
    public static void processData(String name, int value) {
        System.out.println("Processing: " + name + " = " + value);
    }
}
