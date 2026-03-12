// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use spector_multipart::{
    form_data::file::models::{
        FileWithRequiredFilename, UploadFileArrayRequest, UploadFileRequiredFilenameRequest,
        UploadFileSpecificContentTypeRequest,
    },
    models::File1,
    MultiPartClient,
};

const PNG_DATA: &[u8] = include_bytes!("../../../../../node_modules/@typespec/http-specs/assets/image.png");

fn create_client() -> spector_multipart::form_data::file::clients::MultiPartFormDataFileClient {
    MultiPartClient::with_no_credential("http://localhost:3000", None)
        .unwrap()
        .get_multi_part_form_data_client()
        .get_multi_part_form_data_file_client()
}

#[tokio::test]
async fn upload_file_array() {
    let client = create_client();
    let body = UploadFileArrayRequest {
        files: vec![
            File1 {
                contents: PNG_DATA.to_vec(),
                filename: Some("image.png".to_string()),
            },
            File1 {
                contents: PNG_DATA.to_vec(),
                filename: Some("image.png".to_string()),
            },
        ],
    };
    client.upload_file_array(body, None).await.unwrap();
}

#[tokio::test]
async fn upload_file_required_filename() {
    let client = create_client();
    let body = UploadFileRequiredFilenameRequest {
        file: FileWithRequiredFilename {
            contents: PNG_DATA.to_vec(),
            filename: "image.png".to_string(),
        },
    };
    client
        .upload_file_required_filename(body, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn upload_file_specific_content_type() {
    let client = create_client();
    let body = UploadFileSpecificContentTypeRequest {
        file: File1 {
            contents: PNG_DATA.to_vec(),
            filename: Some("image.png".to_string()),
        },
    };
    client
        .upload_file_specific_content_type(body, None)
        .await
        .unwrap();
}
