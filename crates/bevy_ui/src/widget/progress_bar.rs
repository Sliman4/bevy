use std::ops::{AddAssign, Deref};

use bevy_ecs::prelude::Component;
use bevy_ecs::query::Changed;
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::system::Query;
use bevy_reflect::Reflect;

use crate::{Style, Val};

/// Progress struct for ProgressBar.
/// ```
/// # use bevy::prelude::Progress;
/// # use bevy_ui::Val;
///
/// let mut progress_bar = Progress::new(25.0);
///
/// *progress_bar += 50.0;
///
/// let progress_bar_width = Val::Percent(*progress_bar);
/// ```
///
/// Note: values will be clamped between 0.0 and 100.0
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Progress(f32);

impl Deref for Progress {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Progress {
    /// Creates a new instance of [`Progress`]
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// // 100%
    /// let progress_bar = Progress::new(100.0);
    /// ```
    pub fn new(value: f32) -> Self {
        Progress(Self::clamp_value(value))
    }

    /// Creates a new instance of [`Progress`] with 0% done
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// let empty_progress_bar = Progress::empty();
    /// assert!(*empty_progress_bar, 0.0);
    /// ```
    pub fn empty() -> Self {
        Self::new(0.0)
    }

    pub fn set(&mut self, value: f32) {
        self.0 = Self::clamp_value(value)
    }

    /// Check if this [`Progress`] has reached 100%
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// let mut progress_bar = Progress::new(50.0);
    /// assert!(!empty_progress_bar.is_done());
    ///
    /// *progress_bar += 50.0;
    /// assert!(empty_progress_bar.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        (self.0 - 100.0).abs() < f32::EPSILON
    }

    fn clamp_value(value: f32) -> f32 {
        value.clamp(0.0, 100.0)
    }
}

impl AddAssign<f32> for Progress {
    fn add_assign(&mut self, rhs: f32) {
        self.set(**self + rhs)
    }
}

/// Specifies progress bar's animation
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ProgressBarAnimation {
    /// The width of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    ResizeWidth,
    /// The height of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    ResizeHeight,
    /// Both the width and the height of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    ResizeBothDimensions,
    /// A node's size won't be changed when the [`Progress`] changes. Use this if you want to
    /// implement it yourself.
    ///
    /// # Example
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// # use bevy_ecs::query::{With, Changed};
    /// # use bevy_ecs::system::Commands;
    /// # use bevy_math::Size;
    /// # use bevy_render::color::Color;
    /// # use bevy_ui::widget::{Progress, ProgressBarAnimation};
    /// # use bevy_ui::entity::{ProgressBarBundle, UiCameraBundle};
    /// # use bevy_ui::{Style, UiColor, Val};
    ///
    /// #[derive(Component)]
    /// struct MyProgressBarAnimation;
    ///
    /// fn setup(mut commands: Commands) {
    ///     commands.spawn_bundle(UiCameraBundle::default());
    ///
    ///     commands.spawn_bundle(ProgressBarBundle {
    ///         style: Style {
    ///             size: Size::new(Val::Px(100.0), Val::Px(100.0)),
    ///             ..Default::default()
    ///         },
    ///         resize_animation: ProgressBarAnimation::Custom,
    ///         ..Default::default()
    ///     }).insert(MyProgressBarAnimation);
    /// }
    ///
    /// fn animation_system(mut query: Query<(&Progress, &mut UiColor), (With<MyProgressBarAnimation>, Changed<Progress>)>) {
    ///     for (progress, mut color) in query.iter_mut() {
    ///         // change hue from 0 to 100 (from red to green)
    ///         color.0 = Color::Hsla {
    ///             hue: *progress,
    ///             saturation: 0.7,
    ///             lightness: 0.5,
    ///             alpha: 1.0,
    ///         };
    ///     }
    /// }
    /// ```
    Custom,
}

impl Default for ProgressBarAnimation {
    fn default() -> Self {
        ProgressBarAnimation::ResizeWidth
    }
}

/// Updates progress bar [`Size`] if [`Progress`] has changed
pub fn progress_bar_animation_system(
    mut query: Query<(&Progress, &ProgressBarAnimation, &mut Style), Changed<Progress>>,
) {
    for (progress, dimension, mut style) in query.iter_mut() {
        let (resize_width, resize_height) = match dimension {
            ProgressBarAnimation::ResizeWidth => (true, false),
            ProgressBarAnimation::ResizeHeight => (false, true),
            ProgressBarAnimation::ResizeBothDimensions => (true, true),
            ProgressBarAnimation::Custom => (false, false),
        };
        if resize_width {
            style.size.width = Val::Percent(**progress);
        }
        if resize_height {
            style.size.height = Val::Percent(**progress);
        }
    }
}
