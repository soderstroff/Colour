# Colour

> Not a man breathed for several seconds. Then a cloud of darker depth
passed over the moon, and the silhouette of clutching branches faded
out momentarily. At this there was a general cry; muffled with awe,
but husky and almost identical from every throat. For the terror had
not faded with the silhouette, and in a fearsome instant of deeper
darkness the watchers saw wriggling at that tree top height a thousand
tiny points of faint and unhallowed radiance, tipping each bough like
the fire of St. Elmo or the flames that come down on the apostles'
heads at Pentecost. It was a monstrous constellation of unnatural
light, like a glutted swarm of corpse-fed fireflies dancing hellish
sarabands over an accursed marsh, and its colour was that same
nameless intrusion which Ammi had come to recognize and dread. All the
while the shaft of phosphorescence from the well was getting brighter
and brighter, bringing to the minds of the huddled men, a sense of
doom and abnormality which far outraced any image their conscious
minds could form. It was no longer shining out; it was pouring out;
and as the shapeless stream of unplaceable colour left the well it
seemed to flow directly into the sky.

— H. P. Lovecraft, *The Colour Out of Space*

Colour is a low performance, barely concurrent, feature deprived message
passing library.

This is a list of everything you would ever want from an actors library in Erlang:
- Spawn a procedure that runs a function with some initial arguments.  
`PID = spawn(fn, args)`  
- The ability to receive messages and pattern match against them.  
`receive {
  (command, args) -> do_stuff_with(args),
  ( _ , _ ) -> fail_miserably(),
}`  
- Send a message to a process.  
`PID!(message, et_al)`  
- Refer to your PID in a message.  
`PID!(help, self())`  
- Stop.  
`receive {
  stop -> end.
}`  
- Selective message reception.  
`receive {
  m1 -> do1();
  m2 -> do2();
  // There is no default case here
}`  
Selective message reception is a very powerful control mechanism but can be
extremely slow. Selection requires looking through the entire queue of messages
until something matches, and saving the other messages for another receive.
The very worst case is having a queue of messages in reverse order of priority.
Such a queue will take O(N^2) time to traverse.

There are three use cases for selective message reception.
1. You want messages to have priorities. In that case, just use a priority queue.
2. You want to wait for some start signal before you begin processing, because,
despite using asynchronous actors, there is an inherent ordering to how you process
your messages. For this, you should split the work up into two actors, anyway.
One queues the work to do and waits for the configuration message, and the other
is spawned by the first to do the actual processing.
3. You really do need synchronized messages because you are modeling a complicated
state machine. In that case, perhaps selective reception is the best tool.  
- Behavior change through tail-recursive calls.  
See: http://notes.backgroundsignal.com/ErlangStyle.html  
- Default message selection.
`receive {
  num@_ -> num + 4;
}`  
- Global registry of aliases.
`receive {
  (Alert, President) -> registry.get(President) ! Run;  
}`  
- Timeouts.
`receive {
  ImportantMessage -> folow_orders();
  after 20 -> println!("He's gone. We have to go our own way.");
}`  
