//! Bounded Mines behavior reconstructed from a licensed executable and observations.
//! This is independent code, not a recovery of the original implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Hidden,
    Flag,
    Open(u8),
    Exploded,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Ready,
    Playing,
    Won,
    Lost,
}
#[derive(Clone, Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    pub cells: Vec<Cell>,
    pub phase: Phase,
    mines: Vec<bool>,
    seed: u64,
    generated: bool,
    initial: Option<Vec<Cell>>,
}
impl Game {
    pub fn new(width: usize, height: usize, count: usize, seed: u64) -> Self {
        assert!(width >= 4 && height >= 4 && count <= width * height - 9);
        Self {
            width,
            height,
            mine_count: count,
            cells: vec![Cell::Hidden; width * height],
            phase: Phase::Ready,
            mines: vec![false; width * height],
            seed,
            generated: false,
            initial: None,
        }
    }
    pub fn fixed(
        width: usize,
        height: usize,
        mines: &[(usize, usize)],
        first: (usize, usize),
    ) -> Self {
        let mut game = Self::new(width, height, mines.len(), 1);
        for &(x, y) in mines {
            assert!(x < width && y < height);
            game.mines[y * width + x] = true;
        }
        game.generated = true;
        game.phase = Phase::Playing;
        game.reveal(first.0, first.1);
        game.initial = Some(game.cells.clone());
        game
    }
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
    fn neighbors(&self, index: usize) -> Vec<usize> {
        let (x, y) = (index % self.width, index / self.width);
        let mut out = Vec::with_capacity(8);
        for by in y.saturating_sub(1)..=(y + 1).min(self.height - 1) {
            for bx in x.saturating_sub(1)..=(x + 1).min(self.width - 1) {
                let i = by * self.width + bx;
                if i != index {
                    out.push(i);
                }
            }
        }
        out
    }
    fn count(&self, i: usize) -> u8 {
        self.neighbors(i).iter().filter(|&&n| self.mines[n]).count() as u8
    }
    fn place(&mut self, first: usize) {
        let mut excluded = self.neighbors(first);
        excluded.push(first);
        let mut candidates: Vec<_> = (0..self.cells.len())
            .filter(|i| !excluded.contains(i))
            .collect();
        let mut state = self.seed.max(1);
        for i in (1..candidates.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            candidates.swap(i, (state as usize) % (i + 1));
        }
        for &i in candidates.iter().take(self.mine_count) {
            self.mines[i] = true;
        }
        self.generated = true;
        self.phase = Phase::Playing;
    }
    pub fn flag(&mut self, x: usize, y: usize) {
        if matches!(self.phase, Phase::Won | Phase::Lost) {
            return;
        }
        if let Some(i) = self.index(x, y) {
            self.cells[i] = match self.cells[i] {
                Cell::Hidden => Cell::Flag,
                Cell::Flag => Cell::Hidden,
                other => other,
            };
        }
    }
    pub fn reveal(&mut self, x: usize, y: usize) {
        if matches!(self.phase, Phase::Won | Phase::Lost) {
            return;
        }
        let Some(i) = self.index(x, y) else {
            return;
        };
        if self.cells[i] == Cell::Flag {
            return;
        }
        if !self.generated {
            self.place(i);
        }
        match self.cells[i] {
            Cell::Open(n) => {
                let neighbors = self.neighbors(i);
                if neighbors
                    .iter()
                    .filter(|&&n| self.cells[n] == Cell::Flag)
                    .count()
                    != n as usize
                {
                    return;
                }
                // A failed chord is atomic: it exposes the first unflagged mine,
                // without opening safe neighbors first (observed holdout discrepancy).
                if let Some(&mine) = neighbors
                    .iter()
                    .find(|&&n| self.cells[n] == Cell::Hidden && self.mines[n])
                {
                    self.cells[mine] = Cell::Exploded;
                    self.phase = Phase::Lost;
                } else {
                    for next in neighbors {
                        if self.cells[next] == Cell::Hidden {
                            self.open(next);
                        }
                    }
                }
            }
            Cell::Hidden => self.open(i),
            _ => {}
        }
        if self.phase != Phase::Lost
            && (0..self.cells.len())
                .all(|i| self.mines[i] || matches!(self.cells[i], Cell::Open(_)))
        {
            self.phase = Phase::Won;
            for i in 0..self.cells.len() {
                if self.mines[i] {
                    self.cells[i] = Cell::Flag;
                }
            }
        }
    }
    fn open(&mut self, i: usize) {
        if self.mines[i] {
            self.cells[i] = Cell::Exploded;
            self.phase = Phase::Lost;
            return;
        }
        let mut todo = vec![i];
        while let Some(next) = todo.pop() {
            if self.cells[next] != Cell::Hidden {
                continue;
            }
            let n = self.count(next);
            self.cells[next] = Cell::Open(n);
            if n == 0 {
                for other in self.neighbors(next) {
                    if !self.mines[other] && self.cells[other] == Cell::Hidden {
                        todo.push(other);
                    }
                }
            }
        }
    }
    pub fn reset(&mut self) {
        if let Some(initial) = &self.initial {
            self.cells = initial.clone();
            self.phase = Phase::Playing;
        } else {
            self.cells.fill(Cell::Hidden);
            self.mines.fill(false);
            self.generated = false;
            self.phase = Phase::Ready;
        }
    }
    pub fn remaining(&self) -> isize {
        self.mine_count as isize - self.cells.iter().filter(|&&c| c == Cell::Flag).count() as isize
    }
    pub fn revealed(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| matches!(c, Cell::Open(_)))
            .count()
    }
    pub fn visible(&self) -> Vec<Vec<String>> {
        self.cells
            .chunks(self.width)
            .map(|r| {
                r.iter()
                    .map(|c| match c {
                        Cell::Hidden => "H".into(),
                        Cell::Flag => "F".into(),
                        Cell::Exploded => "X".into(),
                        Cell::Open(n) => n.to_string(),
                    })
                    .collect()
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_click_and_neighborhood_are_safe() {
        for seed in 1..=256 {
            for &(x, y) in &[(0, 0), (7, 7), (4, 4)] {
                let mut g = Game::new(8, 8, 10, seed);
                g.reveal(x, y);
                assert_eq!(g.cells[y * 8 + x], Cell::Open(0));
                assert_eq!(g.mines.iter().filter(|&&m| m).count(), 10);
            }
        }
    }
    #[test]
    fn flags_do_not_start_generation_and_can_exceed_counter() {
        let mut g = Game::new(8, 8, 10, 42);
        for x in 0..8 {
            g.flag(x, 0);
            g.flag(x, 1);
        }
        assert_eq!(g.remaining(), -6);
        g.reveal(0, 0);
        assert_eq!(g.phase, Phase::Ready);
        g.reset();
        assert_eq!(g.remaining(), 10);
    }
    #[test]
    fn invalid_actions_preserve_state() {
        let mut g = Game::new(8, 8, 10, 1);
        let cells = g.cells.clone();
        g.reveal(8, 8);
        g.flag(100, 0);
        assert_eq!(g.cells, cells);
        assert_eq!(g.phase, Phase::Ready);
    }
}
