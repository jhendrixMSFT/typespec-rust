// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use spector_multipart::{
    models::{
        Address, BinaryArrayPartsRequest, ComplexPartsRequest, JsonPartRequest,
        MultiBinaryPartsRequest, MultiPartOptionalRequest, MultiPartRequest,
        MultiPartRequestWithWireName,
    },
    multipart::Part,
    MultiPartClient,
};

const JPG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.jpg");
const PNG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.png");

fn create_client() -> spector_multipart::form_data::clients::MultiPartFormDataClient {
    MultiPartClient::with_no_credential("http://localhost:3000", None)
        .unwrap()
        .get_multi_part_form_data_client()
}

#[tokio::test]
async fn anonymous_model() {
    let client = create_client();
    let body = spector_multipart::form_data::models::AnonymousModelRequest {
        profile_image: Part::new(JPG_DATA.to_vec()),
    };
    client.anonymous_model(body, None).await.unwrap();
}

#[tokio::test]
async fn basic() {
    let client = create_client();
    let body = MultiPartRequest {
        id: "123".to_string(),
        profile_image: Part::new(JPG_DATA.to_vec()),
    };
    client.basic(body, None).await.unwrap();
}

#[tokio::test]
async fn binary_array_parts() {
    let client = create_client();
    let body = BinaryArrayPartsRequest {
        id: "123".to_string(),
        pictures: vec![Part::new(PNG_DATA.to_vec()), Part::new(PNG_DATA.to_vec())],
    };
    client.binary_array_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn check_file_name_and_content_type() {
    let client = create_client();
    let body = MultiPartRequest {
        id: "123".to_string(),
        profile_image: Part::new(JPG_DATA.to_vec())
            .content_type("image/jpg")
            .filename("hello.jpg"),
    };
    client
        .check_file_name_and_content_type(body, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn file_array_and_basic() {
    let client = create_client();
    let body = ComplexPartsRequest {
        id: "123".to_string(),
        address: Address {
            city: "X".to_string(),
        },
        profile_image: Part::new(JPG_DATA.to_vec()),
        pictures: vec![Part::new(PNG_DATA.to_vec()), Part::new(PNG_DATA.to_vec())],
    };
    client.file_array_and_basic(body, None).await.unwrap();
}

#[tokio::test]
async fn json_part() {
    let client = create_client();
    let body = JsonPartRequest {
        address: Address {
            city: "X".to_string(),
        },
        profile_image: Part::new(JPG_DATA.to_vec()),
    };
    client.json_part(body, None).await.unwrap();
}

#[tokio::test]
async fn multi_binary_parts_profile_image_only() {
    let client = create_client();
    let body = MultiBinaryPartsRequest {
        profile_image: Part::new(JPG_DATA.to_vec()),
        picture: None,
    };
    client.multi_binary_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn multi_binary_parts_with_picture() {
    let client = create_client();
    let body = MultiBinaryPartsRequest {
        profile_image: Part::new(JPG_DATA.to_vec()),
        picture: Some(Part::new(PNG_DATA.to_vec())),
    };
    client.multi_binary_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn optional_parts_both() {
    let client = create_client();
    let body = MultiPartOptionalRequest {
        id: Some("123".to_string()),
        profile_image: Some(Part::new(JPG_DATA.to_vec())),
    };
    client.optional_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn optional_parts_id_only() {
    let client = create_client();
    let body = MultiPartOptionalRequest {
        id: Some("123".to_string()),
        profile_image: None,
    };
    client.optional_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn optional_parts_profile_image_only() {
    let client = create_client();
    let body = MultiPartOptionalRequest {
        id: None,
        profile_image: Some(Part::new(JPG_DATA.to_vec())),
    };
    client.optional_parts(body, None).await.unwrap();
}

#[tokio::test]
async fn with_wire_name() {
    let client = create_client();
    let body = MultiPartRequestWithWireName {
        identifier: "123".to_string(),
        image: Part::new(JPG_DATA.to_vec()),
    };
    client.with_wire_name(body, None).await.unwrap();
}
