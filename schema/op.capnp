# Mutating operations which transform the state of the database

@0xfe8381c311c28ee5;

using Object = import "object.capnp".Object;

# Operation type
enum Type {
  # Add a new entry to the directory tree
  add @0;
}

# Operation
struct Op {
  # Type of operation to perform
  optype @0 :Type;

  # Path at which relevant object is located
  path @1 :Text;

  # Object relevant to the operation (e.g. to add)
  object @2 :Object;
}
