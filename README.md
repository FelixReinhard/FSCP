# FSCP - Felix Systems Communication Protocol

## Specifications

### Server - Client exchange 
A server starts and listens. Then a client sends a `ClientHello` which includes a version number and optionally a public key.
The Server then accepts this client. 

- If a public key (or certificate) is present, then an encrypted nonce is sent to the client who needs to respond with the decrypted nonce immidiatly after this.
On success the corresponding permissions are granted.
- Otherwise this step is skipped.

A `ServerAccept` is sent, which contains a session number and expiration time. After expiration a `ServerRefreshPermissions` is sent.

This is then followed by a stream of messages from the server to build the Tree. In between the client can always
send a `ClientHash` or `ClientTrigger` message. If a `ClientAddPermissions` is sent the procedure from above is done again. The old certificate does not expire.


## (DEV) Brainstorm
- TCP basis 
- Server client.
- Server defines structure.
- Node based. Everything is a node. Events on all nodes.
  - Data node: Holds any data. Can be changed. has properties like private public. who can change it. Description.
  Events: DataChanged, PropertyChanged,
  Methods: AddChild, RemoveChild, GetChild, GetChildren,
  DataTypes: float8-64 ,  (un)? signed int8-64 , String, Boolean, Tuple (fixed length), Dynamic list of any other DataTypes.

  Uses: LED Strip: 
  - Node called "LEDS" with unnamed nodes as children, each representing one LED. Method Node "ChangeLed(int num, Color color)" where Color = (int, int, int)

  - Method node: can be executed and is defined on server as native function or callback.
  emits Event when started and when finished.

- Subscribe to groups. Node is part of any number of groups and can be Subscribed when changed.



