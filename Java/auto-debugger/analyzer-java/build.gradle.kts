plugins {
    id("java-library")
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
    implementation(project(mapOf("path" to ":analyzer-common")))
    implementation(project(mapOf("path" to ":test-generator-common")))
    implementation(project(mapOf("path" to ":test-generator-java")))
    implementation(platform(libs.log4j.bom))
    implementation(libs.log4j)
    testImplementation(platform(libs.junit.bom))
    testImplementation("org.junit.jupiter:junit-jupiter-api")
    testImplementation("org.junit.jupiter:junit-jupiter-engine")
    testImplementation(libs.mockito)
    testImplementation(project(":test-utils"))
}

tasks.test {
    useJUnitPlatform()
}