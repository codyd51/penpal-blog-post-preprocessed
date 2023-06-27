use std::fmt::{Display, Formatter};
use rand::prelude::*;
use std::iter::Rev;
use std::slice::Iter;

use itertools::{Either, Itertools};
use crate::Direction;

pub(crate) const BOARD_WIDTH: usize = 4;
pub(crate) const BOARD_HEIGHT: usize = 4;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardCoordinate(usize, usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CellContents {
    Empty,
    Occupied(usize),
}

impl CellContents {

    fn as_padded_str(&self) -> String {
        match &self {
            Self::Empty => "    ".to_string(),
            Self::Occupied(value) => format!("{: ^4}", value),
        }
    }


    fn unwrap(&self) -> usize {
        match self {
            Self::Empty => panic!("Expected a non-empty cell"),
            Self::Occupied(val) => *val
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    coords: BoardCoordinate,
    pub(crate) contents: CellContents,
}

impl Cell {
    fn with_coords(coords: BoardCoordinate) -> Self {
        Self {
            coords,
            contents: CellContents::Empty,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self.contents, CellContents::Empty)
    }

}

#[derive(Debug)]
pub(crate) struct Board {
    pub(crate) cells: [Cell; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut cells = vec![];
        for row_idx in 0..BOARD_HEIGHT {
            for col_idx in 0..BOARD_WIDTH {
                cells.push(Cell::with_coords(BoardCoordinate(col_idx, row_idx)));
            }
        }
        Self {
            cells: cells.try_into().unwrap()
        }
    }

    fn move_cell_into_cell(&mut self, source_cell_idx: usize, dest_cell_idx: usize) {
        self.cells[dest_cell_idx].contents = self.cells[source_cell_idx].contents;
        // And empty the source cell, since it's been moved
        self.cells[source_cell_idx].contents = CellContents::Empty;
    }


    fn cell_indexes_by_row(&self) -> Vec<Vec<usize>> {
        let mut cell_indexes_by_row = vec![];
        for col_idx in 0..BOARD_WIDTH {
            let mut cell_indexes_in_row = vec![];
            for row_idx in 0..BOARD_HEIGHT {
                cell_indexes_in_row.push(row_idx + (col_idx * BOARD_WIDTH))
            }
            cell_indexes_by_row.push(cell_indexes_in_row)
        }
        cell_indexes_by_row
    }

    fn cell_indexes_by_col(&self) -> Vec<Vec<usize>> {
        let mut cell_indexes_by_col = vec![];
        for row_idx in 0..BOARD_HEIGHT {
            let mut cell_indexes_in_col = vec![];
            for col_idx in 0..BOARD_WIDTH {
                cell_indexes_in_col.push(row_idx + (col_idx * BOARD_WIDTH))
            }
            cell_indexes_by_col.push(cell_indexes_in_col)
        }
        cell_indexes_by_col
    }
    fn iter_axis_in_direction<'a>(direction: Direction, cell_indexes_by_col: &'a Vec<Vec<usize>>, cell_indexes_by_row: &'a Vec<Vec<usize>>) -> Either<Iter<'a, Vec<usize>>, Rev<Iter<'a, Vec<usize>>>> {
        match direction {
            Direction::Left => Either::Left(cell_indexes_by_col.iter()),
            Direction::Right => Either::Right(cell_indexes_by_col.iter().rev()),
            Direction::Up => Either::Left(cell_indexes_by_row.iter()),
            Direction::Down => Either::Right(cell_indexes_by_row.iter().rev()),
        }
    }

    pub(crate) fn spawn_tile_in_random_location(&mut self) {
        // Pick a random free cell
        let free_cells = self.cells.iter_mut().filter(|elem|{
            elem.is_empty()
        });
        let chosen_cell = free_cells.choose(&mut thread_rng()).unwrap();
        let value = [2, 4].choose(&mut thread_rng()).unwrap();
        chosen_cell.contents = CellContents::Occupied(*value);
    }

    fn push_cells_to_close_empty_gaps_with_perpendicular_rows(&mut self, direction: Direction) {
        let cell_indexes_by_col = self.cell_indexes_by_col();
        let cell_indexes_by_row = self.cell_indexes_by_row();
        loop {
            let mut did_modify_cells = false;
            let row_iter = Self::iter_axis_in_direction(direction, &cell_indexes_by_col, &cell_indexes_by_row);
            for (dest_row, source_row) in row_iter.tuple_windows::<(&Vec<usize>, &Vec<usize>)>() {
                for (dest_cell_idx, source_cell_idx) in dest_row.iter().zip(source_row.iter()) {
                    let dest_cell = &self.cells[*dest_cell_idx];
                    let source_cell = &self.cells[*source_cell_idx];
                    if source_cell.is_empty() {
                        // If the source cell is empty, we have nothing to do
                        continue;
                    }
                    if dest_cell.is_empty() {
                        // If the destination cell is empty, copy the source cell
                        self.move_cell_into_cell(*source_cell_idx, *dest_cell_idx);
                        did_modify_cells = true;
                        break;
                    }
                }
            }
            if !did_modify_cells {
                break;
            }
        }
    }

    fn merge_contiguous_cells_in_direction(&mut self, direction: Direction) {
        let cell_indexes_by_col = self.cell_indexes_by_col();
        let cell_indexes_by_row = self.cell_indexes_by_row();
        let row_iter = Self::iter_axis_in_direction(direction, &cell_indexes_by_col, &cell_indexes_by_row);
        for (dest_row, source_row) in row_iter.tuple_windows::<(&Vec<usize>, &Vec<usize>)>() {
            for (dest_cell_idx, source_cell_idx) in dest_row.iter().zip(source_row.iter()) {
                let dest_cell = &self.cells[*dest_cell_idx];
                let source_cell = &self.cells[*source_cell_idx];
                if source_cell.is_empty() || dest_cell.is_empty() {
                    // If one of the cells is empty, we can't merge them
                    continue;
                }

                let source_value = source_cell.contents.unwrap();
                let dest_value = dest_cell.contents.unwrap();
                if source_value != dest_value {
                    // The cells didn't contain the same value, so we can't merge them
                    continue;
                }

                // Combine into the destination cell
                self.cells[*dest_cell_idx].contents = CellContents::Occupied(dest_value * 2);
                // Clear the contents of the source cell, because it's been merged
                self.cells[*source_cell_idx].contents = CellContents::Empty;
            }
        }
    }

    pub(crate) fn press(&mut self, direction: Direction) {
        // First, push all the elements towards the edge until they meet resistance
        self.push_cells_to_close_empty_gaps_with_perpendicular_rows(direction);
        // Now iterate again and try to merge contiguous tiles that share the same value
        // We need to do this in a separate iteration because the behavior is subtly different:
        // When pushing cells around, we want to recursively push cells until there's no remaining free
        // space.
        // However, when merging cells, we want to stop processing a row as soon as we merge a pair of cells,
        // even if more merges are possible. The user needs to do another turn to perform the next merge.
        self.merge_contiguous_cells_in_direction(direction);
        // The above step may have produced some gaps, so push cells again
        // For example,
        // | 16 | 16 | 16 |  4 |
        // | 32 |    | 16 |  4 |
        self.push_cells_to_close_empty_gaps_with_perpendicular_rows(direction);
    }

    pub(crate) fn is_full(&self) -> bool {
        for cell in self.cells.iter() {
            if cell.contents == CellContents::Empty {
                return false;
            }
        }
        true
    }

    pub(crate) fn empty(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.contents = CellContents::Empty
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Let's just handle a maximum cell contents width of 4 characters, plus 3 spaces on either side for padding
        // This will cause the presentation to go a bit wonky once the player hits the 16,384 tile, but I think
        // they can handle it =)
        let cell_width = 4 + 6;
        let cell_width_including_inter_cell_border = cell_width + 1;

        let horizontal_trim = "-".repeat(cell_width_including_inter_cell_border * BOARD_WIDTH);
        write!(f, "{}-", horizontal_trim)?;

        for cell_indexes_in_row in self.cell_indexes_by_row().iter() {
            // Each tile should occupy a few lines vertically, to bulk out the presentation
            for line_idx in 0..4 {
                let empty_cell_line = format!("|{}", " ".repeat(cell_width));
                match line_idx {
                    1 => {
                        write!(f, "\n")?;
                        for cell_idx in cell_indexes_in_row.iter() {
                            let cell = &self.cells[*cell_idx];
                            let cell_text = cell.contents.as_padded_str();
                            write!(f, "|   {cell_text}   ")?;
                        }
                        write!(f, "|")?
                    }
                    3 => write!(f, "\n{}-", horizontal_trim)?,
                    _ => write!(f, "\n{}|", empty_cell_line.repeat(BOARD_WIDTH))?
                }
            }
        }
        Ok(())
    }
}
