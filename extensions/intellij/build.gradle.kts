plugins {
    kotlin("jvm") version "1.6.20"
    id("org.jetbrains.intellij") version "1.5.2"
}

group = "dev.tigr"
version = "0.3"

sourceSets["main"].java.srcDirs("src/main/gen")

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

// See https://github.com/JetBrains/gradle-intellij-plugin/
intellij {
    version.set("2022.1")
}
tasks {
    patchPluginXml {
        changeNotes.set("""
            <ul>
                <li>Added a built-in compiler, which shows its output at the bottom of every melody file editor window.</li>
                <li>Added support for unicode categories.</li>
            </ul>
        """.trimIndent())
        version.set(project.version.toString())
    }
}
