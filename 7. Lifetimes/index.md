* A way to form the complier that borrowed data will be valid a specific point in time
* All data has a lifetime
* Lifetimes allow:
Borrowed data in a structure
Returning references from functions
* Lifetimes will be check by the complier
* Lifetimes are usually elided, but can be specified manually
* Structures utilizing borrowed data must
Always be created after the owner was created
Always be destroyed before the owner is destroyed
* Examples
```
```