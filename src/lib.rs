//! `libliquefy` is the internal library of the liquefy Liquid Network hub client.
//! The library data models, requests and responses are built around those documented
//! in https://public.liquid.network/swagger, https://rinkeby.liquid.network/swagger
//! and https://limbo.liquid.network/swagger.

pub mod utils {
    //! `utils` contains the utility types and functions used throughout libliquefy.

    use uuid::Uuid;

    /// `UUID` is a UUID value of no specified version.
    pub struct UUID(Uuid);
}

pub mod ws {
    //! `ws` contains the types and functions for the web socket notifications API of the Liquid Network hub.
    //! The notifications can be sent and received on both HTTP and HTTPS.

    use crate::utils::UUID;

    /// `GenericRequest` is a generic request in the WS Notification API.
    pub struct GenericRequest<Op, Args> {
        pub req_op: Op,
        pub req_args: Args,
    }

    /// `PingOp` is the operation type of a `PingRequest`.
    pub struct PingOp;

    /// `PingRequest` is a PING request in the WS Notification API.
    pub type PingRequest = GenericRequest<PingOp, ()>;

    /// `AckOp` is the operation type of an `AckRequest`.
    pub struct AckOp;

    /// `AckRequest` is an ACK request in the WS Notification API.
    pub type AckRequest = GenericRequest<AckOp, UUID>;

    /// `RequestStreams` is a stream of textual data used in `SubscribeRequest` and `UnsubscribeRequest`.
    pub type RequestStreams = Vec<String>;

    /// `SubscribeOp` is the operation type of a `SubscribeRequest`.
    pub struct SubscribeOp;

    /// `SubscribeRequest` is a subscribe request in the WS Notification API.
    pub type SubscribeRequest = GenericRequest<SubscribeOp, RequestStreams>;

    /// `UnsubscribeOp` is the operation type of an `UnsubscribeRequest`.
    pub struct UnsubscribeOp;

    /// `UnsubscribeRequest` is an unsubscribe request in the WS Notification API.
    pub type UnsubscribeRequest = GenericRequest<UnsubscribeOp, RequestStreams>;

    /// `GenericResponse` is a generic response in the WS Notification API.
    pub struct GenericResponse<Type, Data> {
        pub res_type: Type,
        pub res_uuid: UUID,
        pub res_data: Data,
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

    /// `TransferAuditModel` is the equivalent of the TransferAuditSerializer model in the Swagger documentations.
    pub struct TransferAuditModel; // TODO

    /// `WalletAdmissionDataModel` is the equivalent of the WalletAdmissionDataSerializer model in the Swagger documentations.
    pub struct WalletAdmissionDataModel; // TODO

    /// `DepositNotificationModel` is the equivalent of the DepositNotificationSerializer model in the Swagger documentations.
    pub struct DepositNotificationModel; // TODO

    /// `WithdrawalRequestNotificationModel` is the equivalent of the WithdrawalRequestNotificationSerializer model in the Swagger documentations.
    pub struct WithdrawalRequestNotificationModel; // TODO

    /// `WithdrawalNotificationModel` is the equivalent of the WithdrawalNotificationSerializer model in the Swagger documentations.
    pub struct WithdrawalNotificationModel; // TODO

    /// `WalletDataNotificationModel` is the equivalent of the WalletDataNotificationSerializer model in the Swagger documentations.
    pub struct WalletDataNotificationModel; // TODO

    /// `WalletEventModel` is the model of wallet event type data.
    pub enum WalletEventModel {
        TransferAudit(TransferAuditModel),
        WalletAdmissionData(WalletAdmissionDataModel),
        DepositNotification(DepositNotificationModel),
        WithdrawalRequestNotification(WithdrawalRequestNotificationModel),
        WithdrawalNotification(WithdrawalNotificationModel),
        WalletDataNotification(WalletDataNotificationModel),
    }

    /// `TokenEventModel` is the model of token event type data.
    pub struct TokenEventModel(pub TransferAuditModel);

    /// `EventModel` is the model of event data.
    pub enum EventModel {
        Wallet(WalletEventModel),
        Token(TokenEventModel),
    }

    /// `NotificationData` is the data of a `NotificationResponse`.
    pub struct NotificationData {
        pub event_type: EventType,
        pub event_data: EventModel,
    }

    /// `NotificationResponse` is a notification response in the WS Notification API.
    pub type NotificationResponse = GenericResponse<NotificationType, NotificationData>;

    /// `ResponseType` is the type of a `ResponseRequest`.
    pub struct ResponseType;

    /// `Response` is a response in the WS Notification API.
    pub type Response<Data> = GenericResponse<ResponseType, Data>;

    /// `SubscribeData` is the data of a `SubscribeResponse`.
    pub struct SubscribeData; // TODO

    /// `SubscribeResponse` is a subscribe response in the WS Notification API.
    pub type SubscribeResponse = Response<SubscribeData>;

    /// `UnsubscribeData` is the data of an `UnsubscribeResponse`.
    pub struct UnsubscribeData; // TODO

    /// `UnsubscribeResponse` is an unsubscribe response in the WS Notification API.
    pub type UnsubscribeResponse = Response<UnsubscribeData>;
}

pub mod http {
    //! `http` contains the types and functions for the HTTP API of the Liquid Network hub.
}
