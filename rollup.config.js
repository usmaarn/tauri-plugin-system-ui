import typescript from "@rollup/plugin-typescript";

export default {
  input: "guest-js/index.ts",
  output: [
    {
      file: "dist-js/index.js",
      format: "es",
    },
  ],
  plugins: [
    typescript({
      declaration: true,
      declarationDir: "dist-js",
      rootDir: "guest-js",
    }),
  ],
};
