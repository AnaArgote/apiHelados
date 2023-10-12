# Utiliza una imagen base de Rust estable
FROM rust:latest

# Establece el directorio de trabajo en el contenedor
WORKDIR /usr/src/apiHelados

# Copia los archivos relacionados con el proyecto (Cargo.toml y Cargo.lock) para aprovechar las capas de caché
COPY Cargo.toml Cargo.lock ./

# Copia el código fuente completo al contenedor
COPY . .

# Compila la aplicación en modo release
RUN cargo build --release

# Expone el puerto en el que la API escuchará (ajusta el número de puerto según tu aplicación)
EXPOSE 8080

# Comando para iniciar la aplicación cuando se ejecute el contenedor
CMD ["./target/release/apiHelados"]
