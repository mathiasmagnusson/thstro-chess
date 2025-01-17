use super::SquareSpec;
use crate::{Color, Piece, PieceType};
use std::fmt;

/// The general type to represent moves.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Move {
    /// A "normal" move between two squares. This covers most moves,
    /// most notably en passant
    Normal { from: SquareSpec, to: SquareSpec },
    /// We have a special variant for castling as it moves two pieces
    Castling(Castling),
    /// Promotion also gets a special move, as it results in a new
    /// piece and we'd like to record the new piece type
    Promotion {
        from: SquareSpec,
        to: SquareSpec,
        target: PieceType,
    },
}

impl Move {
    /// bada bing bada bong, if it's promotion it returns non
    pub fn new(piece: Piece, from: SquareSpec, to: SquareSpec) -> Option<Self> {
        if piece.piece == PieceType::King && (to - from).d_file == 2 {
            Some(Self::Castling(Castling::Short))
        } else if piece.piece == PieceType::King && (to - from).d_file == -2 {
            Some(Self::Castling(Castling::Long))
        } else if piece.piece == PieceType::Pawn && to.rank == piece.color.opposite().home_rank() {
            None
        } else {
            Some(Self::Normal { from, to })
        }
    }
    /// The `from` square of the move.
    pub fn from(&self, color: Color) -> SquareSpec {
        match self {
            Move::Normal { from, .. } | Move::Promotion { from, .. } => *from,
            Move::Castling(_) => {
                let rank = color.home_rank();
                SquareSpec::new(rank, 4)
            }
        }
    }
    /// The destination of the move.
    pub fn to(&self, color: Color) -> SquareSpec {
        match self {
            Move::Normal { to, .. } | Move::Promotion { to, .. } => *to,
            Move::Castling(c) => {
                let rank = color.home_rank();

                let kt = match c {
                    Castling::Short => 6,
                    Castling::Long => 2,
                };

                SquareSpec::new(rank, kt)
            }
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Normal { from, to } => write!(f, "{}{}", from, to),
            Move::Castling(Castling::Short) => write!(f, "O-O"),
            Move::Castling(Castling::Long) => write!(f, "O-O-O"),
            Move::Promotion { from, to, target } => write!(f, "{}{}={}", from, to, target),
        }
    }
}

/// Enum for the two ways you can castle
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Castling {
    /// Castling king-side
    Short,
    /// Castling queen-side
    Long,
}
