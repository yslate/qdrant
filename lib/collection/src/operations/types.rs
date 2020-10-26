use segment::types::{VectorElementType, PointIdType, TheMap, PayloadKeyType, PayloadType, SeqNumberType, Filter, SearchParams, SegmentConfig};
use serde;
use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};

/// Type of vector in API
pub type VectorType = Vec<VectorElementType>;


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    pub id: PointIdType,
    pub payload: Option<TheMap<PayloadKeyType, PayloadType>>,
    pub vector: Option<Vec<VectorElementType>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CollectionInfo {
    pub vectors_count: usize,
    pub segments_count: usize,
    pub disk_data_size: usize,
    pub ram_data_size: usize,
    pub config: SegmentConfig,
}


#[derive(Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateStatus {
    Acknowledged,
    Completed,
}


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateResult {
    pub operation_id: SeqNumberType,
    pub status: UpdateStatus,
}


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SearchRequest {
    pub vector: Vec<VectorElementType>,
    pub filter: Option<Filter>,
    pub params: Option<SearchParams>,
    pub top: usize,
}


