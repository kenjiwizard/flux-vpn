plugins {
    id("com.android.application") version "8.2.0"
    id("org.jetbrains.kotlin.android") version "1.9.22"
}

android {
    namespace = "org.fluxvpn.app"
    compileSdk = 34
    defaultConfig {
        applicationId = "org.fluxvpn.app"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0"
    }
}
