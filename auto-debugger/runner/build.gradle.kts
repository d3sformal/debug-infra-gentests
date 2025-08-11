plugins {
    id("java")
    id("application")
    alias(libs.plugins.lombok)
}

group = "cz.cuni.mff.d3s"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(mapOf("path" to ":model-common")))
    implementation(project(mapOf("path" to ":model-java")))
    implementation(project(mapOf("path" to ":instrumentor-common")))
    implementation(project(mapOf("path" to ":instrumentor-java")))
    implementation(project(mapOf("path" to ":analyzer-common")))
    implementation(project(mapOf("path" to ":analyzer-java")))
    implementation(project(mapOf("path" to ":test-generator-common")))
    implementation(project(":test-generator-java", "shadow"))
    implementation(project(mapOf("path" to ":test-runner-common")))
    implementation(project(":test-runner-java", "shadow"))
    implementation(libs.picocli)
    implementation(libs.log4j)
    testImplementation(libs.bundles.junit)
}

tasks.test {
    useJUnitPlatform()
}

application {
    mainClass.set("cz.cuni.mff.d3s.autodebugger.runner.Runner")
}