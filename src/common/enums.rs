/// Content type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentType {
	Product,
	Episode,
	Lecture,
	NewspaperMagazine,
	Meditation,
	Misc,
	Performance,
	RadioTvProgram,
	Show,
	Speech,
	Podcast,
}

impl ContentType {
	pub fn to_string(&self) -> &'static str {
		match self {
			ContentType::Product => "Product",
			ContentType::Episode => "Episode",
			ContentType::Lecture => "Lecture",
			ContentType::NewspaperMagazine => "Newspaper / Magazine",
			ContentType::Meditation => "Meditation",
			ContentType::Misc => "Misc",
			ContentType::Performance => "Performance",
			ContentType::RadioTvProgram => "Radio/TV Program",
			ContentType::Show => "Show",
			ContentType::Speech => "Speech",
			ContentType::Podcast => "Podcast",
		}
	}
}

/// Content delivery type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentDeliveryType {
	SinglePartBook,
	MultiPartBook,
	SinglePartIssue,
	MultiPartIssue,
	Periodical,
	PodcastParent,
	PodcastEpisode,
}

impl ContentDeliveryType {
	pub fn to_string(&self) -> &'static str {
		match self {
			ContentDeliveryType::SinglePartBook => "SinglePartBook",
			ContentDeliveryType::MultiPartBook => "MultiPartBook",
			ContentDeliveryType::SinglePartIssue => "SinglePartIssue",
			ContentDeliveryType::MultiPartIssue => "MultiPartIssue",
			ContentDeliveryType::Periodical => "Periodical",
			ContentDeliveryType::PodcastParent => "PodcastParent",
			ContentDeliveryType::PodcastEpisode => "PodcastEpisode",
		}
	}
}

/// Format type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormatType {
	Abridged,
	OriginalRecording,
	Unabridged,
}

impl FormatType {
	pub fn to_string(&self) -> &'static str {
		match self {
			FormatType::Abridged => "abridged",
			FormatType::OriginalRecording => "original_recording",
			FormatType::Unabridged => "unabridged",
		}
	}
}

/// Language enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
	English,
}

impl Language {
	pub fn to_string(&self) -> &'static str {
		match self {
			Language::English => "english",
		}
	}
}

/// Origin marketplace enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OriginMarketplace {
	AF2M0KC94RCEA,
}

impl OriginMarketplace {
	pub fn to_string(&self) -> &'static str {
		match self {
			OriginMarketplace::AF2M0KC94RCEA => "AF2M0KC94RCEA",
		}
	}
}

/// Origin type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OriginType {
	AudibleChannels,
	AudibleComplimentaryOriginal,
	Purchase,
	Subscription,
}

impl OriginType {
	pub fn to_string(&self) -> &'static str {
		match self {
			OriginType::AudibleChannels => "AudibleChannels",
			OriginType::AudibleComplimentaryOriginal => "AudibleComplimentaryOriginal",
			OriginType::Purchase => "Purchase",
			OriginType::Subscription => "Subscription",
		}
	}
}

/// Status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
	Active,
}

impl Status {
	pub fn to_string(&self) -> &'static str {
		match self {
			Status::Active => "Active",
		}
	}
}

/// Thesaurus subject keyword enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThesaurusSubjectKeyword {
	AdventurersExplorers,
	AlternateHistory,
	Comedians,
	Contemporary,
	Dramatizations,
	EasternReligions,
	LaConfidential,
	LiteratureAndFiction,
	Medicine,
	Spirituality,
	StandupComedy,
	Storytelling,
	SwordSorcery,
	Workouts,
	Terrorism,
}

impl ThesaurusSubjectKeyword {
	pub fn to_string(&self) -> &'static str {
		match self {
			ThesaurusSubjectKeyword::AdventurersExplorers => "adventurers_&_explorers",
			ThesaurusSubjectKeyword::AlternateHistory => "alternate_history",
			ThesaurusSubjectKeyword::Comedians => "comedians",
			ThesaurusSubjectKeyword::Contemporary => "contemporary",
			ThesaurusSubjectKeyword::Dramatizations => "dramatizations",
			ThesaurusSubjectKeyword::EasternReligions => "eastern_religions",
			ThesaurusSubjectKeyword::LaConfidential => "la_confidential",
			ThesaurusSubjectKeyword::LiteratureAndFiction => "literature-and-fiction",
			ThesaurusSubjectKeyword::Medicine => "medicine",
			ThesaurusSubjectKeyword::Spirituality => "spirituality",
			ThesaurusSubjectKeyword::StandupComedy => "standup_comedy",
			ThesaurusSubjectKeyword::Storytelling => "storytelling",
			ThesaurusSubjectKeyword::SwordSorcery => "sword_&_sorcery",
			ThesaurusSubjectKeyword::Workouts => "workouts",
			ThesaurusSubjectKeyword::Terrorism => "terrorism",
		}
	}
}

