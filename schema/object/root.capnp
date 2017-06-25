# Root entry of the directory tree (Root DSE in LDAP parlance)

@0xbd68b8af84197771;

using DigestAlg = import "/alg.capnp".DigestAlg;

# Root object
struct Root {
    digestAlg @0 :DigestAlg;
}
