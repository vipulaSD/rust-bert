//! # Ready-to-use NLP pipelines and models
//!
//! Based on Huggingface's pipelines, ready to use end-to-end NLP pipelines are available as part of this crate. The following capabilities are currently available:
//!
//! #### 1. Question Answering
//! Extractive question answering from a given question and context. DistilBERT model finetuned on SQuAD (Stanford Question Answering Dataset)
//!
//! ```no_run
//!# use std::path::PathBuf;
//!# use tch::Device;
//! use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QaInput};
//!# fn main() -> failure::Fallible<()> {
//!# let mut home: PathBuf = dirs::home_dir().unwrap();
//!# home.push("rustbert");
//!# home.push("distilbert-qa");
//!# let config_path = &home.as_path().join("config.json");
//!# let vocab_path = &home.as_path().join("vocab.txt");
//!# let weights_path = &home.as_path().join("model.ot");
//! let device = Device::cuda_if_available();
//! let qa_model = QuestionAnsweringModel::new(vocab_path,
//!                                            config_path,
//!                                            weights_path, device)?;
//!
//! let question = String::from("Where does Amy live ?");
//! let context = String::from("Amy lives in Amsterdam");
//!
//! let answers = qa_model.predict(&vec!(QaInput { question, context }), 1, 32);
//!# Ok(())
//!# }
//! ```
//!
//! Output: \
//! ```no_run
//!# use rust_bert::pipelines::question_answering::Answer;
//!# let output =
//! [
//!     Answer {
//!         score: 0.9976,
//!         start: 13,
//!         end: 21,
//!         answer: "Amsterdam"
//!# .to_owned()
//!     }
//! ]
//!# ;
//! ```
//!
//! #### 2. Natural Language Generation
//! Generate language based on a prompt. GPT2 and GPT available as base models.
//! Include techniques such as beam search, top-k and nucleus sampling, temperature setting and repetition penalty.
//! Supports batch generation of sentences from several prompts. Sequences will be left-padded with the model's padding token if present, the unknown token otherwise.
//! This may impact the results and it is recommended to submit prompts of similar length for best results. Additional information on the input parameters for generation is provided in this module's documentation.
//!
//! ```no_run
//!# use std::path::PathBuf;
//!# use tch::Device;
//! use rust_bert::pipelines::generation::GPT2Generator;
//!# fn main() -> failure::Fallible<()> {
//!# use rust_bert::pipelines::generation::LanguageGenerator;
//! let mut home: PathBuf = dirs::home_dir().unwrap();
//!# home.push("rustbert");
//!# home.push("gpt2");
//!# let config_path = &home.as_path().join("config.json");
//!# let vocab_path = &home.as_path().join("vocab.txt");
//!# let merges_path = &home.as_path().join("merges.txt");
//!# let weights_path = &home.as_path().join("model.ot");
//! let device = Device::cuda_if_available();
//! let model = GPT2Generator::new(vocab_path, merges_path, config_path, weights_path, Default::default(), device)?;
//! let input_context_1 = "The dog";
//! let input_context_2 = "The cat was";
//! let output = model.generate(Some(vec!(input_context_1, input_context_2)), None);
//!# Ok(())
//!# }
//! ```
//! Example output: \
//! ```no_run
//!# let output =
//! [
//!     "The dog's owners, however, did not want to be named. According to the lawsuit, the animal's owner, a 29-year",
//!     "The dog has always been part of the family. \"He was always going to be my dog and he was always looking out for me",
//!     "The dog has been able to stay in the home for more than three months now. \"It's a very good dog. She's",
//!     "The cat was discovered earlier this month in the home of a relative of the deceased. The cat\'s owner, who wished to remain anonymous,",
//!     "The cat was pulled from the street by two-year-old Jazmine.\"I didn't know what to do,\" she said",
//!     "The cat was attacked by two stray dogs and was taken to a hospital. Two other cats were also injured in the attack and are being treated."
//! ]
//!# ;
//!```
//!
//! #### 3. Sentiment analysis
//! Predicts the binary sentiment for a sentence. DistilBERT model finetuned on SST-2.
//! ```no_run
//!# use std::path::PathBuf;
//!# use tch::Device;
//! use rust_bert::pipelines::sentiment::SentimentClassifier;
//!# fn main() -> failure::Fallible<()> {
//!# let mut home: PathBuf = dirs::home_dir().unwrap();
//!# home.push("rustbert");
//!# home.push("distilbert_sst2");
//!# let config_path = &home.as_path().join("config.json");
//!# let vocab_path = &home.as_path().join("vocab.txt");
//!# let weights_path = &home.as_path().join("model.ot");
//! let device = Device::cuda_if_available();
//! let sentiment_classifier = SentimentClassifier::new(vocab_path,
//!                                                     config_path,
//!                                                     weights_path, device)?;
//! let input = [
//!     "Probably my all-time favorite movie, a story of selflessness, sacrifice and dedication to a noble cause, but it's not preachy or boring.",
//!     "This film tried to be too many things all at once: stinging political satire, Hollywood blockbuster, sappy romantic comedy, family values promo...",
//!     "If you like original gut wrenching laughter you will like this movie. If you are young or old then you will love this movie, hell even my mom liked it.",
//! ];
//! let output = sentiment_classifier.predict(&input);
//!# Ok(())
//!# }
//! ```
//! (Example courtesy of [IMDb](http://www.imdb.com))
//!
//! Output: \
//! ```no_run
//!# use rust_bert::pipelines::sentiment::Sentiment;
//!# use rust_bert::pipelines::sentiment::SentimentPolarity::{Positive, Negative};
//!# let output =
//! [
//!    Sentiment { polarity: Positive, score: 0.998 },
//!    Sentiment { polarity: Negative, score: 0.992 },
//!    Sentiment { polarity: Positive, score: 0.999 }
//! ]
//!# ;
//! ```
//!
//! #### 4. Named Entity Recognition
//! Extracts entities (Person, Location, Organization, Miscellaneous) from text. BERT cased large model finetuned on CoNNL03, contributed by the [MDZ Digital Library team at the Bavarian State Library](https://github.com/dbmdz)
//! ```no_run
//!# use std::path::PathBuf;
//!# use tch::Device;
//! use rust_bert::pipelines::ner::NERModel;
//!# fn main() -> failure::Fallible<()> {
//!# let mut home: PathBuf = dirs::home_dir().unwrap();
//!# home.push("rustbert");
//!# home.push("bert-ner");
//!# let config_path = &home.as_path().join("config.json");
//!# let vocab_path = &home.as_path().join("vocab.txt");
//!# let weights_path = &home.as_path().join("model.ot");
//! let device = Device::cuda_if_available();
//! let ner_model = NERModel::new(vocab_path,
//!                               config_path,
//!                               weights_path, device)?;
//! let input = [
//!     "My name is Amy. I live in Paris.",
//!     "Paris is a city in France."
//! ];
//! let output = ner_model.predict(&input);
//!# Ok(())
//!# }
//! ```
//! Output: \
//! ```no_run
//!# use rust_bert::pipelines::question_answering::Answer;
//!# use rust_bert::pipelines::ner::Entity;
//!# let output =
//! [
//!    Entity { word: String::from("Amy"), score: 0.9986, label: String::from("I-PER") },
//!    Entity { word: String::from("Paris"), score: 0.9985, label: String::from("I-LOC") },
//!    Entity { word: String::from("Paris"), score: 0.9988, label: String::from("I-LOC") },
//!    Entity { word: String::from("France"), score: 0.9993, label: String::from("I-LOC") },
//! ]
//!# ;
//! ```
//!

pub mod sentiment;
pub mod ner;
pub mod question_answering;
pub mod generation;