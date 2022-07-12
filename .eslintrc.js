module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: ['prettier'],
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 12,
    sourceType: 'module',
    parser: 'babel-eslint',
  },
  plugins: ['react', 'prettier', 'react'],
  rules: {
    'prettier/prettier': [
      'error',
      {
        singleQuote: true,
        trailingComma: 'all',
      },
    ],
    camelcase: 'off',
    'no-new': 'off',
    'space-before-function-paren': 'off',
    'no-plusplus': 'off',
    'max-len': 'off',
    'func-names': 'off',
    'no-param-reassign': 'off',
  },
};
