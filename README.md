Language for running distributed algorithms.

# Goal

- Abstraction of messaging between nodes.
- Transpile to Rust, because I am too lazy to go till LLVM, and programmers can debug Rust code.
- Type Safety.
- NO POINTERS.
- Easy generics? Do messages need to be generic?
- Will show to Compilers profs, see if worth LLVM-ing. Probably not, since pet project + goes to LLVM eventually anyway.
- Construction of a dynamic network
    - Each network has its own Signature, and the signature must be provided for a node to join the network.
    - Multiple sources fro signature?
- Multithreading inside nodes? Major problem. Reserve `spawn` and `thread` anyway.
    - Probably generalise to automated node setup. Von-Neumann, morituri te salutant.

# Types
    - ONE integer type, str, Array, Vector, Tuple. Map definitely for later.
    - Network: Big Parent type. 
    - Node: Multiple types of Node in a network.
    - Message: Finite set of messages exchanged between nodes in a network.
    - Signal: Something?
        - Timer.
        - File changes? How do I even make a file watcher?
        - Actual system signals? Just make it compile to a handler and system does the rest. Be nice to terminate network across multiple systems with a single Ctrl-C.
    - Builtin Termination Detection, and maybe a Clock for ordering events. Might have to use Matrix clocks.
        - Maybe other algos too? Consensus might be a bit much, but could bake snapshots in.
        - Who even knows?

# Proto-syntax
    - Module tree. A Network is a directory. A Node is a file. Messages can be defined anywhere.
    ```
    message <message-name> {
        field: Type,
        ...
    }
    ```
    
        - Maybe define a new surround mode (like `|<contents>|`) for message construction. Cannot be angular because generics, cannot be any kind of quote because ugly, and cannot be any king of bracket because duh.
    - Each file is named after a node. Each node has send-receive conditions, like 
    ```
    // Recv block
    when <event> do {
        Anything.
        Maybe supply this directly to Rust? Probably want to restrict what someone can do here.
    }
    ```
    - Can specify many of these.
    - Can also put `send <message>` or `send <message> to` inside `recv` blocks.
    - Event can be message, signal, or timer.


# Network

A network consists of one or more nodes of one or more types distributed across a physical network. Each network will have one or more centralised Identity Managers, which function similar to a DNS, storing node ids, ip addresses, and port numbers.

Identities will be consistent across Managers.

# Nodes

Each file is a node. Each instance of a node will be given a unique ID.

## Node knowledge

Distributed algorithms require nodes to have adequate knowledge about network structure and their neighbours. This is handled by the language itself - assume each node knows enough about the network to perform its functions.

<!-- How? Well, before the init block runs, in the underlying main function, there's a broadcasted Identify message, which supplies - and receives in turn - IP addresses and port numbers of all nodes in the network *to which it can broadcast*. As for faraway nodes, it  -->

How? Each node announces itself to an Identity Manager. This begins a two-step synchronisation process where the existence of the node is recorded among *all* Managers, and then *some subset* of nodes in the network are informed of the new node's existence, including all nodes in the same subnet. For localised networks, this results in a complete graph.

For Distributed Algorithms that require a complete graph, one such will be simulated by the language.

# Functions

Oh god. Do we even want these?
Yeah probably.

```
fn <name> (args) -> type {

}
```

Blatant Rust ripoff, because this isn't the cool part.

## Methods

Don't want to copy Rust here, because the impl system is a bit much. Instead, using archetype syntax.
```
message q {};

fn q.foo() {
    // code
}
```

Stuff like that.

# Structs

There's a lot of overlap with the nature of `message`, so new things.

## Datum

Proper structs.
```
datum <name> {
    field: Type,
    ...
}
```

Cannot be `sent` or `yote`. Can be wrapped in something else in order to send, but cannot be mutated after that,
Mutable by default. Always borrowed mutably. Cannot be made immutable. Usual rust mutability rules apply.

## Packet

```
packet <name> {
    field: Type,
    ...
}
```

Can be sent, and treated like structs. Very general.

Borrowed both mutably and immutably as the situation demands.
Might get removed.

## Message

```
message <name> {
    field: Type,
    ...
}
```

Can be sent, but cannot be accessed after init. 
Immutable.

# Enums

Another ripoff, this time of C/CPP, as our enum isn't a sum type.
We already have a proper sum type enum, for *messages*. You want nested sum types, move all alternsatives to the top layer.

```
enum <name> {
    variant,
    ...
}
```

# Events

```
// Creates a NEW timer, and starts it on init.
timer <name> <float>;
// Creates a NEW timer, starts it on init, and restarts it after handling.
clock <name> <float>;
// Generates handler for signal with number <int>.
// 1-31 are reserved for system signals, and are provided in the standard library.
signal <name> <int>;
```

They can be handled using the `when` block, just like messages.
```
when <event> do {
    // Code
}
```

# Sample

```
// Can be Self, in which case node is named after the file,
// or a custom name.
Self {
    id: int,
    state: State,
    leader: Self,
}
/*
Equivalently,

node Node1 {
    id: int,
    state: State,
    leader: Node1,
}

*/

message Ping {
    id: int,
    data: str,
}

message Pong {
    id: int,
    data: str,
}

datum State {
    ping_count: int,
    pong_count: int
}

when Ping do {
    state.pong_count += 1;
    reply Pong {
        id: self.id,
        data: "Pong"
    };
}

when Pong do {
    self.ping_count += 1;
    let dummy = Ping {
        id: self.id,
        data: "Ping"
    };
    send dummy to self.leader;
}

fn dummy_function() -> int {
    42
}

init {
    id = argv[1] as int;
    state = State {
        ping_count: 0,
        pong_count: 0
    };
    send Ping {
        id: self.id,
        data: "Ping"
    }
}
```

# Steps

- Network setup. 
  - This creates an Identity Manager, which is not necessary for localised network but aids in scaling up. 
  - New Identity Managers can be created only if you know the ip and port of the old one - otherwise, you end up with an independent Manager running it's own network.
  - The Manager will be spawned on a default port. If there is another manager in the same subnet, it will remain dormant until the extant Manager crashes.
  - Each manager knows the IP and port of all other managers, as well as the IP and port of all nodes in the subnet.
  - Managers are *completely* transparent. They do not interact with code written, only code generated.
- Node setup. 
  - Create a new node of any type.
  - It broadcasts an Identify message, to which the manager replies with the IP and port of all nodes in the subnet.
  - Init block executes.