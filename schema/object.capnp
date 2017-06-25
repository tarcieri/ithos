# Objects are entries in ithos's directory tree

@0xdb5ce5e163b4ffaa;

using Credential = import "object/credential.capnp".Credential;
using Domain     = import "object/domain.capnp".Domain;
using OrgUnit    = import "object/org_unit.capnp".OrgUnit;
using Root       = import "object/root.capnp".Root;
using System     = import "object/system.capnp".System;

# An entry in ithos's directory tree
struct Object {
  # Sum type for all objects
  value :union {
    # Root entry of the directory tree (Root DSE in LDAP parlance)
    root       @0 :Root;

    # Domain of/within an organization (i.e. DNS domain)
    domain     @1 :Domain;

    # Organizational Unit (Unit/department or other logical grouping)
    orgUnit   @2 :OrgUnit;

    # System User (non-human administrative account)
    system     @3 :System;

    # Encrypted Credentials (stored within ithos itself)
    credential @4 :Credential;
  }
}
