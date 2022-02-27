plugins {
    kotlin("jvm") version "1.5.10"
    id("org.jetbrains.intellij") version "1.4.0"
}

group = "dev.tigr"
version = "0.1"

sourceSets["main"].java.srcDirs("src/main/gen")

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

// See https://github.com/JetBrains/gradle-intellij-plugin/
intellij {
    version.set("2021.2")
}
tasks {
    patchPluginXml {
        changeNotes.set("""
            Added syntax highlighting
        """.trimIndent())
        version.set(project.version.toString())
    }
}