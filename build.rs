extern crate capnpc;
extern crate glob;

use glob::glob;

fn main() {
    let mut cmd = capnpc::CompilerCommand::new();

    // Remove the "schema" prefix from output files
    cmd.src_prefix("schema");

    // Allow absolute imports relative to the toplevel schema directory
    cmd.import_path("schema");

    // Compile all .capnp files in the toplevel schema directory
    for file in glob("schema/**/*.capnp").unwrap() {
        cmd.file(file.unwrap());
    }

    cmd.run().unwrap();
}
