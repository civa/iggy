use bon::Builder;

use crate::clients::consumer::{AutoCommit, AutoCommitWhen};
use crate::consumer::ConsumerKind;
use crate::error::IggyError;
use crate::identifier::Identifier;
use crate::messages::poll_messages::PollingStrategy;
use crate::utils::crypto::EncryptorKind;
use crate::utils::duration::IggyDuration;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Builder, Debug, Clone)]
#[builder(on(String, into))]
pub struct IggyConsumerConfig {
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    auto_commit: AutoCommit,
    batch_size: u32,
    create_stream_if_not_exists: bool,
    create_topic_if_not_exists: bool,
    consumer_name: String,
    consumer_kind: ConsumerKind,
    polling_interval: IggyDuration,
    polling_strategy: PollingStrategy,
    partitions_count: u32,
    replication_factor: Option<u8>,
    encryptor: Option<Arc<EncryptorKind>>,
    polling_retry_interval: IggyDuration,
    init_retries: Option<u32>,
    init_interval: IggyDuration,
}

impl Default for IggyConsumerConfig {
    fn default() -> Self {
        let stream_id = Identifier::from_str_value("test_stream").unwrap();
        let topic_id = Identifier::from_str_value("test_topic").unwrap();

        Self {
            stream_id,
            stream_name: "test_stream".to_string(),
            topic_id,
            topic_name: "test_topic".to_string(),
            auto_commit: AutoCommit::When(AutoCommitWhen::PollingMessages),
            batch_size: 100,
            create_stream_if_not_exists: false,
            create_topic_if_not_exists: false,
            consumer_name: "test_consumer".to_string(),
            consumer_kind: ConsumerKind::ConsumerGroup,
            polling_interval: IggyDuration::from_str("5ms").unwrap(),
            polling_strategy: PollingStrategy::last(),
            partitions_count: 1,
            replication_factor: None,
            encryptor: None,
            polling_retry_interval: IggyDuration::new_from_secs(1),
            init_retries: Some(5),
            init_interval: IggyDuration::new_from_secs(3),
        }
    }
}

impl IggyConsumerConfig {
    /// Creates a new `IggyConsumerConfig` from the given arguments.
    ///
    /// # Args
    ///
    /// * `stream_id` - The stream id.
    /// * `stream_name` - The stream name.
    /// * `topic_id` - The topic id.
    /// * `topic_name` - The topic name.
    /// * `auto_commit` - The auto commit config.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `create_stream_if_not_exists` - Whether to create the stream if it does not exists.
    /// * `create_topic_if_not_exists` - Whether to create the topic if it does not exists.
    /// * `consumer_name` - The consumer name.
    /// * `consumer_kind` - The consumer kind.
    /// * `polling_interval` - The interval between polling for new messages.
    /// * `polling_strategy` - The polling strategy.
    /// * `partitions_count` - The number of partitions.
    /// * `replication_factor` - The replication factor.
    /// * `encryptor` - The encryptor.
    /// * `polling_retry_interval` - The polling retry interval.
    /// * `init_retries` - The number of init retries.
    /// * `init_interval` - The init interval.
    ///
    ///
    /// Returns:
    /// A new `IggyConsumerConfig`.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
        auto_commit: AutoCommit,
        batch_size: u32,
        create_stream_if_not_exists: bool,
        create_topic_if_not_exists: bool,
        consumer_name: String,
        consumer_kind: ConsumerKind,
        polling_interval: IggyDuration,
        polling_strategy: PollingStrategy,
        partitions_count: u32,
        replication_factor: Option<u8>,
        encryptor: Option<Arc<EncryptorKind>>,
        polling_retry_interval: IggyDuration,
        init_retries: Option<u32>,
        init_interval: IggyDuration,
    ) -> Self {
        Self {
            stream_id,
            stream_name,
            topic_id,
            topic_name,
            auto_commit,
            batch_size,
            create_stream_if_not_exists,
            create_topic_if_not_exists,
            consumer_name,
            consumer_kind,
            polling_interval,
            polling_strategy,
            partitions_count,
            replication_factor,
            encryptor,
            polling_retry_interval,
            init_retries,
            init_interval,
        }
    }

    /// Creates a new `IggyConsumerConfig` from the given arguments.
    ///
    /// # Args
    ///
    /// * `stream` - The stream name.
    /// * `topic` - The topic name.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `polling_interval` - The interval between polling for new messages.
    ///
    /// Returns:
    /// A new `IggyConsumerConfig`.
    ///
    pub fn from_stream_topic(
        stream: &str,
        topic: &str,
        batch_size: u32,
        polling_interval: IggyDuration,
    ) -> Result<Self, IggyError> {
        let stream_id = Identifier::from_str_value(stream)?;
        let topic_id = Identifier::from_str_value(topic)?;

        Ok(Self {
            stream_id,
            stream_name: stream.to_string(),
            topic_id,
            topic_name: topic.to_string(),
            auto_commit: AutoCommit::When(AutoCommitWhen::PollingMessages),
            batch_size,
            create_stream_if_not_exists: false,
            create_topic_if_not_exists: false,
            consumer_name: format!("consumer-{}-{}", stream, topic),
            consumer_kind: ConsumerKind::ConsumerGroup,
            polling_interval,
            polling_strategy: PollingStrategy::last(),
            partitions_count: 1,
            replication_factor: None,
            encryptor: None,
            polling_retry_interval: IggyDuration::new_from_secs(1),
            init_retries: Some(5),
            init_interval: IggyDuration::new_from_secs(3),
        })
    }
}

