package targets.extraction;

public class Main {

  public static void main(String[] args) {
    System.out.println("[Instrumentee process] PID: " + ProcessHandle.current().pid());

    // Static method - args
    Test.testMul(3, 5);

    // Instance method - instance fields
    Test t = new Test();
    t.testAdd(7);
  }
}
