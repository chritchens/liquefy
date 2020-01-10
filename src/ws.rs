//! `ws` contains the types and functions for the web socket notifications API of the Liquid Network hub.
//! The notifications can be sent and received on both HTTP and HTTPS.

use crate::utils::UUID;

/// `GenericRequest` is a generic request in the WS Notification API.
pub struct GenericRequest<Op, Args> {
    pub op: Op,
    pub args: Args,
}

/// `PingOp` is the operation type of a `PingRequest`.
pub struct PingOp;

/// `PingRequest` is a PING request in the WS Notification API.
pub type PingRequest = GenericRequest<PingOp, ()>;

/// `AckOp` is the operation type of an `AckRequest`.
pub struct AckOp;

/// `AckRequest` is an ACK request in the WS Notification API.
pub type AckRequest = GenericRequest<AckOp, UUID>;

/// `SubscribeOp` is the operation type of a `SubscribeRequest`.
pub struct SubscribeOp;

/// `SubscribeRequest` is a subscribe request in the WS Notification API.
pub type SubscribeRequest = GenericRequest<SubscribeOp, Vec<String>>;

/// `UnsubscribeOp` is the operation type of an `UnsubscribeRequest`.
pub struct UnsubscribeOp;

/// `UnsubscribeRequest` is an unsubscribe request in the WS Notification API.
pub type UnsubscribeRequest = GenericRequest<UnsubscribeOp, Vec<String>>;

/// `GenericResponse` is a generic response in the WS Notification API.
pub struct GenericResponse<Type, Data> {
    pub r#type: Type,
    pub uuid: UUID,
    pub data: Data,
}

/// `ErrorType` is the type of an `ErrorRequest`.
pub struct ErrorType;

/// `ErrorData` is the data of an `ErrorResponse`.
pub struct ErrorData<Req> {
    pub message: String,
    pub cause: Req,
}

/// `ErrorResponse` is an error response in the WS Notification API.
pub type ErrorResponse<Req> = GenericResponse<ErrorType, ErrorData<Req>>;

/// `NotificationType` is the type of a `NotificationRequest`.
pub struct NotificationType;

/// `WalletEventType` is the set of wallet event types.
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
pub enum TokenPairEventType {
    IncomingSwap,
    MatchedSwap,
}

/// `EventType` is the type of an `EventRequest`.
pub enum EventType {
    Wallet(WalletEventType),
    TokenPair(TokenPairEventType),
}

/// `TransferAudit` is the equivalent of the TransferAuditSerializer model in the Swagger documentations.
pub struct TransferAudit {
    // TODO: "TransferAuditSerializer check the models section of the docs"?
}

/// `WalletAdmissionData` is the equivalent of the WalletAdmissionDataSerializer model in the Swagger documentations.
pub struct WalletAdmissionData {
    pub address: String,
    pub token: String,
    pub eon_number: i64,
    pub wallet_signature: String,
    pub operator_signature: String,
    pub trail_identifier: i64,
}

/// `DepositNotification` is the equivalent of the DepositNotificationSerializer model in the Swagger documentations.
pub struct DepositNotification {
    // TODO: "...Deposit (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WithdrawalRequestNotification` is the equivalent of the WithdrawalRequestNotificationSerializer model in the Swagger documentations.
pub struct WithdrawalRequestNotification {
    // TODO: "...WithdrawalRequest (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WithdrawalNotification` is the equivalent of the WithdrawalNotificationSerializer model in the Swagger documentations.
pub struct WithdrawalNotification {
    // TODO: "...Withdrawal (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WalletDataNotification` is the equivalent of the WalletDataNotificationSerializer model in the Swagger documentations.
pub struct WalletDataNotification {
    // TODO: "...WalletData (check the models section)"?
    pub address: String,
    pub token: String,
}

/// `WalletEvent` is the model of wallet event type data.
pub enum WalletEvent {
    TransferAudit(TransferAudit),
    WalletAdmissionData(WalletAdmissionData),
    DepositNotification(DepositNotification),
    WithdrawalRequestNotification(WithdrawalRequestNotification),
    WithdrawalNotification(WithdrawalNotification),
    WalletDataNotification(WalletDataNotification),
}

/// `TokenEvent` is the model of token event type data.
pub struct TokenEvent(pub TransferAudit);

/// `Event` is the model of event data.
pub enum Event {
    Wallet(WalletEvent),
    Token(TokenEvent),
}

/// `NotificationData` is the data of a `NotificationResponse`.
pub struct NotificationData {
    pub r#type: EventType,
    pub data: Event,
}

/// `NotificationResponse` is a notification response in the WS Notification API.
pub type NotificationResponse = GenericResponse<NotificationType, NotificationData>;

/// `ResponseType` is the type of a `ResponseRequest`.
pub struct ResponseType;

/// `Response` is a response in the WS Notification API.
pub type Response<Data> = GenericResponse<ResponseType, Data>;

/// `StreamData` is response data containing a stream of textual data.
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
