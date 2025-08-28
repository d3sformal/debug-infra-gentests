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
    implementation(project(mapOf("path" to ":runner")))
    testImplementation(libs.bundles.junit)
}

tasks.test {
    useJUnitPlatform()
}