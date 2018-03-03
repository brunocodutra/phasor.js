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
        loader: 'tslint-loader',
        exclude: /node_modules/,
        enforce: 'pre',
        options: {
          typeCheck: true,
        },
      },

      {
        test: /\.ts$/,
        use: ['babel-loader', 'ts-loader'],
        exclude: /node_modules/,
      },
    ]
  },
};
