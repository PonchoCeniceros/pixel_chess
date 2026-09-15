use bevy::prelude::{Component, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

#[derive(Component)]
pub struct ChessPiece {
    pub color: PieceColor,
    pub kind: PieceKind,
    pub square: Square,
}

#[derive(Resource)]
struct ChessGame {
    turn: PieceColor,
}

#[derive(Component)]
pub struct Selected;
