# rust-wasm

Skip to main content
Select language
Skip to search
Technologies
References & Guides
Feedback
Search MDN
Search MDN
Sign in
Compiling from Rust to WebAssembly
See WebAssemblyCompiling from Rust to WebAssembly
English
▼
On this Page
Rust and WebAssembly use cases
Rust Environment Setup
Building our WebAssembly npm package
Using the package on the web
Conclusion
If you have some Rust code, you can compile it into WebAssembly (wasm). This tutorial takes you through all you need to know to compile a Rust project to wasm and use it in an existing web app.

Rust and WebAssembly use cases
There are two main use cases for Rust and WebAssembly:

To build an entire application — an entire web app based in Rust.
To build a part of an application — using Rust in an existing JavaScript frontend.
For now, the Rust team is focusing on the latter case, and so that's what we cover here. For the former case, check out projects like yew.

In this tutorial, we build an npm package using wasm-pack, a tool for building npm packages in Rust. This package will contain only WebAssembly and JavaScript code, and so the users of the package won't need Rust installed. They may not even notice that it's written in WebAssembly.

Rust Environment Setup
Let's go through all the required steps to get our environment set up.

Install Rust
Install Rust by going to the Install Rust page and following the instructions. This installs a tool called "rustup", which lets you manage multiple versions of Rust. By default, it installs the latest stable Rust release, which you can use for general Rust development. Rustup installs rustc, the Rust compiler, as well as cargo, Rust's package manager, rust-std, Rust's standard libraries, and some helpful docs — rust-docs.

Note: Pay attention to the post-install note about needing cargo's bin directory in your system PATH. This is added automatically, but you must restart your terminal for it to take effect.

wasm-pack
To build the package, we need an additional tool, wasm-pack. This helps compile the code to WebAssembly, as well as produce the right packaging for npm. To download and install it, enter the following command into your terminal:

$ cargo install wasm-pack
Install Node.js and get an npm account
We are building an npm package in this tutorial, and so you need to have Node.js and npm installed. Additionally, we are publishing our package to npm, and so you need an npm account as well. They're free of cost. You don't technically have to publish the package, but using it is much easier that way, and so we are assuming that you do in this tutorial.

To get Node.js and npm, go to the Get npm! page and follow the instructions. When it comes to picking a version, choose any one you'd like; this tutorial isn't version-specific.

To get an npm account, go to the npm signup page and fill out the form.

Next, at the command line run npm adduser:

$ npm adduser
Username: yournpmusername
Password:
Email: (this IS public) you@example.com
Fill out your own username, password, and email. If it worked, you see this printed:

Logged in as yournpmusername on https://registry.npmjs.org/.
If something didn't work, contact npm for troubleshooting help.

Building our WebAssembly npm package
Enough setup; let's create a new package in Rust. Navigate to wherever you keep your personal projects, and type this:

$ cargo new --lib hello-wasm
     Created library `hello-wasm` project
This creates a new library in a subdirectory named hello-wasm with everything you need to get going:

+-- Cargo.toml
+-- src
    +-- lib.rs
First, we have Cargo.toml; this is the file that we use to configure our build. If you've used Gemfile from Bundler or package.json from npm, this is likely to be familiar; Cargo works in a similar manner to both of them.

Next, Cargo has generated some Rust code for us in src/lib.rs:

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
We won't use this test code at all, so go ahead and delete it.

Let's write some Rust
Let's put this code into src/lib.rs instead:

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
This is the contents of our Rust project. It has three main parts; let's talk about them in turn. We give a high-level explanation here, and gloss over some details; to learn more about Rust, please check the free online book The Rust Programming Language.

Using wasm-bindgen to communicate between Rust and JavaScript
The first part looks like this:

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
The first line says "hey Rust, we're using a library called wasm_bindgen." Libraries are called "crates" in Rust, and we're using an external one, so we use the extern keyword.

Get it? Cargo ships crates.

The third line contains a use command, which imports code from a library into your code. In this case, we're importing everything in the wasm_bindgen::prelude module. We use these features in the next section.

Before we move on to the next section, we should talk a bit more about wasm-bindgen.

wasm-pack uses wasm-bindgen, another tool, to provide a bridge between the types of JavaScript and Rust. It allows JavaScript to call a Rust API with a string, or a Rust function to catch a JavaScript exception.

We use wasm-bindgen's functionality in our package. In fact, that's the next section.

Calling external functions in JavaScript from Rust
The next part looks like this:

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
The bit inside the #[ ] is called an "attribute", and it modifies the next statement somehow. In this case, that statement is an extern, which tells Rust that we want to call some externally defined functions. The attribute says "wasm-bindgen knows how to find these functions".

The third line is a function signature, written in Rust. It says "the alert function takes one argument, a string named s."

As you might suspect, this is the alert function provided by JavaScript. We call this function in the next section.

Whenever you want to call JavaScript functions, you can add them to this file, and wasm-bindgen takes care of setting everything up for you. Not everything is supported yet, but we're working on it. Please file bugs if something is missing.

