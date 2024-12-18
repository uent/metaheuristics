# Nombre del binario (opcional)
BINARY_NAME=metaheuristics

# El comando por defecto es `build`
.PHONY: all
all: build

# Compilar en modo release
.PHONY: build_prod
build_prod:
	cargo build --release

# Compilar en modo debug
.PHONY: build
build:
	cargo build

# Ejecutar el binario compilado
.PHONY: run
run: build
	./target/debug/$(BINARY_NAME) $(COMANDO) --numero $(NUMERO)

# Limpiar los archivos generados por Cargo
.PHONY: clean
clean:
	cargo clean