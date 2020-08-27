extern crate communicator;

use communicator::network::server;

fn main() {
    communicator::client::connect();
    server::connect();
}
