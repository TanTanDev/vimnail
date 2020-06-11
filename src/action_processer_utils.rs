use crate::action_processer::ActionProcesserBuilder;
use crate::action_type::ActionType;
use crate::direction::Direction;
use crate::edit_type::EditType;
use crate::input_action::InputAction;
use crate::item::ItemType;
use crate::mode::Mode;

use ggez::input::keyboard::KeyCode;

fn configure_modes(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_mode(Mode::Any)
        .with_mode(Mode::Command)
        .with_mode(Mode::Edit)
        .with_mode(Mode::Insert)
        .with_mode(Mode::InsertType(ItemType::Image))
        .with_mode(Mode::InsertType(ItemType::Text));
}

fn configure_command(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::Command,
            InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)),
        )
        .with_inputaction(
            Mode::Command,
            InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)),
        );
}

fn configure_edit(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::Edit,
            InputAction::new(KeyCode::I, ActionType::ChangeMode(Mode::Insert)),
        )
        .with_inputaction(
            Mode::Edit,
            InputAction::new(
                KeyCode::M,
                ActionType::ChangeMode(Mode::EditType(EditType::Move)),
            ),
        )
        .with_inputaction(
            Mode::Edit,
            InputAction::new(
                KeyCode::R,
                ActionType::ChangeMode(Mode::EditType(EditType::Rotate)),
            ),
        )
        .with_inputaction(
            Mode::Edit,
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Command)),
        )
        .with_inputaction(
            Mode::Edit,
            InputAction::new(
                KeyCode::S,
                ActionType::ChangeMode(Mode::EditType(EditType::Scale)),
            ),
        );
}

fn configure_insert(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::Insert,
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Command)),
        )
        .with_inputaction(
            Mode::Insert,
            InputAction::new(KeyCode::E, ActionType::ChangeMode(Mode::Edit)),
        )
        .with_inputaction(
            Mode::Insert,
            InputAction::new(
                KeyCode::I,
                ActionType::ChangeMode(Mode::InsertType(ItemType::Image)),
            ),
        )
        .with_inputaction(
            Mode::Insert,
            InputAction::new(
                KeyCode::T,
                ActionType::ChangeMode(Mode::InsertType(ItemType::Text)),
            ),
        )
        .with_inputaction(
            Mode::Insert,
            InputAction::new(
                KeyCode::P,
                ActionType::ChangeMode(Mode::InsertType(ItemType::ParticleSystem)),
            ),
        );
}

fn configure_insert_image(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::InsertType(ItemType::Image),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)),
        )
        .with_inputaction(
            Mode::InsertType(ItemType::Image),
            InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::Image)),
        )
        .with_inputaction(
            Mode::InsertType(ItemType::Image),
            InputAction::new(KeyCode::L, ActionType::ChangeMode(Mode::Edit)),
        );
}

fn configure_insert_text(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::InsertType(ItemType::Text),
            InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::Text)),
        )
        .with_inputaction(
            Mode::InsertType(ItemType::Text),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)),
        );
}

fn configure_particle_system(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::InsertType(ItemType::ParticleSystem),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Insert)),
        )
        .with_inputaction(
            Mode::InsertType(ItemType::ParticleSystem),
            InputAction::new(KeyCode::I, ActionType::AddItem(ItemType::ParticleSystem)),
        );
}

fn configure_edit_move(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(
                KeyCode::S,
                ActionType::ChangeMode(Mode::EditType(EditType::Scale)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(
                KeyCode::R,
                ActionType::ChangeMode(Mode::EditType(EditType::Rotate)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(KeyCode::H, ActionType::Move(Direction::Left)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(KeyCode::J, ActionType::Move(Direction::Down)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(KeyCode::K, ActionType::Move(Direction::Up)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Move),
            InputAction::new(KeyCode::L, ActionType::Move(Direction::Right)),
        );
}

fn configure_edit_rotate(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(
                KeyCode::M,
                ActionType::ChangeMode(Mode::EditType(EditType::Move)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(
                KeyCode::S,
                ActionType::ChangeMode(Mode::EditType(EditType::Scale)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(KeyCode::L, ActionType::Rotate(Direction::Right)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(KeyCode::H, ActionType::Rotate(Direction::Left)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(KeyCode::J, ActionType::Rotate(Direction::Down)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(KeyCode::K, ActionType::Rotate(Direction::Up)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Rotate),
            InputAction::new(
                KeyCode::M,
                ActionType::ChangeMode(Mode::EditType(EditType::Move)),
            ),
        );
}

fn configure_edit_scale(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(
                KeyCode::R,
                ActionType::ChangeMode(Mode::EditType(EditType::Rotate)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(
                KeyCode::M,
                ActionType::ChangeMode(Mode::EditType(EditType::Move)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(
                KeyCode::U,
                ActionType::ChangeMode(Mode::EditType(EditType::ScaleUniform)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(KeyCode::H, ActionType::Scale(Direction::Left)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(KeyCode::J, ActionType::Scale(Direction::Down)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(KeyCode::K, ActionType::Scale(Direction::Up)),
        )
        .with_inputaction(
            Mode::EditType(EditType::Scale),
            InputAction::new(KeyCode::L, ActionType::Scale(Direction::Right)),
        );
}

fn configure_edit_scale_uniform(action_processer_builder: &mut ActionProcesserBuilder) {
    action_processer_builder
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(
                KeyCode::R,
                ActionType::ChangeMode(Mode::EditType(EditType::Rotate)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(
                KeyCode::M,
                ActionType::ChangeMode(Mode::EditType(EditType::Move)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(
                KeyCode::S,
                ActionType::ChangeMode(Mode::EditType(EditType::Scale)),
            ),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(KeyCode::Escape, ActionType::ChangeMode(Mode::Edit)),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(KeyCode::H, ActionType::ScaleUniform(Direction::Left)),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(KeyCode::J, ActionType::ScaleUniform(Direction::Down)),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(KeyCode::K, ActionType::ScaleUniform(Direction::Up)),
        )
        .with_inputaction(
            Mode::EditType(EditType::ScaleUniform),
            InputAction::new(KeyCode::L, ActionType::ScaleUniform(Direction::Right)),
        );
}

pub fn configure_default(action_processer_builder: &mut ActionProcesserBuilder) {
    configure_modes(action_processer_builder);
    configure_command(action_processer_builder);
    configure_edit(action_processer_builder);
    configure_insert(action_processer_builder);
    configure_insert_image(action_processer_builder);
    configure_insert_text(action_processer_builder);
    configure_particle_system(action_processer_builder);
    configure_edit_move(action_processer_builder);
    configure_edit_rotate(action_processer_builder);
    configure_edit_scale(action_processer_builder);
    configure_edit_scale_uniform(action_processer_builder);
}
