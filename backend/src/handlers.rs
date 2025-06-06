use actix_web::{web, HttpResponse, Responder};
use crate::models::{Transaction, NewTransaction};
use crate::db::AppState;
use chrono::Utc;
use sqlx::types::BigDecimal as SqlxBigDecimal;
use bigdecimal::BigDecimal as JsonBigDecimal;
use std::str::FromStr;

pub async fn add_transaction(
    state: web::Data<AppState>,
    transaction: web::Json<NewTransaction>,
) -> impl Responder {
    // Convert JsonBigDecimal to SqlxBigDecimal via string
    let amount = SqlxBigDecimal::from_str(&transaction.amount.to_string())
        .expect("Failed to convert BigDecimal");

    let tx = sqlx::query!(
        r#"
        INSERT INTO transactions (amount, category, description, date)
        VALUES ($1, $2, $3, $4)
        RETURNING id, amount, category, description, date
        "#,
        amount,
        transaction.category,
        transaction.description,
        Utc::now().naive_utc()
    )
    .fetch_one(&state.pool)
    .await;

    match tx {
        Ok(record) => {
            let transaction = Transaction {
                id: record.id,
                amount: JsonBigDecimal::from_str(&record.amount.to_string())
                    .expect("Failed to convert BigDecimal"),
                category: record.category,
                description: record.description,
                date: record.date,
            };
            HttpResponse::Created().json(transaction)
        }
        Err(e) => {
            eprintln!("Error adding transaction: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_transactions(state: web::Data<AppState>) -> impl Responder {
    let records = sqlx::query!(
        r#"
        SELECT id, amount, category, description, date
        FROM transactions
        ORDER BY date DESC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match records {
        Ok(records) => {
            let transactions: Vec<Transaction> = records
                .into_iter()
                .map(|record| Transaction {
                    id: record.id,
                    amount: JsonBigDecimal::from_str(&record.amount.to_string())
                        .expect("Failed to convert BigDecimal"),
                    category: record.category,
                    description: record.description,
                    date: record.date,
                })
                .collect();
            HttpResponse::Ok().json(transactions)
        }
        Err(e) => {
            eprintln!("Error fetching transactions: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_transaction(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        DELETE FROM transactions
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting transaction: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}