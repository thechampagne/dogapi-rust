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
use std::collections::HashMap;
use std::io::Read;
use crate::error::DogAPIError;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    message: String,
    status: String
}

fn get_request(endpoint: &str) -> Result<String, String> {
    match reqwest::blocking::Client::new().get(format!("https://dog.ceo/api/{}", endpoint))
        .send() {
        Ok(mut response) => {
            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Ok(_) => Ok(body),
                Err(err) => Err(format!("Something went wrong while reading bytes: {}", err))
            }
        },
        Err(err) => Err(err.to_string())
    }
}

/// DISPLAY SINGLE RANDOM IMAGE FROM ALL DOGS COLLECTION
///
/// Returns a random dog image
pub fn random_image() -> Result<String, DogAPIError> {
    match get_request("breeds/image/random") {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Response = json;
                    if data.status != "success" {
                        Err(DogAPIError::Error(data.message))
                    } else {
                        Ok(data.message)
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// DISPLAY MULTIPLE RANDOM IMAGES FROM ALL DOGS COLLECTION
///
/// * `images_number` number of images
///
/// *NOTE* ~ Max number returned is 50
///
/// Return multiple random dog image
pub fn multiple_random_images(images_number: i8) -> Result<Vec<String>, DogAPIError> {
    match get_request(&format!("breeds/image/random/{}", images_number)) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            let mut vector = vec![];
                                            for i in array.iter() {
                                                if let Some(url) = i.as_str() {
                                                    vector.push(url.to_string());
                                                }
                                            }
                                            Ok(vector)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// RANDOM IMAGE FROM A BREED COLLECTION
///
/// * `breed` breed name
///
/// Returns a random dog image from a breed, e.g. hound
pub fn random_image_by_breed(breed: &str) -> Result<String, DogAPIError> {
    match get_request(&format!("breed/{}/images/random", breed.trim())) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Response = json;
                    if data.status != "success" {
                        Err(DogAPIError::Error(data.message))
                    } else {
                        Ok(data.message)
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// MULTIPLE IMAGES FROM A BREED COLLECTION
///
/// * `breed` breed name
/// * `images_number` number of images
///
/// Return multiple random dog image from a breed, e.g. hound
pub fn multiple_random_images_by_breed(breed: &str, images_number: i64) -> Result<Vec<String>, DogAPIError> {
    match get_request(&format!("breed/{}/images/random/{}", breed.trim(), images_number)) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            let mut vector = vec![];
                                            for i in array.iter() {
                                                if let Some(url) = i.as_str() {
                                                    vector.push(url.to_string());
                                                }
                                            }
                                            Ok(vector)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// ALL IMAGES FROM A BREED COLLECTION
///
/// * `breed` breed name
///
/// Returns an array of all the images from a breed, e.g. hound
pub fn images_by_breed(breed: &str) -> Result<Vec<String>, DogAPIError> {
    match get_request(&format!("breed/{}/images", breed.trim())) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            let mut vector = vec![];
                                            for i in array.iter() {
                                                if let Some(url) = i.as_str() {
                                                    vector.push(url.to_string());
                                                }
                                            }
                                            Ok(vector)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// SINGLE RANDOM IMAGE FROM A SUB BREED COLLECTION
///
/// * `breed` breed name
/// * `sub_breed` sub_breed name
///
/// Returns a random dog image from a sub-breed, e.g. Afghan Hound
pub fn random_image_by_sub_breed(breed: &str, sub_breed: &str) -> Result<String, DogAPIError> {
    match get_request(&format!("breed/{}/{}/images/random", breed.trim(), sub_breed.trim())) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Response = json;
                    if data.status != "success" {
                        Err(DogAPIError::Error(data.message))
                    } else {
                        Ok(data.message)
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// MULTIPLE IMAGES FROM A SUB-BREED COLLECTION
///
/// * `breed` breed name
/// * `sub_breed` sub_breed name
/// * `images_number` number of images
///
/// Return multiple random dog images from a sub-breed, e.g. Afghan Hound
pub fn multiple_random_images_by_sub_breed(breed: &str, sub_breed: &str, images_number: i64) -> Result<Vec<String>, DogAPIError> {
    match get_request(&format!("breed/{}/{}/images/random/{}", breed.trim(), sub_breed.trim(),images_number)) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            let mut vector = vec![];
                                            for i in array.iter() {
                                                if let Some(url) = i.as_str() {
                                                    vector.push(url.to_string());
                                                }
                                            }
                                            Ok(vector)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// LIST ALL SUB-BREED IMAGES
///
/// * `breed` breed name
/// * `sub_breed` sub_breed name
///
/// Returns an array of all the images from the sub-breed
pub fn images_by_sub_breed(breed: &str, sub_breed: &str) -> Result<Vec<String>, DogAPIError> {
    match get_request(&format!("breed/{}/{}/images", breed.trim(), sub_breed.trim())) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            let mut vector = vec![];
                                            for i in array.iter() {
                                                if let Some(url) = i.as_str() {
                                                    vector.push(url.to_string());
                                                }
                                            }
                                            Ok(vector)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// LIST ALL BREEDS
///
/// Returns map of all the breeds as keys and sub-breeds as values if it has
pub fn breeds_list() -> Result<HashMap<String, Option<Vec<String>>>, DogAPIError> {
    match get_request("breeds/list/all") {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_object() {
                                        Some(obj) => {
                                            let mut map= HashMap::new();
                                            for (i,v) in obj {
                                                if let Some(value) = v.as_array() {
                                                    if value.is_empty() {
                                                        map.insert(i.to_string(),None);
                                                        continue
                                                    }
                                                    let mut vector = vec![];
                                                    for val in value.iter() {
                                                        if let Some(value) = val.as_str() {
                                                            vector.push(value.to_string())
                                                        }
                                                    }
                                                    map.insert(i.to_string(),Some(vector));
                                                }
                                            }
                                            Ok(map)
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}

/// LIST ALL SUB-BREEDS
///
/// * `breed` breed name
///
/// Returns an array of all the sub-breeds from a breed if it has sub-breeds
pub fn sub_breeds_list(breed: &str) -> Result<Option<Vec<String>>, DogAPIError> {
    match get_request(&format!("breed/{}/list", breed.trim())) {
        Ok(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("status") {
                        Some(status) => match data.get("message") {
                            Some(message) => match status.as_str() {
                                Some(status_str) => if status_str != "success" {
                                    let error = match message.as_str() {
                                        Some(err) => err,
                                        None => "Something went wrong while reading json"
                                    };
                                    Err(DogAPIError::Error(error.to_string()))
                                } else {
                                    match message.as_array() {
                                        Some(array) => {
                                            if array.is_empty() {
                                                Ok(None)
                                            } else {
                                                let mut vector = vec![];
                                                for i in array.iter() {
                                                    if let Some(url) = i.as_str() {
                                                        vector.push(url.to_string());
                                                    }
                                                }
                                                Ok(Some(vector))
                                            }
                                        },
                                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                                    }
                                },
                                None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                            },
                            None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                        },
                        None => Err(DogAPIError::Error(String::from("Something went wrong while reading json")))
                    }
                },
                Err(err) => Err(DogAPIError::Error(format!("Something went wrong while reading json: {}", err.to_string())))
            }
        },
        Err(err) => Err(DogAPIError::Error(String::from(err)))
    }
}