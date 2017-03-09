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
