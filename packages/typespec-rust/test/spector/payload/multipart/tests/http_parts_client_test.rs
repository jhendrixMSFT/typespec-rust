// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use spector_multipart::{
    models::{Address, ComplexHttpPartsModelRequest, FileRequiredMetaData},
    MultiPartClient,
};

const JPG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.jpg");
const PNG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.png");

fn create_client(
) -> spector_multipart::form_data::http_parts::clients::MultiPartFormDataHttpPartsClient {
    MultiPartClient::with_no_credential("http://localhost:3000", None)
        .unwrap()
        .get_multi_part_form_data_client()
        .get_multi_part_form_data_http_parts_client()
}

#[tokio::test]
async fn json_array_and_file_array() {
    let client = create_client();
    let body = ComplexHttpPartsModelRequest {
        id: "123".to_string(),
        address: Address {
            city: "X".to_string(),
        },
        profile_image: FileRequiredMetaData {
            contents: JPG_DATA.to_vec(),
            filename: "hello.jpg".to_string(),
            content_type: "application/octet-stream".to_string(),
        },
        previous_addresses: vec![
            Address {
                city: "Y".to_string(),
            },
            Address {
                city: "Z".to_string(),
            },
        ],
        pictures: vec![
            FileRequiredMetaData {
                contents: PNG_DATA.to_vec(),
                filename: "image.png".to_string(),
                content_type: "application/octet-stream".to_string(),
            },
            FileRequiredMetaData {
                contents: PNG_DATA.to_vec(),
                filename: "image.png".to_string(),
                content_type: "application/octet-stream".to_string(),
            },
        ],
    };
    client.json_array_and_file_array(body, None).await.unwrap();
}
