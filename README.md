# FSCP - Felix Systems Communication Protocol

## Specifications


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
