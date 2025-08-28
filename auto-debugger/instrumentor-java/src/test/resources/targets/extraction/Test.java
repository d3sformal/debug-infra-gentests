package targets.extraction;

public class Test {
    private int f = 10;
    private int g = 20;

    public static int testMul(int x, int y) {
        System.out.println("app: Main.testMul()");
        int a = x;
        int b = y;
        int mul = a * b;
        System.out.println("app: a=" + a);
        System.out.println("app: b=" + b);
        System.out.println("app: mul=" + mul);
        return mul;
    }

    public int testAdd(int x) {
        System.out.println("app: Main.testAdd()");
        int a = x;
        int b = f;
        int c = g;
        int sum = a + b + c;
        System.out.println("app: a=" + a);
        System.out.println("app: b=" + b);
        System.out.println("app: c=" + c);
        System.out.println("app: sum=" + sum);
        return sum;
    }
}
