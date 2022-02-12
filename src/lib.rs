/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//! Dog API client
mod error;
mod dogapi;
pub use error::DogAPIError;
pub use dogapi::random_image;
pub use dogapi::multiple_random_images;
pub use dogapi::random_image_by_breed;
pub use dogapi::multiple_random_images_by_breed;
pub use dogapi::random_image_by_sub_breed;
pub use dogapi::multiple_random_images_by_sub_breed;
pub use dogapi::images_by_breed;
pub use dogapi::images_by_sub_breed;
pub use dogapi::breeds_list;
pub use dogapi::sub_breeds_list;