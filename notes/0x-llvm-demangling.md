# LLVM demangler cannot demangle some generic lambdas with deduced return type

Consider:

```c++
int main() {
  auto auto_lambda = [](auto x) {
    return x * 2;
  };

  float autofloat = auto_lambda(3.14f);
  return 0;
}
```

Demangling of the call operator of the lambda (`_ZZ4mainENK3$_0clIfEEDaT_`) fails.

## ~~Cause~~

in [llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h@4546](../sandbox/00-clang-ast/llvm-project/llvm/include/llvm/Demangle/ItaniumDemangle.h)

```c++
// ParseType()
// ...

  case 'T': {
    // This could be an elaborate type specifier on a <class-enum-type>.
    if (look(1) == 's' || look(1) == 'u' || look(1) == 'e') {
      Result = getDerived().parseClassEnumType();
      break;
    }

    Result = getDerived().parseTemplateParam();
    if (Result == nullptr)
      return nullptr;
```

The following diff **seemed** to have solved it:

```diff
diff --git a/llvm/include/llvm/Demangle/ItaniumDemangle.h b/llvm/include/llvm/Demangle/ItaniumDemangle.h
index b0363c1a7a7..b03692f7006 100644
--- a/llvm/include/llvm/Demangle/ItaniumDemangle.h
+++ b/llvm/include/llvm/Demangle/ItaniumDemangle.h
@@ -5686,7 +5686,7 @@ Node *AbstractManglingParser<Derived, Alloc>::parseEncoding(bool ParseParams) {
   }
 
   NodeArray Params;
-  if (!consumeIf('v')) {
+  if (!consumeIf('v') && !consumeIf("T_")) {
     size_t ParamsBegin = Names.size();
     do {
       Node *Ty = getDerived().parseType();
```

but it fails to demangle `_Z7addAutoIiEDaT_S0_` for example, which stands for
`auto addAuto<int>(int, int)` (and is demangled correctly before applying this patch).

Suddenly `DaT_` became suspect (removing it from the mangled string `_ZZ4mainENK3$_0clIfEEDaT_`, turning it to `_ZZ4mainENK3\$_0clIfEE` gave correct result: `main::$_0::operator()<int>`).

## Cause?

According to [llvm-project/clang/lib/AST/ItaniumMangle.cpp@6881](../sandbox/00-clang-ast/llvm-project/clang/lib/AST/ItaniumMangle.cpp), it is the **first template parameter**.
Which does make sense, however the demangler does not respect this.

The most precise reason for the exclusion of the template parameter was the failure to 
`parseTemplateParam`, specifically, the function ends up returning `nullptr` due to 

* `Level == TemplateParams.size() == 0`

* and `ParsingLambdaParamsAtLevel == (size_t)-1` (bypasssing this check demangles the name to `auto main::$_0::operator()<int>(auto) const`)

Conveniently, there is only one place where `ParsingLambdaParamsAtLevel` gets set (temporarily).
That is inside of parsing of mangled substring `Ul` (lambda):

```c++
  if (consumeIf("Ul")) {
    ScopedOverride<size_t> SwapParams(ParsingLambdaParamsAtLevel,
                                      TemplateParams.size());
```

instead, however, in our mangled name, `3$_0cl` refers to an anonymous struct's `operator()` (`cl`).
Thus (IMO) the parsing never updates the `ParsingLambdaParamsAtLevel` member and the demangling fails. It is curious that none of my examples of lambda variables produce the `'lambda'` mangling.
(perhaps because they are in `main`?)

The following:


```c++
inline auto abcd = [](auto x) {
  return x * 2;
};

inline auto efgh = [](int x) {
  return x * 2;
};


int main() {

  abcd (2);
  efgh(2);
}
```
Produces `_ZNK4abcdMUlT_E_clIiEEDaS0_` for `abcd` which is demangled to:
`auto abcd::'lambda'(auto)::operator()<int>(auto) const`.

Meanwhile `efgh`: `efgh::'lambda'(int)::operator()(int) const`.

Witout `inline` the mangling reverts to `auto $_0::operator()<int>(int) const` for `abcd`
and similarly for `efgh`. The point is that `abcd` **outside** of `main` gets demangled while `auto_lambda` **inside** main doesn't.

I leave this issue for later to be polished and reported. As even the `c++filt` cannot demangle this, mangling itself remains a suspect.
