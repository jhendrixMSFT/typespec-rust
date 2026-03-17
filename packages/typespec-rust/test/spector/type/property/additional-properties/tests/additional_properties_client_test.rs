// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use spector_addl_props::{
    models::{
        DifferentSpreadFloatDerived, DifferentSpreadFloatRecord, DifferentSpreadModelArrayDerived,
        DifferentSpreadModelArrayRecord, DifferentSpreadModelDerived, DifferentSpreadModelRecord,
        DifferentSpreadStringDerived, DifferentSpreadStringRecord,
        ExtendsFloatAdditionalProperties, ExtendsModelAdditionalProperties,
        ExtendsModelArrayAdditionalProperties, ExtendsStringAdditionalProperties,
        ExtendsUnknownAdditionalProperties, ExtendsUnknownAdditionalPropertiesDerived,
        ExtendsUnknownAdditionalPropertiesDiscriminated,
        ExtendsUnknownAdditionalPropertiesDiscriminatedDerived, IsFloatAdditionalProperties,
        IsModelAdditionalProperties, IsModelArrayAdditionalProperties,
        IsStringAdditionalProperties, IsUnknownAdditionalProperties,
        IsUnknownAdditionalPropertiesDerived, IsUnknownAdditionalPropertiesDiscriminated,
        IsUnknownAdditionalPropertiesDiscriminatedDerived, ModelForRecord, MultipleSpreadRecord,
        MultipleSpreadRecordAdditionalProperty, SpreadFloatRecord, SpreadModelArrayRecord,
        SpreadModelRecord, SpreadRecordForNonDiscriminatedUnion,
        SpreadRecordForNonDiscriminatedUnion2, SpreadRecordForNonDiscriminatedUnion3,
        SpreadRecordForNonDiscriminatedUnion2AdditionalProperty,
        SpreadRecordForNonDiscriminatedUnion3AdditionalProperty,
        SpreadRecordForNonDiscriminatedUnionAdditionalProperty, SpreadRecordForUnion,
        SpreadRecordForUnionAdditionalProperty, SpreadStringRecord, WidgetData0, WidgetData1,
        WidgetData2,
    },
    AdditionalPropertiesClient,
};
use std::collections::HashMap;

fn create_client() -> AdditionalPropertiesClient {
    AdditionalPropertiesClient::with_no_credential("http://localhost:3000", None).unwrap()
}

// --- ExtendsString ---

#[tokio::test]
async fn extends_different_spread_float_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_float_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadFloatDerived = resp.into_model().unwrap();
    assert_eq!(value.name, Some("abc".to_string()));
    assert_eq!(value.derived_prop, Some(43.125));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], 43.125);
}

