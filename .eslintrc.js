module.exports = {
  env: {
    browser: true,
  },
  parserOptions: {
    parser: 'babel-eslint',
  },
  extends: ['react-app', 'prettier'],
  plugins: ['prettier', 'import', 'react'],
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
