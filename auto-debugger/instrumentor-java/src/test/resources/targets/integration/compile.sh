#!/bin/bash

# Compile integration test targets
echo "Compiling integration test targets..."

# Create output directory
mkdir -p classes

# Compile all Java files
javac -d classes *.java

# Create JAR files
echo "Creating JAR files..."

# PrimitiveExerciser.jar
cd classes
jar cfe ../PrimitiveExerciser.jar com.example.target.PrimitiveExerciser com/example/target/PrimitiveExerciser.class
cd ..

# SlotChecker.jar
cd classes
jar cfe ../SlotChecker.jar com.example.target.SlotChecker com/example/target/SlotChecker.class
cd ..

# TraceTarget.jar
cd classes
jar cfe ../TraceTarget.jar com.example.target.TraceTarget com/example/target/TraceTarget.class
cd ..

# TraceTargetError.jar
cd classes
jar cfe ../TraceTargetError.jar com.example.target.TraceTargetError com/example/target/TraceTargetError.class
cd ..

echo "Compilation complete!"
echo "Generated JAR files:"
ls -la *.jar
