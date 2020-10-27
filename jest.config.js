module.exports = {
  preset: 'ts-jest',
  setupFilesAfterEnv: ['./jest/setup.js'],
  testMatch: ['**/spec/**/*.spec.ts'],
}
