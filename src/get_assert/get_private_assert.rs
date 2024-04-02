use reqwest::{Client,StatusCode};
use serde_json::{json, Value};
use serde::Deserialize;
use std::thread;
use std::time::Duration;

use crate::get_assert::ResponseData;



pub async fn total_count(cookie:String,csrf:String) -> Result<(), Box<dyn std::error::Error>> {
    println!("获取私有程序总数");

    // let proxy_url = String::from("http://127.0.0.1:8080");
    // let proxy = reqwest::Proxy::http(&proxy_url)?;
    let url = "https://hackerone.com/graphql";

    let json_body = r##"{"operationName":"DiscoveryQuery","variables":{"size":100,"from":0,"query":{},"filter":{"bool":{"filter":[{"bool":{"must_not":{"term":{"team_type":"Engagements::Assessment"}}}},{"bool":{"should":[{"exists":{"field":"structured_scope_stats.URL"}}]}},{"bool":{"should":{"term":{"state":4}}}}]}},"sort":[{"field":"launched_at","direction":"DESC"}],"post_filters":{"my_programs":false,"bookmarked":false,"campaign_teams":false},"product_area":"opportunity_discovery","product_feature":"search"},"query":"query DiscoveryQuery($query: OpportunitiesQuery!, $filter: QueryInput!, $from: Int, $size: Int, $sort: [SortInput!], $post_filters: OpportunitiesFilterInput) {\n  me {\n    id\n    ...OpportunityListMe\n    __typename\n  }\n  opportunities_search(\n    query: $query\n    filter: $filter\n    from: $from\n    size: $size\n    sort: $sort\n    post_filters: $post_filters\n  ) {\n    nodes {\n      ... on OpportunityDocument {\n        id\n        handle\n        __typename\n      }\n      ...OpportunityList\n      __typename\n    }\n    total_count\n    __typename\n  }\n}\n\nfragment OpportunityListMe on User {\n  id\n  ...OpportunityCardMe\n  __typename\n}\n\nfragment OpportunityCardMe on User {\n  id\n  ...BookmarkMe\n  __typename\n}\n\nfragment BookmarkMe on User {\n  id\n  __typename\n}\n\nfragment OpportunityList on OpportunityDocument {\n  id\n  ...OpportunityCard\n  __typename\n}\n\nfragment OpportunityCard on OpportunityDocument {\n  id\n  team_id\n  name\n  handle\n  profile_picture\n  triage_active\n  publicly_visible_retesting\n  allows_private_disclosure\n  allows_bounty_splitting\n  launched_at\n  state\n  offers_bounties\n  last_updated_at\n  currency\n  team_type\n  minimum_bounty_table_value\n  maximum_bounty_table_value\n  cached_response_efficiency_percentage\n  first_response_time\n  structured_scope_stats\n  show_response_efficiency_indicator\n  submission_state\n  resolved_report_count\n  campaign {\n    id\n    campaign_type\n    start_date\n    end_date\n    critical\n    target_audience\n    __typename\n  }\n  gold_standard\n  awarded_report_count\n  awarded_reporter_count\n  h1_clear\n  idv\n  __typename\n}\n"}"##;
    //let json_body = serde_json::to_string(&json_body)?;

    // let proxy_url = "http://127.0.0.1:8080";
    // let proxy = reqwest::Proxy::http(proxy_url)?;
    // let proxy_url1 = "http://127.0.0.1:8080";
    // let proxy1 = reqwest::Proxy::https(proxy_url1)?;

    let client = reqwest::Client::builder()
        // .proxy(proxy)
        // .proxy(proxy1)
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .post(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .header("Cookie", cookie.clone())
        .header("X-Csrf-Token",csrf.clone())
        .header("Referer", "https://hackerone.com/opportunities/all/search?private=true&asset_types=URL&ordering=Newest+programs")
        .header("X-Datadog-Sampling-Priority", 1)
        //.header("Referer", "https://hackerone.com/opportunities/all/search?bbp=true&asset_types=URL&ordering=Newest+programs")
        //.header("X-Datadog-Sampling-Priority", 1)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_body)
        .send()
        .await?;
     // 检查响应状态码
    
    // 打印请求行
    let mut total_count:usize =1;

     match response.status() {
        StatusCode::OK => {
            // 响应成功
            let body = response.text().await?;
            let parsed_response: ResponseData = serde_json::from_str(&body)?;
             // 注意在这里使用了 `await`
            total_count = parsed_response.data.opportunities_search.total_count;
            println!("私有程序总数{}",total_count);
            let _get_hander =  get_all_hander(cookie.clone(),csrf.clone(),total_count).await?;

            println!("Total Count: {}", parsed_response.data.opportunities_search.total_count);
            //println!("Response Body: {}", body);
        }
        status => {
            // 响应出错
          //  println!("{}", response.text().await?);
            println!("Unexpected status code: {}", status);
        }
    }

    Ok(())

}

async fn get_all_hander(cookie:String,csrf:String,total:usize)-> Result<(), Box<dyn std::error::Error>> {
    println!("获取私有程序handler");
    // let proxy_url = "http://127.0.0.1:8080";
    // let proxy = reqwest::Proxy::http(proxy_url)?;
    // let proxy_url1 = "http://127.0.0.1:8080";
    // let proxy1 = reqwest::Proxy::https(proxy_url1)?;
    let first_digit = get_first_digit(total as u32);

    for i in 0..first_digit+1 {
        let url = "https://hackerone.com/graphql";
        let json_str = r#"{"operationName":"DiscoveryQuery","variables":{"size":100,"from":0,"query":{},"filter":{"bool":{"filter":[{"bool":{"must_not":{"term":{"team_type":"Engagements::Assessment"}}}},{"bool":{"should":[{"exists":{"field":"structured_scope_stats.URL"}}]}},{"bool":{"should":{"term":{"state":4}}}}]}},"sort":[{"field":"launched_at","direction":"DESC"}],"post_filters":{"my_programs":false,"bookmarked":false,"campaign_teams":false},"product_area":"opportunity_discovery","product_feature":"search"},"query":"query DiscoveryQuery($query: OpportunitiesQuery!, $filter: QueryInput!, $from: Int, $size: Int, $sort: [SortInput!], $post_filters: OpportunitiesFilterInput) {\n  me {\n    id\n    ...OpportunityListMe\n    __typename\n  }\n  opportunities_search(\n    query: $query\n    filter: $filter\n    from: $from\n    size: $size\n    sort: $sort\n    post_filters: $post_filters\n  ) {\n    nodes {\n      ... on OpportunityDocument {\n        id\n        handle\n        __typename\n      }\n      ...OpportunityList\n      __typename\n    }\n    total_count\n    __typename\n  }\n}\n\nfragment OpportunityListMe on User {\n  id\n  ...OpportunityCardMe\n  __typename\n}\n\nfragment OpportunityCardMe on User {\n  id\n  ...BookmarkMe\n  __typename\n}\n\nfragment BookmarkMe on User {\n  id\n  __typename\n}\n\nfragment OpportunityList on OpportunityDocument {\n  id\n  ...OpportunityCard\n  __typename\n}\n\nfragment OpportunityCard on OpportunityDocument {\n  id\n  team_id\n  name\n  handle\n  profile_picture\n  triage_active\n  publicly_visible_retesting\n  allows_private_disclosure\n  allows_bounty_splitting\n  launched_at\n  state\n  offers_bounties\n  last_updated_at\n  currency\n  team_type\n  minimum_bounty_table_value\n  maximum_bounty_table_value\n  cached_response_efficiency_percentage\n  first_response_time\n  structured_scope_stats\n  show_response_efficiency_indicator\n  submission_state\n  resolved_report_count\n  campaign {\n    id\n    campaign_type\n    start_date\n    end_date\n    critical\n    target_audience\n    __typename\n  }\n  gold_standard\n  awarded_report_count\n  awarded_reporter_count\n  h1_clear\n  idv\n  __typename\n}\n"}"#;
        // 解析 JSON 字符串为 serde_json::Value 对象
         let mut value: Value = serde_json::from_str(json_str).unwrap();
         // 查找 variables 下的 from 字段，并修改其值为新值
    if let Some(variables) = value.get_mut("variables") {
        if let Some(from) = variables.get_mut("from") {
            // 设置新的 from 值
            *from = json!(i as u32 * 100); // 这里设置新的 from 值为 200
        }
    }
     // 将修改后的 serde_json::Value 对象转换回 JSON 字符串
     let modified_json_str = serde_json::to_string(&value).unwrap();
     //println!("{modified_json_str}");

     let client = reqwest::Client::builder()
    //  .proxy(proxy.clone())
    //  .proxy(proxy1.clone())
        .danger_accept_invalid_certs(true)
        .build()?;

        let response = client
        .post(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .header("Cookie", &cookie)
        .header("X-Csrf-Token",&csrf)
        .header("Referer", "https://hackerone.com/opportunities/all/search?private=true&asset_types=URL&ordering=Newest+programs")
        .header("X-Datadog-Sampling-Priority", 1)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(modified_json_str)
        .send()
        .await?;

        let mut handle:Vec<String> = vec![];
        match response.status() {
            StatusCode::OK => {
                // 响应成功
                let body = response.text().await?;
                let parsed_response: ResponseData = serde_json::from_str(&body)?;
                 // 注意在这里使用了 `await`
                // 提取 handle 和 total_count
                for node in parsed_response.data.opportunities_search.nodes {
                    handle.push(node.handle.clone());
            
                 }
                 let _get_assert = get_assertions(cookie.clone(),csrf.clone(),handle.clone()).await?;
            }
            status => {
                // 响应出错
                //println!("{}", response.text().await?);
                println!("Unexpected status code: {}", status);
            }
        }
    
    }
    Ok(())

}

pub async fn get_assertions(
    cookie: String,
    csrf_token: String,
    handle: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("获取私有资产范围");
    let url = "https://hackerone.com/graphql";
    let json_body = r#"{"operationName":"PolicySearchStructuredScopesQuery","variables":{"handle":"indrive","searchString":"","eligibleForSubmission":true,"eligibleForBounty":true,"asmTagIds":[],"assetTypes":[],"from":0,"size":200,"sort":{"field":"cvss_score","direction":"DESC"},"product_area":"h1_assets","product_feature":"policy_scopes"},"query":"query PolicySearchStructuredScopesQuery($handle: String!, $searchString: String, $eligibleForSubmission: Boolean, $eligibleForBounty: Boolean, $minSeverityScore: SeverityRatingEnum, $asmTagIds: [Int], $assetTypes: [StructuredScopeAssetTypeEnum!], $from: Int, $size: Int, $sort: SortInput) {\n  team(handle: $handle) {\n    id\n    structured_scopes_search(\n      search_string: $searchString\n      eligible_for_submission: $eligibleForSubmission\n      eligible_for_bounty: $eligibleForBounty\n      min_severity_score: $minSeverityScore\n      asm_tag_ids: $asmTagIds\n      asset_types: $assetTypes\n      from: $from\n      size: $size\n      sort: $sort\n    ) {\n      nodes {\n        ... on StructuredScopeDocument {\n          id\n          ...PolicyScopeStructuredScopeDocument\n          __typename\n        }\n        __typename\n      }\n      pageInfo {\n        startCursor\n        hasPreviousPage\n        endCursor\n        hasNextPage\n        __typename\n      }\n      total_count\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment PolicyScopeStructuredScopeDocument on StructuredScopeDocument {\n  id\n  identifier\n  display_name\n  instruction\n  cvss_score\n  eligible_for_bounty\n  eligible_for_submission\n  asm_system_tags\n  created_at\n  updated_at\n  attachments {\n    id\n    file_name\n    file_size\n    content_type\n    expiring_url\n    __typename\n  }\n  __typename\n}\n"}"#;
   
    //let mut file_handles: HashMap<String, File> = HashMap::new();

    for i in handle.iter() {
        thread::sleep(Duration::from_secs(5));
        let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

        let mut value: Value = serde_json::from_str(json_body)?;
        if let Some(variables) = value.get_mut("variables") {
            if let Some(handle) = variables.get_mut("handle") {
                *handle = serde_json::Value::String(i.clone());
            }
        }

        let modified_json_str = serde_json::to_string(&value)?;

        let response = client
            .post(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .header("Cookie", &cookie)
            .header("X-Csrf-Token", &csrf_token)
            .header("Referer", "https://hackerone.com/opportunities/all/search?bbp=true&asset_types=URL&ordering=Newest+programs")
            .header("X-Datadog-Sampling-Priority", "1") // Converted to string
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(modified_json_str)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let body = response.text().await?;
                let v: Value = serde_json::from_str(&body)?;

                if let Some(nodes) = v["data"]["team"]["structured_scopes_search"]["nodes"].as_array() {
                        for node in nodes{
                            if let Some(identifier) = node["identifier"].as_str() {
                                println!("Identifier: {}", identifier);
                        }
                        }

                }    
    }
            _ => {
                println!("Response status: {}", response.status());
            }
        }
    }

    Ok(())
}




fn get_first_digit(num: u32) -> u32 {
    let num_str = num.to_string(); // 将数字转换为字符串
    let first_char = num_str.chars().next().unwrap(); // 获取字符串的第一个字符
    first_char.to_digit(10).unwrap() // 将字符转换为数字
}