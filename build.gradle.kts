plugins {
    java
    `java-library`
}
// TODO set up cargo tasks

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.6.2")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.6.2")
}

tasks.test {
    useJUnitPlatform()
    systemProperty("java.library.path", "./fluentbindings/target/debug")
}
