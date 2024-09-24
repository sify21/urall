//mod zip;
//mod tar;
//mod gz;
//mod compress;
//mod z7;
//mod bz2;
//mod xz;
//

use std::fs::File;

fn main() {
    let list = compress_tools::list_archive_files(
        File::open("/home/sify/Downloads/aa.md").unwrap(),
    )
    .unwrap();
    println!("{}", list.len());
    for i in list {
        println!("{}", i);
    }
}
