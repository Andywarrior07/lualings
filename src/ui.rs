use crate::app::App;
use crate::lua_runner::{self, Outcome};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).areas(area);
    let [output_area, hint_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(detail_area);

    render_exercise_list(frame, list_area, app);
    render_output_panel(frame, output_area, app);
    render_hint_panel(frame, hint_area, app);
}

fn render_exercise_list(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .exercises()
        .iter()
        .map(|exercise| {
            let done = app.is_done(&exercise.path);
            let checkbox = if done { "[x]" } else { "[ ]" };
            let item = ListItem::new(format!("{checkbox} {}", exercise.name));
            if done { item.fg(Color::Green) } else { item }
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("Exercises"))
        .highlight_style(Style::new().reversed());

    let mut state = ListState::default();
    state.select(Some(app.selected_index()));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_output_panel(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().title("Output");

    let paragraph = match app.last_run() {
        None => Paragraph::new("Select an exercise and save your changes to see the result here."),
        Some(last_run) => {
            let (status, color) = match &last_run.outcome {
                Outcome::Pass => ("[PASS]".to_string(), Color::Green),
                Outcome::Fail(message) => (format!("[FAIL] {message}"), Color::Red),
                Outcome::Timeout => (
                    format!(
                        "[TIMEOUT] exceeded {:?}",
                        lua_runner::DEFAULT_TIMEOUT_BUDGET
                    ),
                    Color::Yellow,
                ),
            };
            let mut lines = vec![Line::from(status).fg(color)];
            lines.extend(last_run.output.iter().cloned().map(Line::from));
            Paragraph::new(lines)
        }
    };

    frame.render_widget(paragraph.block(block).wrap(Wrap { trim: false }), area);
}
fn render_hint_panel(frame: &mut Frame, area: Rect, app: &App) {
    let paragraph = Paragraph::new(app.selected_exercise().hint.clone())
        .block(Block::bordered().title("Hint"))
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}
