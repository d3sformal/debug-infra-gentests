package com.example;

public class FieldApp {
  public static void main(String[] args) {
    Counter c = new Counter();
    for (int i = 0; i < 3; i++) {
      c.increment();
      System.out.println("value=" + c.getValue());
    }
  }
}

