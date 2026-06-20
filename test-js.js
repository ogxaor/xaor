const { hashPassword, verifyPassword } = require('./index');

try {
  console.log("Testing Node.js FFI wrapper...");
  const password = "my-secure-node-password";
  
  console.log("Hashing password...");
  const hash = hashPassword(password);
  console.log("Generated hash successfully:", hash);
  
  console.log("Verifying correct password...");
  const isValid = verifyPassword(password, hash);
  console.log("Verification result (should be true):", isValid);
  
  console.log("Verifying incorrect password...");
  const isInvalid = verifyPassword("wrong-password", hash);
  console.log("Verification result (should be false):", isInvalid);
  
  if (isValid && !isInvalid) {
    console.log("✅ Node.js FFI wrapper test passed!");
  } else {
    console.error("❌ Node.js FFI wrapper test failed: incorrect validation results.");
    process.exit(1);
  }
} catch (error) {
  console.error("❌ Node.js FFI wrapper test crashed:", error);
  process.exit(1);
}
