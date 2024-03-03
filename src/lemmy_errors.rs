// use core::{
//   fmt,
//   fmt::{Debug, Display},
// };
// use serde::{Deserialize, Serialize};
// use strum_macros::{Display, EnumIter};
// use tracing_error::SpanTrace;
// #[cfg(feature = "full")]
// use ts_rs::TS;

// #[allow(dead_code)]
// pub type LemmyResult<T> = Result<T, LemmyError>;

// pub struct LemmyError {
//   pub error_type: LemmyErrorType,
//   pub inner: anyhow::Error,
//   pub context: SpanTrace,
// }

// #[allow(dead_code)]
// pub const MAX_API_PARAM_ELEMENTS: usize = 1000;

// impl<T> From<T> for LemmyError
// where
//   T: Into<anyhow::Error>,
// {
//   fn from(t: T) -> Self {
//     let cause = t.into();
//     LemmyError {
//       error_type: LemmyErrorType::Unknown,
//       inner: cause,
//       context: SpanTrace::capture(),
//     }
//   }
// }

// impl Debug for LemmyError {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     f.debug_struct("LemmyError")
//       .field("message", &self.error_type)
//       .field("inner", &self.inner)
//       .field("context", &self.context)
//       .finish()
//   }
// }

// impl Display for LemmyError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}: ", &self.error_type)?;
//     writeln!(f, "{:?}", self.inner)?;
//     fmt::Display::fmt(&self.context, f)
//   }
// }

