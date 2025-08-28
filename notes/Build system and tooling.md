# Gradle
Gradle was chosen as the go-to build system for this project. This is mostly due to previous experience with Gradle in work environment.
 
Gradle allows us to define dependencies for each module as well as the entire project. The current layout is as follows:
```
auto-debugger
|--- build.gradle.kts
|--- settings.gradle.kts
|
|--- analyzer
|  |--- build.gradle.kts
|
|--- instrumentor-common
|  |--- build.gradle.kts
|
|--- instrumentor-java
|  |--- build.gradle.kts
|
|--- runner
|  |--- build.gradle.kts
|
|--- test-generator-common
|  |--- build.gradle.kts
|
|--- test-generator-java
   |--- build.gradle.kts
```

The top-level `build.gradle.kts` allows us to define an `allProjects` construct to add dependencies to all modules. However, this is not the way this project handles dependencies.
Instead, all dependencies are listed in `settings.gradle.kts` and mapped to an alias via Gradle's version catalog. The aliases are then referenced in the individual `build.gradle.kts` files.

# Logging
In order to add modifiability, we settled on using *SLF4J* as the logging facade. The logging framework integrated with SLF4J is *log4j*.