use super::{Destination, ServiceMeta};
use crate::db::{builder::SqlBuilder, get_database, Record};

pub async fn distination(svc: &ServiceMeta) -> Destination {
    // TODO: find destination by algorithm from memory
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
    struct RoundRobin {
        id: surrealdb::sql::Thing,
        next: usize,
        service_id: String,
    }
    let query_index = SqlBuilder::new()
        .table("destinations")
        .select(vec!["*".to_string()])
        .r#where("service_id", &svc.id.id.to_string());

    let mut id: Option<surrealdb::sql::Thing> = None;
    let mut index = match query_index.mem_execute().await {
        Ok(mut r) => {
            let index: Option<RoundRobin> = r.take(0).unwrap_or(None);
            if let Some(index) = index {
                id = Some(index.id);
                index.next
            } else {
                0
            }
        }
        Err(_) => 0,
    };
    let dest = svc.destination.get(index).unwrap().clone();
    index += 1;
    if index >= svc.destination.len() {
        index = 0;
    }
    if id.is_none() {
        let _: Option<Record> = match get_database()
            .await
            .memory
            .create("destinations")
            .content(serde_json::json!({
                "next": index,
                "service_id": &svc.id,
            }))
            .await
        {
            Ok(r) => r,
            Err(_) => None,
        };
    } else {
        if let Err(a) = get_database()
            .await
            .memory
            .update::<Option<RoundRobin>>(("destinations", id.unwrap()))
            .merge(serde_json::json!({
                "next": index,
            }))
            .await
        {
            println!("Save index error: {}", a);
        }
    }

    dest
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{builder::SqlBuilder, get_database, Record},
        proxy::services::{Destination, ServiceMeta},
    };

    #[test]
    fn test_round_robin_dest() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
            struct RoundRobin {
                id: surrealdb::sql::Thing,
                next: usize,
                service_id: String,
            }

            let dest: Vec<Destination> = vec![
                Destination {
                    ip: "0.0.0.1".to_string(),
                    port: 80,
                    protocol: "http".to_string(),
                },
                Destination {
                    ip: "0.0.0.2".to_string(),
                    port: 80,
                    protocol: "http".to_string(),
                },
                Destination {
                    ip: "0.0.0.3".to_string(),
                    port: 80,
                    protocol: "http".to_string(),
                },
            ];

            let svc = ServiceMeta {
                id: surrealdb::sql::Thing {
                    tb: "services".to_string(),
                    id: surrealdb::sql::Id::String("test_round_robin".to_string()),
                },
                algorithm: "round_robin".to_string(),
                destination: dest.clone(),
                name: "test".to_string(),
                host: "test.com".to_string(),
            };

            let _: Option<Record> = match get_database()
                .await
                .memory
                .create("destinations")
                .content(serde_json::json!({
                    "next": 0,
                    "service_id": &svc.id.id.to_string(),
                }))
                .await
            {
                Ok(r) => r,
                Err(_) => None,
            };

            let query_index = SqlBuilder::new()
                .table("destinations")
                .select(vec!["*".to_string()])
                .r#where("service_id", &svc.id.id.to_string());

            let mut id: Option<surrealdb::sql::Thing> = None;
            let _ = match query_index.mem_execute().await {
                Ok(mut r) => {
                    let index: Option<RoundRobin> = r.take(0).unwrap_or(None);
                    if let Some(index) = index {
                        id = Some(index.id);
                        index.next
                    } else {
                        0
                    }
                }
                Err(_) => 0,
            };

            if id.is_some() {
                if let Err(a) = get_database()
                    .await
                    .memory
                    .update::<Option<RoundRobin>>(("destinations", id.unwrap()))
                    .merge(serde_json::json!({
                        "next": 0,
                    }))
                    .await
                {
                    println!("Save index error: {}", a);
                }
            }

            let dest1 = super::distination(&svc).await;
            assert_eq!(dest1.ip, dest[0].ip);
            let dest2 = super::distination(&svc).await;
            assert_eq!(dest2.ip, dest[1].ip);
            let dest3 = super::distination(&svc).await;
            assert_eq!(dest3.ip, dest[2].ip);
        });
    }
}