# Conversation

Conversation on discord rust to refactor the struct (thanks Kimundi).

```
10:45] Romu: Hi, I'm reading the rust book about the closure part.
https://doc.rust-lang.org/book/ch13-01-closures.html
[10:45] Romu: They said I could try to refactor their Cacher struct to use a HashMap
[10:46] Romu: not sure I did that in the right way even though it works, could I get advice?
https://replit.com/@RomualdQuantin/CacherHashMap#src/main.rs 
[10:46] Romu: refactored function is the fn value(&mut self, arg: u32) -> u32 
[10:47] Kimundi: That seems right, but I would probably get rid of the value field entriely
[10:47] Romu: really not sure about the let owned_arg = arg.clone(); as I wanted to keep the same signature
[10:47] Kimundi: that clone is unneeded, arg already is a owned type (u32)
[10:48] Romu: so how do I do because the hashmap is asking  a &u32
[10:48] Kimundi: (well every type is onwed like that, what I meant is that it is not a reference where you need to clone the value out of)
[10:48] Kimundi: well you would still pass &arg
[10:50] Romu: I see, and when I arrive to self.store.insert(owned_arg, v);
[10:50] Romu: should I clone at that moment?
[10:50] Kimundi: also just insert arg there
[10:51] Romu: oh yes of course, so I don't need that var at all
[10:51] Kimundi: You do not need to proactively clone. You only need clone if you have a &T and want to get a T from that.
[10:51] Romu: what did you mean by "but I would probably get rid of the value field entriely"
[10:51] Kimundi: And if T implements Copy, which u32 does, then you can even just do a copy via the dereference operator
[10:52] Romu: right
[10:52] Kimundi: Eg at the place where you return v.clone(), you could have also written *v 
[10:52] Romu: noted, I see
[10:52] Kimundi: .clone() would not be wrong, mind you
[10:53] Romu: now with rust I'm scared to create unnecessary thing in memory haha
[10:53] Romu: thanks for that, would you elaborate on "but I would probably get rid of the value field entriely"
[10:53] Romu: what did you mean exactly?
[10:55] Kimundi: Don't be scared 😄 Nothing bad will happen, and any hypothetical issues will just cause an compile error.

In plenty of other languages its common to copy and allocate way more than that. If anything, we tend to try to optimize that too much in Rust, just because we get the tools to do so. But for a lot of code it does not matter if you do extra copies of things all the time.

Besdies, what you say on the syntax level about what gets copied does only roughly map to what will happen at the finam binary after the optimizing compiler is done with your code
[10:56] Kimundi: For value: What I meant is that the original value: Option<u32> existed so that the struct could store the result of evaluating the function. With the HashMap, you can use the existence of an entry in the HashMap for that, so the extra Option field is redundant
[10:59] Romu: I think I see what you mean, but the point of that part in the book was to evaluate the closure the first time it is needed (if I understood what you meant)
[11:00] Romu: need to call something basically, which is what that is: expensive_result.value(intensity)
[11:00] Romu: right?
[11:02] Romu: oh I see, you're talking about the actual value, not the method
[11:02] Romu: will try to remove
[11:02] Kimundi: yup, the field, not the method
[11:02] Kimundi: See also https://doc.rust-lang.org/book/ch13-01-closures.html#limitations-of-the-cacher-implementation
Closures: Anonymous Functions that Can Capture Their Environment - ...
[11:03] Kimundi: The Option-field method has the issue that it only works if the caller always passes the same argument value, otherwise it returns the wrong results
[11:03] Kimundi: and by using a hashmap, you can have a cache for any argument value
[11:04] Romu: right updated https://replit.com/@RomualdQuantin/CacherHashMap
[11:04] Kimundi: Once everything is clear in that regard, I have a final recommendation about how to make the hashmap code shorter and more convenient
[11:04] Romu: awesome, I'm all ears 😄
[11:05] Kimundi: Hm, I think the code was not updated you sent?
[11:06] Romu: hum, is that cache? I see the right code in an incognito window
[11:07] Kimundi: weird, I did a force refresh of the webpage
[11:07] Kimundi: but yes, in a incognito window its updated
[11:08] Kimundi: yup that looks how I would expect
[11:08] Romu: found a publish button, not sure 🤷‍♂️
[11:08] Kimundi: Okay, so a very convenient method on HashMap and BTreeMap is the .entry() API: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
[11:09] Romu: oh nice
[11:09] Romu: I see the or_insert
[11:09] Kimundi: It would allow you to write self.store.entry(arg).or_insert_with(|| (self.calculation)(arg))
[11:10] Romu: right
[11:10] Kimundi: For the spirint of this execise, you would use or_insert_with. or_insert expects the value to already exist as a computed value, which we want to avoid here unless its actually missing 🙂
[11:11] Romu: yep make sense, I don't want to execute if cached 
[11:11] Kimundi: It then returns a &mut u32 to the value in the hashmap, either existing or newly inserted, and you can just return a clone/copy of that u32 value
[11:11] Kimundi: Which basically turns this into a one-liner 😄 
[11:18] Romu: right updated, that's much simpler! 
[11:19] Romu: a bit confused by why or_insert_with is returning a mutable though 
[11:22] Kimundi: The idea behind the entry API is to give you full access to a hashmap entry, even if it has to create it first
[11:23] Kimundi: since it needs to have mutable access to create it anyway, there is no reason to limit you to &T access
[11:24] Kimundi: the return value of or_insert is thus nopt specifc to the to_insert function, but to the entire entry API. Its an API where you can chaing of multiple different methods to accomplish similar goals
[11:25] Romu: right
[11:26] Romu: I'll have to chew a bit more rust on all that 😄
[11:27] Romu: I also can't do that somehow: 
store: HashMap<u32, T>,
[11:27] Romu: in case I want to use String rather than u32
[11:27] Romu: I'll to get a bit further in the book I guess
[11:27] Romu: I think they talk about that in the generic section
[11:27] Kimundi: Well you can if T is a type that exists. Either because you have defined a type named T, or because you have made the struct generic over a type paramter named T
[11:28] Kimundi: But yeah, the book goes over it later 🙂
[11:28] Romu: ok 😉
[11:28] Romu: well thanks a lot for your help, all questions answered, time greatly appreciated 😉
```