#[tokio::test]
async fn extends_different_spread_float_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_float_client();
    let body = DifferentSpreadFloatDerived {
        name: Some("abc".to_string()),
        derived_prop: Some(43.125),
        additional_properties: Some(HashMap::from([("prop".to_string(), 43.125)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_different_spread_model_array_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_model_array_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadModelArrayDerived = resp.into_model().unwrap();
    assert_eq!(value.known_prop, Some("abc".to_string()));
    let derived = value.derived_prop.unwrap();
    assert_eq!(derived.len(), 2);
    assert_eq!(derived[0].state, Some("ok".to_string()));
    assert_eq!(derived[1].state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    let prop_val = &props["prop"];
    assert_eq!(prop_val.len(), 2);
    assert_eq!(prop_val[0].state, Some("ok".to_string()));
    assert_eq!(prop_val[1].state, Some("ok".to_string()));
}

#[tokio::test]
async fn extends_different_spread_model_array_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_model_array_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = DifferentSpreadModelArrayDerived {
        known_prop: Some("abc".to_string()),
        derived_prop: Some(vec![model.clone(), model.clone()]),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            vec![model.clone(), model.clone()],
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_different_spread_model_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_model_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadModelDerived = resp.into_model().unwrap();
    assert_eq!(value.known_prop, Some("abc".to_string()));
    assert_eq!(value.derived_prop.unwrap().state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"].state, Some("ok".to_string()));
}

#[tokio::test]
async fn extends_different_spread_model_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_model_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = DifferentSpreadModelDerived {
        known_prop: Some("abc".to_string()),
        derived_prop: Some(model.clone()),
        additional_properties: Some(HashMap::from([("prop".to_string(), model)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_different_spread_string_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_string_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadStringDerived = resp.into_model().unwrap();
    assert_eq!(value.id, Some(43.125));
    assert_eq!(value.derived_prop, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], "abc");
}

#[tokio::test]
async fn extends_different_spread_string_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_different_spread_string_client();
    let body = DifferentSpreadStringDerived {
        id: Some(43.125),
        derived_prop: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([("prop".to_string(), "abc".to_string())])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_float_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_float_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsFloatAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.id, Some(43.125));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], 43.125);
}

#[tokio::test]
async fn extends_float_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_float_client();
    let body = ExtendsFloatAdditionalProperties {
        id: Some(43.125),
        additional_properties: Some(HashMap::from([("prop".to_string(), 43.125)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_model_array_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_model_array_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsModelArrayAdditionalProperties = resp.into_model().unwrap();
    let known = value.known_prop.unwrap();
    assert_eq!(known.len(), 2);
    assert_eq!(known[0].state, Some("ok".to_string()));
    assert_eq!(known[1].state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    let prop_val = &props["prop"];
    assert_eq!(prop_val.len(), 2);
    assert_eq!(prop_val[0].state, Some("ok".to_string()));
    assert_eq!(prop_val[1].state, Some("ok".to_string()));
}

#[tokio::test]
async fn extends_model_array_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_model_array_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = ExtendsModelArrayAdditionalProperties {
        known_prop: Some(vec![model.clone(), model.clone()]),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            vec![model.clone(), model.clone()],
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_model_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_model_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsModelAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.known_prop.unwrap().state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"].state, Some("ok".to_string()));
}

#[tokio::test]
async fn extends_model_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_model_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = ExtendsModelAdditionalProperties {
        known_prop: Some(model.clone()),
        additional_properties: Some(HashMap::from([("prop".to_string(), model)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_string_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_string_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsStringAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.name, Some("ExtendsStringAdditionalProperties".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], "abc");
}

#[tokio::test]
async fn extends_string_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_string_client();
    let body = ExtendsStringAdditionalProperties {
        name: Some("ExtendsStringAdditionalProperties".to_string()),
        additional_properties: Some(HashMap::from([("prop".to_string(), "abc".to_string())])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_unknown_derived_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_derived_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsUnknownAdditionalPropertiesDerived = resp.into_model().unwrap();
    assert_eq!(value.name, Some("ExtendsUnknownAdditionalProperties".to_string()));
    assert_eq!(value.index, Some(314));
    assert_eq!(value.age, Some(2.71875));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop1"], 32);
    assert_eq!(props["prop2"], true);
    assert_eq!(props["prop3"], "abc");
}

#[tokio::test]
async fn extends_unknown_derived_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_derived_client();
    let body = ExtendsUnknownAdditionalPropertiesDerived {
        name: Some("ExtendsUnknownAdditionalProperties".to_string()),
        index: Some(314),
        age: Some(2.71875),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_unknown_discriminated_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_discriminated_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsUnknownAdditionalPropertiesDiscriminated = resp.into_model().unwrap();
    match value {
        ExtendsUnknownAdditionalPropertiesDiscriminated::ExtendsUnknownAdditionalPropertiesDiscriminatedDerived(derived) => {
            assert_eq!(derived.name, Some("Derived".to_string()));
            assert_eq!(derived.index, Some(314));
            assert_eq!(derived.age, Some(2.71875));
            let props = derived.additional_properties.unwrap();
            assert_eq!(props["prop1"], 32);
            assert_eq!(props["prop2"], true);
            assert_eq!(props["prop3"], "abc");
        }
        _ => panic!("Expected ExtendsUnknownAdditionalPropertiesDiscriminatedDerived variant"),
    }
}

#[tokio::test]
async fn extends_unknown_discriminated_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_discriminated_client();
    let derived = ExtendsUnknownAdditionalPropertiesDiscriminatedDerived {
        name: Some("Derived".to_string()),
        index: Some(314),
        age: Some(2.71875),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    let body: ExtendsUnknownAdditionalPropertiesDiscriminated = derived.into();
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn extends_unknown_get() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_client();
    let resp = sub.get(None).await.unwrap();
    let value: ExtendsUnknownAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.name, Some("ExtendsUnknownAdditionalProperties".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop1"], 32);
    assert_eq!(props["prop2"], true);
    assert_eq!(props["prop3"], "abc");
}

#[tokio::test]
async fn extends_unknown_put() {
    let client = create_client();
    let sub = client.get_additional_properties_extends_unknown_client();
    let body = ExtendsUnknownAdditionalProperties {
        name: Some("ExtendsUnknownAdditionalProperties".to_string()),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_float_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_float_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsFloatAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.id, Some(43.125));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], 43.125);
}

#[tokio::test]
async fn is_float_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_float_client();
    let body = IsFloatAdditionalProperties {
        id: Some(43.125),
        additional_properties: Some(HashMap::from([("prop".to_string(), 43.125)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_model_array_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_model_array_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsModelArrayAdditionalProperties = resp.into_model().unwrap();
    let known = value.known_prop.unwrap();
    assert_eq!(known.len(), 2);
    assert_eq!(known[0].state, Some("ok".to_string()));
    assert_eq!(known[1].state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    let prop_val = &props["prop"];
    assert_eq!(prop_val.len(), 2);
    assert_eq!(prop_val[0].state, Some("ok".to_string()));
    assert_eq!(prop_val[1].state, Some("ok".to_string()));
}

#[tokio::test]
async fn is_model_array_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_model_array_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = IsModelArrayAdditionalProperties {
        known_prop: Some(vec![model.clone(), model.clone()]),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            vec![model.clone(), model.clone()],
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_model_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_model_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsModelAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.known_prop.unwrap().state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"].state, Some("ok".to_string()));
}

#[tokio::test]
async fn is_model_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_model_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = IsModelAdditionalProperties {
        known_prop: Some(model.clone()),
        additional_properties: Some(HashMap::from([("prop".to_string(), model)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_string_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_string_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsStringAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.name, Some("IsStringAdditionalProperties".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], "abc");
}

#[tokio::test]
async fn is_string_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_string_client();
    let body = IsStringAdditionalProperties {
        name: Some("IsStringAdditionalProperties".to_string()),
        additional_properties: Some(HashMap::from([("prop".to_string(), "abc".to_string())])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_unknown_derived_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_derived_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsUnknownAdditionalPropertiesDerived = resp.into_model().unwrap();
    assert_eq!(value.name, Some("IsUnknownAdditionalProperties".to_string()));
    assert_eq!(value.index, Some(314));
    assert_eq!(value.age, Some(2.71875));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop1"], 32);
    assert_eq!(props["prop2"], true);
    assert_eq!(props["prop3"], "abc");
}

#[tokio::test]
async fn is_unknown_derived_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_derived_client();
    let body = IsUnknownAdditionalPropertiesDerived {
        name: Some("IsUnknownAdditionalProperties".to_string()),
        index: Some(314),
        age: Some(2.71875),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_unknown_discriminated_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_discriminated_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsUnknownAdditionalPropertiesDiscriminated = resp.into_model().unwrap();
    match value {
        IsUnknownAdditionalPropertiesDiscriminated::IsUnknownAdditionalPropertiesDiscriminatedDerived(derived) => {
            assert_eq!(derived.name, Some("Derived".to_string()));
            assert_eq!(derived.index, Some(314));
            assert_eq!(derived.age, Some(2.71875));
            let props = derived.additional_properties.unwrap();
            assert_eq!(props["prop1"], 32);
            assert_eq!(props["prop2"], true);
            assert_eq!(props["prop3"], "abc");
        }
        _ => panic!("Expected IsUnknownAdditionalPropertiesDiscriminatedDerived variant"),
    }
}

#[tokio::test]
async fn is_unknown_discriminated_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_discriminated_client();
    let derived = IsUnknownAdditionalPropertiesDiscriminatedDerived {
        name: Some("Derived".to_string()),
        index: Some(314),
        age: Some(2.71875),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    let body: IsUnknownAdditionalPropertiesDiscriminated = derived.into();
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn is_unknown_get() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_client();
    let resp = sub.get(None).await.unwrap();
    let value: IsUnknownAdditionalProperties = resp.into_model().unwrap();
    assert_eq!(value.name, Some("IsUnknownAdditionalProperties".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop1"], 32);
    assert_eq!(props["prop2"], true);
    assert_eq!(props["prop3"], "abc");
}

#[tokio::test]
async fn is_unknown_put() {
    let client = create_client();
    let sub = client.get_additional_properties_is_unknown_client();
    let body = IsUnknownAdditionalProperties {
        name: Some("IsUnknownAdditionalProperties".to_string()),
        additional_properties: Some(HashMap::from([
            ("prop1".to_string(), 32.into()),
            ("prop2".to_string(), true.into()),
            ("prop3".to_string(), "abc".into()),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn multiple_spread_get() {
    let client = create_client();
    let sub = client.get_additional_properties_multiple_spread_client();
    let resp = sub.get(None).await.unwrap();
    let value: MultipleSpreadRecord = resp.into_model().unwrap();
    assert_eq!(value.flag, Some(true));
    let props = value.additional_properties.unwrap();
    assert!(matches!(
        &props["prop1"],
        MultipleSpreadRecordAdditionalProperty::String(s) if s == "abc"
    ));
    assert!(matches!(
        &props["prop2"],
        MultipleSpreadRecordAdditionalProperty::Float32(f) if *f == 43.125
    ));
}

#[tokio::test]
async fn multiple_spread_put() {
    let client = create_client();
    let sub = client.get_additional_properties_multiple_spread_client();
    let body = MultipleSpreadRecord {
        flag: Some(true),
        additional_properties: Some(HashMap::from([
            (
                "prop1".to_string(),
                MultipleSpreadRecordAdditionalProperty::String("abc".to_string()),
            ),
            (
                "prop2".to_string(),
                MultipleSpreadRecordAdditionalProperty::Float32(43.125),
            ),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_different_float_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_float_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadFloatRecord = resp.into_model().unwrap();
    assert_eq!(value.name, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], 43.125);
}

#[tokio::test]
async fn spread_different_float_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_float_client();
    let body = DifferentSpreadFloatRecord {
        name: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([("prop".to_string(), 43.125)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_different_model_array_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_model_array_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadModelArrayRecord = resp.into_model().unwrap();
    assert_eq!(value.known_prop, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    let prop_val = &props["prop"];
    assert_eq!(prop_val.len(), 2);
    assert_eq!(prop_val[0].state, Some("ok".to_string()));
    assert_eq!(prop_val[1].state, Some("ok".to_string()));
}

#[tokio::test]
async fn spread_different_model_array_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_model_array_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = DifferentSpreadModelArrayRecord {
        known_prop: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            vec![model.clone(), model.clone()],
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_different_model_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_model_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadModelRecord = resp.into_model().unwrap();
    assert_eq!(value.known_prop, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"].state, Some("ok".to_string()));
}

#[tokio::test]
async fn spread_different_model_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_model_client();
    let body = DifferentSpreadModelRecord {
        known_prop: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            ModelForRecord {
                state: Some("ok".to_string()),
            },
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_different_string_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_string_client();
    let resp = sub.get(None).await.unwrap();
    let value: DifferentSpreadStringRecord = resp.into_model().unwrap();
    assert_eq!(value.id, Some(43.125));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], "abc");
}

#[tokio::test]
async fn spread_different_string_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_different_string_client();
    let body = DifferentSpreadStringRecord {
        id: Some(43.125),
        additional_properties: Some(HashMap::from([("prop".to_string(), "abc".to_string())])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_float_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_float_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadFloatRecord = resp.into_model().unwrap();
    assert_eq!(value.id, Some(43.125));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], 43.125);
}

#[tokio::test]
async fn spread_float_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_float_client();
    let body = SpreadFloatRecord {
        id: Some(43.125),
        additional_properties: Some(HashMap::from([("prop".to_string(), 43.125)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_model_array_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_model_array_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadModelArrayRecord = resp.into_model().unwrap();
    let known = value.known_prop.unwrap();
    assert_eq!(known.len(), 2);
    assert_eq!(known[0].state, Some("ok".to_string()));
    assert_eq!(known[1].state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    let prop_val = &props["prop"];
    assert_eq!(prop_val.len(), 2);
    assert_eq!(prop_val[0].state, Some("ok".to_string()));
    assert_eq!(prop_val[1].state, Some("ok".to_string()));
}

#[tokio::test]
async fn spread_model_array_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_model_array_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = SpreadModelArrayRecord {
        known_prop: Some(vec![model.clone(), model.clone()]),
        additional_properties: Some(HashMap::from([(
            "prop".to_string(),
            vec![model.clone(), model.clone()],
        )])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_model_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_model_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadModelRecord = resp.into_model().unwrap();
    assert_eq!(value.known_prop.unwrap().state, Some("ok".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"].state, Some("ok".to_string()));
}

#[tokio::test]
async fn spread_model_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_model_client();
    let model = ModelForRecord {
        state: Some("ok".to_string()),
    };
    let body = SpreadModelRecord {
        known_prop: Some(model.clone()),
        additional_properties: Some(HashMap::from([("prop".to_string(), model)])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_record_non_discriminated_union2_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union2_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadRecordForNonDiscriminatedUnion2 = resp.into_model().unwrap();
    assert_eq!(value.name, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    match &props["prop1"] {
        SpreadRecordForNonDiscriminatedUnion2AdditionalProperty::WidgetData2(data) => {
            assert_eq!(data.kind, Some("kind1".to_string()));
            assert_eq!(data.start, Some("2021-01-01T00:00:00Z".to_string()));
        }
        _ => panic!("Expected WidgetData2 for prop1"),
    }
    match &props["prop2"] {
        SpreadRecordForNonDiscriminatedUnion2AdditionalProperty::WidgetData1(data) => {
            assert_eq!(data.kind, Some("kind1".to_string()));
            assert!(data.start.is_some());
            assert!(data.end.is_some());
        }
        _ => panic!("Expected WidgetData1 for prop2"),
    }
}

#[tokio::test]
async fn spread_record_non_discriminated_union2_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union2_client();
    let body = SpreadRecordForNonDiscriminatedUnion2 {
        name: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([
            (
                "prop1".to_string(),
                SpreadRecordForNonDiscriminatedUnion2AdditionalProperty::WidgetData2(WidgetData2 {
                    kind: Some("kind1".to_string()),
                    start: Some("2021-01-01T00:00:00Z".to_string()),
                }),
            ),
            (
                "prop2".to_string(),
                SpreadRecordForNonDiscriminatedUnion2AdditionalProperty::WidgetData1(WidgetData1 {
                    kind: Some("kind1".to_string()),
                    start: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-01T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                    end: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-02T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                }),
            ),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_record_non_discriminated_union3_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union3_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadRecordForNonDiscriminatedUnion3 = resp.into_model().unwrap();
    assert_eq!(value.name, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    match &props["prop1"] {
        SpreadRecordForNonDiscriminatedUnion3AdditionalProperty::WidgetData2Array(arr) => {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0].kind, Some("kind1".to_string()));
            assert_eq!(arr[0].start, Some("2021-01-01T00:00:00Z".to_string()));
            assert_eq!(arr[1].kind, Some("kind1".to_string()));
            assert_eq!(arr[1].start, Some("2021-01-01T00:00:00Z".to_string()));
        }
        _ => panic!("Expected WidgetData2Array for prop1"),
    }
    match &props["prop2"] {
        SpreadRecordForNonDiscriminatedUnion3AdditionalProperty::WidgetData1(data) => {
            assert_eq!(data.kind, Some("kind1".to_string()));
            assert!(data.start.is_some());
            assert!(data.end.is_some());
        }
        _ => panic!("Expected WidgetData1 for prop2"),
    }
}

#[tokio::test]
async fn spread_record_non_discriminated_union3_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union3_client();
    let body = SpreadRecordForNonDiscriminatedUnion3 {
        name: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([
            (
                "prop1".to_string(),
                SpreadRecordForNonDiscriminatedUnion3AdditionalProperty::WidgetData2Array(vec![
                    WidgetData2 {
                        kind: Some("kind1".to_string()),
                        start: Some("2021-01-01T00:00:00Z".to_string()),
                    },
                    WidgetData2 {
                        kind: Some("kind1".to_string()),
                        start: Some("2021-01-01T00:00:00Z".to_string()),
                    },
                ]),
            ),
            (
                "prop2".to_string(),
                SpreadRecordForNonDiscriminatedUnion3AdditionalProperty::WidgetData1(WidgetData1 {
                    kind: Some("kind1".to_string()),
                    start: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-01T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                    end: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-02T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                }),
            ),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_record_non_discriminated_union_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadRecordForNonDiscriminatedUnion = resp.into_model().unwrap();
    assert_eq!(value.name, Some("abc".to_string()));
    let props = value.additional_properties.unwrap();
    match &props["prop1"] {
        SpreadRecordForNonDiscriminatedUnionAdditionalProperty::WidgetData0(data) => {
            assert_eq!(data.kind, Some("kind0".to_string()));
            assert_eq!(data.foo_prop, Some("abc".to_string()));
        }
        _ => panic!("Expected WidgetData0 for prop1"),
    }
    match &props["prop2"] {
        SpreadRecordForNonDiscriminatedUnionAdditionalProperty::WidgetData1(data) => {
            assert_eq!(data.kind, Some("kind1".to_string()));
            assert!(data.start.is_some());
            assert!(data.end.is_some());
        }
        _ => panic!("Expected WidgetData1 for prop2"),
    }
}

#[tokio::test]
async fn spread_record_non_discriminated_union_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_non_discriminated_union_client();
    let body = SpreadRecordForNonDiscriminatedUnion {
        name: Some("abc".to_string()),
        additional_properties: Some(HashMap::from([
            (
                "prop1".to_string(),
                SpreadRecordForNonDiscriminatedUnionAdditionalProperty::WidgetData0(WidgetData0 {
                    kind: Some("kind0".to_string()),
                    foo_prop: Some("abc".to_string()),
                }),
            ),
            (
                "prop2".to_string(),
                SpreadRecordForNonDiscriminatedUnionAdditionalProperty::WidgetData1(WidgetData1 {
                    kind: Some("kind1".to_string()),
                    start: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-01T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                    end: Some(
                        time::OffsetDateTime::parse(
                            "2021-01-02T00:00:00Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                }),
            ),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_record_union_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_union_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadRecordForUnion = resp.into_model().unwrap();
    assert_eq!(value.flag, Some(true));
    let props = value.additional_properties.unwrap();
    assert!(matches!(
        &props["prop1"],
        SpreadRecordForUnionAdditionalProperty::String(s) if s == "abc"
    ));
    assert!(matches!(
        &props["prop2"],
        SpreadRecordForUnionAdditionalProperty::Float32(f) if *f == 43.125
    ));
}

#[tokio::test]
async fn spread_record_union_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_record_union_client();
    let body = SpreadRecordForUnion {
        flag: Some(true),
        additional_properties: Some(HashMap::from([
            (
                "prop1".to_string(),
                SpreadRecordForUnionAdditionalProperty::String("abc".to_string()),
            ),
            (
                "prop2".to_string(),
                SpreadRecordForUnionAdditionalProperty::Float32(43.125),
            ),
        ])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}

#[tokio::test]
async fn spread_string_get() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_string_client();
    let resp = sub.get(None).await.unwrap();
    let value: SpreadStringRecord = resp.into_model().unwrap();
    assert_eq!(value.name, Some("SpreadSpringRecord".to_string()));
    let props = value.additional_properties.unwrap();
    assert_eq!(props["prop"], "abc");
}

#[tokio::test]
async fn spread_string_put() {
    let client = create_client();
    let sub = client.get_additional_properties_spread_string_client();
    let body = SpreadStringRecord {
        name: Some("SpreadSpringRecord".to_string()),
        additional_properties: Some(HashMap::from([("prop".to_string(), "abc".to_string())])),
    };
    sub.put(body.try_into().unwrap(), None).await.unwrap();
}
