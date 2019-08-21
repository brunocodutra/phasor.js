module.exports = {
  moduleFileExtensions: ['js', 'ts'],
  moduleDirectories: ['node_modules', 'src'],
  setupFilesAfterEnv: ['./jest/setup.js'],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
  testMatch: ['**/spec/**/*.spec.ts'],
  collectCoverage: true,
  collectCoverageFrom: ['**/src/**/*.ts'],
  coverageReporters: ['lcov', 'text'],
  globals: {
    'ts-jest': {
      tsConfig: './spec/tsconfig.json',
    },
  },
}
