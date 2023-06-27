use std::fmt::{Display, Formatter};

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

    fn cell_indexes_by_row(&self) -> Vec<Vec<usize>> {
        (0..BOARD_WIDTH).map(|col_idx| {
            (0..BOARD_HEIGHT).map(|row_idx| {
                (row_idx + (col_idx * BOARD_WIDTH))
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }

    fn cell_indexes_by_col(&self) -> Vec<Vec<usize>> {
        (0..BOARD_HEIGHT).map(|row_idx| {
            (0..BOARD_WIDTH).map(|col_idx| {
                (row_idx + (col_idx * BOARD_WIDTH))
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Let's just handle a maximum cell contents width of 4 characters, plus 3 spaces on either side for padding
        // This will cause the presentation to go a bit wonky once the player hits the 16,384 tile, but I think 
        // they can handle it =)
        let cell_width = 4 + 3 + 3;
        let cell_width_including_inter_cell_border = cell_width + 1;

        let horizontal_trim = "-".repeat(cell_width_including_inter_cell_border * BOARD_WIDTH);
        write!(f, "\n{}-\n", horizontal_trim)?;

        for cell_indexes_in_row in self.cell_indexes_by_row().iter() {
            // Each tile should occupy a few lines vertically, to bulk out the presentation
            for line_idx in 0..4 {
                let empty_cell_line = format!("|{}", " ".repeat(cell_width));
                match line_idx {
                    1 => {
                        for cell_idx in cell_indexes_in_row.iter() {
                            let cell = &self.cells[*cell_idx];
                            let cell_text = cell.contents.as_padded_str();
                            write!(f, "|   {cell_text}   ")?;
                        }
                        write!(f, "|\n")?
                    }
                    3 => write!(f, "{}-\n", horizontal_trim)?,
                    _ => write!(f, "{}|\n", empty_cell_line.repeat(BOARD_WIDTH))?
                }
            }
        }

        Ok(())
    }
}
