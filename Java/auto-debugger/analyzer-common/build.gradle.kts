plugins {
    id("java-library")
}

group = "cz.cuni.mff.d3s"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(mapOf("path" to ":model-common")))
    testImplementation(platform(libs.junit.bom))
    testImplementation("org.junit.jupiter:junit-jupiter-api")
    testImplementation("org.junit.jupiter:junit-jupiter-engine")
}

tasks.test {
    useJUnitPlatform()
}