Producing Rust functions that JavaScript can call
The final part is this one:

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
Once again, we see the #[wasm_bindgen] attribute. In this case, it's not modifying an extern block, but a fn; this means that we want this Rust function to be able to be called by JavaScript. It's the opposite of extern: these aren't the functions we need, but rather the functions we're giving out to the world.

This function is named greet, and takes one argument, a string (written &str), name. It then calls the alert function we asked for in the extern block above. It passes a call to the format! macro, which lets us concatenate strings.

The format! macro takes two arguments in this case, a format string, and a variable to put in it. The format string is the "Hello, {}!" bit. It contains {}s, where variables will be interpolated. The variable we're passing is name, the argument to the function, so if we call greet("Steve") we should see "Hello, Steve!".

This is passed to alert(), so when we call this function we will see an alert box with "Hello, Steve!" in it.

Now that our library is written, let's build it.

Compiling our code to WebAssembly
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

Building the package
Now that we've got everything set up, let's build the package. Type this into your terminal:

$ wasm-pack build --scope mynpmusername
This does a number of things (and they take a lot of time, especially the first time you run wasm-pack). To learn about them in detail, check out this blog post on Mozilla Hacks. In short, wasm-pack build:

Compiles your Rust code to WebAssembly.
Runs wasm-bindgen on that WebAssembly, generating a JavaScript file that wraps up that WebAssembly file into a module npm can understand.
Creates a pkg directory and move that JavaScript file and your WebAssembly code into it.
Reads your Cargo.toml and produces an equivalent package.json.
Copies your README.md (if you have one) into the package.
The end result? You have an npm package inside of the pkg directory.

A digression about code size
If you check out the generated WebAssembly code size, it may be a few hundred kilobytes. We haven't instructed Rust to optimize for size at all, and doing so cuts down on the size a lot. This is beyond the scope of this tutorial, but if you'd like to learn more, check out the Rust WebAssembly Working Group's documentation on Shrinking .wasm Size.

Publishing our package to npm
Let's publish our new package to the npm registry:

$ cd pkg
$ npm publish --access=public
We now have an npm package, written in Rust, but compiled to WebAssembly. It's ready for use from JavaScript, and doesn't require the user to have Rust installed; the code included was the WebAssembly code, not the Rust source.

Using the package on the web
Let's build a website that uses our new package. Many people use npm packages through various bundler tools, and we'll be using one of them, webpack, in this tutorial. It's only a little bit complex, and shows a realistic use-case.

Let's move back out of the pkg directory, and make a new directory, site, to try this out in:

$ cd ../..
$ mkdir site
$ cd site
Create a new file, package.json, and put the following code in it:

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
Note that you need to fill in your own username, after the @, in the dependencies section.

Next, we need to configure Webpack. Create webpack.config.js and put the following in it:

const path = require('path');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};
Now we need an HTML file; create index.html and give it the following contents:

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
Finally, create the index.js referenced in the HTML file and give it these contents:

const js = import("./node_modules/@yournpmusername/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
Note that you need to fill in your npm username again.

This imports the new module from the node_modules folder. This isn't considered a best practice, but this is a demo, so it's OK for now. Once it's loaded, it calls the greet function from that module, passing "WebAssembly" as a string. Note how there's nothing special here, yet we're calling into Rust code. As far as the JavaScript code can tell, this is just a normal module.

We're done making files. Let's give this a shot:

$ npm install
$ npm run serve
This starts a web server. Load http://localhost:8080 and an alert box appears on the screen, with Hello, WebAssembly! in it. We've successfully called from JavaScript into Rust, and from Rust into JavaScript.

Conclusion
This is the end of our tutorial; we hope you've found it useful.

There's lots of exciting work going on in this space; if you'd like to help make it even better, check out the Rust Webassembly Working Group.

Metadata
Last modified: Jun 3, 2020, by MDN contributors
Related Topics
WebAssembly home page
Tutorials
WebAssembly concepts
Compiling from C/C++ to WebAssembly
Compiling from Rust to WebAssembly
Using the WebAssembly JavaScript API
Understanding WebAssembly text format
Converting WebAssembly text format to wasm
Loading and running WebAssembly code
Caching compiled WebAssembly modules
Exported WebAssembly functions
Object reference
WebAssembly
WebAssembly.Module
WebAssembly.Global
WebAssembly.Instance
WebAssembly.Memory
WebAssembly.Table
WebAssembly.CompileError
WebAssembly.LinkError
WebAssembly.RuntimeError
Learn the best of web development
Get the latest and greatest from MDN delivered straight to your inbox.


E-mail
you@example.com
Sign up now
Hide Newsletter Sign-up
MDN Web Docs
Web Technologies
Learn Web Development
About MDN
Feedback
About
MDN Web Docs Store
Contact Us
Firefox
MDN
Mozilla
© 2005-2020 Mozilla and individual contributors. Content is available under these licenses.

Terms
Privacy
Cookies
