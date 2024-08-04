# Inter-Node communication with Chord

So far we have seen [how to identify Nodes in a P2P network](./p2p_identity.md), yet it is still unclear how Nodes actually _communicate_ between each other. There are many different ways that achieve this. Over the next two sections, we will be focusing on how the **Chord** and **Kademlia** algorithms can be used for this very purpose.

> Since **Kademlia** is a modified version of **Chord**, we will only be exploring Chord in this context, in particular regarding how Nodes determine _which other Nodes to connect to_. We will therefore _not_ be diving into a full explanation of the [original Chord paper](./res/pdf/chord_sigcomm.pdf).

## Discoverability

Given a network of arbitrary, growing size, we are trying to find a way to communicate efficiently from one end of the network to the other. Suppose we have a starting Node \\(S\\) and a destination Node \\(D\\): that means that starting from \\(S\\), we are trying to find the quickest viable path to \\(D\\). We can formulate the following constraints:

1. The path from \\(S\\) to \\(D\\) should be traversable in the smallest amount of steps, or _network jumps_. A _network jump_ is when two Nodes connect to each other.

2. Finding this path should be very fast. There is no point in finding an optimal path if the time it takes to find it is much slower than the path itself.

3. Steps **[1.]** and **[2.]** should maintain the properties of decentralization of our P2P network. This means that we cannot rely on a centralized server which is responsible for keeping an overview of the state of the network.

Why is this hard? Can't we just directly connect our starting Node \\(S\\) to it's destination Node \\(D\\)? While that might be possible, it is not always the case. This is because there is no central entity keeping track of all the Nodes in the Network. 

Importantly, it is not feasible for every Node to keep track of _every other Node_ in the network, as this would mean that whenever a new Node enters the network it would have to update _all other Nodes_. The number of updates required for this to function grows linearly, making this approach not scalable, meaning it is not feasible at a large scale.

> In a P2P network, each Node can only keep a partial map of the network at large, as keeping an up-to-date map of the entire network would grow too costly over time. **Discoverability** is the process by which Nodes use this partial map to connect to neighboring Nodes, which in turn try to get closer to the required destination.

## Routing Tables

Due to constraints in keeping an up-to-date map of the network, Nodes can only keep track of a small set of neighbors. **Chord** (as far as we will see here) is an algorithm for determining _which Nodes in the network_ make up these neighbors (hereafter referred to as **peers**). These peers are organized together into a **routing table**, which can be seen as a sort of imperfect map of the network that can then be used to try and get closer to the target destination.

\\[\text{Given the following}\\]

\\(n := \text{node GUID}\\)

\\(N := \text{maximum number of nodes in the network}\\)

\\(m := \log_2(N), \text{ s.t } 2^{m} \ge N\\)

\\(m' := \text{number of peers in a routing table}, m' \lll m\\)

\\(k := \text{index of each peer in the routing table}, 0 \lt k \lt m'\\)

\\[\text{We define the } k^{th} \text{ peer in the routing table as:}\\]

> \\[peer[k] = n + 2^{k-1} \mod 2^m\\]

Let's break this down step by step:

1. Every Node in the network is keeping track of at most \\(m'\\) other peers. This value is chosen experimentally: the higher the value of \\(m'\\), the faster it will be for Nodes to communicate between each other, but the slower it will be for new Nodes to enter the network. A high value of \\(m'\\) also risks _congesting_ the network by flooding it with too many messages.

2. Nodes keep as peers the Nodes which are closest to them in Keyspace. In fact, the distance between peers _doubles_ each time. This means Nodes know more about the state of the network immediately around them, but are also able to connect to more distant Nodes if the destination is far away from them.

Essentially, this formula is creating a vague map of the network which mostly focuses on the local landscape, but with the mountains in the background in case it is ever needed to reach them. If you need more information, you can ask other Nodes along the way, as we will be seeing.

<div style="text-align: center;">

![](./res/vector/p2p/chord1.png)
_Fig. 1: Identifier circle showing the \\(m\\) peers of Node 0_

</div>

As we can see in _Fig. 1_, Node 0 is aware of Nodes 1, 2 and 4 as peers. It is not aware of _any other Node_ in the network and cannot reach them directly.

<div style="text-align: center;">

![](./res/vector/p2p/chord2.png)

_Fig. 2: Identifier circle showing the \\(m\\) peers of Nodes 0, 1 and 2_

</div>

_Fig. 2_ expands this view by showing the peers of Nodes 0, 1 and 2. Notice how all together these Nodes can now connect to Nodes 0, 1, 2, 3, 4, 5 and 6 (we assume connections go both ways).

For a Node \\(S\\) to reach a target destination \\(D\\), we apply the following algorithm:

1. If \\(D \in S\\)'s routing table, then \\(S\\) can reach the target destination immediately.

2. If \\(D \notin S\\)'s routing table, then we perform a network jump to the next Node in \\(S\\)'s routing table which is closest to \\(D\\).

3. We repeat step **[2.]** until \\(D\\) is part of the routing table of the current node, at which point we can jump directly to it.

> Using this algorithm, it can be proved that, starting at any node, it will take at most \\(log(n)\\) jumps to reach any other node in the network, where \\(n\\) is the total number of nodes in the network.

<div style="text-align: center;">

![](./res/vector/p2p/chord4.png)
_Fig. 3: Chord allows for efficient \\(O(\log(n))\\) traversal of the network_

</div>

_Fig. 3_ illustrates a practical example of this algorithm: 

1. Starting at Node 0, we want to reach Node 3. 

2. Node 3 is not part of Node 0's peers, so we jump to the next closest Node in its routing table, Node 2.

3. Node 3 is part of Node 2's peers, so we can jump to it directly.

> In this way, we were able to reach Node 3 starting at Node 0 in only 2 network jumps, even if Node 3 is not part of Node 0's peers.

## Joining a P2P Network

So far we have seen how a Node can reach any target destination in the Network with only a limited routing table of peers. This however assumes a static network. How do we handle the case where new Nodes need to join? For this we need to introduce a new concept: **Boot Nodes**.

> A **Boot Node** is a Node which is listed publicly (for example on the internet), to which new Nodes can connect for the first time.
>
> _This does not break the condition that no Node can have greater power over the network, as any Node can choose to become a Boot Node._

When a new Node \\(N\\) connects to a Boot Node \\(B\\) for the first time, it coppies \\(B\\)'s routing table. This is not ideal however, as \\(B\\) does not have the same GUID as \\(N\\), and so it's routing table will not match. \\(N\\) will then use this routing table to connect to _other Nodes_ throughout the network and establish contact with its peers so as to create _it's own routing table._

It is as if you started a journey with no map, so you go to the nearest stranger and ask him for a copy of theirs. It still isn't a perfect fit but it is a good starting point: you use it to go around and talk to _other strangers_ and copy parts of _their maps_ until you have the map that is best suited to your journey.

<div style="text-align: center;">

![](./res/vector/p2p/chord5.png)
_Fig. 4: A new Node \\(N\\) joining the network by connecting to a Boot Node_

</div>

The actual process is a bit more involved, as it might result in updating the routing tables of other Nodes, but this will be discussed in greater details in the next section on [Kademlia](./Kademlia.md).
