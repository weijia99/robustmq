#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonReply {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClusterType {
    PlacementCenter = 0,
    StorageEngine = 1,
    MqttBrokerServer = 2,
    AmqpBrokerServer = 3,
}
impl ClusterType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClusterType::PlacementCenter => "PlacementCenter",
            ClusterType::StorageEngine => "StorageEngine",
            ClusterType::MqttBrokerServer => "MQTTBrokerServer",
            ClusterType::AmqpBrokerServer => "AMQPBrokerServer",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlacementCenter" => Some(Self::PlacementCenter),
            "StorageEngine" => Some(Self::StorageEngine),
            "MQTTBrokerServer" => Some(Self::MqttBrokerServer),
            "AMQPBrokerServer" => Some(Self::AmqpBrokerServer),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GenerageIdType {
    UniqInt = 0,
    UniqStr = 1,
}
impl GenerageIdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GenerageIdType::UniqInt => "UniqInt",
            GenerageIdType::UniqStr => "UniqStr",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UniqInt" => Some(Self::UniqInt),
            "UniqStr" => Some(Self::UniqStr),
            _ => None,
        }
    }
}
