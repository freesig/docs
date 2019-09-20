# Simple micro blog tutorial
Welcome to the simple micro blog tutorial in the core concepts tutorial series.
The aim of this tutorial is to show how entry's can be linked to each other in a Holochain app. 
A link is simply a relationship between two entries. It's a useful way to find some data from something you already know. For example, you could link from your agent to their blog posts.

You will be building on the previous [hello world]() tutorial and making a super simple blog app.
The apps users will be able to post a blog post and then retrieve other users posts.

## DNA hash
The way you run your conductor has changed from `hc run` to calling `holochain` directly, as a consequence the hash of your apps DNA now lives in the `conductor-config.toml` file. However anytime you change your code and run `hc package` the hash will be different. So you will need to update the `conductor-config.toml` file.
Enter the nix-shell:
```bash
nix-shell https://holochain.love
```
Package your app:
```
hc package
```
Copy the DNA hash:
```
DNA hash: QmfKyAk2jXgESca2zju6QbkLqUM1xEjqDsmHRgRxoFp39q
```
Update the `conductor-config.toml` dna hash:
```toml
[[dnas]]
  id = "hello"
  file = "dist/hello_holo.dna.json"
  hash = "QmfKyAk2jXgESca2zju6QbkLqUM1xEjqDsmHRgRxoFp39q"
```
## Post
We will store our posts as a `Post` struct that holds a message `String`, a timestamp `u64`, and an author_id `Address`.
Remove the Person struct and add the Post struct:
[![asciicast](https://asciinema.org/a/tpHdFyTkVnfK5fkVWkgYDjH4c.svg)](https://asciinema.org/a/tpHdFyTkVnfK5fkVWkgYDjH4c)
## Entry
Update the person entry to post entry:
[![asciicast](https://asciinema.org/a/aYwqCZ2w2b4D3vAZw4F4unOfz.svg)](https://asciinema.org/a/aYwqCZ2w2b4D3vAZw4F4unOfz)
## Agent ID
Now you have a post entry but you also need some way to find the posts an agent makes.
To do this you can create an agent entry which you will use to link to the posts that the user makes.

Create an agent entry by adding the following lines below the `post_entry_def`.

Add an `agent_entry_def` which returns the `ValidatingEntryType`:
```rust
#[entry_def]
fn agent_entry_def() -> ValidatingEntryType {
```
Start the `entry!` macrofor the agent entry:
```rust
    entry!(
        name: "agent",
        description: "Hash of agent",
```
Set sharing to public so other agents can find this agents posts:
```rust
        sharing: Sharing::Public,
```
Add basic validation to make sure this is the `Agent` type that is passed in:
```rust
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Agent>| {
            Ok(())
        },
```
Now you need want to link this agent entry to the post entry.
Add the `to!` link macro:
```rust
        links: [
        to!(
```
Define a link from this entry to the `post` entry called `author_post`:
```rust
           "post",
           link_type: "author_post",
```
Add empty validation for this link:
```rust
           validation_package: || {
               hdk::ValidationPackageDefinition::Entry
           },
           validation: |_validation_data: hdk::LinkValidationData| {
               Ok(())
           }
        )
        ]
    )
}
```

## Create a post
Remove the `create_person` function.

You need a function for creating a new post.
Think about the ingredients that go into the `Post` structure, message, timestamp and the agent id.
The message will come from the UI. For simplicity the timestamp will come from the UI as well. Time is a pretty tricky concept in the distributed world and requires careful planning.
The agent's id will come from the special constant `AGENT_ADDRESS`.

Add a public `create_post` function that takes a message `String` and timestamp `u64`:
```rust
#[zome_fn("hc_public")]
pub fn create_post(message: String, timestamp: u64) -> ZomeApiResult<Address> {
```
Create the `Post` using the message, timestamp and this agents address:
```rust
    let post = Post {
        message,
        timestamp,
        author_id: hdk::AGENT_ADDRESS.clone(),
    };
```
Create the `Agent` struct from the `AGENT_ADDRESS`, turn it into an `Entry` and commit it:
```rust
    let agent_id = Agent { id: hdk::AGENT_ADDRESS.clone().into() };
    let entry = Entry::App("agent".into(), agent_id.into());
    let agent_address = hdk::commit_entry(&entry)?;
```
Commit the post entry:
```rust
    let entry = Entry::App("post".into(), post.into());
    let address = hdk::commit_entry(&entry)?;
```
Create a `author_post` link from this agent to the post:
```rust
    hdk::link_entries(&agent_address, &address, "author_post", "")?;
```
Return everything is Ok with the post's address:
```rust
    Ok(address)
}
```
## Retrieve all of a users posts 
Add the `retrieve_posts` public function that takes an address and returns a vector of posts:
```rust
#[zome_fn("hc_public")]
fn retrieve_posts(address: Address) -> ZomeApiResult<Vec<Post>> {
```
Create the `Agent` struct from the `AGENT_ADDRESS`, turn it into an `Entry` and commit it:
```rust
    let agent_id = Agent { id: hdk::AGENT_ADDRESS.clone().into() };
    let entry = Entry::App("agent".into(), agent_id.into());
    let agent_address = hdk::commit_entry(&entry)?;
```
Get all the links from the agents address and load them as the `Post` type:
```rust
    hdk::utils::get_links_and_load_type(
        &address,
```
Match on `author_post` links with `Any` tag:
```rust
        LinkMatch::Exactly("author_post"),
        LinkMatch::Any,
    )
}
```
You will need to add the `link::LinkMatch` to your use statements:
```rust
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};
```
## Get the agents id
You will need the agents id in the UI later so that you can pass try getting the posts for another agent.
Add a public `get_agent_id` function that returns an `Address`:
```rust
#[zome_fn("hc_public")]
fn get_agent_id() -> ZomeApiResult<Address> {
```
For this app you can use the agents address as their id:
```rust
    Ok(hdk::AGENT_ADDRESS.clone())
}
```
## Show the agents id in the UI
Let's start on the UI.
Go to your gui code and open up the `index.html` file.
To make it easy to pass around agent ids, you can display the id for the instance that each gui is currently targeting.
This should happen when the page loads and when the instance id changes.

Add an `onload` event to the body that will call the `get_agent_id` function when the page loads:
```html
<body onload="get_agent_id()">
```
Add an `onfocusout` event to the instance text box that will call the same function when unfocused:
```html
<input type="text" id="instance" onfocusout="get_agent_id()" placeholder="Enter your instance id">
```

Now open up the `hello.js` file and add the `get_agent_id` function:
```javascript
function get_agent_id() {
```
Get the instance value and setup a zome call connection:
```javascript
  var instance = document.getElementById('instance').value;
  holochainclient.connect({ url: "ws://localhost:3401"}).then(({callZome, close}) => {
```
Call the `get_agent_id` zome function and then update the `agent_id` element with the result:
```javascript
    callZome(instance, 'hello', 'get_agent_id')({}).then((result) => update_element(result, 'agent_id'))
  })
}
```
## Update the UI to create a post
Back in `index.html` update the "create person" html to use a `textarea` and call the `create_post` function:
[![asciicast](https://asciinema.org/a/mAPERkw51QbQQp2KZkTxZnwDB.svg)](https://asciinema.org/a/mAPERkw51QbQQp2KZkTxZnwDB)
## Update th UI to retrieve posts from an agents id
Update the retrieve person html to retrieve posts:
[![asciicast](https://asciinema.org/a/0eQ1giTdu4BEOnQghXax1ALBE.svg)](https://asciinema.org/a/0eQ1giTdu4BEOnQghXax1ALBE)
## Call create_post from javascript
In the `hello.js` file add the `create_post` function that your HTML calls:
```javascript
function create_post() {
```
Get the post message and instance id:
```javascript
  var message = document.getElementById('post').value;
  var instance = document.getElementById('instance').value;
```
Create a timestamp for now:
```javascript
  var timestamp = Date.now();
```
Make a zome call to `create_post` with the message and timestamp:
```javascript
  holochainclient.connect({ url: "ws://localhost:3401"}).then(({callZome, close}) => {
    callZome(instance, 'hello', 'create_post')({message: message, timestamp: timestamp }).then((result) => update_element(result, 'post_address'))
  })
}
```
## Update the posts list dynamically
Add an empty list below the `post_agent_id` text box:
```html
<ul id="posts_output"></ul>
```
In the `hello.js` file add the following lines to update the `posts_output` dynamically.

Add the `display_posts` function:
```javascript
function display_posts(result) {
```
Get the `posts_output` html element:
```javascript
  var list = document.getElementById('posts_output');
```
Wipe the contents of the list:
```javascript
  list.innerHTML = "";
```
Parse the zome result as JSON:
```javascript
  var output = JSON.parse(result);
```
Sort the posts by their timestamps:
```javascript
  var posts = output.Ok.sort((a, b) => a.timestamp - b.timestamp);
```
For each post add a `<li>` element that contains the post's message:
```javascript
  for (post of posts) {
    var node = document.createElement("LI");
    var textnode = document.createTextNode(post.message);
    node.appendChild(textnode);
    list.appendChild(node);
  }
}
```
## Get this agents id
Add the `get_agent_id` function:
```javascript
function get_agent_id() {
  var instance = document.getElementById('instance').value;
```
Call the `get_agent_id` function and update the `agent_id` element:
```javascript
  holochainclient.connect({ url: "ws://localhost:3401"}).then(({callZome, close}) => {
    callZome(instance, 'hello', 'get_agent_id')({}).then((result) => update_element(result, 'agent_id'))
  })
}
```
## Retrieve an agents posts
This is very similar to `retrieving_person` so just update that function:
[![asciicast](https://asciinema.org/a/oiFGzlKexjVVMrNxf7Gc00Oiw.svg)](https://asciinema.org/a/oiFGzlKexjVVMrNxf7Gc00Oiw)
