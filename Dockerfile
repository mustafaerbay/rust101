# Use a Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Rust project files to the container
COPY . .

# Compile the Rust project
RUN cargo build --release

# Set the entrypoint command to run the compiled binary
CMD ["./target/release/sum_variables"]
