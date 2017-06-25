# inode-like metadata associated with entries

@0xc8f8da0fc8bda2eb;

using BlockID = Data;
using Timestamp = UInt64;

# Metadata entry
struct Metadata {
  # Block where this entry first appeared
  createdId @0 :BlockID;

  # Last block that modified this entry
  updatedId @1 :BlockID;

  # Time when this block was created
  createdAt @2 :Timestamp;

  # Time when this block was last updated
  updatedAt @3 :Timestamp;

  # Counter incremented whenever an entry is modified
  version @4 :UInt32;
}
