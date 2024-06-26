// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Contains an interface for enumerating assets in existence or owned by a given account.
//!
//! See the [`crate::traits::fungibles`] doc for more information about fungibles traits.

/// Interface for enumerating assets in existence or owned by a given account.
pub trait Inspect<AccountId>: super::Inspect<AccountId> {
	type AssetsIterator;

	/// Returns an iterator of the collections in existence.
	fn asset_ids() -> Self::AssetsIterator;
}
