# Multiplayer Tic Tac Toe

Challenge your foe to a game of Tic-Tac-Toe via Internet by sending them a link. 

## How to run the server on your local machine:

Make sure you have a working Rust Toolchain, instructions for that on https://rustup.rs/ .

Then simply clone the repo, navigate to the folder and 
```sh
cargo run
```
Then open localhost:8080 in your browser. 


If you don't want to install a Rust toolchain or want to deploy on a server that does not have one, you can build a Docker-Image with the included Dockerfile. 

```sh
docker build -t mytictactoeserver . 
# This may take a while
docker run -p 8080:8080 mytictactoeserver
```