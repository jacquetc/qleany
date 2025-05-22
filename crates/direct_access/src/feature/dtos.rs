use common::entities::Feature;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FeatureDto {
    pub id: EntityId,
    pub name: String,
    pub use_cases: Vec<EntityId>,
}

impl From<FeatureDto> for Feature {
    fn from(feature_dto: FeatureDto) -> Self {
        Feature {
            id: feature_dto.id,
            name: feature_dto.name,
            use_cases: feature_dto.use_cases,
        }
    }
}

impl From<&FeatureDto> for Feature {
    fn from(feature_dto: &FeatureDto) -> Self {
        Feature {
            id: feature_dto.id,
            name: feature_dto.name.clone(),
            use_cases: feature_dto.use_cases.clone(),
        }
    }
}

impl From<Feature> for FeatureDto {
    fn from(feature: Feature) -> Self {
        FeatureDto {
            id: feature.id,
            name: feature.name,
            use_cases: feature.use_cases,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFeatureDto {
    pub name: String,
    pub use_cases: Vec<EntityId>,
}

impl From<CreateFeatureDto> for Feature {
    fn from(create_feature_dto: CreateFeatureDto) -> Self {
        Feature {
            id: 0,
            name: create_feature_dto.name,
            use_cases: create_feature_dto.use_cases,
        }
    }
}

impl From<&CreateFeatureDto> for Feature {
    fn from(create_feature_dto: &CreateFeatureDto) -> Self {
        Feature {
            id: 0,
            name: create_feature_dto.name.clone(),
            use_cases: create_feature_dto.use_cases.clone(),
        }
    }
}

impl From<Feature> for CreateFeatureDto {
    fn from(feature: Feature) -> Self {
        CreateFeatureDto {
            name: feature.name,
            use_cases: feature.use_cases,
        }
    }
}

pub use common::direct_access::feature::FeatureRelationshipField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRelationshipDto {
    pub id: EntityId,
    pub field: FeatureRelationshipField,
    pub right_ids: Vec<EntityId>,
}
