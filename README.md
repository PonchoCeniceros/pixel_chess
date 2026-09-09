# Pixel Chess

Un juego de ajedrez 2D con estética pixel art desarrollado en **Rust** utilizando **Bevy**.

El proyecto tiene como objetivo servir como práctica para aprender desarrollo de videojuegos con Rust y, al mismo tiempo y explorar la arquitectura **ECS (Entity Component System)** de Bevy.

## Tecnologías

- **Rust**
- **Bevy 0.19**
- Pixel Art
- ECS (Entity Component System)

## Objetivo

Construir progresivamente un juego de ajedrez funcional:

- Tablero y piezas en pixel art.
- Selección de piezas mediante mouse.
- Movimiento de piezas.
- Validación de movimientos.
- Capturas.
- Turnos.
- Jaque y jaque mate.
- Promoción de peones.
- Enroque.
- Interfaces y feedback visual.

El proyecto se desarrolla de forma incremental, implementando primero las bases del juego y posteriormente las reglas completas del ajedrez.

## Arquitectura

Por ahora el proyecto utiliza una separación sencilla en tres capas:

```text
src/
├── domain.rs
├── application.rs
├── infrastructure.rs
└── main.rs
```

## Referencias

[docs.rs/bevy](https://docs.rs/bevy/latest/bevy/?search=)


