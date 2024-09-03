# Use a Rust base image
FROM rust:1.80.0

# Set the working directory
WORKDIR /app

# Copy the Rust script into the container
COPY . /app

# Compile the Rust script
#RUN cargo build --release

CMD ["tail", "-f", "/dev/null"]