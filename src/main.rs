use goose::prelude::*;

async fn human_readable_create(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user
        .post_json(
            "/",
            &serde_json::json!({
                "text": "Hello world!",
                "human_readable": true,
            }),
        )
        .await?;

    Ok(())
}

async fn normal_random_create(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user
        .post_json(
            "/",
            &serde_json::json!({
                "text": "Hello world!",
            }),
        )
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("HumanReadable").register_transaction(transaction!(human_readable_create)),
        )
        .register_scenario(
            scenario!("NormalRandom").register_transaction(transaction!(normal_random_create)),
        )
        .set_default(GooseDefault::ReportFile, "report.html")?
        .set_default(GooseDefault::NoResetMetrics, true)?
        .execute()
        .await?;

    Ok(())
}
