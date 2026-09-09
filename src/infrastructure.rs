// Bevy Prelude:
// nos da acceso a los tipos más usados de Bevy.
use bevy::prelude::*;

// Dominio del ajedrez:
// nos da acceso a los tipos definidos en `domain.rs`.
use crate::domain::*;

/**
 * constanstes para el renderizado y "fisicas" del tablero
 */
pub const GAME_THEME: &str = "black";
pub const BOARD_SIZE: f32 = 180.0;
pub const BOARD_SCALE: f32 = 2.0;
pub const PIECE_SCALE: f32 = 1.5;
pub const SQUARE_SIZE: f32 = BOARD_SIZE / 8.0;
/**
 * UTILS: mapear posicion en el dominio del tablero al dominio del videojuego
 */
fn square_to_world(square: Square) -> Vec3 {
    let x = -BOARD_SIZE / 2.0 + SQUARE_SIZE / 2.0 + square.file as f32 * SQUARE_SIZE;

    let y = -BOARD_SIZE / 2.0 + SQUARE_SIZE / 2.0 + square.rank as f32 * SQUARE_SIZE;

    Vec3::new(x * BOARD_SCALE, y * BOARD_SCALE, 1.0)
}

/**
 * UTILS: mapear posicion en el dominio del videojuego al dominio del tablero
 */
fn world_to_square(position: Vec2) -> Option<Square> {
    let board_size = BOARD_SIZE * BOARD_SCALE;

    let x = position.x + board_size / 2.0;
    let y = position.y + board_size / 2.0;

    if x < 0.0 || x >= board_size || y < 0.0 || y >= board_size {
        return None;
    }

    let file = (x / (SQUARE_SIZE * BOARD_SCALE)) as u8;
    let rank = (y / (SQUARE_SIZE * BOARD_SCALE)) as u8;

    Some(Square { file, rank })
}

/**
 * UTILS: renderizar una pieza en el tablero
 */
fn spawn_piece(
    commands: &mut Commands,
    asset_server: &AssetServer,
    color: PieceColor,
    kind: PieceKind,
    square: Square,
) {
    let color_name = match color {
        PieceColor::White => "white",
        PieceColor::Black => "black",
    };

    let piece_name = match kind {
        PieceKind::Pawn => "pawn",
        PieceKind::Knight => "knight",
        PieceKind::Bishop => "bishop",
        PieceKind::Rook => "rook",
        PieceKind::Queen => "queen",
        PieceKind::King => "king",
    };

    let path = format!("{GAME_THEME}/{color_name}_{piece_name}.png");

    let image = asset_server.load(path);

    commands.spawn((
        ChessPiece {
            color,
            kind,
            square,
        },
        Sprite { image, ..default() },
        Transform {
            translation: square_to_world(square),
            scale: Vec3::splat(PIECE_SCALE),
            ..default()
        },
    ));
}

/**
 * UTILS: renderizar todo el juego en el tablero
 */
fn spawn_game(commands: &mut Commands, asset_server: &AssetServer) {
    // Piezas negras
    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Rook,
        Square { file: 0, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Knight,
        Square { file: 1, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Bishop,
        Square { file: 2, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Queen,
        Square { file: 3, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::King,
        Square { file: 4, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Bishop,
        Square { file: 5, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Knight,
        Square { file: 6, rank: 7 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::Black,
        PieceKind::Rook,
        Square { file: 7, rank: 7 },
    );

    for file in 0..8 {
        spawn_piece(
            commands,
            asset_server,
            PieceColor::Black,
            PieceKind::Pawn,
            Square { file, rank: 6 },
        );
    }

    // Piezas blancas
    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Rook,
        Square { file: 0, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Knight,
        Square { file: 1, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Bishop,
        Square { file: 2, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Queen,
        Square { file: 3, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::King,
        Square { file: 4, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Bishop,
        Square { file: 5, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Knight,
        Square { file: 6, rank: 0 },
    );

    spawn_piece(
        commands,
        asset_server,
        PieceColor::White,
        PieceKind::Rook,
        Square { file: 7, rank: 0 },
    );

    for file in 0..8 {
        spawn_piece(
            commands,
            asset_server,
            PieceColor::White,
            PieceKind::Pawn,
            Square { file, rank: 1 },
        );
    }
}

/**
 * SETUP: iniciar una partida de ajedrez
 */
pub fn init_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // colocar la camara
    commands.spawn(Camera2d);
    // renderizar el tablero de juego
    let board = asset_server.load(format!("{GAME_THEME}/board.png"));
    commands.spawn((
        Sprite {
            image: board,
            ..default()
        },
        Transform::from_scale(Vec3::splat(BOARD_SCALE)),
    ));
    // renderizar las piezas sobre el tablero
    spawn_game(&mut commands, &asset_server);
}

/**
 * seleccionar una pieza del tablero
 */
pub fn select_piece(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &ChessPiece)>,
    mut commands: Commands,
) {
    // si el boton del mouse no fue presionado, no vamos
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();

    // voy a desestructurar (y "desempaquetar") la posicion del click en la ventada
    // por desempaquetar me refiero a hacer pattern matching sobre el Option y extraigo el Vec2 contenido en Some
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // voy a desestructurar (y "desempaquetar") la posicion del click respecto a la camara
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // voy a calcular el escaque que se selecciono
    let Some(square) = world_to_square(world_position) else {
        return;
    };

    // voy a localizar cual fue la pieza que "seleccione con el mouse" y a marcarla justamente como
    // seleccionada
    for (entity, piece) in &pieces {
        if piece.square == square {
            commands.entity(entity).insert(Selected);
            println!("Seleccioné {:?} en {:?}", piece.kind, square);
        }
    }
}

/**
 * buscar y seleccionar a los reyes en el tablero
 */
pub fn select_king(mut commands: Commands, query: Query<(Entity, &ChessPiece)>) {
    for (entity, piece) in &query {
        if piece.kind == PieceKind::King {
            commands.entity(entity).insert(Selected);
        }
    }
}
