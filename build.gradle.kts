plugins {
    `java-library`
}

repositories {
    mavenCentral()
    maven(url = "https://maven.onehippo.com/maven2/")
}

dependencies {
    implementation("javax.annotation:jsr305:1.0")
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.6.2")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.6.2")
}

tasks {
    test {
        useJUnitPlatform()
        systemProperty("java.library.path", "./fluentbindings/target/debug")
    }

    // TODO build for range of platforms (using rustup)
    register<Exec>("cargoBuild") {
        group = "Rust"
        description = "Builds the bindings crate"

        workingDir = file("fluentbindings")
        executable = "cargo"
        args("build", "--release")
    }

    // TODO include binaries in jar
}
