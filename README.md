# rust-wasm

The following is NOT MINE! For my own reference, these notes were copied from  <https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm>

## Compiling our code to WebAssembly

To compile our code correctly, we first need to configure it with Cargo.toml. Open this file, and change its contents to look like this:

     [package]
     name = "hello-wasm"
     version = "0.1.0"
     authors = ["Your Name <you@example.com>"]
     description = "A sample project with wasm-pack"
     license = "MIT/Apache-2.0"
     repository = "https://github.com/yourgithubusername/hello-wasm"

     [lib]
     crate-type = ["cdylib"]

     [dependencies]
     wasm-bindgen = "0.2"
     
Fill in your own repository and use the same info that git uses for the authors field.

The big part to add is the stuff at the bottom. The first part — [lib] — tells Rust to build a cdylib version of our package; we won't get into what that means in this tutorial. For more, consult the Cargo and Rust Linkage documentation.

The last section is the [dependencies] section. Here's where we tell Cargo what version of wasm-bindgen we want to depend on; in this case, that's any 0.2.z version (but not 0.3.0 or above).

## Building the package

Now that we've got everything set up, let's build the package. Type this into your terminal:

     $ wasm-pack build --scope mynpmusername

This does a number of things (and they take a lot of time, especially the first time you run wasm-pack). To learn about them in detail, check out this blog post on Mozilla Hacks. In short, wasm-pack build:

Compiles your Rust code to WebAssembly.

Runs wasm-bindgen on that WebAssembly, generating a JavaScript file that wraps up that WebAssembly file into a module npm can understand.

Creates a pkg directory and move that JavaScript file and your WebAssembly code into it.

Reads your Cargo.toml and produces an equivalent package.json.

Copies your README.md (if you have one) into the package.

The end result? You have an npm package inside of the pkg directory.

### A digression about code size

If you check out the generated WebAssembly code size, it may be a few hundred kilobytes. We haven't instructed Rust to optimize for size at all, and doing so cuts down on the size a lot. This is beyond the scope of this tutorial, but if you'd like to learn more, check out the Rust WebAssembly Working Group's documentation on Shrinking .wasm Size.

## Publishing our package to npm

Let's publish our new package to the npm registry:

     $ cd pkg
     $ npm publish --access=public

We now have an npm package, written in Rust, but compiled to WebAssembly. It's ready for use from JavaScript, and doesn't require the user to have Rust installed; the code included was the WebAssembly code, not the Rust source.

## Using the package on the web

Let's build a website that uses our new package. Many people use npm packages through various bundler tools, and we'll be using one of them, webpack, in this tutorial. It's only a little bit complex, and shows a realistic use-case.

Let's move back out of the pkg directory, and make a new directory, site, to try this out in:

     $ cd ../..
     $ mkdir site
     $ cd site
     
Create a new file, package.json, and put the following code in it:

```json
{
  "scripts": {
    "serve": "webpack-dev-server"
  },
  "dependencies": {
    "@mynpmusername/hello-wasm": "^0.1.0"
  },
  "devDependencies": {
    "webpack": "^4.25.1",
    "webpack-cli": "^3.1.2",
    "webpack-dev-server": "^3.1.10"
  }
}
```

Note that you need to fill in your own username, after the @, in the dependencies section.

Next, we need to configure Webpack. Create webpack.config.js and put the following in it:

```rust
const path = require('path');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};
```

Now we need an HTML file; create index.html and give it the following contents:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>hello-wasm example</title>
  </head>
  <body>
    <script src="./index.js"></script>
  </body>
</html>
```

Finally, create the index.js referenced in the HTML file and give it these contents:

```js
const js = import("./node_modules/@yournpmusername/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
```

Note that you need to fill in your npm username again.

This imports the new module from the node_modules folder. This isn't considered a best practice, but this is a demo, so it's OK for now. Once it's loaded, it calls the greet function from that module, passing "WebAssembly" as a string. Note how there's nothing special here, yet we're calling into Rust code. As far as the JavaScript code can tell, this is just a normal module.

We're done making files. Let's give this a shot:

     $ npm install
     $ npm run serve
     
This starts a web server. Load http://localhost:8080 and an alert box appears on the screen, with Hello, WebAssembly! in it. We've successfully called from JavaScript into Rust, and from Rust into JavaScript.
