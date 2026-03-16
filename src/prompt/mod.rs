/*
 * Copyright 2026 Luc Lenôtre
 *
 * This file is part of Maestro.
 *
 * Maestro is free software: you can redistribute it and/or modify it under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * Maestro is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * Maestro. If not, see <https://www.gnu.org/licenses/>.
 */

//! TODO doc

pub mod term;

use crate::install::{InstallInfo, InstallProgress};

/// Enumeration of installation steps.
#[derive(Clone, Copy)]
pub enum InstallStep {
	Welcome,
	Localization,
	SystemInfo,
	CreateAdmin,
	Partitions,
	Install,
	Finished,
}

impl InstallStep {
	/// Returns the number of the step.
	pub fn get_number(&self) -> u32 {
		match self {
			Self::Welcome => 0,
			Self::Localization => 1,
			Self::SystemInfo => 2,
			Self::CreateAdmin => 3,
			Self::Partitions => 4,
			Self::Install => 5,
			Self::Finished => 6,
		}
	}

	/// Returns the name of the step.
	pub fn get_name(&self) -> Option<&'static str> {
		match self {
			Self::Welcome => None,
			Self::Localization => Some("Localization"),
			Self::SystemInfo => Some("System informations"),
			Self::CreateAdmin => Some("Creating administrator user"),
			Self::Partitions => Some("Disk partitions"),
			Self::Install => Some("Installation"),
			Self::Finished => Some("Finished"),
		}
	}

	/// Returns the step next to the current.
	/// If this is the last step, the function returns None.
	pub fn get_next(&self) -> Option<Self> {
		match self {
			Self::Welcome => Some(Self::Localization),
			Self::Localization => Some(Self::SystemInfo),
			Self::SystemInfo => Some(Self::CreateAdmin),
			Self::CreateAdmin => Some(Self::Partitions),
			Self::Partitions => Some(Self::Install),
			Self::Install => Some(Self::Finished),
			Self::Finished => None,
		}
	}
}

/// Trait to be implemented for each ways of ask the user for informations about the installation.
pub trait InstallPrompt {
	/// Returns the current step.
	/// If the function returns None, the installation is finished.
	fn get_current_step(&self) -> Option<InstallStep>;
	/// Prompts the next step.
	fn next_step(&mut self);

	/// Returns prompted informations.
	fn get_infos(&self) -> InstallInfo;

	/// Updates the current progress of the installation.
	fn update_progress(&mut self, progress: &InstallProgress);
}
