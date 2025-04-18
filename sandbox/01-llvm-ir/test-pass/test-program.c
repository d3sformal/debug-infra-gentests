#include <stdio.h>

void foo(int x) { printf("C FOO: %d\n", x); }

double bar(float x) {
  printf("C BAR: %lf\n", x);
  return x * x;
}

int baz(int y, float z) {
  printf("C BAZ: %d %lf\n", y, z);
  foo(y);
  return bar(z);
}

int doubleBaz(int y, double z) {
  printf("C DOUBLE BAZ: %d %lf\n", y, z);
  foo(y);
  return bar(z);
}

typedef struct {
  int (*x)();
} S;

int constStructFunc() { return 32; }

void typedefConsumer1(S *s) {
  s->x();
  return;
}

typedef S AnotherS;

void typedefConsumer2(AnotherS *s) {
  s->x();
  return;
}

int main() {
  foo(3);

  S s = {.x = constStructFunc};

  typedefConsumer1(&s);
  typedefConsumer2(&s);

  baz(1, 2.71f);
  doubleBaz(1, 3.14159);
  s.x();
  printf("End\n");
  return 0;
}