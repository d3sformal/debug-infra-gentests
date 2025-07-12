plugins {
    id("java")
    alias(libs.plugins.lombok)
}

group = "cz.cuni.mff.d3s"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(mapOf("path" to ":model-java")))
    implementation(project(mapOf("path" to ":instrumentor-common")))
    implementation(project(mapOf("path" to ":instrumentor-java")))
    implementation(libs.picocli)
    testImplementation(libs.bundles.junit)
}

tasks.test {
    useJUnitPlatform()
}