TODO

* Start with RTCP/RTP demuxing
* Go into the RTCP pipeline from there, since the parsing for that is done


building the pipeline
============================
Originally the idea was for all Node types to be Clone, because:
  a) this makes accessing the shared data easy: the pipeline has a ref to the node and the 'control plane' has a ref, so when the control plane needs to make a change it has a handle
  b) this makes building the pipeline more ergonomic, because it can be built "in order".  If nodes aren't clone, then you have to add node C to node B before you add node B to node A (since you lose your handle on it when doing so)

But, it also has some drawbacks:
  a) nodes can't avoid holding data that isn't shared in an Arc<Mutex> because then they wouldn't be Clone
  b) Holding the 'next' reference outside of an Arc<Mutex> makes a big difference, so even if 2 places had handles to the node, changing the 'next' node value wouldn't take effect if it wasn't on the node instance that was actually _in_ the pipeline.

'b' could be addressed by differentiating "Nodes" and "NodeDataHandles" (which would only contain the shared data), but I also think it'd be fine for the control plane to just hold the reference to the shared data, not the node itself.

I'd like to enable the optimization described in 'a', so we'll make Nodes not Clone, and we can somewhat address the pipeline ergonomics issue by adding an "attach_end" method to Node which would add the new node at the _end_ of that node chain instead of directly to the node it's called on.  This doesn't work with (sub)pipelines with something like a Demuxer in them (because it wouldn't know which path to append it to) but we can still construct the linear subpipelines this way and then attach them together.

Another option here would be to use some builder type to make building the pipeline easier which then transitions to the 'real' type for faster execution at runtime.  I like this idea,
because currently building pipelines is very confusing, but I'm not sure how to make it work yet.  One thing I tried was changing the 'NextNode' type (which is used to handle the reference to the next node in a pipeline) to an enum which was either 'shared' or 'owned': while building the pipeline it could be 'shared' and then transitioned to 'owned', but the overhead of matching against the enum was significant when I tested it.

Now that we have the core logic separated out into inner handlers, maybe we can have a 'BuilderNode' which is simple and then transition to the real nodes--but I'll need to play with how to do that programmatically. 


stats tracking nodes
================================
My original idea was to have nodes that would track stats and they could just wrap other nodes, making the stat tracking transparent and composable.  Something like:

struct StatsTracker {
   packets_received: u32,
   packets_forwarded: u32,
   inner: Box<dyn Node>
}

And then we implement Node for StatsTracker.  But this doesn't work: we want to increase packets_received when StatsTracker::process_packet is called from the previous node, but then we don't have any "place" to set as inner's 'next' such that we can track how many packets were forwarded.  So then I realized we really need two "inner" nodes inside StatsTracker: one which would function as the ingress side and receive packets from the prior node and forward them to inner, and another egress node which we could attach to inner and then forward on.  But then we have a problem that we need to hold the handle to the ingress/egress Node pointers in two places:
  1. The prior node needs the Box<dyn Node> for the ingress node
  2. The inner node needs the Box<dyn Node> for the egress node
So then StatsTracker can't hold on to those handles to grab the stats.  We could, instead, used SharedData for the stats and have StatsTracker hold on to those, but instead I think we'll just have a struct for tracking stats that a node can hold on to and interact with more directly.  It's less dynamic, but probably better for performance and not too much of a pain to use.  E.g.:

struct PacketStatsTracker {
  packets_received: u32,
  packets_forwarded: u32,
}

impl PacketStatsTracker {
    fn packet_received(&mut self) {
    	self.packets_received += 1;
    }

    fn packet_forwarded(&mut self) {
    	self.packets forwarded += 1;
    }
}
The downside here is that nodes need to remember to call self.stats_tracker.packet_received() and self.stats_tracker.packet_forwarded().  Maybe instead we could do this by having the nodes emit 'events' which things could attach to?  This would pair well with a "two layer" scheme for a node, where the "top" layer could be consistent for all--the problem is that it doesn't work with things like Demuxer.

--> The two layer scheme ended up working well: We have (so far) a single 'Node' implementation that contains some 'PacketHandler' within it.  The node impl can handle the common stuff like tracking stats, while the packet handlers just focus on that logic and don't have to worry about the node-level stuff.


node control plane/shared data
===============================
Many nodes (the 'packet handlers' now, really) need access to outside information that can change.  For example, rtp parser needs access to the signaled payload types.  For most of these I'm hoping that a SharedData with a RwLock inside will work fine.  I do think _some_ examples could maybe use a tokio::sync::watch.  For example, the tcc generator that just needs to know the signaled tcc extension id.  It may be that the overhead is too high to do this for every packet (especially when it doesn't change often), but worth a try.

I ended up running some experiments with this (most can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=910756d3a165c799864be36f868d184c).  This test compares:

* Raw (exclusive ownership)
* tokio's watch
* std::sync::Mutex
* std::sync::RwLock

Note that there is no contention, but for the most part these things will be written to seldomly.  I also tried 'ArcSwap' (not on playground, since the crate isn't available) but for me that took longer than all the others--there's a chance I wasn't using it correctly/optimally.  Outputs from a run of that test:

```
raw took 52ms
watch took 156ms
rwlock took 344ms
mutex took 362ms
arcswap took 934ms
```

So, for now, it looks like RwLock will work.
