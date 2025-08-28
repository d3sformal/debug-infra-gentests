package com.example;

public class StaticApp {
  public static void main(String[] args) {
    for (int i = 0; i < 3; i++) {
      Globals.bump();
      System.out.println("X=" + Globals.X);
    }
  }
}

