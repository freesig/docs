# Hello Test Tutorial
Welcome to the hello test tutorial. Today you will be learning how to test your Holochain apps. This tutorial will add to the previous [hello holo]() tutorial. So make sure you do that one first.

Testing is a really important part of building higher quality apps but it's also a an excellent way to think through how your app will be used.

## Understand the tests 
When you ran `hc init` in the previous tutorial Holochain already generated some tests for you.

The tests are written in javascript and use the Holochain testing framework [Diorama]().

Open up the `hello_holo/test/index.js` in your favourite text editor.

Have a look through the code and remove the unneeded section.

Imports required to do testing.
```javascript
const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')
```
this will print `unhandledRejection err` if something is not defined.
```javascript
process.on('unhandledRejection', error => {
  console.error('got unhandledRejection:', error);
});
```
The path to your compiled dna.
```javascript
const dnaPath = path.join(__dirname, "../dist/hello_holo.dna.json")
const dna = Diorama.dna(dnaPath, 'hello_holo')
```

Setup the testing environment.
This creates two nodes: Alice and Bob.
```javascript
const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})
```

This is the test that holochain generated based on the `my_entry` struct. It won't work for our hello holo tutorial so let's remove it.
Remove the following section:
```javascript
diorama.registerScenario("description of example test", async (s, t, { alice }) => {
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("my_zome", "create_my_entry", {"entry" : {"content":"sample content"}})
  const result = await alice.call("my_zome", "get_my_entry", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'my_entry', '{"content":"sample content"}' ] } })
})
```
This line will run the tests that you have set up.
```javascript
diorama.run()
```

## Create a test scenario
Tests are organized by creating scenarios. Think of them as a series of actions the user takes when interacting with your app.

For this test you simply want to create the Alice user and get her to call the `hello_holo` zome function.
Then check that you get the result `Hello Holo`.

Place the following just above `diorama.run()`.

Register a test scenario that checks `hello_holo()` returns the correct value:
```javascript
diorama.registerScenario("Test hello holo", async (s, t, { alice }) => {
```
Make a call to the `hello_holo` Zome function passing no arguments:
```javascript
  const result = await alice.call("hello", "hello_holo", {});
```
Make sure the result is ok:
```javascript
  t.ok(result.Ok);
```
Check that the result matches what you expected:
```javascript
  t.deepEqual(result, { Ok: 'Hello Holo' })
})
```
## Run the test 
Now in the `hello_helo` directory of run the test like this:
```bash
$ hc test
```
This will compile and run the test scenario you just wrote.
You will see a lot of output but if everything went ok then right at the end you will see:
```
# tests 2
# pass  2

# ok
```
Congratulations you have tested your first Holochain app. Look at you go :sparkles: 
