use bevy::prelude::*;

use crate::chess::{board::{Board, Captured, RestoreBoardEvent}, moves::{GenerateMovesEvent, Moves, PieceMovedEvent}, piece::{Piece, PieceColor, SelectedPiece}, position::Position};

#[derive(Component)]
pub struct CheckDetector;

#[derive(Event)]
pub struct CheckIllegalMovesEvent;

struct CheckIllegalMoves {
    moving_piece: Entity,
    enemies: Vec<Entity>,
    check_detectors: Vec<Entity>,
}

impl Command for CheckIllegalMoves {
    fn apply(self, world: &mut World) {
        let moves = world.get::<Moves>(self.moving_piece).unwrap().clone();
        world.entity_mut(self.moving_piece).remove::<Moves>();

        let mut new_moves = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();
            Moves::new(asset_server)
        };

        for (position, mmove) in moves.positions {
            world.trigger(PieceMovedEvent::new(mmove, false));
            world.flush();

            // If the move we're checking captures the piece, it shouldn't be considered
            let enemies: Vec<_> = self.enemies.iter().filter(|&&enemy| world.get::<Captured>(enemy).is_none()).collect();
            let enemy_batch: Vec<_> = enemies.iter().map(|&&enemy| (enemy, Moves::new_no_move_indicators())).collect();

            world.insert_batch(enemy_batch.clone());
            world.trigger(GenerateMovesEvent);

            let mut is_legal = true;

            'outer: for &enemy in enemies {
                let moves = world.get::<Moves>(enemy).unwrap();

                for &check_detector in &self.check_detectors {
                    if moves.positions.contains_key(world.get::<Position>(check_detector).unwrap()) {
                        is_legal = false;
                        break 'outer;
                    }
                }

                world.entity_mut(enemy).remove::<Moves>();
            }

            world.trigger(RestoreBoardEvent);

            if !is_legal {
                continue;
            }

            let is_capture = {
                let board = world.query::<&Board>().single(world).unwrap();
                !board.is_empty(position)
            };

            new_moves.insert(&mut world.commands(), position, mmove, is_capture);
        }

        world.entity_mut(self.moving_piece).insert(new_moves);
    }
}

pub fn check_illegal_moves(_event: On<CheckIllegalMovesEvent>, mut commands: Commands, piece: Single<(Entity, &PieceColor), (With<Piece>, With<SelectedPiece>, With<Moves>)>,
    check_detectors: Query<(Entity, &PieceColor), (With<Piece>, With<CheckDetector>)>, enemies: Query<(Entity, &PieceColor), With<Piece>>) {

    commands.queue(CheckIllegalMoves {
        moving_piece: piece.0,
        enemies: enemies.iter().filter_map(|(enemy, color)| match color == piece.1 {
            true => None,
            false => Some(enemy),
        }).collect(),
        check_detectors: check_detectors.iter().filter_map(|(check_detector, color)| match color == piece.1 {
            true => Some(check_detector),
            false => None,
        }).collect(),
    });
}
