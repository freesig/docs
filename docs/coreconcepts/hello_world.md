# Hello World
The goal of this tutorial is to add an entry in Alice's instance and then retrieve that same entry in Bob's instance.

## Make your entry public 
So far the only entry you have had has been private.
But this isn't that useful if you want your users to be able to share entries on the same network.

Open up your `zomes/hello/code/src/lib.rs` file.

Change the entry sharing to `Sharing::Public`:
[![asciicast](https://asciinema.org/a/K0Vj50CIVNSYWr5RIbbrc6V3s.svg)](https://asciinema.org/a/K0Vj50CIVNSYWr5RIbbrc6V3s)

## Add bob to the test
Before you made a test where Alice made a few zome calls and verified the results. 
Now to test that the entries can be shared between agents on the same DNA you can use Bob in your tests to interact with Alice.

Open up your `test/index.js` file and add / update the following lines:

Add `bob` to the Scenario:
```javascript
diorama.registerScenario("Test hello holo", async (s, t, { alice }) => {
diorama.registerScenario("Test hello holo", async (s, t, { alice, bob }) => {
```
Make the `retrieve_person` call with the result from `create_person`:
```javascript
const bob_retrieve_result = await bob.call("hello", "retrieve_person", {"address": create_result.Ok});
```
Check that the result was Ok:
```javascript
t.ok(bob_retrieve_result.Ok);
```
Check that the result does indeed match the person entry that Alice created:
```javascript
t.deepEqual(bob_retrieve_result, { Ok: { App: [ 'person', '{"name":"Alice"}' ] }})
```

Your test should look like this:
```javascript
diorama.registerScenario("Test hello holo", async (s, t, { alice, bob }) => {
  const result = await alice.call("hello", "hello_holo", {});
  t.ok(result.Ok);

  t.deepEqual(result, { Ok: 'Hello Holo' })
  
  const create_result = await alice.call("hello", "create_person", {"person": { "name" : "Alice" }});
  t.ok(create_result.Ok);
  
  const retrieve_result = await alice.call("hello", "retrieve_person", {"address": create_result.Ok});
  t.ok(retrieve_result.Ok);
  
  t.deepEqual(retrieve_result, { Ok: { App: [ 'person', '{"name":"Alice"}' ] }})
  
  const bob_retrieve_result = await bob.call("hello", "retrieve_person", {"address": create_result.Ok});
  t.ok(bob_retrieve_result.Ok);
  
  t.deepEqual(bob_retrieve_result, { Ok: { App: [ 'person', '{"name":"Alice"}' ] }})
})
```
### Run the test
Enter the nix-shell if you don't have it open already:
```bash
nix-shell https://holochain.love
```
Now run the test and make sure it passes:
```bash
nix-shell] hc test
```
```
1..7
# tests 7
# pass  7

# ok
```
## Conductor
Now it would be cool to see this happen for real outside of a test.
Up till now you have only used `hc run` to run a single instance of a node.
However, in order to have two separate instances communicate we need to run `holochain` directly and pass it a config file.

Before you can create the config file, you will need to generate some keys for your agents.

Use keygen in your nix-shell to generate a key for each agent:
```
nix-shell] hc keygen -n
```
This will output something similar to the following:
```
Generating keystore (this will take a few moments)...

Succesfully created new agent keystore.

Public address: HcSCJhRioEqzvx9sooOfw6ANditrqdcxwfV7p7KP6extmnmzJIs83uKmfO9b8kz
Keystore written to: /Users/user/Library/Preferences/org.holochain.holochain/keys/HcSCJhRioEqzvx9sooOfw6ANditrqdcxwfV7p7KP6extmnmzJIs83uKmfO9b8kz

You can set this file in a conductor config as keystore_file for an agent.
```
Take note of the `Public address`, you will need it later. 

```bash
cp /Users/user/Library/Preferences/org.holochain.holochain/keys/HcSCJhRioEqzvx9sooOfw6ANditrqdcxwfV7p7KP6extmnmzJIs83uKmfO9b8kz agent1.key
```

Now do this again but cp it to agent2.key:
```bash
cp /Users/..... agent2.key
```
### Config file
Create a new file in the root directory of your project called `conductor-config.toml`.

Add an agent with id `test_agent1` and name `Agent 1`:
```toml
# -----------  Agents  -----------
[[agents]]
  id = "test_agent1"
  name = "Agent 1"
```
Use the public address from `hc keygen` that you made for agent 1 before here:
```toml
  public_address = "HcScJtQkeJ4hkdqtxrdgh8jPybRynkaxmCSFZxKG6Nsf3tei6zRAnEdy97Qzeua"
  keystore_file = "./agent1.key"
```
Add an agent with id `test_agent2` and name `Agent 2`:
```toml
[[agents]]
  id = "test_agent2"
  name = "Agent 2"
```
Use the public address from `hc keygen` that you made for agent 2 before here:
```toml
  public_address = "HcSCinAe4uWs66r7pxnBn9xNcCJ5kivg5MqhvZ7rbmqbfzeg8e4Aun8ziwiersz" 
  keystore_file = "./agent2.key"
```

Run package and get your dna's hash:

```
nix-shell] hc package
```
You will see something similar to this:
```
DNA hash: QmS7wUJj6XZR1SBVk1idGh6bK8gN6RNSFXP2GoC8yCJUzn
```

Add the dna with id `hello` and use the DNA hash you just made above:
```toml
# -----------  DNAs  -----------
[[dnas]]
  id = "hello"
  file = "dist/hello_holo.dna.json"
  hash = "QmZcje6BoEURaX1vCrsDY11v7fHMd6spmVpzkQ1Vxp9n6D"
```
Add the Alice instance with the `hello` dna:
```toml
[[instances]]
  id = "Alice"
  dna = "hello"
  agent = "test_agent1"
[instances.storage]
  type = "memory"
```
Add the Bob instance with the same `hello` dna:
```toml
[[instances]]
  id = "Bob"
  dna = "hello"
  agent = "test_agent2"
[instances.storage]
  type = "memory"
```
Setup the websocket connection on socket `3041`:
```toml
[[interfaces]]
  id = "websocket_interface"
[interfaces.driver]
  type = "websocket"
  port = 3401
```
Add your instances to this interface:
```toml
[[interfaces.instances]]
  id = "Alice"
[[interfaces.instances]]
  id = "Bob"
```

## Allow the users to choose their agent
Before you can use two agents you need a way for the UI to specify which agent the user wants to use.
You can do this by setting the instance id in the zome call.
You can think of an instance as a running version of a DNA. Like a variable is an instance of a struct.

Open the `gui/index.html`.

Add a text box for your users to set the agent id:
```html
<input type="text" id="instance" placeholder="Enter your instance id"><br>
```

Open the `gui/index.js` and do the following for every callZome call:
[![asciicast](https://asciinema.org/a/Tp2xSDERlohFXy90LP7Yu4HQR.svg)](https://asciinema.org/a/Tp2xSDERlohFXy90LP7Yu4HQR)
## Run the app and two UIs
Now the fun part where you get to play with what you just wrote.

Open up a three terminal windows and enter the nix-shell in each one:
```bash
nix-shell https://holochain.love
```
#### Terminal one
Go to the root folder of your app:
```
nix-shell] cd /path/to/my/app
```
Start by running the conductor.
It's a bit different this time, instead of `hc run` you will use `holochain` directly:
```
nix-shell] holochain -c conductor-config.toml
```
#### Terminal two
Go to the root folder of your gui:
```
nix-shell] cd /path/to/my/gui
```
Run a gui on port `8000`:
```
nix-shell] python -m SimpleHTTPServer 8000
```
#### Terminal there
Go to the root folder of your gui:
```
nix-shell] cd /path/to/my/gui
```
Run a gui on port `8001`:
```
nix-shell] python -m SimpleHTTPServer 8001
```
### Open up the browser
Open two tabs.

#### Tab one
Go to `0.0.0.0:8000`.
Enter `Alice` into the `Enter your instance id` text box.
#### Tab two
Go to `0.0.0.0:8001`.
Enter `Bob` into the `Enter your instance id` text box.

#### Tab one - Alice
Create a person with your name:
![](https://i.imgur.com/6PEDn6y.png)

#### Tab two - Bob
Copy the address from the Alice tab and retrieve the person entry:
![](https://i.imgur.com/ps9RBr2.png)
