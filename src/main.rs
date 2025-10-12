mod intent;
use crate::intent::{IntentClassifier, IntentId, TrainingExample, TrainingSource};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new classifier
    let classifier = IntentClassifier::new().await?;

    // Predict an intent
    let prediction = classifier.predict_intent("merge these JSON files together").await?;
    println!("Intent: {}, Confidence: {:.3}", prediction.intent, prediction.confidence.value());

    // Add custom training data
    let example = TrainingExample {
        text: "calculate the sum of these numbers".to_string(),
        intent: IntentId::from("math_operation"),
        confidence: 1.0,
        source: TrainingSource::Programmatic,
    };
    classifier.add_training_example(example).await?;

    // Get statistics
    let stats = classifier.get_stats().await;
    println!("Training examples: {}", stats.training_examples);

    Ok(())
}