/// Enhanced codec enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnhancedCodec {
	Lc128_44100_Stereo,
	Lc32_22050_Stereo,
	Lc64_22050_Stereo,
	Lc64_44100_Stereo,
	Aax,
	Format4,
	Mp42232,
	Mp42264,
	Mp444128,
	Mp44464,
	Piff2232,
	Piff2264,
	Piff44128,
	Piff4464,
}

impl EnhancedCodec {
	pub fn to_string(&self) -> &'static str {
		match self {
			EnhancedCodec::Lc128_44100_Stereo => "LC_128_44100_stereo",
			EnhancedCodec::Lc32_22050_Stereo => "LC_32_22050_stereo",
			EnhancedCodec::Lc64_22050_Stereo => "LC_64_22050_stereo",
			EnhancedCodec::Lc64_44100_Stereo => "LC_64_44100_stereo",
			EnhancedCodec::Aax => "aax",
			EnhancedCodec::Format4 => "format4",
			EnhancedCodec::Mp42232 => "mp42232",
			EnhancedCodec::Mp42264 => "mp42264",
			EnhancedCodec::Mp444128 => "mp444128",
			EnhancedCodec::Mp44464 => "mp44464",
			EnhancedCodec::Piff2232 => "piff2232",
			EnhancedCodec::Piff2264 => "piff2264",
			EnhancedCodec::Piff44128 => "piff44128",
			EnhancedCodec::Piff4464 => "piff4464",
		}
	}
}

/// Available codec format enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AvailableCodecFormat {
	Enhanced,
	Format4,
}

impl AvailableCodecFormat {
	pub fn to_string(&self) -> &'static str {
		match self {
			AvailableCodecFormat::Enhanced => "Enhanced",
			AvailableCodecFormat::Format4 => "Format4",
		}
	}
}

/// Name enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name {
	Aax,
	Aax22_32,
	Aax22_64,
	Aax44_128,
	Aax44_64,
	Format4,
	Mp422_32,
	Mp422_64,
	Mp444_128,
	Mp444_64,
	Piff22_32,
	Piff22_64,
	Piff44_128,
	Piff44_64,
}

impl Name {
	pub fn to_string(&self) -> &'static str {
		match self {
			Name::Aax => "aax",
			Name::Aax22_32 => "aax_22_32",
			Name::Aax22_64 => "aax_22_64",
			Name::Aax44_128 => "aax_44_128",
			Name::Aax44_64 => "aax_44_64",
			Name::Format4 => "format4",
			Name::Mp422_32 => "mp4_22_32",
			Name::Mp422_64 => "mp4_22_64",
			Name::Mp444_128 => "mp4_44_128",
			Name::Mp444_64 => "mp4_44_64",
			Name::Piff22_32 => "piff_22_32",
			Name::Piff22_64 => "piff_22_64",
			Name::Piff44_128 => "piff_44_128",
			Name::Piff44_64 => "piff_44_64",
		}
	}
}

/// Root enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Root {
	EditorsPicks,
	ExploreBy,
	Genres,
	InstitutionsHpMarketing,
	RodizioBuckets,
	RodizioGenres,
	ShortsPrime,
	ShortsCurated,
	ShortsSandbox,
}

impl Root {
	pub fn to_string(&self) -> &'static str {
		match self {
			Root::EditorsPicks => "EditorsPicks",
			Root::ExploreBy => "ExploreBy",
			Root::Genres => "Genres",
			Root::InstitutionsHpMarketing => "InstitutionsHpMarketing",
			Root::RodizioBuckets => "RodizioBuckets",
			Root::RodizioGenres => "RodizioGenres",
			Root::ShortsPrime => "ShortsPrime",
			Root::ShortsCurated => "ShortsCurated",
			Root::ShortsSandbox => "ShortsSandbox",
		}
	}
}

