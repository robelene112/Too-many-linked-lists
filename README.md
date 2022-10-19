### Linked List implementation in rust<br>

<br>

<p>This repository features a linked list implementation in Rust according to chapter 2 of the book 'Learning Rust With Entirely Too Many Linked Lists'. The purpose of this project is to give myself a broader understanding of one of the commonly taught data-structures. </p>

<br>

<p>This implementation: <br>
1) never allocates unused memory on the heap in the tail node;<br>
2) makes use of null-pointer optimization;<br>
3) allocates all nodes on the heap uniformly (instead of having the first node allocated on the stack);
4) uses i32 type for the node value.</p>

</p>
Future commits will include more functions like: adding/deleting nodes, stringifying the list, merging/splitting the list etc.
<p>
