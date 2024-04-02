pub mod get_bbp_assert;
pub mod get_private_assert;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize};


#[derive(Debug, Deserialize)]
struct Data {
    opportunities_search: OpportunitiesSearch,
}

#[derive(Debug, Deserialize)]
struct OpportunitiesSearch {
    nodes: Vec<Node>,
    total_count: usize,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: String,
    handle: String,
    // 其他字段也可以在这里添加
}

// 定义结构体用于解析 JSON 响应
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    data: Data,
}