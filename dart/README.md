# Xaor Dart / Flutter Package

Dart FFI bindings for the **Xaor Cryptographic Engine** — an adaptive, memory-hard, chaos-keyed password hashing algorithm.

## Installation

Run this command in your terminal:

```bash
dart pub add xaor
# or for Flutter:
flutter pub add xaor
```

Or add this to your `pubspec.yaml`:

```yaml
dependencies:
  xaor: ^0.2.0
```

## Setup

Ensure the compiled dynamic library (`xaor.dll`, `libxaor.so`, or `libxaor.dylib`) is available in your runtime folder or system library paths.

## Quick Start

```dart
import 'package:xaor/xaor.dart';

void main() {
  final password = 'my-secure-dart-password';

  // 1. Hash a password
  final hash = Xaor.hashPassword(password);
  print('Generated Hash: $hash');

  // 2. Verify a password
  final isValid = Xaor.verifyPassword(password, hash);
  print('Is valid: $isValid'); // true
}
```
