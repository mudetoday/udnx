# UDNX - Bevy Third-Person Prototype

Primitive third-person character controller built with Bevy 0.20, targeting desktop and Android.

## Desktop

```bash
cargo run
```

Controls: WASD / Arrow keys to move the red cube. Camera follows behind.

## Android Build

### Prerequisites

- Rust with Android target: `rustup target add aarch64-linux-android`
- `cargo install cargo-ndk`
- Android SDK + NDK (via Android Studio recommended)
- JDK 17+
- Set `ANDROID_HOME` / `ANDROID_SDK_ROOT` and `ANDROID_NDK_ROOT`

### Build steps

1. Build the Rust library for Android:

```bash
cargo ndk -t arm64-v8a -P 26 -o android/app/src/main/jniLibs build --release
```

(For other ABIs add `-t armeabi-v7a` etc.)

2. Generate Gradle wrapper if missing:

```bash
cd android
gradle wrapper
```

3. Build & install APK:

```bash
cd android
./gradlew installDebug
```

Or open the `android/` folder in Android Studio and run.

### Notes

- Touch control is basic (touch anywhere relative to screen center).
- Performance on mobile may require further optimization (disable shadows, MSAA, etc. — already partially done).
- Add launcher icons in `android/app/src/main/res/mipmap-*` or let Android Studio generate them.
- Library name is `udnx` (matches `System.loadLibrary("udnx")`).

## Structure

- `src/lib.rs` — main game logic (shared between desktop & Android)
- `src/main.rs` — desktop entry point
- `android/` — Gradle Android project
