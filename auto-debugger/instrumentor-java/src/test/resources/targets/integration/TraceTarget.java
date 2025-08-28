package com.example.target;

public class TraceTarget {
    public static void main(String[] args) {
        // The target method is called twice with distinct values.
        processData("first_call", 100);
        processData("second_call", 200);
    }
    
    // This is the method we will instrument.
    public static void processData(String name, int value) {
        System.out.println("Processing: " + name + " = " + value);
    }
}
