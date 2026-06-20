import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;
import 'package:ffi/ffi.dart';
import 'package:path/path.dart' as p;

// C API types definitions
typedef XaorHashC = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8> password);
typedef XaorHashDart = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8> password);

typedef XaorVerifyC =
    ffi.Int32 Function(ffi.Pointer<Utf8> password, ffi.Pointer<Utf8> stored);
typedef XaorVerifyDart =
    int Function(ffi.Pointer<Utf8> password, ffi.Pointer<Utf8> stored);

typedef XaorFreeStringC = ffi.Void Function(ffi.Pointer<Utf8> ptr);
typedef XaorFreeStringDart = void Function(ffi.Pointer<Utf8> ptr);

typedef XaorLastErrorC = ffi.Pointer<Utf8> Function();
typedef XaorLastErrorDart = ffi.Pointer<Utf8> Function();

class Xaor {
  static ffi.DynamicLibrary? _lib;

  static ffi.DynamicLibrary get _library {
    if (_lib != null) return _lib!;

    String libName;
    if (Platform.isWindows) {
      libName = 'xaor.dll';
    } else if (Platform.isMacOS) {
      libName = 'libxaor.dylib';
    } else {
      libName = 'libxaor.so';
    }

    // Try finding the library in local targets for dev, or root folder
    final possiblePaths = [
      libName,
      p.join(Directory.current.path, libName),
      p.join(Directory.current.path, 'target', 'release', libName),
      p.join(Directory.current.path, 'target', 'debug', libName),
      p.join(Directory.current.path, '..', libName),
      p.join(Directory.current.path, 'dart', libName),
    ];

    for (final path in possiblePaths) {
      try {
        _lib = ffi.DynamicLibrary.open(path);
        return _lib!;
      } catch (_) {}
    }

    // Default system fallback
    try {
      _lib = ffi.DynamicLibrary.open(libName);
      return _lib!;
    } catch (e) {
      throw Exception(
        'Failed to load the xaor shared library ($libName). '
        'Ensure the library file is in your executable directory, target/release, or system library path. Error: $e',
      );
    }
  }

  // Bind C functions
  static final _hash = _library.lookupFunction<XaorHashC, XaorHashDart>(
    'xaor_hash',
  );
  static final _verify = _library.lookupFunction<XaorVerifyC, XaorVerifyDart>(
    'xaor_verify',
  );
  static final _freeString = _library
      .lookupFunction<XaorFreeStringC, XaorFreeStringDart>('xaor_free_string');
  static final _lastError = _library
      .lookupFunction<XaorLastErrorC, XaorLastErrorDart>('xaor_last_error');

  /// Hash a password using Xaor's memory-hard pipeline.
  static String hashPassword(String password) {
    final passwordPtr = password.toNativeUtf8();
    final hashPtr = _hash(passwordPtr);
    malloc.free(passwordPtr);

    if (hashPtr == ffi.nullptr) {
      final errPtr = _lastError();
      final err = errPtr == ffi.nullptr
          ? 'Unknown error'
          : errPtr.toDartString();
      if (errPtr != ffi.nullptr) _freeString(errPtr);
      throw Exception('Hashing failed: $err');
    }

    final hash = hashPtr.toDartString();
    _freeString(hashPtr);
    return hash;
  }

  /// Verify a password against a stored Xaor hash string.
  static bool verifyPassword(String password, String hash) {
    final passwordPtr = password.toNativeUtf8();
    final hashPtr = hash.toNativeUtf8();
    final result = _verify(passwordPtr, hashPtr);
    malloc.free(passwordPtr);
    malloc.free(hashPtr);

    if (result == -1) {
      final errPtr = _lastError();
      final err = errPtr == ffi.nullptr
          ? 'Unknown error'
          : errPtr.toDartString();
      if (errPtr != ffi.nullptr) _freeString(errPtr);
      throw Exception('Verification failed: $err');
    }

    return result == 1;
  }
}
