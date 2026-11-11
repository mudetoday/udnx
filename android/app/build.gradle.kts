plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "org.bevyengine.udnx"
    compileSdk = 35

    defaultConfig {
        applicationId = "org.bevyengine.udnx"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        ndk {
            abiFilters.addAll(listOf("arm64-v8a", "armeabi-v7a", "x86_64"))
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"))
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    implementation(libs.appcompat)
    implementation(libs.core)
    implementation(libs.material)
    implementation(libs.games.activity)
    implementation(libs.core.ktx)
}
