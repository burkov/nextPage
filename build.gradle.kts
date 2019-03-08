import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.3.20"
    id("com.github.johnrengelman.shadow") version "5.0.0"
}

group = "com.github.burkov"
version = "1.0-SNAPSHOT"

repositories {
    jcenter()
}

val jar by tasks.getting(Jar::class) {
    manifest {
        attributes["Main-Class"] = "com.github.burkov.nextPage.MainKt"
    }
}

dependencies {
    implementation(kotlin("stdlib-jdk8"))
    implementation("com.github.kittinunf.fuel", "fuel", "2.0.0")
    implementation("org.jsoup", "jsoup", "1.11.3")
    implementation("org.apache.commons", "commons-text", "1.6")
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}