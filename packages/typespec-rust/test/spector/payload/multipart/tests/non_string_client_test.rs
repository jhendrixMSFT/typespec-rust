// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use spector_multipart::{form_data::http_parts::non_string::models::FloatRequest, MultiPartClient};

fn create_client() -> spector_multipart::form_data::http_parts::non_string::clients::MultiPartFormDataHttpPartsNonStringClient{
    MultiPartClient::with_no_credential("http://localhost:3000", None)
        .unwrap()
        .get_multi_part_form_data_client()
        .get_multi_part_form_data_http_parts_client()
        .get_multi_part_form_data_http_parts_non_string_client()
}

#[tokio::test]
async fn float() {
    let client = create_client();
    let body = FloatRequest { temperature: 0.5 };
    client.float(body, None).await.unwrap();
}
