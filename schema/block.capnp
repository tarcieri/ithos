# Blocks which represent transactional batches of ops in ithos's log

@0x87998bf69f1aa321;

using BlockID = Data;
using Object = import "object.capnp".Object;
using Op = import "op.capnp".Object;
using Signature = import "signature.capnp".Signature;
using Timestamp = UInt64;

# Block body (inner part to be signed)
struct Body {
  # ID of previous block in log
  parentId @0 :BlockID;

  # Time at which this block was generated
  timestamp @1 :Timestamp;

  # Operations to perform
  ops @2 :List(Op);

  # Text comment as to why change was performed
  comment @3 :Text;
}

# Out-of-band data used to satisfy block consensus
struct Witness {
    # Signatures which authenticate the block
    signatures @0 :List(Signature);
}

# Outer block (only inner portion is signed)
struct Block {
  # Inner (signed) portion of the block
  body @0 :Body;

  # Out-of-band data which satisfies the consensus program
  witness @1 :Witness;
}
