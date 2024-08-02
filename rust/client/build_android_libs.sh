
export ANDROID_NDK_HOME=~/Android/Sdk/ndk
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -o ./jniLibs build --release
