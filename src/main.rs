extern crate clap;
use clap::Parser;
use ls_util::{List, ListImpl};

fn main() {
    let ls = List::parse();
    let ls_impl = ListImpl::new(ls);
    ls_impl.list_dir_contents();
}
