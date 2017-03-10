# data-structures

An attempt to implement standard data structures in Rust or C (https://en.wikipedia.org/wiki/List_of_data_structures).

## List
(also called `sequence`)

Properties:
 * `finite` set of values,
 * `ordered` values,
 * `no restriction` on content (the same value can occure many times)

Operations:
 * create an empty list,
 * prepend one item,
 * append one item,
 * get the first item of the list (the `head` of the list),
 * get the `tail` of the list (access all the items except the first one as a list)

Manipulations:
 * iteration
 * recursion

Implementations:
 * linked list (most of the time),
 * array

Remarks:
 * Infinite analog: `stream`

### Linked list
(implementation of a `list`)

Linear list of nodes. One node contains the data and a pointer to the next item.

Advantages:
 * fast for inserting and removing items: no need for reallocation or reorganization as a normal array;
for one inserted or removed node, only one other node has to be modified (the pointer).
 * `insertion` and `deletion` easily implemented (even for nodes in the middle of the list),
 * no initial size to set,

Disadvantages:
 * O(n) find a specific node requires to browse (scan) all the nodes, from the first one to the last one.
 * They use a lot of memory to store the pointers on the next item,
 * they have to be read from the first item to the last one,

Implementations:
 * stack,
 * queue
