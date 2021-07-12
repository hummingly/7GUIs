// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    /* ... */
  },
  plugins: [
    ["@snowpack/plugin-babel", {
      transformOptions: {
        "presets": [
          [
            "@babel/preset-env",
            {
              "targets": {
                "esmodules": true
              }
            }
          ],
          [
            "@babel/preset-react",
            {
              "runtime": "automatic"
            }
          ]
        ]
      }
      ,
    },]

  ],
  packageOptions: {
    knownEntrypoints: ["react/jsx-runtime"]
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  }
};
