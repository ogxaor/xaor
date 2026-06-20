/**
 * Hash a password using the memory-hard Xaor pipeline.
 *
 * @param password The plaintext password to hash.
 * @returns The self-describing PHC hash string.
 * @throws {Error} If hashing fails.
 */
export function hashPassword(password: string): string;

/**
 * Verify a password against a stored Xaor hash string.
 *
 * @param password The plaintext password to verify.
 * @param hash The stored Xaor hash string.
 * @returns `true` if the password is valid, `false` otherwise.
 * @throws {Error} If verification fails.
 */
export function verifyPassword(password: string, hash: string): boolean;
