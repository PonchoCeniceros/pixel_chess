mod application;
mod domain;
mod infrastructure;

// Bevy Prelude:
// `bevy::prelude::*` nos da acceso a los tipos más usados de Bevy.
//
// App             → construye y configura la aplicación Bevy.
// DefaultPlugins  → conjunto de plugins básicos de Bevy.
// Startup/Update  → horarios (schedules) donde ejecutamos nuestros sistemas.
// Commands        → solicita cambios estructurales al ECS.
// Query           → busca entidades que tengan determinados Components.
// Res<T>          → acceso de solo lectura a un Resource.
// Component       → marca un struct como Component de ECS.
// Entity          → identificador de una entidad.
//
// Renderizado:
// Sprite          → indica qué imagen/sprite dibujar.
// Transform       → posición, rotación y escala.
// Vec2/Vec3       → vectores 2D/3D.
// Camera/Camera2d  → cámara del juego.
// GlobalTransform  → transformación global.
//
// Assets:
// AssetServer     → carga assets desde `assets/`.
// Handle<T>       → referencia a un asset.
// Image           → asset de imagen.
// ImagePlugin     → configuración de imágenes.
//
// Input:
// ButtonInput<T>  → estado de botones.
// MouseButton     → botones del mouse.
//
// ECS:
// With<T>         → entidades que tienen T.
// Without<T>      → entidades que NO tienen T.
//
// Otros:
// PluginGroup     → permite configurar grupos de plugins.
// default()       → construye valores usando Default.

use crate::infrastructure::*;
use bevy::prelude::*;

/**
 * Capa de infraestructura
 */
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                // ventana del tamaño del tablero (360) + margen
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "pixel_chess".into(),
                        resolution: (400, 400).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, init_game)
        .add_systems(Update, select_piece)
        .run();
}
