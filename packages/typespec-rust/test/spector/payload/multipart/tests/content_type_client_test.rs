// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use spector_multipart::{
    models::{
        FileOptionalContentType, FileRequiredMetaData, FileSpecificContentType,
        FileWithHttpPartOptionalContentTypeRequest, FileWithHttpPartRequiredContentTypeRequest,
        FileWithHttpPartSpecificContentTypeRequest,
    },
    MultiPartClient,
};

const JPG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.jpg");

fn create_client() -> spector_multipart::form_data::http_parts::content_type::clients::MultiPartFormDataHttpPartsContentTypeClient{
    MultiPartClient::with_no_credential("http://localhost:3000", None)
        .unwrap()
        .get_multi_part_form_data_client()
        .get_multi_part_form_data_http_parts_client()
        .get_multi_part_form_data_http_parts_content_type_client()
}

#[tokio::test]
async fn image_jpeg_content_type() {
    let client = create_client();
    let body = FileWithHttpPartSpecificContentTypeRequest {
        profile_image: FileSpecificContentType {
            contents: JPG_DATA.to_vec(),
            filename: "hello.jpg".to_string(),
        },
    };
    client.image_jpeg_content_type(body, None).await.unwrap();
}

#[tokio::test]
async fn optional_content_type() {
    let client = create_client();
    let body = FileWithHttpPartOptionalContentTypeRequest {
        profile_image: FileOptionalContentType {
            contents: JPG_DATA.to_vec(),
            filename: "hello.jpg".to_string(),
            content_type: None,
        },
    };
    client.optional_content_type(body, None).await.unwrap();
}

#[tokio::test]
async fn required_content_type() {
    let client = create_client();
    let body = FileWithHttpPartRequiredContentTypeRequest {
        profile_image: FileRequiredMetaData {
            contents: JPG_DATA.to_vec(),
            filename: "hello.jpg".to_string(),
            content_type: "application/octet-stream".to_string(),
        },
    };
    client.required_content_type(body, None).await.unwrap();
}
