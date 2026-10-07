use reagent_mines_rs::Game;
use serde::Deserialize;
#[derive(Deserialize)]
struct Board {
    id: String,
    width: usize,
    height: usize,
    first: (usize, usize),
    mines: Vec<(usize, usize)>,
}
#[derive(Deserialize)]
struct Action {
    kind: String,
    #[serde(default)]
    x: usize,
    #[serde(default)]
    y: usize,
}
#[derive(Deserialize)]
struct Step {
    action: Action,
    visible: Vec<Vec<String>>,
}
#[derive(Deserialize)]
struct Trace {
    id: String,
    board_id: String,
    initial: Vec<Vec<String>>,
    steps: Vec<Step>,
}
fn compare(data: &str) {
    let boards: Vec<Board> = serde_json::from_str(include_str!("../fixtures/boards.json")).unwrap();
    let traces: Vec<Trace> = serde_json::from_str(data).unwrap();
    for t in traces {
        let b = boards.iter().find(|b| b.id == t.board_id).unwrap();
        let mut g = Game::fixed(b.width, b.height, &b.mines, b.first);
        assert_eq!(g.visible(), t.initial, "{} initial", t.id);
        for (i, s) in t.steps.iter().enumerate() {
            match s.action.kind.as_str() {
                "reveal" => g.reveal(s.action.x, s.action.y),
                "flag" => g.flag(s.action.x, s.action.y),
                "reset" => g.reset(),
                "outside" => g.reveal(usize::MAX, usize::MAX),
                other => panic!("Unsupported observed action {other}"),
            }
            assert_eq!(g.visible(), s.visible, "{} step {}", t.id, i + 1);
        }
    }
}
#[test]
fn calibration_traces() {
    compare(include_str!("../fixtures/calibration.json"));
}

#[test]
fn holdout_traces() {
    compare(include_str!("../fixtures/holdout.json"));
}

// Black-box regressions: a flag interrupts an empty-region flood; removing
// that flag and clicking the neighboring open zero leaves the hole covered.
// Both original traces were captured before applying the compatibility fix.
#[test]
fn observed_flagged_empty_region_does_not_resume_on_zero_click() {
    compare(include_str!("../fixtures/flagged-empty-region.json"));
}
