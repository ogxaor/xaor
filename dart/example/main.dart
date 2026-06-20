import 'package:xaor/xaor.dart';

void main() {
  print('Testing Dart FFI wrapper...');
  final password = 'my-secure-dart-password';
  
  print('Hashing password...');
  final hash = Xaor.hashPassword(password);
  print('Generated Hash Successfully: $hash');
  
  print('Verifying correct password...');
  final isValid = Xaor.verifyPassword(password, hash);
  print('Verification Result (should be true): $isValid');
  
  print('Verifying incorrect password...');
  final isInvalid = Xaor.verifyPassword('wrong-password', hash);
  print('Verification Result (should be false): $isInvalid');

  if (isValid && !isInvalid) {
    print('OK: Dart FFI wrapper test passed!');
  } else {
    print('ERROR: Dart FFI wrapper test failed.');
  }
}
