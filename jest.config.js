module.exports = {
  moduleFileExtensions: ['js', 'ts'],
  moduleDirectories: ['node_modules', 'src'],
  setupTestFrameworkScriptFile: './spec/jest/setup.js',
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
  testMatch: ['**/spec/**/*.spec.ts'],
  collectCoverage: true,
  collectCoverageFrom: ['**/src/**/*.ts'],
  coverageReporters: ['text'],
  globals: {
    'ts-jest': {
      tsConfigFile: './spec/tsconfig.json',
    },
  },
}
