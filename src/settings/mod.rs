pub mod registry;
pub mod widgets;

use eframe::egui;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChangeStatus {
    #[default]
    Unchanged = 0,
    Changed = 1,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SettingKind {
    None = 0,
}

pub trait Setting<Ctx>: Sized + Clone + PartialEq<Self> {
    fn kind(&self) -> SettingKind;
    // TODO: Need Context<Ctx> type to store user context.
    /// A type that implements `Setting` should be renderable in a Settings GUI.
    /// In a Settings GUI, 
    /// # Parameters:
    /// - `current`:
    fn render(&mut self, current: &Self, context: &mut Ctx, ui: &mut egui::Ui) -> ChangeStatus;
}


pub struct Settings {
    
}