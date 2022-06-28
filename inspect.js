revision:  bf7dd01
[
  {
    'process.env': {
      NODE_ENV: '"development"',
      BASE_URL: '"/"',
      MONA_VERSION: '"5.7.1"',
      MONA_BUILD_DATE: '"2022/6/28"',
      MONA_REVISION: '"bf7dd01"'
    }
  }
]
{
  mode: 'development',
  context: 'E:\\vue\\genshin_artifact',
  output: {
    hashFunction: 'xxhash64',
    path: 'E:\\vue\\genshin_artifact\\dist',
    filename: 'js/[name].js',
    publicPath: '/',
    chunkFilename: 'js/[name].js'
  },
  resolve: {
    alias: {
      '@': 'E:\\vue\\genshin_artifact\\src',
      vue$: 'vue/dist/vue.runtime.esm-bundler.js',
      '@c': 'E:\\vue\\genshin_artifact\\src\\components',
      '@asset': 'E:\\vue\\genshin_artifact\\src\\assets',
      '@util': 'E:\\vue\\genshin_artifact\\src\\utils',
      '@alg': 'E:\\vue\\genshin_artifact\\src\\algorithms',
      '@page': 'E:\\vue\\genshin_artifact\\src\\pages',
      '@worker': 'E:\\vue\\genshin_artifact\\src\\workers',
      '@const': 'E:\\vue\\genshin_artifact\\src\\constants',
      '@enemy': 'E:\\vue\\genshin_artifact\\src\\enemies',
      mona: 'E:\\vue\\genshin_artifact\\mona_wasm\\pkg',
      '@wasm': 'E:\\vue\\genshin_artifact\\src\\wasm',
      '@character': 'E:\\vue\\genshin_artifact\\src\\assets\\character',
      '@weapon': 'E:\\vue\\genshin_artifact\\src\\assets\\weapon',
      '@targetFunction': 'E:\\vue\\genshin_artifact\\src\\assets\\target_function',
      '@potentialFunction': 'E:\\vue\\genshin_artifact\\src\\assets\\potential_function',
      '@buff': 'E:\\vue\\genshin_artifact\\src\\assets\\buff',
      '@image': 'E:\\vue\\genshin_artifact\\src\\images',
      '@artifact': 'E:\\vue\\genshin_artifact\\src\\assets\\artifacts'
    },
    extensions: [
      '.tsx',
      '.ts',
      '.mjs',
      '.js',
      '.jsx',
      '.vue',
      '.json',
      '.wasm',
      '.vue',
      '.png',
      '.jpg',
      '.webp'
    ],
    modules: [
      'node_modules',
      'E:\\vue\\genshin_artifact\\node_modules',
      'E:\\vue\\genshin_artifact\\node_modules\\@vue\\cli-service\\node_modules'
    ]
  },
  resolveLoader: {
    modules: [
      'E:\\vue\\genshin_artifact\\node_modules\\@vue\\cli-plugin-typescript\\node_modules',
      'E:\\vue\\genshin_artifact\\node_modules\\@vue\\cli-plugin-babel\\node_modules',
      'node_modules',
      'E:\\vue\\genshin_artifact\\node_modules',
      'E:\\vue\\genshin_artifact\\node_modules\\@vue\\cli-service\\node_modules'
    ]
  },
  module: {
    noParse: /^(vue|vue-router|vuex|vuex-router-sync)$/,
    rules: [
      /* config.module.rule('esm') */
      {
        test: /\.m?jsx?$/,
        resolve: {
          fullySpecified: false
        }
      },
      /* config.module.rule('vue') */
      {
        test: /\.vue$/,
        use: [
          /* config.module.rule('vue').use('vue-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-loader\\dist\\index.js',
            options: {
              cacheDirectory: 'E:\\vue\\genshin_artifact\\node_modules\\.cache\\vue-loader',
              cacheIdentifier: 'c0ba2d4e',
              babelParserPlugins: [
                'jsx',
                'classProperties',
                'decorators-legacy'
              ]
            }
          },
          /* config.module.rule('vue').use('ifdef') */
          {
            loader: 'ifdef-loader',
            options: {
              USE_CDN: false
            }
          }
        ]
      },
      /* config.module.rule('vue-style') */
      {
        test: /\.vue$/,
        resourceQuery: /type=style/,
        sideEffects: true
      },
      /* config.module.rule('pug') */
      {
        test: /\.pug$/,
        oneOf: [
          /* config.module.rule('pug').oneOf('pug-vue') */
          {
            resourceQuery: /vue/,
            use: [
              /* config.module.rule('pug').oneOf('pug-vue').use('pug-plain-loader') */
              {
                loader: 'pug-plain-loader'
              }
            ]
          },
          /* config.module.rule('pug').oneOf('pug-template') */
          {
            use: [
              /* config.module.rule('pug').oneOf('pug-template').use('raw') */
              {
                loader: 'raw-loader'
              },
              /* config.module.rule('pug').oneOf('pug-template').use('pug-plain-loader') */
              {
                loader: 'pug-plain-loader'
              }
            ]
          }
        ]
      },
      /* config.module.rule('svg') */
      {
        test: /\.(svg)(\?.*)?$/,
        type: 'asset/resource',
        generator: {
          filename: 'img/[name].[hash:8][ext]'
        }
      },
      /* config.module.rule('images') */
      {
        test: /\.(png|jpe?g|gif|webp|avif)(\?.*)?$/,
        type: 'asset',
        generator: {
          filename: 'img/[name].[hash:8][ext]'
        }
      },
      /* config.module.rule('media') */
      {
        test: /\.(mp4|webm|ogg|mp3|wav|flac|aac)(\?.*)?$/,
        type: 'asset',
        generator: {
          filename: 'media/[name].[hash:8][ext]'
        }
      },
      /* config.module.rule('fonts') */
      {
        test: /\.(woff2?|eot|ttf|otf)(\?.*)?$/i,
        type: 'asset',
        generator: {
          filename: 'fonts/[name].[hash:8][ext]'
        }
      },
      /* config.module.rule('css') */
      {
        test: /\.css$/,
        oneOf: [
          /* config.module.rule('css').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('css').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('css').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('css').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('css').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('css').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('css').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('css').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('css').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('css').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('css').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('css').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('css').oneOf('normal') */
          {
            use: [
              /* config.module.rule('css').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('css').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('css').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('postcss') */
      {
        test: /\.p(ost)?css$/,
        oneOf: [
          /* config.module.rule('postcss').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('postcss').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('postcss').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('postcss').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('postcss').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('postcss').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('postcss').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('postcss').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('postcss').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('postcss').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('postcss').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('postcss').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          },
          /* config.module.rule('postcss').oneOf('normal') */
          {
            use: [
              /* config.module.rule('postcss').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('postcss').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('postcss').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('scss') */
      {
        test: /\.scss$/,
        oneOf: [
          /* config.module.rule('scss').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('scss').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('scss').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('scss').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('scss').oneOf('vue-modules').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('scss').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('scss').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('scss').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('scss').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('scss').oneOf('vue').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('scss').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('scss').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('scss').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('scss').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('scss').oneOf('normal-modules').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('scss').oneOf('normal') */
          {
            use: [
              /* config.module.rule('scss').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('scss').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('scss').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('scss').oneOf('normal').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('sass') */
      {
        test: /\.sass$/,
        oneOf: [
          /* config.module.rule('sass').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('sass').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('sass').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('sass').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('sass').oneOf('vue-modules').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  sassOptions: {
                    indentedSyntax: true
                  }
                }
              }
            ]
          },
          /* config.module.rule('sass').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('sass').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('sass').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('sass').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('sass').oneOf('vue').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  sassOptions: {
                    indentedSyntax: true
                  }
                }
              }
            ]
          },
          /* config.module.rule('sass').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('sass').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('sass').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('sass').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('sass').oneOf('normal-modules').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  sassOptions: {
                    indentedSyntax: true
                  }
                }
              }
            ]
          },
          /* config.module.rule('sass').oneOf('normal') */
          {
            use: [
              /* config.module.rule('sass').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('sass').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('sass').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('sass').oneOf('normal').use('sass-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\sass-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  sassOptions: {
                    indentedSyntax: true
                  }
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('less') */
      {
        test: /\.less$/,
        oneOf: [
          /* config.module.rule('less').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('less').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('less').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('less').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('less').oneOf('vue-modules').use('less-loader') */
              {
                loader: 'less-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('less').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('less').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('less').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('less').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('less').oneOf('vue').use('less-loader') */
              {
                loader: 'less-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('less').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('less').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('less').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('less').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('less').oneOf('normal-modules').use('less-loader') */
              {
                loader: 'less-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('less').oneOf('normal') */
          {
            use: [
              /* config.module.rule('less').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('less').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('less').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('less').oneOf('normal').use('less-loader') */
              {
                loader: 'less-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('stylus') */
      {
        test: /\.styl(us)?$/,
        oneOf: [
          /* config.module.rule('stylus').oneOf('vue-modules') */
          {
            resourceQuery: /module/,
            use: [
              /* config.module.rule('stylus').oneOf('vue-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('stylus').oneOf('vue-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2,
                  modules: {
                    localIdentName: '[name]_[local]_[hash:base64:5]',
                    auto: () => true
                  }
                }
              },
              /* config.module.rule('stylus').oneOf('vue-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('stylus').oneOf('vue-modules').use('stylus-loader') */
              {
                loader: 'stylus-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('stylus').oneOf('vue') */
          {
            resourceQuery: /\?vue/,
            use: [
              /* config.module.rule('stylus').oneOf('vue').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('stylus').oneOf('vue').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('stylus').oneOf('vue').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('stylus').oneOf('vue').use('stylus-loader') */
              {
                loader: 'stylus-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('stylus').oneOf('normal-modules') */
          {
            test: /\.module\.\w+$/,
            use: [
              /* config.module.rule('stylus').oneOf('normal-modules').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('stylus').oneOf('normal-modules').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('stylus').oneOf('normal-modules').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('stylus').oneOf('normal-modules').use('stylus-loader') */
              {
                loader: 'stylus-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          },
          /* config.module.rule('stylus').oneOf('normal') */
          {
            use: [
              /* config.module.rule('stylus').oneOf('normal').use('vue-style-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\vue-style-loader\\index.js',
                options: {
                  sourceMap: false,
                  shadowMode: false
                }
              },
              /* config.module.rule('stylus').oneOf('normal').use('css-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\css-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  importLoaders: 2
                }
              },
              /* config.module.rule('stylus').oneOf('normal').use('postcss-loader') */
              {
                loader: 'E:\\vue\\genshin_artifact\\node_modules\\postcss-loader\\dist\\cjs.js',
                options: {
                  sourceMap: false,
                  postcssOptions: {
                    plugins: [
                      function () { /* omitted long function */ }
                    ]
                  }
                }
              },
              /* config.module.rule('stylus').oneOf('normal').use('stylus-loader') */
              {
                loader: 'stylus-loader',
                options: {
                  sourceMap: false
                }
              }
            ]
          }
        ]
      },
      /* config.module.rule('js') */
      {
        test: /\.m?jsx?$/,
        exclude: [
          function () { /* omitted long function */ }
        ],
        use: [
          /* config.module.rule('js').use('babel-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\babel-loader\\lib\\index.js',
            options: {
              cacheCompression: false,
              cacheDirectory: 'E:\\vue\\genshin_artifact\\node_modules\\.cache\\babel-loader',
              cacheIdentifier: '6dbda1ba'
            }
          },
          /* config.module.rule('js').use('ifdef') */
          {
            loader: 'ifdef-loader',
            options: {
              USE_CDN: false
            }
          }
        ]
      },
      /* config.module.rule('ts') */
      {
        test: /\.ts$/,
        use: [
          /* config.module.rule('ts').use('babel-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\babel-loader\\lib\\index.js'
          },
          /* config.module.rule('ts').use('ts-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\ts-loader\\index.js',
            options: {
              transpileOnly: true,
              appendTsSuffixTo: [
                '\\.vue$'
              ],
              happyPackMode: false
            }
          }
        ]
      },
      /* config.module.rule('tsx') */
      {
        test: /\.tsx$/,
        use: [
          /* config.module.rule('tsx').use('babel-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\babel-loader\\lib\\index.js'
          },
          /* config.module.rule('tsx').use('ts-loader') */
          {
            loader: 'E:\\vue\\genshin_artifact\\node_modules\\ts-loader\\index.js',
            options: {
              transpileOnly: true,
              happyPackMode: false,
              appendTsxSuffixTo: [
                '\\.vue$'
              ]
            }
          }
        ]
      }
    ]
  },
  optimization: {
    realContentHash: false,
    splitChunks: {
      cacheGroups: {
        defaultVendors: {
          name: 'chunk-vendors',
          test: /[\\/]node_modules[\\/]/,
          priority: -10,
          chunks: 'initial'
        },
        common: {
          name: 'chunk-common',
          minChunks: 2,
          priority: -20,
          chunks: 'initial',
          reuseExistingChunk: true
        }
      }
    },
    minimizer: [
      /* config.optimization.minimizer('terser') */
      new TerserPlugin(
        {
          terserOptions: {
            compress: {
              arrows: false,
              collapse_vars: false,
              comparisons: false,
              computed_props: false,
              hoist_funs: false,
              hoist_props: false,
              hoist_vars: false,
              inline: false,
              loops: false,
              negate_iife: false,
              properties: false,
              reduce_funcs: false,
              reduce_vars: false,
              switches: false,
              toplevel: false,
              typeofs: false,
              booleans: true,
              if_return: true,
              sequences: true,
              unused: true,
              conditionals: true,
              dead_code: true,
              evaluate: true
            },
            mangle: {
              safari10: true
            }
          },
          parallel: true,
          extractComments: false
        }
      )
    ]
  },
  plugins: [
    /* config.plugin('vue-loader') */
    new VueLoaderPlugin(),
    /* config.plugin('feature-flags') */
    new DefinePlugin(
      {
        __VUE_OPTIONS_API__: 'true',
        __VUE_PROD_DEVTOOLS__: 'false'
      }
    ),
    /* config.plugin('define') */
    new DefinePlugin(
      {
        'process.env': {
          NODE_ENV: '"development"',
          BASE_URL: '"/"',
          MONA_VERSION: '"5.7.1"',
          MONA_BUILD_DATE: '"2022/6/28"',
          MONA_REVISION: '"bf7dd01"'
        }
      }
    ),
    /* config.plugin('case-sensitive-paths') */
    new CaseSensitivePathsPlugin(),
    /* config.plugin('friendly-errors') */
    new FriendlyErrorsWebpackPlugin(
      {
        additionalTransformers: [
          function () { /* omitted long function */ }
        ],
        additionalFormatters: [
          function () { /* omitted long function */ }
        ]
      }
    ),
    /* config.plugin('html') */
    new HtmlWebpackPlugin(
      {
        title: 'genshin_artifacts',
        scriptLoading: 'defer',
        templateParameters: function () { /* omitted long function */ },
        template: 'E:\\vue\\genshin_artifact\\public\\index.html'
      }
    ),
    /* config.plugin('copy') */
    new CopyPlugin(
      {
        patterns: [
          {
            from: 'E:\\vue\\genshin_artifact\\public',
            to: 'E:\\vue\\genshin_artifact\\dist',
            toType: 'dir',
            noErrorOnMissing: true,
            globOptions: {
              ignore: [
                '**/.DS_Store',
                'E:/vue/genshin_artifact/public/index.html'
              ]
            },
            info: {
              minimized: true
            }
          }
        ]
      }
    ),
    /* config.plugin('eslint') */
    new ESLintWebpackPlugin(
      {
        extensions: [
          '.js',
          '.jsx',
          '.vue',
          '.ts',
          '.tsx'
        ],
        cwd: 'E:\\vue\\genshin_artifact',
        cache: true,
        cacheLocation: 'E:\\vue\\genshin_artifact\\node_modules\\.cache\\eslint\\8c822818.json',
        context: 'E:\\vue\\genshin_artifact',
        failOnWarning: false,
        failOnError: true,
        eslintPath: 'E:\\vue\\genshin_artifact\\node_modules\\eslint',
        formatter: 'stylish'
      }
    ),
    /* config.plugin('fork-ts-checker') */
    new ForkTsCheckerWebpackPlugin(
      {
        typescript: {
          extensions: {
            vue: {
              enabled: true,
              compiler: 'E:\\vue\\genshin_artifact\\node_modules\\vue\\compiler-sfc\\index.js'
            }
          },
          diagnosticOptions: {
            semantic: true,
            syntactic: false
          }
        }
      }
    ),
    /* config.plugin('monaco') */
    {},
    {
      apply: function () { /* omitted long function */ }
    },
    {
      apply: function () { /* omitted long function */ }
    },
    {
      apply: function () { /* omitted long function */ }
    }
  ],
  entry: {
    app: [
      './src/main.ts'
    ]
  },
  externals: {
    vue: [
      'https://npm.elemecdn.com/vue@3.2.36/dist/vue.global.prod.js',
      'Vue'
    ],
    'vue-router': [
      'https://npm.elemecdn.com/vue-router@4.0.16/dist/vue-router.global.prod.js',
      'VueRouter'
    ],
    'echarts/core': [
      'https://npm.elemecdn.com/echarts@5.3.3/core.js',
      'echarts'
    ],
    'vue-echarts': [
      'https://npm.elemecdn.com/vue-echarts@6.1.0/dist/index.umd.min.js',
      'VueECharts'
    ],
    'monaco-editor': 'var monaco'
  },
  externalsType: 'script',
  experiments: {
    asyncWebAssembly: true
  }
}
