plugins {
    // Apply the shared build logic from a convention plugin.
    // The shared code is located in `buildSrc/src/main/kotlin/kotlin-jvm.gradle.kts`.
    id("buildsrc.convention.kotlin-jvm")

    // Apply the Application plugin to add support for building an executable JVM application.
//    application
}

dependencies {
    // Project "app" depends on project "utils". (Project paths are separated with ":", so ":utils" refers to the top-level "utils" project.)
    implementation(project(":utilities"))
    implementation(kotlin("reflect"))
    // Apply the kotlinx bundle of dependencies from the version catalog (`gradle/libs.versions.toml`).
    implementation(libs.bundles.kotlinxEcosystem)
    testImplementation(kotlin("test"))
}

group = "org.gristle.puzzle-utilities.advent"

tasks.test {
    maxHeapSize = "2g"  // Adjust as needed (e.g., "1g", "4g")
}

//application {
//    // Define the Fully Qualified Name for the application main class
//    // (Note that Kotlin compiles `App.kt` to a class with FQN `com.example.app.AppKt`.)
//    mainClass = "org.gristle.app.AppKt"
//}
