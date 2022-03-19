plugins {
    kotlin("jvm") version "1.5.10"
    id("org.jetbrains.intellij") version "1.4.0"
}

group = "dev.tigr"
version = "0.2"

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
            <ul>
                <li>Added let and lazy keyword support</li>
                <li>Added &lt;backspace&gt;, &lt;boundary&gt;, and &lt;alphanumeric&gt; support</li>
                <li>Renamed &lt;alphabet&gt; to &lt;alphabetic&gt;</li>
                <li>Added multi-line comment support</li>
                <li>Added character range support</li>
                <li>Added .melody file extension support</li>
                <li>Switch to new Melody logo!</li>
            </ul>
        """.trimIndent())
        version.set(project.version.toString())
    }
}