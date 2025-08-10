# Evaluation

We choose to evaluate the tool set on the [`keepassxc`](https://github.com/keepassxreboot/keepassxc) project. Keepassxc is a password manager application written in C++. It is a rather large project built with `cmake` that can be built with `clang`.

## Setup

We cloned the project, commit: `b342be457153aa098c8bbce0f8b0fd3baa8c158f`, and performed the following:

1. fixed paths of the clang plugins and the `hooklib`, we denote this by using a `//FIXPATH` placeholder
2. applied the following changes to the project (we use `//FIXPATH` 5 times here):

```diff
diff --git a/src/CMakeLists.txt b/src/CMakeLists.txt
index 06b3fee2..bb27ec5c 100644
--- a/src/CMakeLists.txt
+++ b/src/CMakeLists.txt
@@ -188,6 +188,9 @@ set(keepassx_SOURCES
 streams/qtiocompressor.cpp
 streams/StoreDataStream.cpp
 streams/SymmetricCipherStream.cpp)
+
+set_source_files_properties(core/PasswordGenerator.cpp COMPILE_FLAGS "${COMPILE_FLAGS} -mllvm -llcap-mapdir=//FIXPATH/modmaps -mllvm -Call -Xclang -load -Xclang //FIXPATH/libfn-pass.so -Xclang -fpass-plugin=//FIXPATH/libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so")
+
 if(APPLE)
 set(keepassx_SOURCES
 ${keepassx_SOURCES}
@@ -343,10 +346,17 @@ configure_file(git-info.h.cmake ${CMAKE_CURRENT_BINARY_DIR}/git-info.h)
 add_library(autotype STATIC ${autotype_SOURCES})
 target_link_libraries(autotype Qt5::Core Qt5::Widgets)
 
+add_library(hooklib SHARED IMPORTED)
+set_target_properties(hooklib PROPERTIES
+IMPORTED_LOCATION //FIXPATH/libmy-hook.so
+)
+
+
 add_library(keepassx_core STATIC ${keepassx_SOURCES})
 
 set_target_properties(keepassx_core PROPERTIES COMPILE_DEFINITIONS KEEPASSX_BUILDING_CORE)
 target_link_libraries(keepassx_core
+        hooklib
 autotype
 ${keepassxcbrowser_LIB}
 ${qrcode_LIB}
@@ -407,7 +417,8 @@ if(WIN32)
 endif()
 
 add_executable(${PROGNAME} WIN32 ${keepassx_SOURCES_MAINEXE} ${WIN32_ProductVersionFiles})
-target_link_libraries(${PROGNAME} keepassx_core)
+
+target_link_libraries(${PROGNAME} keepassx_core hooklib)
 
 set_target_properties(${PROGNAME} PROPERTIES ENABLE_EXPORTS ON)
 
diff --git a/src/autotype/xcb/CMakeLists.txt b/src/autotype/xcb/CMakeLists.txt
index 0704de63..a1bf0d6d 100644
--- a/src/autotype/xcb/CMakeLists.txt
+++ b/src/autotype/xcb/CMakeLists.txt
@@ -3,7 +3,13 @@ include_directories(SYSTEM ${X11_X11_INCLUDE_PATH})
 set(autotype_XCB_SOURCES AutoTypeXCB.cpp)
 
 add_library(keepassxc-autotype-xcb MODULE ${autotype_XCB_SOURCES})
-target_link_libraries(keepassxc-autotype-xcb keepassx_core Qt5::Core Qt5::Widgets Qt5::X11Extras ${X11_X11_LIB} ${X11_Xi_LIB} ${X11_XTest_LIB})
+
+add_library(hooklib SHARED IMPORTED)
+set_target_properties(hooklib PROPERTIES
+IMPORTED_LOCATION //FIXPATH/libmy-hook.so
+)
+
+target_link_libraries(keepassxc-autotype-xcb hooklib keepassx_core Qt5::Core Qt5::Widgets Qt5::X11Extras ${X11_X11_LIB} ${X11_Xi_LIB} ${X11_XTest_LIB})
 install(TARGETS keepassxc-autotype-xcb
 BUNDLE DESTINATION . COMPONENT Runtime
 LIBRARY DESTINATION ${PLUGIN_INSTALL_DIR} COMPONENT Runtime)

```

3. followed the project's [development environment setup](https://github.com/keepassxreboot/keepassxc/wiki/Set-up-Build-Environment-on-Linux)
4. ensured that our patched `clang++` is "installed" (this is only in the case of a clean environment that was set up according to the `keepassxc`'s setup)
5. followed the project's [build instructions](https://github.com/keepassxreboot/keepassxc/blob/develop/INSTALL.md#build-steps) up to the `cmake` usage
6. run `cmake -DCMAKE_CXX_COMPILER=clang++ -DWITH_TESTS=false ../`
7. run `make`

For the argument tracing instrumentation, we instead use the following `set_source_files_properties`:

```diff
+
+set_source_files_properties(core/PasswordGenerator.cpp COMPILE_FLAGS "${COMPILE_FLAGS} -mllvm -llcap-mapdir=//FIXPATH/modmaps -mllvm -Arg -mllvm -llcap-fn-targets-file=//FIXPATH/selection.bin -Xclang -load -Xclang //FIXPATH/libfn-pass.so -Xclang -fpass-plugin=//FIXPATH/libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so")
+
```

## Running the tool - call tracing + capture

We then executed the `llcap-server` for call tracing and argument capture. Both times, we opened the password generator and interacted with the password length UI element:

```shell
# in sandbox/02-ipc/llcap-server
llcap-server -vvvv --modmap //FIXPATH/modmaps trace-calls -o ./selected-fnis-kpass.bin /path/to/keepassxc/build/src/keepassxc

cp ./selected-fnis-kpass.bin //FIXPATH/selection.bin
```

The tool reported the following:

```
0 - 33152 - randomGen() (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
1 - 1030 - PasswordGenerator::passwordGroups() const (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
2 - 431 - PasswordGenerator::isValid() const (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
3 - 397 - PasswordGenerator::numCharClasses() const (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
4 - 214 - PasswordGenerator::generatePassword() const (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
5 - 211 - PasswordGenerator::setLength(int) (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
6 - 211 - PasswordGenerator::setCharClasses(QFlags<PasswordGenerator::CharClass> const&) (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
7 - 211 - PasswordGenerator::setFlags(QFlags<PasswordGenerator::GeneratorFlag> const&) (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
8 - 194 - PasswordGenerator::setCustomCharacterSet(QString const&) (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
9 - 194 - PasswordGenerator::setExcludedCharacterSet(QString const&) (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
10 - 1 - PasswordGenerator::PasswordGenerator() (module /path/to/keepassxc/src/core/PasswordGenerator.cpp)
Module map summary:
Total Modules loaded: 33
Total Functions loaded: 179
Total traced calls: 36246
Traces originated from 1 modules
```

We selected the `PasswordGenerator::setLength(int)` function (as it is the only valid candidate for the instrumentation) and performed argument capture (again, interacting with the UI element):

```shell
llcap-server -vvvv  --modmap //FIXPATH/modmaps trace-calls capture-args -s ./selected-fnis-kpass.bin -o ./kpass-trcs-dir /path/to/keepassxc/build/src/keepassxc
```

Then, we inspect the packets:

```shell
llcap-server -- -vvvv  --modmap //FIXPATH/modmaps trace-calls test -s ./selected-fnis-kpass.bin -c ./kpass-trcs-dir/ --inspect-packets F3199851-01000000
```

With the following result:

```log
P | [inspect_packet] Module: /path/to/keepassxc/src/core/PasswordGenerator.cpp
P | [inspect_packet] Function: PasswordGenerator::setLength(int)
P | [inspect_packet] Packet Description: [Fixed(0), Fixed(4)]
P | [inspect_packet] Packet index: 0
P | [inspect_packet] Raw packet: [20, 0, 0, 0]
P | [inspect_packet] Packet index: 1
P | [inspect_packet] Raw packet: [20, 0, 0, 0]

----------- ...omitted for brevity... -----------

P | [inspect_packet] Raw packet: [32, 0, 0, 0]
P | [inspect_packet] Packet index: 53
P | [inspect_packet] Raw packet: [31, 0, 0, 0]
P | [inspect_packet] Packet index: 54
P | [inspect_packet] Raw packet: [30, 0, 0, 0]
P | [inspect_packet] Packet index: 55
P | [inspect_packet] Raw packet: [40, 0, 0, 0]
P | [inspect_packet] Packet index: 56
P | [inspect_packet] Raw packet: [50, 0, 0, 0]
P | [inspect_packet] Packet index: 57
P | [inspect_packet] Raw packet: [60, 0, 0, 0]
P | [inspect_packet] Packet index: 58
P | [inspect_packet] Raw packet: [70, 0, 0, 0]
P | [inspect_packet] Packet index: 59
P | [inspect_packet] Raw packet: [80, 0, 0, 0]
P | [inspect_packet] Packet index: 60
P | [inspect_packet] Raw packet: [90, 0, 0, 0]
```

Reader shall notice the last few entries - these were created by "clicking" on the right side of the UI slider, incrementing by `10`. The values do correspond to the values set while performing argument capture. The initial recorded values may have been influenced by previous runs of the program.

## Running the tool - testing

Testing a GUI application in the way set up as above can be vague/tricky. Essentially, the tool will be relying on these major properties:

1. the instrumented function gets called at all
2. the fork is successful - forking everything (the GUI library can present a major hurdle here)

We believe, however, that according to the goals of the project (i.e. to create an initial set of tools that allow this kind of testing) the main evaluation criterion here is demonstrating both instrumentation passes can be performed and function.

We shortened the capture trace to the last seven argument packets.
 
While the concerns above have not manifested when testing, we encountered a keepass-specific issue
that prevented the execution of more tests. When our tool terminated the tested application, it kept
running some process in the background. This, when attempting to run a test of the second call, prevented the launch of the application (keepass expects only one instance of the program to be running).

Nonetheless, the first call has been instrumented. The application initializes its internal data, calling the function even before we can interact with it. This means we cannot effectively close the application as the GUI is not loaded yet. The results then report test case timeouts as we would
expect.

We attempted to use the `-llcap-instrument-fn-exit` option of the LLVM pass, which terminates the test case automatically when the instrumented function is left. This results in the "pass" test status, indicating that the end of the function has been reached in a short time:

```log
P | [main] ---------------------------------------------------------------
P | [main] Test results (7): 
P | [main] Module ID | Function ID |  Call  | Packet | Result
P | [main]  F3199851 |  01000000 |   1 |   0 | Pass
P | [main]  F3199851 |  01000000 |   1 |   1 | Pass
P | [main]  F3199851 |  01000000 |   1 |   2 | Pass
P | [main]  F3199851 |  01000000 |   1 |   3 | Pass
P | [main]  F3199851 |  01000000 |   1 |   4 | Pass
P | [main]  F3199851 |  01000000 |   1 |   5 | Pass
P | [main]  F3199851 |  01000000 |   1 |   6 | Pass
P | [main] ---------------------------------------------------------------
P | [main] Exiting...
```

## Conclusion

* can be improved - config in files to make it more "configurable"
  * e.g. to limit build file modifications to pass the `-mllvm -xyz` llvm pass arguments
* works even on a larger project with complex structure/dependencies
* verified function exit instrumentation is "useful"

# TODO

* investigate and note multiple compilation isses (compile the same file for 2 different subprojects -> module ID collision)
* try to add support for a custom type
  * somethign meaningful like `QString` requires linking and building hooklib with other headers and libraries! (should be at least mentioned in the development guide?) 
* performace? very much not sure
  * we can reason about the instrumentation in terms of overhead on a higher level
    * note that the second instrumentation only instruments the target function