impl IggyConsumerConfig {
    pub fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    pub fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn auto_commit(&self) -> AutoCommit {
        self.auto_commit
    }

    pub fn batch_size(&self) -> u32 {
        self.batch_size
    }
    pub fn create_stream_if_not_exists(&self) -> bool {
        self.create_stream_if_not_exists
    }

    pub fn create_topic_if_not_exists(&self) -> bool {
        self.create_topic_if_not_exists
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn consumer_kind(&self) -> ConsumerKind {
        self.consumer_kind
    }

    pub fn polling_interval(&self) -> IggyDuration {
        self.polling_interval
    }

    pub fn polling_strategy(&self) -> PollingStrategy {
        self.polling_strategy
    }

    pub fn partitions_count(&self) -> u32 {
        self.partitions_count
    }

    pub fn replication_factor(&self) -> Option<u8> {
        self.replication_factor
    }

    pub fn encryptor(&self) -> Option<Arc<EncryptorKind>> {
        self.encryptor.clone()
    }

    pub fn polling_retry_interval(&self) -> IggyDuration {
        self.polling_retry_interval
    }

    pub fn init_retries(&self) -> Option<u32> {
        self.init_retries
    }

    pub fn init_interval(&self) -> IggyDuration {
        self.init_interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_equal() {
        let stream_id = Identifier::from_str_value("test_stream").unwrap();
        let topic_id = Identifier::from_str_value("test_topic").unwrap();

        // Builder is generated by the bon macro
        let config = IggyConsumerConfig::builder()
            .stream_id(stream_id)
            .stream_name("test_stream".to_string())
            .topic_id(topic_id)
            .topic_name("test_topic".to_string())
            .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
            .batch_size(100)
            .create_stream_if_not_exists(true)
            .create_topic_if_not_exists(true)
            .consumer_name("test_consumer".to_string())
            .consumer_kind(ConsumerKind::ConsumerGroup)
            .polling_interval(IggyDuration::from_str("5ms").unwrap())
            .polling_strategy(PollingStrategy::last())
            .polling_retry_interval(IggyDuration::new_from_secs(1))
            .partitions_count(1)
            .init_retries(3)
            .init_interval(IggyDuration::new_from_secs(3))
            .build();

        assert_eq!(
            config.stream_id(),
            &Identifier::from_str_value("test_stream").unwrap()
        );
        assert_eq!(config.stream_name(), "test_stream");
        assert_eq!(
            config.topic_id(),
            &Identifier::from_str_value("test_topic").unwrap()
        );
        assert_eq!(config.topic_name(), "test_topic");
        assert_eq!(
            config.auto_commit(),
            AutoCommit::When(AutoCommitWhen::PollingMessages)
        );
        assert_eq!(config.batch_size(), 100);
        assert!(config.create_stream_if_not_exists());
        assert!(config.create_topic_if_not_exists());
        assert_eq!(config.consumer_name(), "test_consumer");
        assert_eq!(config.consumer_kind(), ConsumerKind::ConsumerGroup);
        assert_eq!(
            config.polling_interval(),
            IggyDuration::from_str("5ms").unwrap()
        );
        assert_eq!(config.polling_strategy(), PollingStrategy::last());
        assert_eq!(config.partitions_count(), 1);

        assert_eq!(
            config.polling_retry_interval(),
            IggyDuration::new_from_secs(1)
        );
        assert_eq!(config.init_retries(), Some(3));

        assert_eq!(config.init_interval(), IggyDuration::new_from_secs(3));
    }

    #[test]
    fn should_be_default() {
        let stream_id = Identifier::from_str_value("test_stream").unwrap();
        let topic_id = Identifier::from_str_value("test_topic").unwrap();

        let config = IggyConsumerConfig::default();
        assert_eq!(config.stream_id(), &stream_id);
        assert_eq!(config.stream_name(), "test_stream");
        assert_eq!(config.topic_id(), &topic_id);
        assert_eq!(config.topic_name(), "test_topic");
        assert_eq!(
            config.auto_commit(),
            AutoCommit::When(AutoCommitWhen::PollingMessages)
        );
        assert_eq!(config.batch_size(), 100);
        assert!(!config.create_stream_if_not_exists());
        assert!(!config.create_topic_if_not_exists());
        assert_eq!(config.consumer_name(), "test_consumer");
        assert_eq!(config.consumer_kind(), ConsumerKind::ConsumerGroup);
        assert_eq!(
            config.polling_interval(),
            IggyDuration::from_str("5ms").unwrap()
        );
        assert_eq!(config.polling_strategy(), PollingStrategy::last());
        assert_eq!(config.partitions_count(), 1);
        assert_eq!(config.replication_factor(), None);

        assert_eq!(config.polling_retry_interval(), IggyDuration::ONE_SECOND);
        assert_eq!(config.init_retries(), Some(5));
        assert_eq!(config.init_interval(), IggyDuration::new_from_secs(3));
    }

    #[test]
    fn should_be_new() {
        let config = IggyConsumerConfig::new(
            Identifier::from_str_value("test_stream").unwrap(),
            "test_stream".to_string(),
            Identifier::from_str_value("test_topic").unwrap(),
            "test_topic".to_string(),
            AutoCommit::When(AutoCommitWhen::PollingMessages),
            100,
            false,
            false,
            "test_consumer".to_string(),
            ConsumerKind::ConsumerGroup,
            IggyDuration::from_str("5ms").unwrap(),
            PollingStrategy::last(),
            1,
            None,
            None,
            IggyDuration::new_from_secs(1),
            Some(3),
            IggyDuration::new_from_secs(3),
        );
        assert_eq!(
            config.stream_id(),
            &Identifier::from_str_value("test_stream").unwrap(),
        );
        assert_eq!(config.stream_name(), "test_stream");
        assert_eq!(
            config.topic_id(),
            &Identifier::from_str_value("test_topic").unwrap()
        );
        assert_eq!(config.topic_name(), "test_topic");
        assert_eq!(
            config.auto_commit(),
            AutoCommit::When(AutoCommitWhen::PollingMessages)
        );
        assert_eq!(config.batch_size(), 100);
        assert!(!config.create_stream_if_not_exists());
        assert!(!config.create_topic_if_not_exists());
        assert_eq!(config.consumer_name(), "test_consumer");
        assert_eq!(config.consumer_kind(), ConsumerKind::ConsumerGroup);
        assert_eq!(
            config.polling_interval(),
            IggyDuration::from_str("5ms").unwrap()
        );
        assert_eq!(config.polling_strategy(), PollingStrategy::last());
        assert_eq!(config.partitions_count(), 1);
        assert_eq!(config.replication_factor(), None);

        assert_eq!(
            config.polling_retry_interval(),
            IggyDuration::new_from_secs(1)
        );
        assert_eq!(config.init_retries(), Some(3));
        assert_eq!(config.init_interval(), IggyDuration::new_from_secs(3));
    }

    #[test]
    fn should_be_from_stream_topic() {
        let res = IggyConsumerConfig::from_stream_topic(
            "test_stream",
            "test_topic",
            100,
            IggyDuration::from_str("5ms").unwrap(),
        );
        assert!(res.is_ok());
        let config = res.unwrap();

        assert_eq!(config.stream_name(), "test_stream");
        assert_eq!(config.topic_name(), "test_topic");
        assert_eq!(config.batch_size(), 100);
        assert!(!config.create_stream_if_not_exists());
        assert!(!config.create_topic_if_not_exists());
        assert_eq!(config.consumer_name(), "consumer-test_stream-test_topic");
        assert_eq!(config.consumer_kind(), ConsumerKind::ConsumerGroup);
        assert_eq!(
            config.polling_interval(),
            IggyDuration::from_str("5ms").unwrap()
        );
        assert_eq!(config.polling_strategy(), PollingStrategy::last());
        assert_eq!(config.partitions_count(), 1);
        assert_eq!(config.replication_factor(), None);
    }
}
