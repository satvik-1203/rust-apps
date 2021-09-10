mod server;
use server::Server;

fn main() {
    let server = Server::New("127.0.0.1:8080");
    server.Run();
}
