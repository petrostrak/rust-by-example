## Phantom type parameters
A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. These extra parameters hold no storage values, and have no runtime behavior.