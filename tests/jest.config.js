module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    testTimeout: 200000,
    setupFilesAfterEnv: ['tests/jestSetup.ts'],
    globals: {
      'ts-jest': {
        tsconfig: './tsconfig.json',
      },
    }
   }