// #[derive(Default, Display, Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
// #[cfg_attr(feature = "full", derive(TS))]
// #[cfg_attr(feature = "full", ts(export))]
// #[serde(tag = "error", content = "message", rename_all = "snake_case")]
// pub enum LemmyErrorType {
//   ReportReasonRequired,
//   ReportTooLong,
//   NotAModerator,
//   NotAnAdmin,
//   CantBlockYourself,
//   CantBlockAdmin,
//   CouldntUpdateUser,
//   PasswordsDoNotMatch,
//   EmailNotVerified,
//   EmailRequired,
//   CouldntUpdateComment,
//   CouldntUpdatePrivateMessage,
//   CannotLeaveAdmin,
//   NoLinesInHtml,
//   SiteMetadataPageIsNotDoctypeHtml,
//   PictrsResponseError(String),
//   PictrsPurgeResponseError(String),
//   PictrsCachingDisabled,
//   ImageUrlMissingPathSegments,
//   ImageUrlMissingLastPathSegment,
//   PictrsApiKeyNotProvided,
//   NoContentTypeHeader,
//   NotAnImageType,
//   NotAModOrAdmin,
//   NoAdmins,
//   NotTopAdmin,
//   NotTopMod,
//   NotLoggedIn,
//   SiteBan,
//   Deleted,
//   BannedFromCommunity,
//   CouldntFindCommunity,
//   CouldntFindPerson,
//   PersonIsBlocked,
//   DownvotesAreDisabled,
//   InstanceIsPrivate,
//   InvalidPassword,
//   SiteDescriptionLengthOverflow,
//   HoneypotFailed,
//   RegistrationApplicationIsPending,
//   CantEnablePrivateInstanceAndFederationTogether,
//   Locked,
//   CouldntCreateComment,
//   MaxCommentDepthReached,
//   NoCommentEditAllowed,
//   OnlyAdminsCanCreateCommunities,
//   CommunityAlreadyExists,
//   LanguageNotAllowed,
//   OnlyModsCanPostInCommunity,
//   CouldntUpdatePost,
//   NoPostEditAllowed,
//   CouldntFindPost,
//   EditPrivateMessageNotAllowed,
//   SiteAlreadyExists,
//   ApplicationQuestionRequired,
//   InvalidDefaultPostListingType,
//   RegistrationClosed,
//   RegistrationApplicationAnswerRequired,
//   EmailAlreadyExists,
//   FederationForbiddenByStrictAllowList,
//   PersonIsBannedFromCommunity,
//   ObjectIsNotPublic,
//   InvalidCommunity,
//   CannotCreatePostOrCommentInDeletedOrRemovedCommunity,
//   CannotReceivePage,
//   NewPostCannotBeLocked,
//   OnlyLocalAdminCanRemoveCommunity,
//   OnlyLocalAdminCanRestoreCommunity,
//   NoIdGiven,
//   IncorrectLogin,
//   InvalidQuery,
//   ObjectNotLocal,
//   PostIsLocked,
//   PersonIsBannedFromSite(String),
//   InvalidVoteValue,
//   PageDoesNotSpecifyCreator,
//   PageDoesNotSpecifyGroup,
//   NoCommunityFoundInCc,
//   NoEmailSetup,
//   EmailSmtpServerNeedsAPort,
//   MissingAnEmail,
//   RateLimitError,
//   InvalidName,
//   InvalidDisplayName,
//   InvalidMatrixId,
//   InvalidPostTitle,
//   InvalidBodyField,
//   BioLengthOverflow,
//   MissingTotpToken,
//   MissingTotpSecret,
//   IncorrectTotpToken,
//   CouldntParseTotpSecret,
//   CouldntGenerateTotp,
//   TotpAlreadyEnabled,
//   CouldntLikeComment,
//   CouldntSaveComment,
//   CouldntCreateReport,
//   CouldntResolveReport,
//   CommunityModeratorAlreadyExists,
//   CommunityUserAlreadyBanned,
//   CommunityBlockAlreadyExists,
//   CommunityFollowerAlreadyExists,
//   CouldntUpdateCommunityHiddenStatus,
//   PersonBlockAlreadyExists,
//   UserAlreadyExists,
//   TokenNotFound,
//   CouldntLikePost,
//   CouldntSavePost,
//   CouldntMarkPostAsRead,
//   CouldntUpdateCommunity,
//   CouldntUpdateReplies,
//   CouldntUpdatePersonMentions,
//   PostTitleTooLong,
//   CouldntCreatePost,
//   CouldntCreatePrivateMessage,
//   CouldntUpdatePrivate,
//   SystemErrLogin,
//   CouldntSetAllRegistrationsAccepted,
//   CouldntSetAllEmailVerified,
//   Banned,
//   CouldntGetComments,
//   CouldntGetPosts,
//   InvalidUrl,
//   EmailSendFailed,
//   Slurs,
//   CouldntFindObject,
//   RegistrationDenied(Option<String>),
//   FederationDisabled,
//   DomainBlocked(String),
//   DomainNotInAllowList(String),
//   FederationDisabledByStrictAllowList,
//   SiteNameRequired,
//   SiteNameLengthOverflow,
//   PermissiveRegex,
//   InvalidRegex,
//   CaptchaIncorrect,
//   PasswordResetLimitReached,
//   CouldntCreateAudioCaptcha,
//   InvalidUrlScheme,
//   CouldntSendWebmention,
//   ContradictingFilters,
//   InstanceBlockAlreadyExists,
//   AuthCookieInsecure,
//   TooManyItems,
//   CommunityHasNoFollowers,
//   BanExpirationInPast,
//   InvalidUnixTime,
//   InvalidBotAction,
//   #[default]
//   Unknown,
// }

// impl From<LemmyErrorType> for LemmyError {
//   fn from(error_type: LemmyErrorType) -> Self {
//     let inner = anyhow::anyhow!("{}", error_type);
//     LemmyError {
//       error_type,
//       inner,
//       context: SpanTrace::capture(),
//     }
//   }
// }

// pub trait LemmyErrorExt<T, E: Into<anyhow::Error>> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError>;
// }

// impl<T, E: Into<anyhow::Error>> LemmyErrorExt<T, E> for Result<T, E> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError> {
//     self.map_err(|error| LemmyError {
//       error_type,
//       inner: error.into(),
//       context: SpanTrace::capture(),
//     })
//   }
// }
// pub trait LemmyErrorExt2<T> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError>;
//   fn into_anyhow(self) -> Result<T, anyhow::Error>;
// }

// impl<T> LemmyErrorExt2<T> for Result<T, LemmyError> {
//   fn with_lemmy_type(self, error_type: LemmyErrorType) -> Result<T, LemmyError> {
//     self.map_err(|mut e| {
//       e.error_type = error_type;
//       e
//     })
//   }
//   fn into_anyhow(self) -> Result<T, anyhow::Error> {
//     self.map_err(|e| e.inner)
//   }
// }
