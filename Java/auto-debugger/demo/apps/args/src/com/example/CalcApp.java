package com.example;

public class CalcApp {
  public static void main(String[] args) {
    Calculator c = new Calculator();
    // invoke target method multiple times
    System.out.println(c.add(1, 2));
    System.out.println(c.add(3, 4));
    System.out.println(c.add(5, 6));
  }
}