/// Review format enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReviewFormat {
	Freeform,
	Guided,
}

impl ReviewFormat {
	pub fn to_string(&self) -> &'static str {
		match self {
			ReviewFormat::Freeform => "Freeform",
			ReviewFormat::Guided => "Guided",
		}
	}
}

/// Guided response question type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuestionType {
	Genre,
	Misc,
	Overall,
	Performance,
	Story,
}

impl QuestionType {
	pub fn to_string(&self) -> &'static str {
		match self {
			QuestionType::Genre => "Genre",
			QuestionType::Misc => "Misc",
			QuestionType::Overall => "Overall",
			QuestionType::Performance => "Performance",
			QuestionType::Story => "Story",
		}
	}
}

/// Plan name enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanName {
	AyceRomance,
	ComplimentaryOriginalMemberBenefit,
	Radio,
	Rodizio,
	SpecialBenefit,
}

impl PlanName {
	pub fn to_string(&self) -> &'static str {
		match self {
			PlanName::AyceRomance => "AyceRomance",
			PlanName::ComplimentaryOriginalMemberBenefit => "ComplimentaryOriginalMemberBenefit",
			PlanName::Radio => "Radio",
			PlanName::Rodizio => "Rodizio",
			PlanName::SpecialBenefit => "SpecialBenefit",
		}
	}
}

/// List price class currency code enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyCode {
	USD,
}

impl CurrencyCode {
	pub fn to_string(&self) -> &'static str {
		match self {
			CurrencyCode::USD => "USD",
		}
	}
}

/// List price class merchant ID enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MerchantId {
	A2ZO8JX97D5MN9,
}

impl MerchantId {
	pub fn to_string(&self) -> &'static str {
		match self {
			MerchantId::A2ZO8JX97D5MN9 => "A2ZO8JX97D5MN9",
		}
	}
}

/// List price class type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeEnum {
	List,
	Member,
	Sale,
	Ws4VUpsell,
}

impl TypeEnum {
	pub fn to_string(&self) -> &'static str {
		match self {
			TypeEnum::List => "list",
			TypeEnum::Member => "member",
			TypeEnum::Sale => "sale",
			TypeEnum::Ws4VUpsell => "ws4v_upsell",
		}
	}
}

/// Relationship to product enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationshipToProduct {
	Child,
	Parent,
}

impl RelationshipToProduct {
	pub fn to_string(&self) -> &'static str {
		match self {
			RelationshipToProduct::Child => "child",
			RelationshipToProduct::Parent => "parent",
		}
	}
}

/// Relationship type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationshipType {
	Component,
	Episode,
	MerchantTitleAuthority,
	Season,
	Series,
}

impl RelationshipType {
	pub fn to_string(&self) -> &'static str {
		match self {
			RelationshipType::Component => "component",
			RelationshipType::Episode => "episode",
			RelationshipType::MerchantTitleAuthority => "merchant_title_authority",
			RelationshipType::Season => "season",
			RelationshipType::Series => "series",
		}
	}
}

/// Rejection reason enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RejectionReason {
	ContentEligibility,
	RequesterEligibility,
	GenericError,
}

impl RejectionReason {
	pub fn to_string(&self) -> &'static str {
		match self {
			RejectionReason::ContentEligibility => "ContentEligibility",
			RejectionReason::RequesterEligibility => "RequesterEligibility",
			RejectionReason::GenericError => "GenericError",
		}
	}
}

/// Validation type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationType {
	Client,
	Ownership,
	Membership,
	AYCL,
}

impl ValidationType {
	pub fn to_string(&self) -> &'static str {
		match self {
			ValidationType::Client => "Client",
			ValidationType::Ownership => "Ownership",
			ValidationType::Membership => "Membership",
			ValidationType::AYCL => "AYCL",
		}
	}
}

/// Record type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordType {
	Bookmark,
	Clip,
	LastHeard,
	Note,
}

impl RecordType {
	pub fn to_string(&self) -> &'static str {
		match self {
			RecordType::Bookmark => "audible.bookmark",
			RecordType::Clip => "audible.clip",
			RecordType::LastHeard => "audible.last_heard",
			RecordType::Note => "audible.note",
		}
	}
}