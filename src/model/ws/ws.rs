//! `ws` contains the types and functions for the web socket notifications API of the Liquid Network hub.
//! The notifications can be sent and received on both HTTP and HTTPS.

use crate::utils::UUID;

use serde::{Deserialize, Serialize};

/// `GenericRequest` is a generic request in the WS Notification API.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GenericRequest<Op, Args> {
    pub op: Op,
    pub args: Args,
}

/// `PingOp` is the operation type of a `PingRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PingOp;

/// `PingRequest` is a PING request in the WS Notification API.
pub type PingRequest = GenericRequest<PingOp, ()>;

/// `AckOp` is the operation type of an `AckRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AckOp;

/// `AckRequest` is an ACK request in the WS Notification API.
pub type AckRequest = GenericRequest<AckOp, UUID>;

/// `SubscribeOp` is the operation type of a `SubscribeRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubscribeOp;

/// `SubscribeRequest` is a subscribe request in the WS Notification API.
pub type SubscribeRequest = GenericRequest<SubscribeOp, Vec<String>>;

/// `UnsubscribeOp` is the operation type of an `UnsubscribeRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeOp;

/// `UnsubscribeRequest` is an unsubscribe request in the WS Notification API.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub type UnsubscribeRequest = GenericRequest<UnsubscribeOp, Vec<String>>;

/// `GenericResponse` is a generic response in the WS Notification API.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GenericResponse<Type, Data> {
    pub r#type: Type,
    pub uuid: UUID,
    pub data: Data,
}

/// `ErrorType` is the type of an `ErrorRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorType;

/// `ErrorData` is the data of an `ErrorResponse`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorData<Req> {
    pub message: String,
    pub cause: Req,
}

/// `ErrorResponse` is an error response in the WS Notification API.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub type ErrorResponse<Req> = GenericResponse<ErrorType, ErrorData<Req>>;

/// `NotificationType` is the type of a `NotificationRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NotificationType;

/// `WalletEventType` is the set of wallet event types.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum WalletEventType {
    IncomingTransfer,
    IncomingReceipt,
    IncomingConfirmation,
    TimeoutTransfer,
    MatchedSwap,
    FinalizedSwap,
    CanceledSwap,
    RegisteredWallet,
    ConfirmedDeposit,
    RequestWithdrawal,
    ConfirmedWithdrawal,
    CheckpointCreated,
}

/// `TokenPairEventType` is the set of tokenpair event types.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenPairEventType {
    IncomingSwap,
    MatchedSwap,
}

/// `EventType` is the type of an `EventRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Wallet(WalletEventType),
    TokenPair(TokenPairEventType),
}

/// `TransferAudit` is the equivalent of the TransferAuditSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransferAudit {
    // TODO: "TransferAuditSerializer check the models section of the docs"?
}

/// `WalletAdmissionData` is the equivalent of the WalletAdmissionDataSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletAdmissionData {
    pub address: String,
    pub token: String,
    pub eon_number: i64,
    pub wallet_signature: String,
    pub operator_signature: String,
    pub trail_identifier: i64,
}

/// `DepositNotification` is the equivalent of the DepositNotificationSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DepositNotification {
    // TODO: "...Deposit (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WithdrawalRequestNotification` is the equivalent of the WithdrawalRequestNotificationSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequestNotification {
    // TODO: "...WithdrawalRequest (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WithdrawalNotification` is the equivalent of the WithdrawalNotificationSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalNotification {
    // TODO: "...Withdrawal (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WalletDataNotification` is the equivalent of the WalletDataNotificationSerializer model in the Swagger documentations.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletDataNotification {
    // TODO: "...WalletData (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WalletEvent` is the model of wallet event type data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum WalletEvent {
    TransferAudit(TransferAudit),
    WalletAdmissionData(WalletAdmissionData),
    DepositNotification(DepositNotification),
    WithdrawalRequestNotification(WithdrawalRequestNotification),
    WithdrawalNotification(WithdrawalNotification),
    WalletDataNotification(WalletDataNotification),
}

/// `TokenEvent` is the model of token event type data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenEvent(pub TransferAudit);

/// `Event` is the model of event data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Wallet(WalletEvent),
    Token(TokenEvent),
}

/// `NotificationData` is the data of a `NotificationResponse`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NotificationData {
    pub r#type: EventType,
    pub data: Event,
}

/// `NotificationResponse` is a notification response in the WS Notification API.
pub type NotificationResponse = GenericResponse<NotificationType, NotificationData>;

/// `ResponseType` is the type of a `ResponseRequest`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseType;

/// `Response` is a response in the WS Notification API.
pub type Response<Data> = GenericResponse<ResponseType, Data>;

/// `StreamData` is response data containing a stream of textual data.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StreamData<Op> {
    pub op: Op,
    pub stream: Vec<String>,
}

/// `SubscribeData` is the data of a `SubscribeResponse`.
pub type SubscribeData = StreamData<SubscribeOp>;

/// `SubscribeResponse` is a subscribe response in the WS Notification API.
pub type SubscribeResponse = Response<SubscribeData>;

/// `UnsubscribeData` is the data of an `UnsubscribeResponse`.
pub type UnsubscribeData = StreamData<UnsubscribeOp>;

/// `UnsubscribeResponse` is an unsubscribe response in the WS Notification API.
pub type UnsubscribeResponse = Response<UnsubscribeData>;
