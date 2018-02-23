module.exports = {
  output: {
    filename: 'index.js',
    libraryTarget: 'umd',
    globalObject: 'this', // FIXME: https://github.com/webpack/webpack/issues/6522
  },

  resolve: {
    extensions: ['.js', '.ts'],
  },

  module: {
    rules: [
      {
        test: /\.ts$/,
        use: ['babel-loader', 'ts-loader'],
        exclude: /node_modules/,
      },
    ]
  },
};
