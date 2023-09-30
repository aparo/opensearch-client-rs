#[derive(Clone, Debug)]
pub struct AccountDetails {
  backend_roles: Result<Vec<String>, String>,
  custom_attribute_names: Result<Vec<String>, String>,
  is_hidden: Result<Option<bool>, String>,
  is_internal_user: Result<Option<bool>, String>,
  is_reserved: Result<Option<bool>, String>,
  roles: Result<Vec<String>, String>,
  tenants: Result<Option<super::UserTenants>, String>,
  user_name: Result<Option<String>, String>,
  user_requested_tenant: Result<Option<String>, String>,
}

impl Default for AccountDetails {
  fn default() -> Self {
    Self {
      backend_roles: Ok(Default::default()),
      custom_attribute_names: Ok(Default::default()),
      is_hidden: Ok(Default::default()),
      is_internal_user: Ok(Default::default()),
      is_reserved: Ok(Default::default()),
      roles: Ok(Default::default()),
      tenants: Ok(Default::default()),
      user_name: Ok(Default::default()),
      user_requested_tenant: Ok(Default::default()),
    }
  }
}

impl AccountDetails {
  pub fn backend_roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.backend_roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
    self
  }

  pub fn custom_attribute_names<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.custom_attribute_names = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for custom_attribute_names: {}", e));
    self
  }

  pub fn is_hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.is_hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for is_hidden: {}", e));
    self
  }

  pub fn is_internal_user<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.is_internal_user = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for is_internal_user: {}", e));
    self
  }

  pub fn is_reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.is_reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for is_reserved: {}", e));
    self
  }

  pub fn roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for roles: {}", e));
    self
  }

  pub fn tenants<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserTenants>>,
    T::Error: std::fmt::Display, {
    self.tenants = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for tenants: {}", e));
    self
  }

  pub fn user_name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.user_name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for user_name: {}", e));
    self
  }

  pub fn user_requested_tenant<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.user_requested_tenant = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for user_requested_tenant: {}", e));
    self
  }
}

impl std::convert::TryFrom<AccountDetails> for super::AccountDetails {
  type Error = String;

  fn try_from(value: AccountDetails) -> Result<Self, String> {
    Ok(Self {
      backend_roles: value.backend_roles?,
      custom_attribute_names: value.custom_attribute_names?,
      is_hidden: value.is_hidden?,
      is_internal_user: value.is_internal_user?,
      is_reserved: value.is_reserved?,
      roles: value.roles?,
      tenants: value.tenants?,
      user_name: value.user_name?,
      user_requested_tenant: value.user_requested_tenant?,
    })
  }
}

impl From<super::AccountDetails> for AccountDetails {
  fn from(value: super::AccountDetails) -> Self {
    Self {
      backend_roles: Ok(value.backend_roles),
      custom_attribute_names: Ok(value.custom_attribute_names),
      is_hidden: Ok(value.is_hidden),
      is_internal_user: Ok(value.is_internal_user),
      is_reserved: Ok(value.is_reserved),
      roles: Ok(value.roles),
      tenants: Ok(value.tenants),
      user_name: Ok(value.user_name),
      user_requested_tenant: Ok(value.user_requested_tenant),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ActionGroup {
  allowed_actions: Result<Vec<String>, String>,
  description: Result<Option<String>, String>,
  hidden: Result<Option<bool>, String>,
  reserved: Result<Option<bool>, String>,
  static_: Result<Option<bool>, String>,
  type_: Result<Option<String>, String>,
}

impl Default for ActionGroup {
  fn default() -> Self {
    Self {
      allowed_actions: Ok(Default::default()),
      description: Ok(Default::default()),
      hidden: Ok(Default::default()),
      reserved: Ok(Default::default()),
      static_: Ok(Default::default()),
      type_: Ok(Default::default()),
    }
  }
}

impl ActionGroup {
  pub fn allowed_actions<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.allowed_actions = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
    self
  }

  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }

  pub fn hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hidden: {}", e));
    self
  }

  pub fn reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for reserved: {}", e));
    self
  }

  pub fn static_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.static_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for static_: {}", e));
    self
  }

  pub fn type_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.type_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for type_: {}", e));
    self
  }
}

impl std::convert::TryFrom<ActionGroup> for super::ActionGroup {
  type Error = String;

  fn try_from(value: ActionGroup) -> Result<Self, String> {
    Ok(Self {
      allowed_actions: value.allowed_actions?,
      description: value.description?,
      hidden: value.hidden?,
      reserved: value.reserved?,
      static_: value.static_?,
      type_: value.type_?,
    })
  }
}

impl From<super::ActionGroup> for ActionGroup {
  fn from(value: super::ActionGroup) -> Self {
    Self {
      allowed_actions: Ok(value.allowed_actions),
      description: Ok(value.description),
      hidden: Ok(value.hidden),
      reserved: Ok(value.reserved),
      static_: Ok(value.static_),
      type_: Ok(value.type_),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ActionObjectStructure {
  add: Result<Option<super::UserDefinedStructure>, String>,
  remove: Result<Option<super::UserDefinedStructure>, String>,
  remove_index: Result<Option<super::UserDefinedStructure>, String>,
}

impl Default for ActionObjectStructure {
  fn default() -> Self {
    Self {
      add: Ok(Default::default()),
      remove: Ok(Default::default()),
      remove_index: Ok(Default::default()),
    }
  }
}

impl ActionObjectStructure {
  pub fn add<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
    T::Error: std::fmt::Display, {
    self.add = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for add: {}", e));
    self
  }

  pub fn remove<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
    T::Error: std::fmt::Display, {
    self.remove = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for remove: {}", e));
    self
  }

  pub fn remove_index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
    T::Error: std::fmt::Display, {
    self.remove_index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for remove_index: {}", e));
    self
  }
}

impl std::convert::TryFrom<ActionObjectStructure> for super::ActionObjectStructure {
  type Error = String;

  fn try_from(value: ActionObjectStructure) -> Result<Self, String> {
    Ok(Self {
      add: value.add?,
      remove: value.remove?,
      remove_index: value.remove_index?,
    })
  }
}

impl From<super::ActionObjectStructure> for ActionObjectStructure {
  fn from(value: super::ActionObjectStructure) -> Self {
    Self {
      add: Ok(value.add),
      remove: Ok(value.remove),
      remove_index: Ok(value.remove_index),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AuditConfig {
  audit: Result<Option<super::AuditLogsConfig>, String>,
  compliance: Result<Option<super::ComplianceConfig>, String>,
  enabled: Result<Option<bool>, String>,
}

impl Default for AuditConfig {
  fn default() -> Self {
    Self {
      audit: Ok(Default::default()),
      compliance: Ok(Default::default()),
      enabled: Ok(Default::default()),
    }
  }
}

impl AuditConfig {
  pub fn audit<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::AuditLogsConfig>>,
    T::Error: std::fmt::Display, {
    self.audit = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for audit: {}", e));
    self
  }

  pub fn compliance<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ComplianceConfig>>,
    T::Error: std::fmt::Display, {
    self.compliance = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for compliance: {}", e));
    self
  }

  pub fn enabled<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.enabled = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for enabled: {}", e));
    self
  }
}

impl std::convert::TryFrom<AuditConfig> for super::AuditConfig {
  type Error = String;

  fn try_from(value: AuditConfig) -> Result<Self, String> {
    Ok(Self {
      audit: value.audit?,
      compliance: value.compliance?,
      enabled: value.enabled?,
    })
  }
}

impl From<super::AuditConfig> for AuditConfig {
  fn from(value: super::AuditConfig) -> Self {
    Self {
      audit: Ok(value.audit),
      compliance: Ok(value.compliance),
      enabled: Ok(value.enabled),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AuditConfigWithReadOnly {
  config: Result<Option<super::AuditConfig>, String>,
  readonly: Result<Vec<String>, String>,
}

impl Default for AuditConfigWithReadOnly {
  fn default() -> Self {
    Self {
      config: Ok(Default::default()),
      readonly: Ok(Default::default()),
    }
  }
}

impl AuditConfigWithReadOnly {
  pub fn config<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::AuditConfig>>,
    T::Error: std::fmt::Display, {
    self.config = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for config: {}", e));
    self
  }

  pub fn readonly<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.readonly = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for readonly: {}", e));
    self
  }
}

impl std::convert::TryFrom<AuditConfigWithReadOnly> for super::AuditConfigWithReadOnly {
  type Error = String;

  fn try_from(value: AuditConfigWithReadOnly) -> Result<Self, String> {
    Ok(Self {
      config: value.config?,
      readonly: value.readonly?,
    })
  }
}

impl From<super::AuditConfigWithReadOnly> for AuditConfigWithReadOnly {
  fn from(value: super::AuditConfigWithReadOnly) -> Self {
    Self {
      config: Ok(value.config),
      readonly: Ok(value.readonly),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AuditLogsConfig {
  disabled_rest_categories: Result<Vec<String>, String>,
  disabled_transport_categories: Result<Vec<String>, String>,
  enable_rest: Result<Option<bool>, String>,
  enable_transport: Result<Option<bool>, String>,
  exclude_sensitive_headers: Result<Option<bool>, String>,
  ignore_requests: Result<Vec<String>, String>,
  ignore_users: Result<Vec<String>, String>,
  log_request_body: Result<Option<bool>, String>,
  resolve_bulk_requests: Result<Option<bool>, String>,
  resolve_indices: Result<Option<bool>, String>,
}

impl Default for AuditLogsConfig {
  fn default() -> Self {
    Self {
      disabled_rest_categories: Ok(Default::default()),
      disabled_transport_categories: Ok(Default::default()),
      enable_rest: Ok(Default::default()),
      enable_transport: Ok(Default::default()),
      exclude_sensitive_headers: Ok(Default::default()),
      ignore_requests: Ok(Default::default()),
      ignore_users: Ok(Default::default()),
      log_request_body: Ok(Default::default()),
      resolve_bulk_requests: Ok(Default::default()),
      resolve_indices: Ok(Default::default()),
    }
  }
}

impl AuditLogsConfig {
  pub fn disabled_rest_categories<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.disabled_rest_categories = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for disabled_rest_categories: {}", e));
    self
  }

  pub fn disabled_transport_categories<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.disabled_transport_categories = value.try_into().map_err(|e| {
      format!(
        "error converting supplied value for disabled_transport_categories: {}",
        e
      )
    });
    self
  }

  pub fn enable_rest<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.enable_rest = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for enable_rest: {}", e));
    self
  }

  pub fn enable_transport<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.enable_transport = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for enable_transport: {}", e));
    self
  }

  pub fn exclude_sensitive_headers<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.exclude_sensitive_headers = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for exclude_sensitive_headers: {}", e));
    self
  }

  pub fn ignore_requests<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.ignore_requests = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for ignore_requests: {}", e));
    self
  }

  pub fn ignore_users<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.ignore_users = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for ignore_users: {}", e));
    self
  }

  pub fn log_request_body<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.log_request_body = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for log_request_body: {}", e));
    self
  }

  pub fn resolve_bulk_requests<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.resolve_bulk_requests = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for resolve_bulk_requests: {}", e));
    self
  }

  pub fn resolve_indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.resolve_indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for resolve_indices: {}", e));
    self
  }
}

impl std::convert::TryFrom<AuditLogsConfig> for super::AuditLogsConfig {
  type Error = String;

  fn try_from(value: AuditLogsConfig) -> Result<Self, String> {
    Ok(Self {
      disabled_rest_categories: value.disabled_rest_categories?,
      disabled_transport_categories: value.disabled_transport_categories?,
      enable_rest: value.enable_rest?,
      enable_transport: value.enable_transport?,
      exclude_sensitive_headers: value.exclude_sensitive_headers?,
      ignore_requests: value.ignore_requests?,
      ignore_users: value.ignore_users?,
      log_request_body: value.log_request_body?,
      resolve_bulk_requests: value.resolve_bulk_requests?,
      resolve_indices: value.resolve_indices?,
    })
  }
}

impl From<super::AuditLogsConfig> for AuditLogsConfig {
  fn from(value: super::AuditLogsConfig) -> Self {
    Self {
      disabled_rest_categories: Ok(value.disabled_rest_categories),
      disabled_transport_categories: Ok(value.disabled_transport_categories),
      enable_rest: Ok(value.enable_rest),
      enable_transport: Ok(value.enable_transport),
      exclude_sensitive_headers: Ok(value.exclude_sensitive_headers),
      ignore_requests: Ok(value.ignore_requests),
      ignore_users: Ok(value.ignore_users),
      log_request_body: Ok(value.log_request_body),
      resolve_bulk_requests: Ok(value.resolve_bulk_requests),
      resolve_indices: Ok(value.resolve_indices),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CatAllPitSegmentsResponseContent {
  content: Result<Option<super::CatPitSegment>, String>,
}

impl Default for CatAllPitSegmentsResponseContent {
  fn default() -> Self {
    Self {
      content: Ok(Default::default()),
    }
  }
}

impl CatAllPitSegmentsResponseContent {
  pub fn content<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::CatPitSegment>>,
    T::Error: std::fmt::Display, {
    self.content = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for content: {}", e));
    self
  }
}

impl std::convert::TryFrom<CatAllPitSegmentsResponseContent> for super::CatAllPitSegmentsResponseContent {
  type Error = String;

  fn try_from(value: CatAllPitSegmentsResponseContent) -> Result<Self, String> {
    Ok(Self {
      content: value.content?,
    })
  }
}

impl From<super::CatAllPitSegmentsResponseContent> for CatAllPitSegmentsResponseContent {
  fn from(value: super::CatAllPitSegmentsResponseContent) -> Self {
    Self {
      content: Ok(value.content),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CatPitSegment {
  committed: Result<Option<bool>, String>,
  compound: Result<Option<bool>, String>,
  docs_count: Result<Option<i32>, String>,
  docs_deleted: Result<Option<i32>, String>,
  generation: Result<Option<i32>, String>,
  index: Result<Option<String>, String>,
  ip: Result<Option<String>, String>,
  prirep: Result<Option<bool>, String>,
  searchable: Result<Option<bool>, String>,
  segment: Result<Option<String>, String>,
  shard: Result<Option<i32>, String>,
  size: Result<Option<String>, String>,
  size_memory: Result<Option<i32>, String>,
  version: Result<Option<String>, String>,
}

impl Default for CatPitSegment {
  fn default() -> Self {
    Self {
      committed: Ok(Default::default()),
      compound: Ok(Default::default()),
      docs_count: Ok(Default::default()),
      docs_deleted: Ok(Default::default()),
      generation: Ok(Default::default()),
      index: Ok(Default::default()),
      ip: Ok(Default::default()),
      prirep: Ok(Default::default()),
      searchable: Ok(Default::default()),
      segment: Ok(Default::default()),
      shard: Ok(Default::default()),
      size: Ok(Default::default()),
      size_memory: Ok(Default::default()),
      version: Ok(Default::default()),
    }
  }
}

impl CatPitSegment {
  pub fn committed<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.committed = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for committed: {}", e));
    self
  }

  pub fn compound<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.compound = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for compound: {}", e));
    self
  }

  pub fn docs_count<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.docs_count = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for docs_count: {}", e));
    self
  }

  pub fn docs_deleted<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.docs_deleted = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for docs_deleted: {}", e));
    self
  }

  pub fn generation<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.generation = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for generation: {}", e));
    self
  }

  pub fn index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index: {}", e));
    self
  }

  pub fn ip<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.ip = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for ip: {}", e));
    self
  }

  pub fn prirep<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.prirep = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for prirep: {}", e));
    self
  }

  pub fn searchable<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.searchable = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for searchable: {}", e));
    self
  }

  pub fn segment<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.segment = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for segment: {}", e));
    self
  }

  pub fn shard<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.shard = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shard: {}", e));
    self
  }

  pub fn size<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.size = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for size: {}", e));
    self
  }

  pub fn size_memory<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.size_memory = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for size_memory: {}", e));
    self
  }

  pub fn version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.version = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for version: {}", e));
    self
  }
}

impl std::convert::TryFrom<CatPitSegment> for super::CatPitSegment {
  type Error = String;

  fn try_from(value: CatPitSegment) -> Result<Self, String> {
    Ok(Self {
      committed: value.committed?,
      compound: value.compound?,
      docs_count: value.docs_count?,
      docs_deleted: value.docs_deleted?,
      generation: value.generation?,
      index: value.index?,
      ip: value.ip?,
      prirep: value.prirep?,
      searchable: value.searchable?,
      segment: value.segment?,
      shard: value.shard?,
      size: value.size?,
      size_memory: value.size_memory?,
      version: value.version?,
    })
  }
}

impl From<super::CatPitSegment> for CatPitSegment {
  fn from(value: super::CatPitSegment) -> Self {
    Self {
      committed: Ok(value.committed),
      compound: Ok(value.compound),
      docs_count: Ok(value.docs_count),
      docs_deleted: Ok(value.docs_deleted),
      generation: Ok(value.generation),
      index: Ok(value.index),
      ip: Ok(value.ip),
      prirep: Ok(value.prirep),
      searchable: Ok(value.searchable),
      segment: Ok(value.segment),
      shard: Ok(value.shard),
      size: Ok(value.size),
      size_memory: Ok(value.size_memory),
      version: Ok(value.version),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CatPitSegmentsBodyParams {
  pit_id: Result<Vec<String>, String>,
}

impl Default for CatPitSegmentsBodyParams {
  fn default() -> Self {
    Self {
      pit_id: Err("no value supplied for pit_id".to_string()),
    }
  }
}

impl CatPitSegmentsBodyParams {
  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }
}

impl std::convert::TryFrom<CatPitSegmentsBodyParams> for super::CatPitSegmentsBodyParams {
  type Error = String;

  fn try_from(value: CatPitSegmentsBodyParams) -> Result<Self, String> {
    Ok(Self { pit_id: value.pit_id? })
  }
}

impl From<super::CatPitSegmentsBodyParams> for CatPitSegmentsBodyParams {
  fn from(value: super::CatPitSegmentsBodyParams) -> Self {
    Self {
      pit_id: Ok(value.pit_id),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CatPitSegmentsResponseContent {
  content: Result<Option<super::CatPitSegment>, String>,
}

impl Default for CatPitSegmentsResponseContent {
  fn default() -> Self {
    Self {
      content: Ok(Default::default()),
    }
  }
}

impl CatPitSegmentsResponseContent {
  pub fn content<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::CatPitSegment>>,
    T::Error: std::fmt::Display, {
    self.content = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for content: {}", e));
    self
  }
}

impl std::convert::TryFrom<CatPitSegmentsResponseContent> for super::CatPitSegmentsResponseContent {
  type Error = String;

  fn try_from(value: CatPitSegmentsResponseContent) -> Result<Self, String> {
    Ok(Self {
      content: value.content?,
    })
  }
}

impl From<super::CatPitSegmentsResponseContent> for CatPitSegmentsResponseContent {
  fn from(value: super::CatPitSegmentsResponseContent) -> Self {
    Self {
      content: Ok(value.content),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CertificatesDetail {
  issuer_dn: Result<Option<String>, String>,
  not_after: Result<Option<String>, String>,
  not_before: Result<Option<String>, String>,
  san: Result<Option<String>, String>,
  subject_dn: Result<Option<String>, String>,
}

impl Default for CertificatesDetail {
  fn default() -> Self {
    Self {
      issuer_dn: Ok(Default::default()),
      not_after: Ok(Default::default()),
      not_before: Ok(Default::default()),
      san: Ok(Default::default()),
      subject_dn: Ok(Default::default()),
    }
  }
}

impl CertificatesDetail {
  pub fn issuer_dn<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.issuer_dn = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for issuer_dn: {}", e));
    self
  }

  pub fn not_after<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.not_after = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for not_after: {}", e));
    self
  }

  pub fn not_before<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.not_before = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for not_before: {}", e));
    self
  }

  pub fn san<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.san = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for san: {}", e));
    self
  }

  pub fn subject_dn<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.subject_dn = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for subject_dn: {}", e));
    self
  }
}

impl std::convert::TryFrom<CertificatesDetail> for super::CertificatesDetail {
  type Error = String;

  fn try_from(value: CertificatesDetail) -> Result<Self, String> {
    Ok(Self {
      issuer_dn: value.issuer_dn?,
      not_after: value.not_after?,
      not_before: value.not_before?,
      san: value.san?,
      subject_dn: value.subject_dn?,
    })
  }
}

impl From<super::CertificatesDetail> for CertificatesDetail {
  fn from(value: super::CertificatesDetail) -> Self {
    Self {
      issuer_dn: Ok(value.issuer_dn),
      not_after: Ok(value.not_after),
      not_before: Ok(value.not_before),
      san: Ok(value.san),
      subject_dn: Ok(value.subject_dn),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ChangePasswordRequestContent {
  current_password: Result<String, String>,
  password: Result<String, String>,
}

impl Default for ChangePasswordRequestContent {
  fn default() -> Self {
    Self {
      current_password: Err("no value supplied for current_password".to_string()),
      password: Err("no value supplied for password".to_string()),
    }
  }
}

impl ChangePasswordRequestContent {
  pub fn current_password<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.current_password = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for current_password: {}", e));
    self
  }

  pub fn password<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.password = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for password: {}", e));
    self
  }
}

impl std::convert::TryFrom<ChangePasswordRequestContent> for super::ChangePasswordRequestContent {
  type Error = String;

  fn try_from(value: ChangePasswordRequestContent) -> Result<Self, String> {
    Ok(Self {
      current_password: value.current_password?,
      password: value.password?,
    })
  }
}

impl From<super::ChangePasswordRequestContent> for ChangePasswordRequestContent {
  fn from(value: super::ChangePasswordRequestContent) -> Self {
    Self {
      current_password: Ok(value.current_password),
      password: Ok(value.password),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ChangePasswordResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for ChangePasswordResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl ChangePasswordResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<ChangePasswordResponseContent> for super::ChangePasswordResponseContent {
  type Error = String;

  fn try_from(value: ChangePasswordResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::ChangePasswordResponseContent> for ChangePasswordResponseContent {
  fn from(value: super::ChangePasswordResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ClusterGetSettingsResponseContent {
  defaults: Result<Option<super::UserDefinedValueMap>, String>,
  persistent: Result<Option<super::UserDefinedValueMap>, String>,
  transient: Result<Option<super::UserDefinedValueMap>, String>,
}

impl Default for ClusterGetSettingsResponseContent {
  fn default() -> Self {
    Self {
      defaults: Ok(Default::default()),
      persistent: Ok(Default::default()),
      transient: Ok(Default::default()),
    }
  }
}

impl ClusterGetSettingsResponseContent {
  pub fn defaults<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.defaults = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for defaults: {}", e));
    self
  }

  pub fn persistent<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.persistent = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for persistent: {}", e));
    self
  }

  pub fn transient<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.transient = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for transient: {}", e));
    self
  }
}

impl std::convert::TryFrom<ClusterGetSettingsResponseContent> for super::ClusterGetSettingsResponseContent {
  type Error = String;

  fn try_from(value: ClusterGetSettingsResponseContent) -> Result<Self, String> {
    Ok(Self {
      defaults: value.defaults?,
      persistent: value.persistent?,
      transient: value.transient?,
    })
  }
}

impl From<super::ClusterGetSettingsResponseContent> for ClusterGetSettingsResponseContent {
  fn from(value: super::ClusterGetSettingsResponseContent) -> Self {
    Self {
      defaults: Ok(value.defaults),
      persistent: Ok(value.persistent),
      transient: Ok(value.transient),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ClusterPutSettingsBodyParams {
  persistent: Result<Option<super::UserDefinedValueMap>, String>,
  transient: Result<Option<super::UserDefinedValueMap>, String>,
}

impl Default for ClusterPutSettingsBodyParams {
  fn default() -> Self {
    Self {
      persistent: Ok(Default::default()),
      transient: Ok(Default::default()),
    }
  }
}

impl ClusterPutSettingsBodyParams {
  pub fn persistent<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.persistent = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for persistent: {}", e));
    self
  }

  pub fn transient<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.transient = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for transient: {}", e));
    self
  }
}

impl std::convert::TryFrom<ClusterPutSettingsBodyParams> for super::ClusterPutSettingsBodyParams {
  type Error = String;

  fn try_from(value: ClusterPutSettingsBodyParams) -> Result<Self, String> {
    Ok(Self {
      persistent: value.persistent?,
      transient: value.transient?,
    })
  }
}

impl From<super::ClusterPutSettingsBodyParams> for ClusterPutSettingsBodyParams {
  fn from(value: super::ClusterPutSettingsBodyParams) -> Self {
    Self {
      persistent: Ok(value.persistent),
      transient: Ok(value.transient),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ClusterPutSettingsResponseContent {
  acknowledged: Result<Option<bool>, String>,
  persistent: Result<Option<super::UserDefinedValueMap>, String>,
  transient: Result<Option<super::UserDefinedValueMap>, String>,
}

impl Default for ClusterPutSettingsResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
      persistent: Ok(Default::default()),
      transient: Ok(Default::default()),
    }
  }
}

impl ClusterPutSettingsResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }

  pub fn persistent<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.persistent = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for persistent: {}", e));
    self
  }

  pub fn transient<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.transient = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for transient: {}", e));
    self
  }
}

impl std::convert::TryFrom<ClusterPutSettingsResponseContent> for super::ClusterPutSettingsResponseContent {
  type Error = String;

  fn try_from(value: ClusterPutSettingsResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
      persistent: value.persistent?,
      transient: value.transient?,
    })
  }
}

impl From<super::ClusterPutSettingsResponseContent> for ClusterPutSettingsResponseContent {
  fn from(value: super::ClusterPutSettingsResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
      persistent: Ok(value.persistent),
      transient: Ok(value.transient),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ComplianceConfig {
  enabled: Result<Option<bool>, String>,
  external_config: Result<Option<bool>, String>,
  internal_config: Result<Option<bool>, String>,
  read_ignore_users: Result<Vec<String>, String>,
  read_metadata_only: Result<Option<bool>, String>,
  read_watched_fields: Result<Option<serde_json::Value>, String>,
  write_ignore_users: Result<Vec<String>, String>,
  write_log_diffs: Result<Option<bool>, String>,
  write_metadata_only: Result<Option<bool>, String>,
  write_watched_indices: Result<Vec<String>, String>,
}

impl Default for ComplianceConfig {
  fn default() -> Self {
    Self {
      enabled: Ok(Default::default()),
      external_config: Ok(Default::default()),
      internal_config: Ok(Default::default()),
      read_ignore_users: Ok(Default::default()),
      read_metadata_only: Ok(Default::default()),
      read_watched_fields: Ok(Default::default()),
      write_ignore_users: Ok(Default::default()),
      write_log_diffs: Ok(Default::default()),
      write_metadata_only: Ok(Default::default()),
      write_watched_indices: Ok(Default::default()),
    }
  }
}

impl ComplianceConfig {
  pub fn enabled<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.enabled = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for enabled: {}", e));
    self
  }

  pub fn external_config<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.external_config = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for external_config: {}", e));
    self
  }

  pub fn internal_config<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.internal_config = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for internal_config: {}", e));
    self
  }

  pub fn read_ignore_users<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.read_ignore_users = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for read_ignore_users: {}", e));
    self
  }

  pub fn read_metadata_only<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.read_metadata_only = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for read_metadata_only: {}", e));
    self
  }

  pub fn read_watched_fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.read_watched_fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for read_watched_fields: {}", e));
    self
  }

  pub fn write_ignore_users<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.write_ignore_users = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for write_ignore_users: {}", e));
    self
  }

  pub fn write_log_diffs<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.write_log_diffs = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for write_log_diffs: {}", e));
    self
  }

  pub fn write_metadata_only<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.write_metadata_only = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for write_metadata_only: {}", e));
    self
  }

  pub fn write_watched_indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.write_watched_indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for write_watched_indices: {}", e));
    self
  }
}

impl std::convert::TryFrom<ComplianceConfig> for super::ComplianceConfig {
  type Error = String;

  fn try_from(value: ComplianceConfig) -> Result<Self, String> {
    Ok(Self {
      enabled: value.enabled?,
      external_config: value.external_config?,
      internal_config: value.internal_config?,
      read_ignore_users: value.read_ignore_users?,
      read_metadata_only: value.read_metadata_only?,
      read_watched_fields: value.read_watched_fields?,
      write_ignore_users: value.write_ignore_users?,
      write_log_diffs: value.write_log_diffs?,
      write_metadata_only: value.write_metadata_only?,
      write_watched_indices: value.write_watched_indices?,
    })
  }
}

impl From<super::ComplianceConfig> for ComplianceConfig {
  fn from(value: super::ComplianceConfig) -> Self {
    Self {
      enabled: Ok(value.enabled),
      external_config: Ok(value.external_config),
      internal_config: Ok(value.internal_config),
      read_ignore_users: Ok(value.read_ignore_users),
      read_metadata_only: Ok(value.read_metadata_only),
      read_watched_fields: Ok(value.read_watched_fields),
      write_ignore_users: Ok(value.write_ignore_users),
      write_log_diffs: Ok(value.write_log_diffs),
      write_metadata_only: Ok(value.write_metadata_only),
      write_watched_indices: Ok(value.write_watched_indices),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateActionGroupResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for CreateActionGroupResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl CreateActionGroupResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateActionGroupResponseContent> for super::CreateActionGroupResponseContent {
  type Error = String;

  fn try_from(value: CreateActionGroupResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::CreateActionGroupResponseContent> for CreateActionGroupResponseContent {
  fn from(value: super::CreateActionGroupResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreatePitResponseContent {
  creation_time: Result<Option<i64>, String>,
  pit_id: Result<Option<String>, String>,
  shard: Result<Option<super::ShardStatistics>, String>,
}

impl Default for CreatePitResponseContent {
  fn default() -> Self {
    Self {
      creation_time: Ok(Default::default()),
      pit_id: Ok(Default::default()),
      shard: Ok(Default::default()),
    }
  }
}

impl CreatePitResponseContent {
  pub fn creation_time<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.creation_time = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for creation_time: {}", e));
    self
  }

  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }

  pub fn shard<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ShardStatistics>>,
    T::Error: std::fmt::Display, {
    self.shard = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shard: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreatePitResponseContent> for super::CreatePitResponseContent {
  type Error = String;

  fn try_from(value: CreatePitResponseContent) -> Result<Self, String> {
    Ok(Self {
      creation_time: value.creation_time?,
      pit_id: value.pit_id?,
      shard: value.shard?,
    })
  }
}

impl From<super::CreatePitResponseContent> for CreatePitResponseContent {
  fn from(value: super::CreatePitResponseContent) -> Self {
    Self {
      creation_time: Ok(value.creation_time),
      pit_id: Ok(value.pit_id),
      shard: Ok(value.shard),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateRoleMappingResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for CreateRoleMappingResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl CreateRoleMappingResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateRoleMappingResponseContent> for super::CreateRoleMappingResponseContent {
  type Error = String;

  fn try_from(value: CreateRoleMappingResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::CreateRoleMappingResponseContent> for CreateRoleMappingResponseContent {
  fn from(value: super::CreateRoleMappingResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateRoleResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for CreateRoleResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl CreateRoleResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateRoleResponseContent> for super::CreateRoleResponseContent {
  type Error = String;

  fn try_from(value: CreateRoleResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::CreateRoleResponseContent> for CreateRoleResponseContent {
  fn from(value: super::CreateRoleResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateTenantParams {
  description: Result<Option<String>, String>,
}

impl Default for CreateTenantParams {
  fn default() -> Self {
    Self {
      description: Ok(Default::default()),
    }
  }
}

impl CreateTenantParams {
  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateTenantParams> for super::CreateTenantParams {
  type Error = String;

  fn try_from(value: CreateTenantParams) -> Result<Self, String> {
    Ok(Self {
      description: value.description?,
    })
  }
}

impl From<super::CreateTenantParams> for CreateTenantParams {
  fn from(value: super::CreateTenantParams) -> Self {
    Self {
      description: Ok(value.description),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateTenantResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for CreateTenantResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl CreateTenantResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateTenantResponseContent> for super::CreateTenantResponseContent {
  type Error = String;

  fn try_from(value: CreateTenantResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::CreateTenantResponseContent> for CreateTenantResponseContent {
  fn from(value: super::CreateTenantResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct CreateUserResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for CreateUserResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl CreateUserResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<CreateUserResponseContent> for super::CreateUserResponseContent {
  type Error = String;

  fn try_from(value: CreateUserResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::CreateUserResponseContent> for CreateUserResponseContent {
  fn from(value: super::CreateUserResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DataStream {
  generation: Result<Option<i64>, String>,
  indices: Result<Vec<super::DataStreamIndex>, String>,
  name: Result<Option<String>, String>,
  status: Result<Option<super::DataStreamStatus>, String>,
  template: Result<Option<String>, String>,
  timestamp_field: Result<Option<super::DataStreamTimestampField>, String>,
}

impl Default for DataStream {
  fn default() -> Self {
    Self {
      generation: Ok(Default::default()),
      indices: Ok(Default::default()),
      name: Ok(Default::default()),
      status: Ok(Default::default()),
      template: Ok(Default::default()),
      timestamp_field: Ok(Default::default()),
    }
  }
}

impl DataStream {
  pub fn generation<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.generation = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for generation: {}", e));
    self
  }

  pub fn indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::DataStreamIndex>>,
    T::Error: std::fmt::Display, {
    self.indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for indices: {}", e));
    self
  }

  pub fn name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for name: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::DataStreamStatus>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }

  pub fn template<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.template = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for template: {}", e));
    self
  }

  pub fn timestamp_field<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::DataStreamTimestampField>>,
    T::Error: std::fmt::Display, {
    self.timestamp_field = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timestamp_field: {}", e));
    self
  }
}

impl std::convert::TryFrom<DataStream> for super::DataStream {
  type Error = String;

  fn try_from(value: DataStream) -> Result<Self, String> {
    Ok(Self {
      generation: value.generation?,
      indices: value.indices?,
      name: value.name?,
      status: value.status?,
      template: value.template?,
      timestamp_field: value.timestamp_field?,
    })
  }
}

impl From<super::DataStream> for DataStream {
  fn from(value: super::DataStream) -> Self {
    Self {
      generation: Ok(value.generation),
      indices: Ok(value.indices),
      name: Ok(value.name),
      status: Ok(value.status),
      template: Ok(value.template),
      timestamp_field: Ok(value.timestamp_field),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DataStreamIndex {
  index_name: Result<Option<String>, String>,
  index_uuid: Result<Option<String>, String>,
}

impl Default for DataStreamIndex {
  fn default() -> Self {
    Self {
      index_name: Ok(Default::default()),
      index_uuid: Ok(Default::default()),
    }
  }
}

impl DataStreamIndex {
  pub fn index_name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index_name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index_name: {}", e));
    self
  }

  pub fn index_uuid<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index_uuid = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index_uuid: {}", e));
    self
  }
}

impl std::convert::TryFrom<DataStreamIndex> for super::DataStreamIndex {
  type Error = String;

  fn try_from(value: DataStreamIndex) -> Result<Self, String> {
    Ok(Self {
      index_name: value.index_name?,
      index_uuid: value.index_uuid?,
    })
  }
}

impl From<super::DataStreamIndex> for DataStreamIndex {
  fn from(value: super::DataStreamIndex) -> Self {
    Self {
      index_name: Ok(value.index_name),
      index_uuid: Ok(value.index_uuid),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DataStreamTimestampField {
  name: Result<Option<String>, String>,
}

impl Default for DataStreamTimestampField {
  fn default() -> Self {
    Self {
      name: Ok(Default::default()),
    }
  }
}

impl DataStreamTimestampField {
  pub fn name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for name: {}", e));
    self
  }
}

impl std::convert::TryFrom<DataStreamTimestampField> for super::DataStreamTimestampField {
  type Error = String;

  fn try_from(value: DataStreamTimestampField) -> Result<Self, String> {
    Ok(Self { name: value.name? })
  }
}

impl From<super::DataStreamTimestampField> for DataStreamTimestampField {
  fn from(value: super::DataStreamTimestampField) -> Self {
    Self { name: Ok(value.name) }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteActionGroupResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteActionGroupResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteActionGroupResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteActionGroupResponseContent> for super::DeleteActionGroupResponseContent {
  type Error = String;

  fn try_from(value: DeleteActionGroupResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteActionGroupResponseContent> for DeleteActionGroupResponseContent {
  fn from(value: super::DeleteActionGroupResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteAllPitsResponseContent {
  pits: Result<Vec<super::PitsDetailsDeleteAll>, String>,
}

impl Default for DeleteAllPitsResponseContent {
  fn default() -> Self {
    Self {
      pits: Ok(Default::default()),
    }
  }
}

impl DeleteAllPitsResponseContent {
  pub fn pits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::PitsDetailsDeleteAll>>,
    T::Error: std::fmt::Display, {
    self.pits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pits: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteAllPitsResponseContent> for super::DeleteAllPitsResponseContent {
  type Error = String;

  fn try_from(value: DeleteAllPitsResponseContent) -> Result<Self, String> {
    Ok(Self { pits: value.pits? })
  }
}

impl From<super::DeleteAllPitsResponseContent> for DeleteAllPitsResponseContent {
  fn from(value: super::DeleteAllPitsResponseContent) -> Self {
    Self { pits: Ok(value.pits) }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteDistinguishedNamesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteDistinguishedNamesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteDistinguishedNamesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteDistinguishedNamesResponseContent> for super::DeleteDistinguishedNamesResponseContent {
  type Error = String;

  fn try_from(value: DeleteDistinguishedNamesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteDistinguishedNamesResponseContent> for DeleteDistinguishedNamesResponseContent {
  fn from(value: super::DeleteDistinguishedNamesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeletePitBodyParams {
  pit_id: Result<Vec<String>, String>,
}

impl Default for DeletePitBodyParams {
  fn default() -> Self {
    Self {
      pit_id: Err("no value supplied for pit_id".to_string()),
    }
  }
}

impl DeletePitBodyParams {
  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeletePitBodyParams> for super::DeletePitBodyParams {
  type Error = String;

  fn try_from(value: DeletePitBodyParams) -> Result<Self, String> {
    Ok(Self { pit_id: value.pit_id? })
  }
}

impl From<super::DeletePitBodyParams> for DeletePitBodyParams {
  fn from(value: super::DeletePitBodyParams) -> Self {
    Self {
      pit_id: Ok(value.pit_id),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeletePitResponseContent {
  pits: Result<Vec<super::DeletedPit>, String>,
}

impl Default for DeletePitResponseContent {
  fn default() -> Self {
    Self {
      pits: Ok(Default::default()),
    }
  }
}

impl DeletePitResponseContent {
  pub fn pits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::DeletedPit>>,
    T::Error: std::fmt::Display, {
    self.pits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pits: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeletePitResponseContent> for super::DeletePitResponseContent {
  type Error = String;

  fn try_from(value: DeletePitResponseContent) -> Result<Self, String> {
    Ok(Self { pits: value.pits? })
  }
}

impl From<super::DeletePitResponseContent> for DeletePitResponseContent {
  fn from(value: super::DeletePitResponseContent) -> Self {
    Self { pits: Ok(value.pits) }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteRoleMappingResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteRoleMappingResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteRoleMappingResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteRoleMappingResponseContent> for super::DeleteRoleMappingResponseContent {
  type Error = String;

  fn try_from(value: DeleteRoleMappingResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteRoleMappingResponseContent> for DeleteRoleMappingResponseContent {
  fn from(value: super::DeleteRoleMappingResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteRoleResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteRoleResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteRoleResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteRoleResponseContent> for super::DeleteRoleResponseContent {
  type Error = String;

  fn try_from(value: DeleteRoleResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteRoleResponseContent> for DeleteRoleResponseContent {
  fn from(value: super::DeleteRoleResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteTenantResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteTenantResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteTenantResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteTenantResponseContent> for super::DeleteTenantResponseContent {
  type Error = String;

  fn try_from(value: DeleteTenantResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteTenantResponseContent> for DeleteTenantResponseContent {
  fn from(value: super::DeleteTenantResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeleteUserResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for DeleteUserResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl DeleteUserResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeleteUserResponseContent> for super::DeleteUserResponseContent {
  type Error = String;

  fn try_from(value: DeleteUserResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::DeleteUserResponseContent> for DeleteUserResponseContent {
  fn from(value: super::DeleteUserResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DeletedPit {
  pit_id: Result<Option<String>, String>,
  successful: Result<Option<bool>, String>,
}

impl Default for DeletedPit {
  fn default() -> Self {
    Self {
      pit_id: Ok(Default::default()),
      successful: Ok(Default::default()),
    }
  }
}

impl DeletedPit {
  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }

  pub fn successful<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.successful = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for successful: {}", e));
    self
  }
}

impl std::convert::TryFrom<DeletedPit> for super::DeletedPit {
  type Error = String;

  fn try_from(value: DeletedPit) -> Result<Self, String> {
    Ok(Self {
      pit_id: value.pit_id?,
      successful: value.successful?,
    })
  }
}

impl From<super::DeletedPit> for DeletedPit {
  fn from(value: super::DeletedPit) -> Self {
    Self {
      pit_id: Ok(value.pit_id),
      successful: Ok(value.successful),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DistinguishedNames {
  nodes_dn: Result<Vec<String>, String>,
}

impl Default for DistinguishedNames {
  fn default() -> Self {
    Self {
      nodes_dn: Ok(Default::default()),
    }
  }
}

impl DistinguishedNames {
  pub fn nodes_dn<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.nodes_dn = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for nodes_dn: {}", e));
    self
  }
}

impl std::convert::TryFrom<DistinguishedNames> for super::DistinguishedNames {
  type Error = String;

  fn try_from(value: DistinguishedNames) -> Result<Self, String> {
    Ok(Self {
      nodes_dn: value.nodes_dn?,
    })
  }
}

impl From<super::DistinguishedNames> for DistinguishedNames {
  fn from(value: super::DistinguishedNames) -> Self {
    Self {
      nodes_dn: Ok(value.nodes_dn),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DynamicConfig {
  dynamic: Result<Option<super::DynamicOptions>, String>,
}

impl Default for DynamicConfig {
  fn default() -> Self {
    Self {
      dynamic: Ok(Default::default()),
    }
  }
}

impl DynamicConfig {
  pub fn dynamic<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::DynamicOptions>>,
    T::Error: std::fmt::Display, {
    self.dynamic = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for dynamic: {}", e));
    self
  }
}

impl std::convert::TryFrom<DynamicConfig> for super::DynamicConfig {
  type Error = String;

  fn try_from(value: DynamicConfig) -> Result<Self, String> {
    Ok(Self {
      dynamic: value.dynamic?,
    })
  }
}

impl From<super::DynamicConfig> for DynamicConfig {
  fn from(value: super::DynamicConfig) -> Self {
    Self {
      dynamic: Ok(value.dynamic),
    }
  }
}

#[derive(Clone, Debug)]
pub struct DynamicOptions {
  auth_failure_listeners: Result<Option<serde_json::Value>, String>,
  authc: Result<Option<serde_json::Value>, String>,
  authz: Result<Option<serde_json::Value>, String>,
  disable_intertransport_auth: Result<Option<bool>, String>,
  disable_rest_auth: Result<Option<bool>, String>,
  do_not_fail_on_forbidden: Result<Option<bool>, String>,
  do_not_fail_on_forbidden_empty: Result<Option<bool>, String>,
  filtered_alias_mode: Result<Option<String>, String>,
  hosts_resolver_mode: Result<Option<String>, String>,
  http: Result<Option<serde_json::Value>, String>,
  kibana: Result<Option<serde_json::Value>, String>,
  multi_rolespan_enabled: Result<Option<bool>, String>,
  respect_request_indices_options: Result<Option<bool>, String>,
}

impl Default for DynamicOptions {
  fn default() -> Self {
    Self {
      auth_failure_listeners: Ok(Default::default()),
      authc: Ok(Default::default()),
      authz: Ok(Default::default()),
      disable_intertransport_auth: Ok(Default::default()),
      disable_rest_auth: Ok(Default::default()),
      do_not_fail_on_forbidden: Ok(Default::default()),
      do_not_fail_on_forbidden_empty: Ok(Default::default()),
      filtered_alias_mode: Ok(Default::default()),
      hosts_resolver_mode: Ok(Default::default()),
      http: Ok(Default::default()),
      kibana: Ok(Default::default()),
      multi_rolespan_enabled: Ok(Default::default()),
      respect_request_indices_options: Ok(Default::default()),
    }
  }
}

impl DynamicOptions {
  pub fn auth_failure_listeners<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.auth_failure_listeners = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for auth_failure_listeners: {}", e));
    self
  }

  pub fn authc<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.authc = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for authc: {}", e));
    self
  }

  pub fn authz<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.authz = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for authz: {}", e));
    self
  }

  pub fn disable_intertransport_auth<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.disable_intertransport_auth = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for disable_intertransport_auth: {}", e));
    self
  }

  pub fn disable_rest_auth<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.disable_rest_auth = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for disable_rest_auth: {}", e));
    self
  }

  pub fn do_not_fail_on_forbidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.do_not_fail_on_forbidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for do_not_fail_on_forbidden: {}", e));
    self
  }

  pub fn do_not_fail_on_forbidden_empty<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.do_not_fail_on_forbidden_empty = value.try_into().map_err(|e| {
      format!(
        "error converting supplied value for do_not_fail_on_forbidden_empty: {}",
        e
      )
    });
    self
  }

  pub fn filtered_alias_mode<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.filtered_alias_mode = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for filtered_alias_mode: {}", e));
    self
  }

  pub fn hosts_resolver_mode<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.hosts_resolver_mode = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hosts_resolver_mode: {}", e));
    self
  }

  pub fn http<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.http = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for http: {}", e));
    self
  }

  pub fn kibana<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.kibana = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for kibana: {}", e));
    self
  }

  pub fn multi_rolespan_enabled<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.multi_rolespan_enabled = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for multi_rolespan_enabled: {}", e));
    self
  }

  pub fn respect_request_indices_options<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.respect_request_indices_options = value.try_into().map_err(|e| {
      format!(
        "error converting supplied value for respect_request_indices_options: {}",
        e
      )
    });
    self
  }
}

impl std::convert::TryFrom<DynamicOptions> for super::DynamicOptions {
  type Error = String;

  fn try_from(value: DynamicOptions) -> Result<Self, String> {
    Ok(Self {
      auth_failure_listeners: value.auth_failure_listeners?,
      authc: value.authc?,
      authz: value.authz?,
      disable_intertransport_auth: value.disable_intertransport_auth?,
      disable_rest_auth: value.disable_rest_auth?,
      do_not_fail_on_forbidden: value.do_not_fail_on_forbidden?,
      do_not_fail_on_forbidden_empty: value.do_not_fail_on_forbidden_empty?,
      filtered_alias_mode: value.filtered_alias_mode?,
      hosts_resolver_mode: value.hosts_resolver_mode?,
      http: value.http?,
      kibana: value.kibana?,
      multi_rolespan_enabled: value.multi_rolespan_enabled?,
      respect_request_indices_options: value.respect_request_indices_options?,
    })
  }
}

impl From<super::DynamicOptions> for DynamicOptions {
  fn from(value: super::DynamicOptions) -> Self {
    Self {
      auth_failure_listeners: Ok(value.auth_failure_listeners),
      authc: Ok(value.authc),
      authz: Ok(value.authz),
      disable_intertransport_auth: Ok(value.disable_intertransport_auth),
      disable_rest_auth: Ok(value.disable_rest_auth),
      do_not_fail_on_forbidden: Ok(value.do_not_fail_on_forbidden),
      do_not_fail_on_forbidden_empty: Ok(value.do_not_fail_on_forbidden_empty),
      filtered_alias_mode: Ok(value.filtered_alias_mode),
      hosts_resolver_mode: Ok(value.hosts_resolver_mode),
      http: Ok(value.http),
      kibana: Ok(value.kibana),
      multi_rolespan_enabled: Ok(value.multi_rolespan_enabled),
      respect_request_indices_options: Ok(value.respect_request_indices_options),
    }
  }
}

#[derive(Clone, Debug)]
pub struct FlushCacheResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for FlushCacheResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl FlushCacheResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<FlushCacheResponseContent> for super::FlushCacheResponseContent {
  type Error = String;

  fn try_from(value: FlushCacheResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::FlushCacheResponseContent> for FlushCacheResponseContent {
  fn from(value: super::FlushCacheResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct GetAllPitsResponseContent {
  pits: Result<Vec<super::PitDetail>, String>,
}

impl Default for GetAllPitsResponseContent {
  fn default() -> Self {
    Self {
      pits: Ok(Default::default()),
    }
  }
}

impl GetAllPitsResponseContent {
  pub fn pits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::PitDetail>>,
    T::Error: std::fmt::Display, {
    self.pits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pits: {}", e));
    self
  }
}

impl std::convert::TryFrom<GetAllPitsResponseContent> for super::GetAllPitsResponseContent {
  type Error = String;

  fn try_from(value: GetAllPitsResponseContent) -> Result<Self, String> {
    Ok(Self { pits: value.pits? })
  }
}

impl From<super::GetAllPitsResponseContent> for GetAllPitsResponseContent {
  fn from(value: super::GetAllPitsResponseContent) -> Self {
    Self { pits: Ok(value.pits) }
  }
}

#[derive(Clone, Debug)]
pub struct GetCertificatesResponseContent {
  http_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
  transport_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
}

impl Default for GetCertificatesResponseContent {
  fn default() -> Self {
    Self {
      http_certificates_list: Ok(Default::default()),
      transport_certificates_list: Ok(Default::default()),
    }
  }
}

impl GetCertificatesResponseContent {
  pub fn http_certificates_list<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
    T::Error: std::fmt::Display, {
    self.http_certificates_list = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for http_certificates_list: {}", e));
    self
  }

  pub fn transport_certificates_list<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
    T::Error: std::fmt::Display, {
    self.transport_certificates_list = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for transport_certificates_list: {}", e));
    self
  }
}

impl std::convert::TryFrom<GetCertificatesResponseContent> for super::GetCertificatesResponseContent {
  type Error = String;

  fn try_from(value: GetCertificatesResponseContent) -> Result<Self, String> {
    Ok(Self {
      http_certificates_list: value.http_certificates_list?,
      transport_certificates_list: value.transport_certificates_list?,
    })
  }
}

impl From<super::GetCertificatesResponseContent> for GetCertificatesResponseContent {
  fn from(value: super::GetCertificatesResponseContent) -> Self {
    Self {
      http_certificates_list: Ok(value.http_certificates_list),
      transport_certificates_list: Ok(value.transport_certificates_list),
    }
  }
}

#[derive(Clone, Debug)]
pub struct GetResponseContent {
  fields: Result<Option<super::UserDefinedValueMap>, String>,
  found: Result<bool, String>,
  id: Result<String, String>,
  index: Result<String, String>,
  primary_term: Result<Option<i64>, String>,
  routing: Result<Option<String>, String>,
  seq_no: Result<Option<i64>, String>,
  source: Result<Option<super::UserDefinedValueMap>, String>,
  type_: Result<Option<String>, String>,
  version: Result<Option<i32>, String>,
}

impl Default for GetResponseContent {
  fn default() -> Self {
    Self {
      fields: Ok(Default::default()),
      found: Err("no value supplied for found".to_string()),
      id: Err("no value supplied for id".to_string()),
      index: Err("no value supplied for index".to_string()),
      primary_term: Ok(Default::default()),
      routing: Ok(Default::default()),
      seq_no: Ok(Default::default()),
      source: Ok(Default::default()),
      type_: Ok(Default::default()),
      version: Ok(Default::default()),
    }
  }
}

impl GetResponseContent {
  pub fn fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for fields: {}", e));
    self
  }

  pub fn found<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<bool>,
    T::Error: std::fmt::Display, {
    self.found = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for found: {}", e));
    self
  }

  pub fn id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for id: {}", e));
    self
  }

  pub fn index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index: {}", e));
    self
  }

  pub fn primary_term<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.primary_term = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for primary_term: {}", e));
    self
  }

  pub fn routing<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.routing = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for routing: {}", e));
    self
  }

  pub fn seq_no<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.seq_no = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for seq_no: {}", e));
    self
  }

  pub fn source<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.source = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for source: {}", e));
    self
  }

  pub fn type_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.type_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for type_: {}", e));
    self
  }

  pub fn version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.version = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for version: {}", e));
    self
  }
}

impl std::convert::TryFrom<GetResponseContent> for super::GetResponseContent {
  type Error = String;

  fn try_from(value: GetResponseContent) -> Result<Self, String> {
    Ok(Self {
      fields: value.fields?,
      found: value.found?,
      id: value.id?,
      index: value.index?,
      primary_term: value.primary_term?,
      routing: value.routing?,
      seq_no: value.seq_no?,
      source: value.source?,
      type_: value.type_?,
      version: value.version?,
    })
  }
}

impl From<super::GetResponseContent> for GetResponseContent {
  fn from(value: super::GetResponseContent) -> Self {
    Self {
      fields: Ok(value.fields),
      found: Ok(value.found),
      id: Ok(value.id),
      index: Ok(value.index),
      primary_term: Ok(value.primary_term),
      routing: Ok(value.routing),
      seq_no: Ok(value.seq_no),
      source: Ok(value.source),
      type_: Ok(value.type_),
      version: Ok(value.version),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Hits {
  fields: Result<Option<serde_json::Value>, String>,
  id: Result<Option<String>, String>,
  index: Result<Option<String>, String>,
  score: Result<Option<f64>, String>,
  source: Result<Option<serde_json::Value>, String>,
  type_: Result<Option<String>, String>,
}

impl Default for Hits {
  fn default() -> Self {
    Self {
      fields: Ok(Default::default()),
      id: Ok(Default::default()),
      index: Ok(Default::default()),
      score: Ok(Default::default()),
      source: Ok(Default::default()),
      type_: Ok(Default::default()),
    }
  }
}

impl Hits {
  pub fn fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for fields: {}", e));
    self
  }

  pub fn id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for id: {}", e));
    self
  }

  pub fn index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index: {}", e));
    self
  }

  pub fn score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<f64>>,
    T::Error: std::fmt::Display, {
    self.score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for score: {}", e));
    self
  }

  pub fn source<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.source = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for source: {}", e));
    self
  }

  pub fn type_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.type_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for type_: {}", e));
    self
  }
}

impl std::convert::TryFrom<Hits> for super::Hits {
  type Error = String;

  fn try_from(value: Hits) -> Result<Self, String> {
    Ok(Self {
      fields: value.fields?,
      id: value.id?,
      index: value.index?,
      score: value.score?,
      source: value.source?,
      type_: value.type_?,
    })
  }
}

impl From<super::Hits> for Hits {
  fn from(value: super::Hits) -> Self {
    Self {
      fields: Ok(value.fields),
      id: Ok(value.id),
      index: Ok(value.index),
      score: Ok(value.score),
      source: Ok(value.source),
      type_: Ok(value.type_),
    }
  }
}

#[derive(Clone, Debug)]
pub struct HitsMetadata {
  hits: Result<Vec<super::Hits>, String>,
  max_score: Result<Option<f64>, String>,
  total: Result<Option<super::Total>, String>,
}

impl Default for HitsMetadata {
  fn default() -> Self {
    Self {
      hits: Ok(Default::default()),
      max_score: Ok(Default::default()),
      total: Ok(Default::default()),
    }
  }
}

impl HitsMetadata {
  pub fn hits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::Hits>>,
    T::Error: std::fmt::Display, {
    self.hits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hits: {}", e));
    self
  }

  pub fn max_score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<f64>>,
    T::Error: std::fmt::Display, {
    self.max_score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for max_score: {}", e));
    self
  }

  pub fn total<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::Total>>,
    T::Error: std::fmt::Display, {
    self.total = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for total: {}", e));
    self
  }
}

impl std::convert::TryFrom<HitsMetadata> for super::HitsMetadata {
  type Error = String;

  fn try_from(value: HitsMetadata) -> Result<Self, String> {
    Ok(Self {
      hits: value.hits?,
      max_score: value.max_score?,
      total: value.total?,
    })
  }
}

impl From<super::HitsMetadata> for HitsMetadata {
  fn from(value: super::HitsMetadata) -> Self {
    Self {
      hits: Ok(value.hits),
      max_score: Ok(value.max_score),
      total: Ok(value.total),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndexPermission {
  allowed_actions: Result<Vec<String>, String>,
  fls: Result<Vec<String>, String>,
  index_patterns: Result<Vec<String>, String>,
  masked_fields: Result<Vec<String>, String>,
}

impl Default for IndexPermission {
  fn default() -> Self {
    Self {
      allowed_actions: Ok(Default::default()),
      fls: Ok(Default::default()),
      index_patterns: Ok(Default::default()),
      masked_fields: Ok(Default::default()),
    }
  }
}

impl IndexPermission {
  pub fn allowed_actions<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.allowed_actions = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
    self
  }

  pub fn fls<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.fls = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for fls: {}", e));
    self
  }

  pub fn index_patterns<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.index_patterns = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index_patterns: {}", e));
    self
  }

  pub fn masked_fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.masked_fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for masked_fields: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndexPermission> for super::IndexPermission {
  type Error = String;

  fn try_from(value: IndexPermission) -> Result<Self, String> {
    Ok(Self {
      allowed_actions: value.allowed_actions?,
      fls: value.fls?,
      index_patterns: value.index_patterns?,
      masked_fields: value.masked_fields?,
    })
  }
}

impl From<super::IndexPermission> for IndexPermission {
  fn from(value: super::IndexPermission) -> Self {
    Self {
      allowed_actions: Ok(value.allowed_actions),
      fls: Ok(value.fls),
      index_patterns: Ok(value.index_patterns),
      masked_fields: Ok(value.masked_fields),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesCreateBodyParams {
  aliases: Result<Option<super::UserDefinedValueMap>, String>,
  mapping: Result<Option<super::UserDefinedValueMap>, String>,
  settings: Result<Option<super::UserDefinedValueMap>, String>,
}

impl Default for IndicesCreateBodyParams {
  fn default() -> Self {
    Self {
      aliases: Ok(Default::default()),
      mapping: Ok(Default::default()),
      settings: Ok(Default::default()),
    }
  }
}

impl IndicesCreateBodyParams {
  pub fn aliases<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.aliases = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for aliases: {}", e));
    self
  }

  pub fn mapping<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.mapping = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for mapping: {}", e));
    self
  }

  pub fn settings<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.settings = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for settings: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesCreateBodyParams> for super::IndicesCreateBodyParams {
  type Error = String;

  fn try_from(value: IndicesCreateBodyParams) -> Result<Self, String> {
    Ok(Self {
      aliases: value.aliases?,
      mapping: value.mapping?,
      settings: value.settings?,
    })
  }
}

impl From<super::IndicesCreateBodyParams> for IndicesCreateBodyParams {
  fn from(value: super::IndicesCreateBodyParams) -> Self {
    Self {
      aliases: Ok(value.aliases),
      mapping: Ok(value.mapping),
      settings: Ok(value.settings),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesCreateDataStreamResponseContent {
  acknowledged: Result<Option<bool>, String>,
}

impl Default for IndicesCreateDataStreamResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
    }
  }
}

impl IndicesCreateDataStreamResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesCreateDataStreamResponseContent> for super::IndicesCreateDataStreamResponseContent {
  type Error = String;

  fn try_from(value: IndicesCreateDataStreamResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesCreateDataStreamResponseContent> for IndicesCreateDataStreamResponseContent {
  fn from(value: super::IndicesCreateDataStreamResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesCreateResponseContent {
  acknowledged: Result<bool, String>,
  index: Result<String, String>,
  shards_acknowledged: Result<bool, String>,
}

impl Default for IndicesCreateResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Err("no value supplied for acknowledged".to_string()),
      index: Err("no value supplied for index".to_string()),
      shards_acknowledged: Err("no value supplied for shards_acknowledged".to_string()),
    }
  }
}

impl IndicesCreateResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<bool>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }

  pub fn index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index: {}", e));
    self
  }

  pub fn shards_acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<bool>,
    T::Error: std::fmt::Display, {
    self.shards_acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards_acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesCreateResponseContent> for super::IndicesCreateResponseContent {
  type Error = String;

  fn try_from(value: IndicesCreateResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
      index: value.index?,
      shards_acknowledged: value.shards_acknowledged?,
    })
  }
}

impl From<super::IndicesCreateResponseContent> for IndicesCreateResponseContent {
  fn from(value: super::IndicesCreateResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
      index: Ok(value.index),
      shards_acknowledged: Ok(value.shards_acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesDeleteDataStreamResponseContent {
  acknowledged: Result<Option<bool>, String>,
}

impl Default for IndicesDeleteDataStreamResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
    }
  }
}

impl IndicesDeleteDataStreamResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesDeleteDataStreamResponseContent> for super::IndicesDeleteDataStreamResponseContent {
  type Error = String;

  fn try_from(value: IndicesDeleteDataStreamResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesDeleteDataStreamResponseContent> for IndicesDeleteDataStreamResponseContent {
  fn from(value: super::IndicesDeleteDataStreamResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesDeleteResponseContent {
  acknowledged: Result<Option<bool>, String>,
}

impl Default for IndicesDeleteResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
    }
  }
}

impl IndicesDeleteResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesDeleteResponseContent> for super::IndicesDeleteResponseContent {
  type Error = String;

  fn try_from(value: IndicesDeleteResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesDeleteResponseContent> for IndicesDeleteResponseContent {
  fn from(value: super::IndicesDeleteResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesGetDataStreamResponseContent {
  data_streams: Result<Vec<super::DataStream>, String>,
}

impl Default for IndicesGetDataStreamResponseContent {
  fn default() -> Self {
    Self {
      data_streams: Ok(Default::default()),
    }
  }
}

impl IndicesGetDataStreamResponseContent {
  pub fn data_streams<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::DataStream>>,
    T::Error: std::fmt::Display, {
    self.data_streams = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for data_streams: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesGetDataStreamResponseContent> for super::IndicesGetDataStreamResponseContent {
  type Error = String;

  fn try_from(value: IndicesGetDataStreamResponseContent) -> Result<Self, String> {
    Ok(Self {
      data_streams: value.data_streams?,
    })
  }
}

impl From<super::IndicesGetDataStreamResponseContent> for IndicesGetDataStreamResponseContent {
  fn from(value: super::IndicesGetDataStreamResponseContent) -> Self {
    Self {
      data_streams: Ok(value.data_streams),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesGetDataStreamWithNameResponseContent {
  data_streams: Result<Vec<super::DataStream>, String>,
}

impl Default for IndicesGetDataStreamWithNameResponseContent {
  fn default() -> Self {
    Self {
      data_streams: Ok(Default::default()),
    }
  }
}

impl IndicesGetDataStreamWithNameResponseContent {
  pub fn data_streams<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<super::DataStream>>,
    T::Error: std::fmt::Display, {
    self.data_streams = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for data_streams: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesGetDataStreamWithNameResponseContent>
  for super::IndicesGetDataStreamWithNameResponseContent
{
  type Error = String;

  fn try_from(value: IndicesGetDataStreamWithNameResponseContent) -> Result<Self, String> {
    Ok(Self {
      data_streams: value.data_streams?,
    })
  }
}

impl From<super::IndicesGetDataStreamWithNameResponseContent> for IndicesGetDataStreamWithNameResponseContent {
  fn from(value: super::IndicesGetDataStreamWithNameResponseContent) -> Self {
    Self {
      data_streams: Ok(value.data_streams),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesPutMappingPostResponseContent {
  acknowledged: Result<Option<bool>, String>,
}

impl Default for IndicesPutMappingPostResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
    }
  }
}

impl IndicesPutMappingPostResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesPutMappingPostResponseContent> for super::IndicesPutMappingPostResponseContent {
  type Error = String;

  fn try_from(value: IndicesPutMappingPostResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesPutMappingPostResponseContent> for IndicesPutMappingPostResponseContent {
  fn from(value: super::IndicesPutMappingPostResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesPutMappingPutResponseContent {
  acknowledged: Result<Option<bool>, String>,
}

impl Default for IndicesPutMappingPutResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Ok(Default::default()),
    }
  }
}

impl IndicesPutMappingPutResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesPutMappingPutResponseContent> for super::IndicesPutMappingPutResponseContent {
  type Error = String;

  fn try_from(value: IndicesPutMappingPutResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesPutMappingPutResponseContent> for IndicesPutMappingPutResponseContent {
  fn from(value: super::IndicesPutMappingPutResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesUpdateAliasesBodyParams {
  actions: Result<Option<super::ActionObjectStructure>, String>,
}

impl Default for IndicesUpdateAliasesBodyParams {
  fn default() -> Self {
    Self {
      actions: Ok(Default::default()),
    }
  }
}

impl IndicesUpdateAliasesBodyParams {
  pub fn actions<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ActionObjectStructure>>,
    T::Error: std::fmt::Display, {
    self.actions = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for actions: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesUpdateAliasesBodyParams> for super::IndicesUpdateAliasesBodyParams {
  type Error = String;

  fn try_from(value: IndicesUpdateAliasesBodyParams) -> Result<Self, String> {
    Ok(Self {
      actions: value.actions?,
    })
  }
}

impl From<super::IndicesUpdateAliasesBodyParams> for IndicesUpdateAliasesBodyParams {
  fn from(value: super::IndicesUpdateAliasesBodyParams) -> Self {
    Self {
      actions: Ok(value.actions),
    }
  }
}

#[derive(Clone, Debug)]
pub struct IndicesUpdateAliasesResponseContent {
  acknowledged: Result<bool, String>,
}

impl Default for IndicesUpdateAliasesResponseContent {
  fn default() -> Self {
    Self {
      acknowledged: Err("no value supplied for acknowledged".to_string()),
    }
  }
}

impl IndicesUpdateAliasesResponseContent {
  pub fn acknowledged<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<bool>,
    T::Error: std::fmt::Display, {
    self.acknowledged = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
    self
  }
}

impl std::convert::TryFrom<IndicesUpdateAliasesResponseContent> for super::IndicesUpdateAliasesResponseContent {
  type Error = String;

  fn try_from(value: IndicesUpdateAliasesResponseContent) -> Result<Self, String> {
    Ok(Self {
      acknowledged: value.acknowledged?,
    })
  }
}

impl From<super::IndicesUpdateAliasesResponseContent> for IndicesUpdateAliasesResponseContent {
  fn from(value: super::IndicesUpdateAliasesResponseContent) -> Self {
    Self {
      acknowledged: Ok(value.acknowledged),
    }
  }
}

#[derive(Clone, Debug)]
pub struct InfoResponseContent {
  cluster_name: Result<Option<String>, String>,
  cluster_uuid: Result<Option<String>, String>,
  name: Result<Option<String>, String>,
  tagline: Result<Option<String>, String>,
  version: Result<Option<super::InfoVersion>, String>,
}

impl Default for InfoResponseContent {
  fn default() -> Self {
    Self {
      cluster_name: Ok(Default::default()),
      cluster_uuid: Ok(Default::default()),
      name: Ok(Default::default()),
      tagline: Ok(Default::default()),
      version: Ok(Default::default()),
    }
  }
}

impl InfoResponseContent {
  pub fn cluster_name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.cluster_name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for cluster_name: {}", e));
    self
  }

  pub fn cluster_uuid<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.cluster_uuid = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for cluster_uuid: {}", e));
    self
  }

  pub fn name<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.name = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for name: {}", e));
    self
  }

  pub fn tagline<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.tagline = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for tagline: {}", e));
    self
  }

  pub fn version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::InfoVersion>>,
    T::Error: std::fmt::Display, {
    self.version = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for version: {}", e));
    self
  }
}

impl std::convert::TryFrom<InfoResponseContent> for super::InfoResponseContent {
  type Error = String;

  fn try_from(value: InfoResponseContent) -> Result<Self, String> {
    Ok(Self {
      cluster_name: value.cluster_name?,
      cluster_uuid: value.cluster_uuid?,
      name: value.name?,
      tagline: value.tagline?,
      version: value.version?,
    })
  }
}

impl From<super::InfoResponseContent> for InfoResponseContent {
  fn from(value: super::InfoResponseContent) -> Self {
    Self {
      cluster_name: Ok(value.cluster_name),
      cluster_uuid: Ok(value.cluster_uuid),
      name: Ok(value.name),
      tagline: Ok(value.tagline),
      version: Ok(value.version),
    }
  }
}

#[derive(Clone, Debug)]
pub struct InfoVersion {
  build_date: Result<Option<String>, String>,
  build_hash: Result<Option<String>, String>,
  build_snapshot: Result<Option<bool>, String>,
  build_type: Result<Option<String>, String>,
  distribution: Result<Option<String>, String>,
  lucene_version: Result<Option<String>, String>,
  minimum_index_compatibility_version: Result<Option<String>, String>,
  minimum_wire_compatibility_version: Result<Option<String>, String>,
  number: Result<Option<String>, String>,
}

impl Default for InfoVersion {
  fn default() -> Self {
    Self {
      build_date: Ok(Default::default()),
      build_hash: Ok(Default::default()),
      build_snapshot: Ok(Default::default()),
      build_type: Ok(Default::default()),
      distribution: Ok(Default::default()),
      lucene_version: Ok(Default::default()),
      minimum_index_compatibility_version: Ok(Default::default()),
      minimum_wire_compatibility_version: Ok(Default::default()),
      number: Ok(Default::default()),
    }
  }
}

impl InfoVersion {
  pub fn build_date<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.build_date = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for build_date: {}", e));
    self
  }

  pub fn build_hash<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.build_hash = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for build_hash: {}", e));
    self
  }

  pub fn build_snapshot<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.build_snapshot = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for build_snapshot: {}", e));
    self
  }

  pub fn build_type<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.build_type = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for build_type: {}", e));
    self
  }

  pub fn distribution<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.distribution = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for distribution: {}", e));
    self
  }

  pub fn lucene_version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.lucene_version = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for lucene_version: {}", e));
    self
  }

  pub fn minimum_index_compatibility_version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.minimum_index_compatibility_version = value.try_into().map_err(|e| {
      format!(
        "error converting supplied value for minimum_index_compatibility_version: {}",
        e
      )
    });
    self
  }

  pub fn minimum_wire_compatibility_version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.minimum_wire_compatibility_version = value.try_into().map_err(|e| {
      format!(
        "error converting supplied value for minimum_wire_compatibility_version: {}",
        e
      )
    });
    self
  }

  pub fn number<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.number = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for number: {}", e));
    self
  }
}

impl std::convert::TryFrom<InfoVersion> for super::InfoVersion {
  type Error = String;

  fn try_from(value: InfoVersion) -> Result<Self, String> {
    Ok(Self {
      build_date: value.build_date?,
      build_hash: value.build_hash?,
      build_snapshot: value.build_snapshot?,
      build_type: value.build_type?,
      distribution: value.distribution?,
      lucene_version: value.lucene_version?,
      minimum_index_compatibility_version: value.minimum_index_compatibility_version?,
      minimum_wire_compatibility_version: value.minimum_wire_compatibility_version?,
      number: value.number?,
    })
  }
}

impl From<super::InfoVersion> for InfoVersion {
  fn from(value: super::InfoVersion) -> Self {
    Self {
      build_date: Ok(value.build_date),
      build_hash: Ok(value.build_hash),
      build_snapshot: Ok(value.build_snapshot),
      build_type: Ok(value.build_type),
      distribution: Ok(value.distribution),
      lucene_version: Ok(value.lucene_version),
      minimum_index_compatibility_version: Ok(value.minimum_index_compatibility_version),
      minimum_wire_compatibility_version: Ok(value.minimum_wire_compatibility_version),
      number: Ok(value.number),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchActionGroupResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchActionGroupResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchActionGroupResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchActionGroupResponseContent> for super::PatchActionGroupResponseContent {
  type Error = String;

  fn try_from(value: PatchActionGroupResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchActionGroupResponseContent> for PatchActionGroupResponseContent {
  fn from(value: super::PatchActionGroupResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchActionGroupsResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchActionGroupsResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchActionGroupsResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchActionGroupsResponseContent> for super::PatchActionGroupsResponseContent {
  type Error = String;

  fn try_from(value: PatchActionGroupsResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchActionGroupsResponseContent> for PatchActionGroupsResponseContent {
  fn from(value: super::PatchActionGroupsResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchConfigurationResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchConfigurationResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchConfigurationResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchConfigurationResponseContent> for super::PatchConfigurationResponseContent {
  type Error = String;

  fn try_from(value: PatchConfigurationResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchConfigurationResponseContent> for PatchConfigurationResponseContent {
  fn from(value: super::PatchConfigurationResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchDistinguishedNamesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchDistinguishedNamesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchDistinguishedNamesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchDistinguishedNamesResponseContent> for super::PatchDistinguishedNamesResponseContent {
  type Error = String;

  fn try_from(value: PatchDistinguishedNamesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchDistinguishedNamesResponseContent> for PatchDistinguishedNamesResponseContent {
  fn from(value: super::PatchDistinguishedNamesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchOperation {
  op: Result<String, String>,
  path: Result<String, String>,
  value: Result<Option<serde_json::Value>, String>,
}

impl Default for PatchOperation {
  fn default() -> Self {
    Self {
      op: Err("no value supplied for op".to_string()),
      path: Err("no value supplied for path".to_string()),
      value: Ok(Default::default()),
    }
  }
}

impl PatchOperation {
  pub fn op<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.op = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for op: {}", e));
    self
  }

  pub fn path<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<String>,
    T::Error: std::fmt::Display, {
    self.path = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for path: {}", e));
    self
  }

  pub fn value<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.value = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for value: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchOperation> for super::PatchOperation {
  type Error = String;

  fn try_from(value: PatchOperation) -> Result<Self, String> {
    Ok(Self {
      op: value.op?,
      path: value.path?,
      value: value.value?,
    })
  }
}

impl From<super::PatchOperation> for PatchOperation {
  fn from(value: super::PatchOperation) -> Self {
    Self {
      op: Ok(value.op),
      path: Ok(value.path),
      value: Ok(value.value),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchRoleMappingResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchRoleMappingResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchRoleMappingResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchRoleMappingResponseContent> for super::PatchRoleMappingResponseContent {
  type Error = String;

  fn try_from(value: PatchRoleMappingResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchRoleMappingResponseContent> for PatchRoleMappingResponseContent {
  fn from(value: super::PatchRoleMappingResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchRoleMappingsResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchRoleMappingsResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchRoleMappingsResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchRoleMappingsResponseContent> for super::PatchRoleMappingsResponseContent {
  type Error = String;

  fn try_from(value: PatchRoleMappingsResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchRoleMappingsResponseContent> for PatchRoleMappingsResponseContent {
  fn from(value: super::PatchRoleMappingsResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchRoleResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchRoleResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchRoleResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchRoleResponseContent> for super::PatchRoleResponseContent {
  type Error = String;

  fn try_from(value: PatchRoleResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchRoleResponseContent> for PatchRoleResponseContent {
  fn from(value: super::PatchRoleResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchRolesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchRolesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchRolesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchRolesResponseContent> for super::PatchRolesResponseContent {
  type Error = String;

  fn try_from(value: PatchRolesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchRolesResponseContent> for PatchRolesResponseContent {
  fn from(value: super::PatchRolesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchTenantResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchTenantResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchTenantResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchTenantResponseContent> for super::PatchTenantResponseContent {
  type Error = String;

  fn try_from(value: PatchTenantResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchTenantResponseContent> for PatchTenantResponseContent {
  fn from(value: super::PatchTenantResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchTenantsResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchTenantsResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchTenantsResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchTenantsResponseContent> for super::PatchTenantsResponseContent {
  type Error = String;

  fn try_from(value: PatchTenantsResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchTenantsResponseContent> for PatchTenantsResponseContent {
  fn from(value: super::PatchTenantsResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchUserResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchUserResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchUserResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchUserResponseContent> for super::PatchUserResponseContent {
  type Error = String;

  fn try_from(value: PatchUserResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchUserResponseContent> for PatchUserResponseContent {
  fn from(value: super::PatchUserResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PatchUsersResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for PatchUsersResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl PatchUsersResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<PatchUsersResponseContent> for super::PatchUsersResponseContent {
  type Error = String;

  fn try_from(value: PatchUsersResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::PatchUsersResponseContent> for PatchUsersResponseContent {
  fn from(value: super::PatchUsersResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PitDetail {
  creation_time: Result<Option<i64>, String>,
  keep_alive: Result<Option<i64>, String>,
  pit_id: Result<Option<String>, String>,
}

impl Default for PitDetail {
  fn default() -> Self {
    Self {
      creation_time: Ok(Default::default()),
      keep_alive: Ok(Default::default()),
      pit_id: Ok(Default::default()),
    }
  }
}

impl PitDetail {
  pub fn creation_time<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.creation_time = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for creation_time: {}", e));
    self
  }

  pub fn keep_alive<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.keep_alive = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for keep_alive: {}", e));
    self
  }

  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }
}

impl std::convert::TryFrom<PitDetail> for super::PitDetail {
  type Error = String;

  fn try_from(value: PitDetail) -> Result<Self, String> {
    Ok(Self {
      creation_time: value.creation_time?,
      keep_alive: value.keep_alive?,
      pit_id: value.pit_id?,
    })
  }
}

impl From<super::PitDetail> for PitDetail {
  fn from(value: super::PitDetail) -> Self {
    Self {
      creation_time: Ok(value.creation_time),
      keep_alive: Ok(value.keep_alive),
      pit_id: Ok(value.pit_id),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PitsDetailsDeleteAll {
  pit_id: Result<Option<String>, String>,
  successful: Result<Option<bool>, String>,
}

impl Default for PitsDetailsDeleteAll {
  fn default() -> Self {
    Self {
      pit_id: Ok(Default::default()),
      successful: Ok(Default::default()),
    }
  }
}

impl PitsDetailsDeleteAll {
  pub fn pit_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.pit_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
    self
  }

  pub fn successful<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.successful = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for successful: {}", e));
    self
  }
}

impl std::convert::TryFrom<PitsDetailsDeleteAll> for super::PitsDetailsDeleteAll {
  type Error = String;

  fn try_from(value: PitsDetailsDeleteAll) -> Result<Self, String> {
    Ok(Self {
      pit_id: value.pit_id?,
      successful: value.successful?,
    })
  }
}

impl From<super::PitsDetailsDeleteAll> for PitsDetailsDeleteAll {
  fn from(value: super::PitsDetailsDeleteAll) -> Self {
    Self {
      pit_id: Ok(value.pit_id),
      successful: Ok(value.successful),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ReloadHttpCertificatesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for ReloadHttpCertificatesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl ReloadHttpCertificatesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<ReloadHttpCertificatesResponseContent> for super::ReloadHttpCertificatesResponseContent {
  type Error = String;

  fn try_from(value: ReloadHttpCertificatesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::ReloadHttpCertificatesResponseContent> for ReloadHttpCertificatesResponseContent {
  fn from(value: super::ReloadHttpCertificatesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ReloadTransportCertificatesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for ReloadTransportCertificatesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl ReloadTransportCertificatesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<ReloadTransportCertificatesResponseContent>
  for super::ReloadTransportCertificatesResponseContent
{
  type Error = String;

  fn try_from(value: ReloadTransportCertificatesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::ReloadTransportCertificatesResponseContent> for ReloadTransportCertificatesResponseContent {
  fn from(value: super::ReloadTransportCertificatesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct RemoteStoreRestoreBodyParams {
  indices: Result<Vec<String>, String>,
}

impl Default for RemoteStoreRestoreBodyParams {
  fn default() -> Self {
    Self {
      indices: Err("no value supplied for indices".to_string()),
    }
  }
}

impl RemoteStoreRestoreBodyParams {
  pub fn indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for indices: {}", e));
    self
  }
}

impl std::convert::TryFrom<RemoteStoreRestoreBodyParams> for super::RemoteStoreRestoreBodyParams {
  type Error = String;

  fn try_from(value: RemoteStoreRestoreBodyParams) -> Result<Self, String> {
    Ok(Self {
      indices: value.indices?,
    })
  }
}

impl From<super::RemoteStoreRestoreBodyParams> for RemoteStoreRestoreBodyParams {
  fn from(value: super::RemoteStoreRestoreBodyParams) -> Self {
    Self {
      indices: Ok(value.indices),
    }
  }
}

#[derive(Clone, Debug)]
pub struct RemoteStoreRestoreInfo {
  indices: Result<Vec<String>, String>,
  shards: Result<Option<super::RemoteStoreRestoreShardsInfo>, String>,
  snapshot: Result<Option<String>, String>,
}

impl Default for RemoteStoreRestoreInfo {
  fn default() -> Self {
    Self {
      indices: Ok(Default::default()),
      shards: Ok(Default::default()),
      snapshot: Ok(Default::default()),
    }
  }
}

impl RemoteStoreRestoreInfo {
  pub fn indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for indices: {}", e));
    self
  }

  pub fn shards<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::RemoteStoreRestoreShardsInfo>>,
    T::Error: std::fmt::Display, {
    self.shards = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards: {}", e));
    self
  }

  pub fn snapshot<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.snapshot = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for snapshot: {}", e));
    self
  }
}

impl std::convert::TryFrom<RemoteStoreRestoreInfo> for super::RemoteStoreRestoreInfo {
  type Error = String;

  fn try_from(value: RemoteStoreRestoreInfo) -> Result<Self, String> {
    Ok(Self {
      indices: value.indices?,
      shards: value.shards?,
      snapshot: value.snapshot?,
    })
  }
}

impl From<super::RemoteStoreRestoreInfo> for RemoteStoreRestoreInfo {
  fn from(value: super::RemoteStoreRestoreInfo) -> Self {
    Self {
      indices: Ok(value.indices),
      shards: Ok(value.shards),
      snapshot: Ok(value.snapshot),
    }
  }
}

#[derive(Clone, Debug)]
pub struct RemoteStoreRestoreResponseContent {
  accepted: Result<Option<bool>, String>,
  remote_store: Result<Option<super::RemoteStoreRestoreInfo>, String>,
}

impl Default for RemoteStoreRestoreResponseContent {
  fn default() -> Self {
    Self {
      accepted: Ok(Default::default()),
      remote_store: Ok(Default::default()),
    }
  }
}

impl RemoteStoreRestoreResponseContent {
  pub fn accepted<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.accepted = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for accepted: {}", e));
    self
  }

  pub fn remote_store<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::RemoteStoreRestoreInfo>>,
    T::Error: std::fmt::Display, {
    self.remote_store = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for remote_store: {}", e));
    self
  }
}

impl std::convert::TryFrom<RemoteStoreRestoreResponseContent> for super::RemoteStoreRestoreResponseContent {
  type Error = String;

  fn try_from(value: RemoteStoreRestoreResponseContent) -> Result<Self, String> {
    Ok(Self {
      accepted: value.accepted?,
      remote_store: value.remote_store?,
    })
  }
}

impl From<super::RemoteStoreRestoreResponseContent> for RemoteStoreRestoreResponseContent {
  fn from(value: super::RemoteStoreRestoreResponseContent) -> Self {
    Self {
      accepted: Ok(value.accepted),
      remote_store: Ok(value.remote_store),
    }
  }
}

#[derive(Clone, Debug)]
pub struct RemoteStoreRestoreShardsInfo {
  failed: Result<Option<i32>, String>,
  successful: Result<Option<i32>, String>,
  total: Result<Option<i32>, String>,
}

impl Default for RemoteStoreRestoreShardsInfo {
  fn default() -> Self {
    Self {
      failed: Ok(Default::default()),
      successful: Ok(Default::default()),
      total: Ok(Default::default()),
    }
  }
}

impl RemoteStoreRestoreShardsInfo {
  pub fn failed<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.failed = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for failed: {}", e));
    self
  }

  pub fn successful<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.successful = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for successful: {}", e));
    self
  }

  pub fn total<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.total = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for total: {}", e));
    self
  }
}

impl std::convert::TryFrom<RemoteStoreRestoreShardsInfo> for super::RemoteStoreRestoreShardsInfo {
  type Error = String;

  fn try_from(value: RemoteStoreRestoreShardsInfo) -> Result<Self, String> {
    Ok(Self {
      failed: value.failed?,
      successful: value.successful?,
      total: value.total?,
    })
  }
}

impl From<super::RemoteStoreRestoreShardsInfo> for RemoteStoreRestoreShardsInfo {
  fn from(value: super::RemoteStoreRestoreShardsInfo) -> Self {
    Self {
      failed: Ok(value.failed),
      successful: Ok(value.successful),
      total: Ok(value.total),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Role {
  cluster_permission: Result<Vec<String>, String>,
  description: Result<Option<String>, String>,
  hidden: Result<Option<bool>, String>,
  index_permission: Result<Option<super::IndexPermission>, String>,
  reserved: Result<Option<bool>, String>,
  static_: Result<Option<bool>, String>,
  tenant_permissions: Result<Vec<String>, String>,
}

impl Default for Role {
  fn default() -> Self {
    Self {
      cluster_permission: Ok(Default::default()),
      description: Ok(Default::default()),
      hidden: Ok(Default::default()),
      index_permission: Ok(Default::default()),
      reserved: Ok(Default::default()),
      static_: Ok(Default::default()),
      tenant_permissions: Ok(Default::default()),
    }
  }
}

impl Role {
  pub fn cluster_permission<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.cluster_permission = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for cluster_permission: {}", e));
    self
  }

  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }

  pub fn hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hidden: {}", e));
    self
  }

  pub fn index_permission<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::IndexPermission>>,
    T::Error: std::fmt::Display, {
    self.index_permission = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index_permission: {}", e));
    self
  }

  pub fn reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for reserved: {}", e));
    self
  }

  pub fn static_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.static_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for static_: {}", e));
    self
  }

  pub fn tenant_permissions<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.tenant_permissions = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for tenant_permissions: {}", e));
    self
  }
}

impl std::convert::TryFrom<Role> for super::Role {
  type Error = String;

  fn try_from(value: Role) -> Result<Self, String> {
    Ok(Self {
      cluster_permission: value.cluster_permission?,
      description: value.description?,
      hidden: value.hidden?,
      index_permission: value.index_permission?,
      reserved: value.reserved?,
      static_: value.static_?,
      tenant_permissions: value.tenant_permissions?,
    })
  }
}

impl From<super::Role> for Role {
  fn from(value: super::Role) -> Self {
    Self {
      cluster_permission: Ok(value.cluster_permission),
      description: Ok(value.description),
      hidden: Ok(value.hidden),
      index_permission: Ok(value.index_permission),
      reserved: Ok(value.reserved),
      static_: Ok(value.static_),
      tenant_permissions: Ok(value.tenant_permissions),
    }
  }
}

#[derive(Clone, Debug)]
pub struct RoleMapping {
  and_backend_roles: Result<Vec<String>, String>,
  backend_roles: Result<Vec<String>, String>,
  description: Result<Option<String>, String>,
  hidden: Result<Option<bool>, String>,
  hosts: Result<Vec<String>, String>,
  reserved: Result<Option<bool>, String>,
  users: Result<Vec<String>, String>,
}

impl Default for RoleMapping {
  fn default() -> Self {
    Self {
      and_backend_roles: Ok(Default::default()),
      backend_roles: Ok(Default::default()),
      description: Ok(Default::default()),
      hidden: Ok(Default::default()),
      hosts: Ok(Default::default()),
      reserved: Ok(Default::default()),
      users: Ok(Default::default()),
    }
  }
}

impl RoleMapping {
  pub fn and_backend_roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.and_backend_roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for and_backend_roles: {}", e));
    self
  }

  pub fn backend_roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.backend_roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
    self
  }

  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }

  pub fn hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hidden: {}", e));
    self
  }

  pub fn hosts<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.hosts = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hosts: {}", e));
    self
  }

  pub fn reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for reserved: {}", e));
    self
  }

  pub fn users<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.users = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for users: {}", e));
    self
  }
}

impl std::convert::TryFrom<RoleMapping> for super::RoleMapping {
  type Error = String;

  fn try_from(value: RoleMapping) -> Result<Self, String> {
    Ok(Self {
      and_backend_roles: value.and_backend_roles?,
      backend_roles: value.backend_roles?,
      description: value.description?,
      hidden: value.hidden?,
      hosts: value.hosts?,
      reserved: value.reserved?,
      users: value.users?,
    })
  }
}

impl From<super::RoleMapping> for RoleMapping {
  fn from(value: super::RoleMapping) -> Self {
    Self {
      and_backend_roles: Ok(value.and_backend_roles),
      backend_roles: Ok(value.backend_roles),
      description: Ok(value.description),
      hidden: Ok(value.hidden),
      hosts: Ok(value.hosts),
      reserved: Ok(value.reserved),
      users: Ok(value.users),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SearchBodyParams {
  docvalue_fields: Result<Option<String>, String>,
  explain: Result<Option<bool>, String>,
  fields: Result<Vec<String>, String>,
  from: Result<Option<i32>, String>,
  indices_boost: Result<Vec<serde_json::Value>, String>,
  min_score: Result<Option<i32>, String>,
  query: Result<Option<super::UserDefinedObjectStructure>, String>,
  seq_no_primary_term: Result<Option<bool>, String>,
  size: Result<Option<i32>, String>,
  source: Result<Option<String>, String>,
  stats: Result<Option<String>, String>,
  terminate_after: Result<Option<i32>, String>,
  timeout: Result<Option<super::Time>, String>,
  version: Result<Option<bool>, String>,
}

impl Default for SearchBodyParams {
  fn default() -> Self {
    Self {
      docvalue_fields: Ok(Default::default()),
      explain: Ok(Default::default()),
      fields: Ok(Default::default()),
      from: Ok(Default::default()),
      indices_boost: Ok(Default::default()),
      min_score: Ok(Default::default()),
      query: Ok(Default::default()),
      seq_no_primary_term: Ok(Default::default()),
      size: Ok(Default::default()),
      source: Ok(Default::default()),
      stats: Ok(Default::default()),
      terminate_after: Ok(Default::default()),
      timeout: Ok(Default::default()),
      version: Ok(Default::default()),
    }
  }
}

impl SearchBodyParams {
  pub fn docvalue_fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.docvalue_fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for docvalue_fields: {}", e));
    self
  }

  pub fn explain<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.explain = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for explain: {}", e));
    self
  }

  pub fn fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for fields: {}", e));
    self
  }

  pub fn from<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.from = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for from: {}", e));
    self
  }

  pub fn indices_boost<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.indices_boost = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for indices_boost: {}", e));
    self
  }

  pub fn min_score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.min_score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for min_score: {}", e));
    self
  }

  pub fn query<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedObjectStructure>>,
    T::Error: std::fmt::Display, {
    self.query = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for query: {}", e));
    self
  }

  pub fn seq_no_primary_term<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.seq_no_primary_term = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for seq_no_primary_term: {}", e));
    self
  }

  pub fn size<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.size = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for size: {}", e));
    self
  }

  pub fn source<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.source = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for source: {}", e));
    self
  }

  pub fn stats<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.stats = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for stats: {}", e));
    self
  }

  pub fn terminate_after<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.terminate_after = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for terminate_after: {}", e));
    self
  }

  pub fn timeout<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::Time>>,
    T::Error: std::fmt::Display, {
    self.timeout = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timeout: {}", e));
    self
  }

  pub fn version<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.version = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for version: {}", e));
    self
  }
}

impl std::convert::TryFrom<SearchBodyParams> for super::SearchBodyParams {
  type Error = String;

  fn try_from(value: SearchBodyParams) -> Result<Self, String> {
    Ok(Self {
      docvalue_fields: value.docvalue_fields?,
      explain: value.explain?,
      fields: value.fields?,
      from: value.from?,
      indices_boost: value.indices_boost?,
      min_score: value.min_score?,
      query: value.query?,
      seq_no_primary_term: value.seq_no_primary_term?,
      size: value.size?,
      source: value.source?,
      stats: value.stats?,
      terminate_after: value.terminate_after?,
      timeout: value.timeout?,
      version: value.version?,
    })
  }
}

impl From<super::SearchBodyParams> for SearchBodyParams {
  fn from(value: super::SearchBodyParams) -> Self {
    Self {
      docvalue_fields: Ok(value.docvalue_fields),
      explain: Ok(value.explain),
      fields: Ok(value.fields),
      from: Ok(value.from),
      indices_boost: Ok(value.indices_boost),
      min_score: Ok(value.min_score),
      query: Ok(value.query),
      seq_no_primary_term: Ok(value.seq_no_primary_term),
      size: Ok(value.size),
      source: Ok(value.source),
      stats: Ok(value.stats),
      terminate_after: Ok(value.terminate_after),
      timeout: Ok(value.timeout),
      version: Ok(value.version),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SearchGetResponseContent {
  hits: Result<Option<super::HitsMetadata>, String>,
  scroll_id: Result<Option<String>, String>,
  shards: Result<Option<super::ShardStatistics>, String>,
  timed_out: Result<Option<bool>, String>,
  took: Result<Option<i64>, String>,
}

impl Default for SearchGetResponseContent {
  fn default() -> Self {
    Self {
      hits: Ok(Default::default()),
      scroll_id: Ok(Default::default()),
      shards: Ok(Default::default()),
      timed_out: Ok(Default::default()),
      took: Ok(Default::default()),
    }
  }
}

impl SearchGetResponseContent {
  pub fn hits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::HitsMetadata>>,
    T::Error: std::fmt::Display, {
    self.hits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hits: {}", e));
    self
  }

  pub fn scroll_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.scroll_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
    self
  }

  pub fn shards<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ShardStatistics>>,
    T::Error: std::fmt::Display, {
    self.shards = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards: {}", e));
    self
  }

  pub fn timed_out<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.timed_out = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
    self
  }

  pub fn took<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.took = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for took: {}", e));
    self
  }
}

impl std::convert::TryFrom<SearchGetResponseContent> for super::SearchGetResponseContent {
  type Error = String;

  fn try_from(value: SearchGetResponseContent) -> Result<Self, String> {
    Ok(Self {
      hits: value.hits?,
      scroll_id: value.scroll_id?,
      shards: value.shards?,
      timed_out: value.timed_out?,
      took: value.took?,
    })
  }
}

impl From<super::SearchGetResponseContent> for SearchGetResponseContent {
  fn from(value: super::SearchGetResponseContent) -> Self {
    Self {
      hits: Ok(value.hits),
      scroll_id: Ok(value.scroll_id),
      shards: Ok(value.shards),
      timed_out: Ok(value.timed_out),
      took: Ok(value.took),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SearchGetWithIndexResponseContent {
  hits: Result<Option<super::HitsMetadata>, String>,
  scroll_id: Result<Option<String>, String>,
  shards: Result<Option<super::ShardStatistics>, String>,
  timed_out: Result<Option<bool>, String>,
  took: Result<Option<i64>, String>,
}

impl Default for SearchGetWithIndexResponseContent {
  fn default() -> Self {
    Self {
      hits: Ok(Default::default()),
      scroll_id: Ok(Default::default()),
      shards: Ok(Default::default()),
      timed_out: Ok(Default::default()),
      took: Ok(Default::default()),
    }
  }
}

impl SearchGetWithIndexResponseContent {
  pub fn hits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::HitsMetadata>>,
    T::Error: std::fmt::Display, {
    self.hits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hits: {}", e));
    self
  }

  pub fn scroll_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.scroll_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
    self
  }

  pub fn shards<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ShardStatistics>>,
    T::Error: std::fmt::Display, {
    self.shards = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards: {}", e));
    self
  }

  pub fn timed_out<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.timed_out = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
    self
  }

  pub fn took<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.took = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for took: {}", e));
    self
  }
}

impl std::convert::TryFrom<SearchGetWithIndexResponseContent> for super::SearchGetWithIndexResponseContent {
  type Error = String;

  fn try_from(value: SearchGetWithIndexResponseContent) -> Result<Self, String> {
    Ok(Self {
      hits: value.hits?,
      scroll_id: value.scroll_id?,
      shards: value.shards?,
      timed_out: value.timed_out?,
      took: value.took?,
    })
  }
}

impl From<super::SearchGetWithIndexResponseContent> for SearchGetWithIndexResponseContent {
  fn from(value: super::SearchGetWithIndexResponseContent) -> Self {
    Self {
      hits: Ok(value.hits),
      scroll_id: Ok(value.scroll_id),
      shards: Ok(value.shards),
      timed_out: Ok(value.timed_out),
      took: Ok(value.took),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SearchPostResponseContent {
  hits: Result<Option<super::HitsMetadata>, String>,
  scroll_id: Result<Option<String>, String>,
  shards: Result<Option<super::ShardStatistics>, String>,
  timed_out: Result<Option<bool>, String>,
  took: Result<Option<i64>, String>,
}

impl Default for SearchPostResponseContent {
  fn default() -> Self {
    Self {
      hits: Ok(Default::default()),
      scroll_id: Ok(Default::default()),
      shards: Ok(Default::default()),
      timed_out: Ok(Default::default()),
      took: Ok(Default::default()),
    }
  }
}

impl SearchPostResponseContent {
  pub fn hits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::HitsMetadata>>,
    T::Error: std::fmt::Display, {
    self.hits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hits: {}", e));
    self
  }

  pub fn scroll_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.scroll_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
    self
  }

  pub fn shards<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ShardStatistics>>,
    T::Error: std::fmt::Display, {
    self.shards = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards: {}", e));
    self
  }

  pub fn timed_out<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.timed_out = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
    self
  }

  pub fn took<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.took = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for took: {}", e));
    self
  }
}

impl std::convert::TryFrom<SearchPostResponseContent> for super::SearchPostResponseContent {
  type Error = String;

  fn try_from(value: SearchPostResponseContent) -> Result<Self, String> {
    Ok(Self {
      hits: value.hits?,
      scroll_id: value.scroll_id?,
      shards: value.shards?,
      timed_out: value.timed_out?,
      took: value.took?,
    })
  }
}

impl From<super::SearchPostResponseContent> for SearchPostResponseContent {
  fn from(value: super::SearchPostResponseContent) -> Self {
    Self {
      hits: Ok(value.hits),
      scroll_id: Ok(value.scroll_id),
      shards: Ok(value.shards),
      timed_out: Ok(value.timed_out),
      took: Ok(value.took),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SearchPostWithIndexResponseContent {
  hits: Result<Option<super::HitsMetadata>, String>,
  scroll_id: Result<Option<String>, String>,
  shards: Result<Option<super::ShardStatistics>, String>,
  timed_out: Result<Option<bool>, String>,
  took: Result<Option<i64>, String>,
}

impl Default for SearchPostWithIndexResponseContent {
  fn default() -> Self {
    Self {
      hits: Ok(Default::default()),
      scroll_id: Ok(Default::default()),
      shards: Ok(Default::default()),
      timed_out: Ok(Default::default()),
      took: Ok(Default::default()),
    }
  }
}

impl SearchPostWithIndexResponseContent {
  pub fn hits<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::HitsMetadata>>,
    T::Error: std::fmt::Display, {
    self.hits = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hits: {}", e));
    self
  }

  pub fn scroll_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.scroll_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
    self
  }

  pub fn shards<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::ShardStatistics>>,
    T::Error: std::fmt::Display, {
    self.shards = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shards: {}", e));
    self
  }

  pub fn timed_out<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.timed_out = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
    self
  }

  pub fn took<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i64>>,
    T::Error: std::fmt::Display, {
    self.took = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for took: {}", e));
    self
  }
}

impl std::convert::TryFrom<SearchPostWithIndexResponseContent> for super::SearchPostWithIndexResponseContent {
  type Error = String;

  fn try_from(value: SearchPostWithIndexResponseContent) -> Result<Self, String> {
    Ok(Self {
      hits: value.hits?,
      scroll_id: value.scroll_id?,
      shards: value.shards?,
      timed_out: value.timed_out?,
      took: value.took?,
    })
  }
}

impl From<super::SearchPostWithIndexResponseContent> for SearchPostWithIndexResponseContent {
  fn from(value: super::SearchPostWithIndexResponseContent) -> Self {
    Self {
      hits: Ok(value.hits),
      scroll_id: Ok(value.scroll_id),
      shards: Ok(value.shards),
      timed_out: Ok(value.timed_out),
      took: Ok(value.took),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SecurityHealthResponseContent {
  message: Result<Option<String>, String>,
  mode: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for SecurityHealthResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      mode: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl SecurityHealthResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn mode<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.mode = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for mode: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<SecurityHealthResponseContent> for super::SecurityHealthResponseContent {
  type Error = String;

  fn try_from(value: SecurityHealthResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      mode: value.mode?,
      status: value.status?,
    })
  }
}

impl From<super::SecurityHealthResponseContent> for SecurityHealthResponseContent {
  fn from(value: super::SecurityHealthResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      mode: Ok(value.mode),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ShardStatistics {
  failed: Result<Option<i32>, String>,
  skipped: Result<Option<i32>, String>,
  successful: Result<Option<i32>, String>,
  total: Result<Option<i32>, String>,
}

impl Default for ShardStatistics {
  fn default() -> Self {
    Self {
      failed: Ok(Default::default()),
      skipped: Ok(Default::default()),
      successful: Ok(Default::default()),
      total: Ok(Default::default()),
    }
  }
}

impl ShardStatistics {
  pub fn failed<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.failed = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for failed: {}", e));
    self
  }

  pub fn skipped<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.skipped = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for skipped: {}", e));
    self
  }

  pub fn successful<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.successful = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for successful: {}", e));
    self
  }

  pub fn total<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.total = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for total: {}", e));
    self
  }
}

impl std::convert::TryFrom<ShardStatistics> for super::ShardStatistics {
  type Error = String;

  fn try_from(value: ShardStatistics) -> Result<Self, String> {
    Ok(Self {
      failed: value.failed?,
      skipped: value.skipped?,
      successful: value.successful?,
      total: value.total?,
    })
  }
}

impl From<super::ShardStatistics> for ShardStatistics {
  fn from(value: super::ShardStatistics) -> Self {
    Self {
      failed: Ok(value.failed),
      skipped: Ok(value.skipped),
      successful: Ok(value.successful),
      total: Ok(value.total),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Tenant {
  description: Result<Option<String>, String>,
  hidden: Result<Option<bool>, String>,
  reserved: Result<Option<bool>, String>,
  static_: Result<Option<bool>, String>,
}

impl Default for Tenant {
  fn default() -> Self {
    Self {
      description: Ok(Default::default()),
      hidden: Ok(Default::default()),
      reserved: Ok(Default::default()),
      static_: Ok(Default::default()),
    }
  }
}

impl Tenant {
  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }

  pub fn hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hidden: {}", e));
    self
  }

  pub fn reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for reserved: {}", e));
    self
  }

  pub fn static_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.static_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for static_: {}", e));
    self
  }
}

impl std::convert::TryFrom<Tenant> for super::Tenant {
  type Error = String;

  fn try_from(value: Tenant) -> Result<Self, String> {
    Ok(Self {
      description: value.description?,
      hidden: value.hidden?,
      reserved: value.reserved?,
      static_: value.static_?,
    })
  }
}

impl From<super::Tenant> for Tenant {
  fn from(value: super::Tenant) -> Self {
    Self {
      description: Ok(value.description),
      hidden: Ok(value.hidden),
      reserved: Ok(value.reserved),
      static_: Ok(value.static_),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Total {
  relation: Result<Option<super::Relation>, String>,
  value: Result<Option<i32>, String>,
}

impl Default for Total {
  fn default() -> Self {
    Self {
      relation: Ok(Default::default()),
      value: Ok(Default::default()),
    }
  }
}

impl Total {
  pub fn relation<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::Relation>>,
    T::Error: std::fmt::Display, {
    self.relation = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for relation: {}", e));
    self
  }

  pub fn value<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<i32>>,
    T::Error: std::fmt::Display, {
    self.value = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for value: {}", e));
    self
  }
}

impl std::convert::TryFrom<Total> for super::Total {
  type Error = String;

  fn try_from(value: Total) -> Result<Self, String> {
    Ok(Self {
      relation: value.relation?,
      value: value.value?,
    })
  }
}

impl From<super::Total> for Total {
  fn from(value: super::Total) -> Self {
    Self {
      relation: Ok(value.relation),
      value: Ok(value.value),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UpdateAuditConfigurationResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for UpdateAuditConfigurationResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl UpdateAuditConfigurationResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<UpdateAuditConfigurationResponseContent> for super::UpdateAuditConfigurationResponseContent {
  type Error = String;

  fn try_from(value: UpdateAuditConfigurationResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::UpdateAuditConfigurationResponseContent> for UpdateAuditConfigurationResponseContent {
  fn from(value: super::UpdateAuditConfigurationResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UpdateConfigurationResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for UpdateConfigurationResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl UpdateConfigurationResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<UpdateConfigurationResponseContent> for super::UpdateConfigurationResponseContent {
  type Error = String;

  fn try_from(value: UpdateConfigurationResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::UpdateConfigurationResponseContent> for UpdateConfigurationResponseContent {
  fn from(value: super::UpdateConfigurationResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UpdateDistinguishedNamesResponseContent {
  message: Result<Option<String>, String>,
  status: Result<Option<String>, String>,
}

impl Default for UpdateDistinguishedNamesResponseContent {
  fn default() -> Self {
    Self {
      message: Ok(Default::default()),
      status: Ok(Default::default()),
    }
  }
}

impl UpdateDistinguishedNamesResponseContent {
  pub fn message<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.message = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for message: {}", e));
    self
  }

  pub fn status<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.status = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for status: {}", e));
    self
  }
}

impl std::convert::TryFrom<UpdateDistinguishedNamesResponseContent> for super::UpdateDistinguishedNamesResponseContent {
  type Error = String;

  fn try_from(value: UpdateDistinguishedNamesResponseContent) -> Result<Self, String> {
    Ok(Self {
      message: value.message?,
      status: value.status?,
    })
  }
}

impl From<super::UpdateDistinguishedNamesResponseContent> for UpdateDistinguishedNamesResponseContent {
  fn from(value: super::UpdateDistinguishedNamesResponseContent) -> Self {
    Self {
      message: Ok(value.message),
      status: Ok(value.status),
    }
  }
}

#[derive(Clone, Debug)]
pub struct User {
  attributes: Result<Option<super::UserAttributes>, String>,
  backend_roles: Result<Vec<String>, String>,
  description: Result<Option<String>, String>,
  hash: Result<Option<String>, String>,
  hidden: Result<Option<bool>, String>,
  opendistro_security_roles: Result<Vec<String>, String>,
  reserved: Result<Option<bool>, String>,
  static_: Result<Option<bool>, String>,
}

impl Default for User {
  fn default() -> Self {
    Self {
      attributes: Ok(Default::default()),
      backend_roles: Ok(Default::default()),
      description: Ok(Default::default()),
      hash: Ok(Default::default()),
      hidden: Ok(Default::default()),
      opendistro_security_roles: Ok(Default::default()),
      reserved: Ok(Default::default()),
      static_: Ok(Default::default()),
    }
  }
}

impl User {
  pub fn attributes<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserAttributes>>,
    T::Error: std::fmt::Display, {
    self.attributes = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for attributes: {}", e));
    self
  }

  pub fn backend_roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.backend_roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
    self
  }

  pub fn description<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.description = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for description: {}", e));
    self
  }

  pub fn hash<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.hash = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hash: {}", e));
    self
  }

  pub fn hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for hidden: {}", e));
    self
  }

  pub fn opendistro_security_roles<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.opendistro_security_roles = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for opendistro_security_roles: {}", e));
    self
  }

  pub fn reserved<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.reserved = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for reserved: {}", e));
    self
  }

  pub fn static_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.static_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for static_: {}", e));
    self
  }
}

impl std::convert::TryFrom<User> for super::User {
  type Error = String;

  fn try_from(value: User) -> Result<Self, String> {
    Ok(Self {
      attributes: value.attributes?,
      backend_roles: value.backend_roles?,
      description: value.description?,
      hash: value.hash?,
      hidden: value.hidden?,
      opendistro_security_roles: value.opendistro_security_roles?,
      reserved: value.reserved?,
      static_: value.static_?,
    })
  }
}

impl From<super::User> for User {
  fn from(value: super::User) -> Self {
    Self {
      attributes: Ok(value.attributes),
      backend_roles: Ok(value.backend_roles),
      description: Ok(value.description),
      hash: Ok(value.hash),
      hidden: Ok(value.hidden),
      opendistro_security_roles: Ok(value.opendistro_security_roles),
      reserved: Ok(value.reserved),
      static_: Ok(value.static_),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UserDefinedObjectStructure {
  bool: Result<Option<serde_json::Value>, String>,
  boosting: Result<Option<serde_json::Value>, String>,
  combined_fields: Result<Option<serde_json::Value>, String>,
  constant_score: Result<Option<serde_json::Value>, String>,
  dis_max: Result<Option<serde_json::Value>, String>,
  distance_feature: Result<Option<serde_json::Value>, String>,
  exists: Result<Option<serde_json::Value>, String>,
  field_masking_span: Result<Option<serde_json::Value>, String>,
  function_score: Result<Option<serde_json::Value>, String>,
  fuzzy: Result<Option<super::UserDefinedValueMap>, String>,
  geo_bounding_box: Result<Option<serde_json::Value>, String>,
  geo_distance: Result<Option<serde_json::Value>, String>,
  geo_polygon: Result<Option<serde_json::Value>, String>,
  geo_shape: Result<Option<serde_json::Value>, String>,
  has_child: Result<Option<serde_json::Value>, String>,
  has_parent: Result<Option<serde_json::Value>, String>,
  ids: Result<Option<serde_json::Value>, String>,
  intervals: Result<Option<super::UserDefinedValueMap>, String>,
  knn: Result<Option<serde_json::Value>, String>,
  match_: Result<Option<super::UserDefinedValueMap>, String>,
  match_all: Result<Option<serde_json::Value>, String>,
  match_bool_prefix: Result<Option<super::UserDefinedValueMap>, String>,
  match_none: Result<Option<serde_json::Value>, String>,
  match_phrase: Result<Option<super::UserDefinedValueMap>, String>,
  match_phrase_prefix: Result<Option<super::UserDefinedValueMap>, String>,
  more_like_this: Result<Option<serde_json::Value>, String>,
  multi_match: Result<Option<serde_json::Value>, String>,
  nested: Result<Option<serde_json::Value>, String>,
  parent_id: Result<Option<serde_json::Value>, String>,
  percolate: Result<Option<serde_json::Value>, String>,
  pinned: Result<Option<serde_json::Value>, String>,
  prefix: Result<Option<super::UserDefinedValueMap>, String>,
  query_string: Result<Option<serde_json::Value>, String>,
  range: Result<Option<super::UserDefinedValueMap>, String>,
  rank_feature: Result<Option<serde_json::Value>, String>,
  regexp: Result<Option<super::UserDefinedValueMap>, String>,
  script: Result<Option<serde_json::Value>, String>,
  script_score: Result<Option<serde_json::Value>, String>,
  shape: Result<Option<serde_json::Value>, String>,
  simple_query_string: Result<Option<serde_json::Value>, String>,
  span_containing: Result<Option<serde_json::Value>, String>,
  span_first: Result<Option<serde_json::Value>, String>,
  span_multi: Result<Option<serde_json::Value>, String>,
  span_near: Result<Option<serde_json::Value>, String>,
  span_not: Result<Option<serde_json::Value>, String>,
  span_or: Result<Option<serde_json::Value>, String>,
  span_term: Result<Option<super::UserDefinedValueMap>, String>,
  span_within: Result<Option<serde_json::Value>, String>,
  term: Result<Option<super::UserDefinedValueMap>, String>,
  terms: Result<Option<serde_json::Value>, String>,
  terms_set: Result<Option<super::UserDefinedValueMap>, String>,
  wildcard: Result<Option<super::UserDefinedValueMap>, String>,
  wrapper: Result<Option<serde_json::Value>, String>,
}

impl Default for UserDefinedObjectStructure {
  fn default() -> Self {
    Self {
      bool: Ok(Default::default()),
      boosting: Ok(Default::default()),
      combined_fields: Ok(Default::default()),
      constant_score: Ok(Default::default()),
      dis_max: Ok(Default::default()),
      distance_feature: Ok(Default::default()),
      exists: Ok(Default::default()),
      field_masking_span: Ok(Default::default()),
      function_score: Ok(Default::default()),
      fuzzy: Ok(Default::default()),
      geo_bounding_box: Ok(Default::default()),
      geo_distance: Ok(Default::default()),
      geo_polygon: Ok(Default::default()),
      geo_shape: Ok(Default::default()),
      has_child: Ok(Default::default()),
      has_parent: Ok(Default::default()),
      ids: Ok(Default::default()),
      intervals: Ok(Default::default()),
      knn: Ok(Default::default()),
      match_: Ok(Default::default()),
      match_all: Ok(Default::default()),
      match_bool_prefix: Ok(Default::default()),
      match_none: Ok(Default::default()),
      match_phrase: Ok(Default::default()),
      match_phrase_prefix: Ok(Default::default()),
      more_like_this: Ok(Default::default()),
      multi_match: Ok(Default::default()),
      nested: Ok(Default::default()),
      parent_id: Ok(Default::default()),
      percolate: Ok(Default::default()),
      pinned: Ok(Default::default()),
      prefix: Ok(Default::default()),
      query_string: Ok(Default::default()),
      range: Ok(Default::default()),
      rank_feature: Ok(Default::default()),
      regexp: Ok(Default::default()),
      script: Ok(Default::default()),
      script_score: Ok(Default::default()),
      shape: Ok(Default::default()),
      simple_query_string: Ok(Default::default()),
      span_containing: Ok(Default::default()),
      span_first: Ok(Default::default()),
      span_multi: Ok(Default::default()),
      span_near: Ok(Default::default()),
      span_not: Ok(Default::default()),
      span_or: Ok(Default::default()),
      span_term: Ok(Default::default()),
      span_within: Ok(Default::default()),
      term: Ok(Default::default()),
      terms: Ok(Default::default()),
      terms_set: Ok(Default::default()),
      wildcard: Ok(Default::default()),
      wrapper: Ok(Default::default()),
    }
  }
}

impl UserDefinedObjectStructure {
  pub fn bool<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.bool = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for bool: {}", e));
    self
  }

  pub fn boosting<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.boosting = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for boosting: {}", e));
    self
  }

  pub fn combined_fields<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.combined_fields = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for combined_fields: {}", e));
    self
  }

  pub fn constant_score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.constant_score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for constant_score: {}", e));
    self
  }

  pub fn dis_max<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.dis_max = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for dis_max: {}", e));
    self
  }

  pub fn distance_feature<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.distance_feature = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for distance_feature: {}", e));
    self
  }

  pub fn exists<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.exists = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for exists: {}", e));
    self
  }

  pub fn field_masking_span<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.field_masking_span = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for field_masking_span: {}", e));
    self
  }

  pub fn function_score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.function_score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for function_score: {}", e));
    self
  }

  pub fn fuzzy<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.fuzzy = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for fuzzy: {}", e));
    self
  }

  pub fn geo_bounding_box<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.geo_bounding_box = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for geo_bounding_box: {}", e));
    self
  }

  pub fn geo_distance<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.geo_distance = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for geo_distance: {}", e));
    self
  }

  pub fn geo_polygon<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.geo_polygon = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for geo_polygon: {}", e));
    self
  }

  pub fn geo_shape<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.geo_shape = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for geo_shape: {}", e));
    self
  }

  pub fn has_child<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.has_child = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for has_child: {}", e));
    self
  }

  pub fn has_parent<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.has_parent = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for has_parent: {}", e));
    self
  }

  pub fn ids<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.ids = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for ids: {}", e));
    self
  }

  pub fn intervals<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.intervals = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for intervals: {}", e));
    self
  }

  pub fn knn<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.knn = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for knn: {}", e));
    self
  }

  pub fn match_<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.match_ = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_: {}", e));
    self
  }

  pub fn match_all<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.match_all = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_all: {}", e));
    self
  }

  pub fn match_bool_prefix<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.match_bool_prefix = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_bool_prefix: {}", e));
    self
  }

  pub fn match_none<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.match_none = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_none: {}", e));
    self
  }

  pub fn match_phrase<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.match_phrase = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_phrase: {}", e));
    self
  }

  pub fn match_phrase_prefix<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.match_phrase_prefix = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for match_phrase_prefix: {}", e));
    self
  }

  pub fn more_like_this<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.more_like_this = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for more_like_this: {}", e));
    self
  }

  pub fn multi_match<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.multi_match = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for multi_match: {}", e));
    self
  }

  pub fn nested<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.nested = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for nested: {}", e));
    self
  }

  pub fn parent_id<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.parent_id = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for parent_id: {}", e));
    self
  }

  pub fn percolate<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.percolate = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for percolate: {}", e));
    self
  }

  pub fn pinned<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.pinned = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for pinned: {}", e));
    self
  }

  pub fn prefix<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.prefix = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for prefix: {}", e));
    self
  }

  pub fn query_string<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.query_string = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for query_string: {}", e));
    self
  }

  pub fn range<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.range = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for range: {}", e));
    self
  }

  pub fn rank_feature<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.rank_feature = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for rank_feature: {}", e));
    self
  }

  pub fn regexp<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.regexp = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for regexp: {}", e));
    self
  }

  pub fn script<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.script = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for script: {}", e));
    self
  }

  pub fn script_score<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.script_score = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for script_score: {}", e));
    self
  }

  pub fn shape<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.shape = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for shape: {}", e));
    self
  }

  pub fn simple_query_string<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.simple_query_string = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for simple_query_string: {}", e));
    self
  }

  pub fn span_containing<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_containing = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_containing: {}", e));
    self
  }

  pub fn span_first<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_first = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_first: {}", e));
    self
  }

  pub fn span_multi<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_multi = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_multi: {}", e));
    self
  }

  pub fn span_near<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_near = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_near: {}", e));
    self
  }

  pub fn span_not<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_not = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_not: {}", e));
    self
  }

  pub fn span_or<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_or = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_or: {}", e));
    self
  }

  pub fn span_term<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.span_term = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_term: {}", e));
    self
  }

  pub fn span_within<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.span_within = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for span_within: {}", e));
    self
  }

  pub fn term<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.term = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for term: {}", e));
    self
  }

  pub fn terms<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.terms = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for terms: {}", e));
    self
  }

  pub fn terms_set<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.terms_set = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for terms_set: {}", e));
    self
  }

  pub fn wildcard<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
    T::Error: std::fmt::Display, {
    self.wildcard = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for wildcard: {}", e));
    self
  }

  pub fn wrapper<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.wrapper = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for wrapper: {}", e));
    self
  }
}

impl std::convert::TryFrom<UserDefinedObjectStructure> for super::UserDefinedObjectStructure {
  type Error = String;

  fn try_from(value: UserDefinedObjectStructure) -> Result<Self, String> {
    Ok(Self {
      bool: value.bool?,
      boosting: value.boosting?,
      combined_fields: value.combined_fields?,
      constant_score: value.constant_score?,
      dis_max: value.dis_max?,
      distance_feature: value.distance_feature?,
      exists: value.exists?,
      field_masking_span: value.field_masking_span?,
      function_score: value.function_score?,
      fuzzy: value.fuzzy?,
      geo_bounding_box: value.geo_bounding_box?,
      geo_distance: value.geo_distance?,
      geo_polygon: value.geo_polygon?,
      geo_shape: value.geo_shape?,
      has_child: value.has_child?,
      has_parent: value.has_parent?,
      ids: value.ids?,
      intervals: value.intervals?,
      knn: value.knn?,
      match_: value.match_?,
      match_all: value.match_all?,
      match_bool_prefix: value.match_bool_prefix?,
      match_none: value.match_none?,
      match_phrase: value.match_phrase?,
      match_phrase_prefix: value.match_phrase_prefix?,
      more_like_this: value.more_like_this?,
      multi_match: value.multi_match?,
      nested: value.nested?,
      parent_id: value.parent_id?,
      percolate: value.percolate?,
      pinned: value.pinned?,
      prefix: value.prefix?,
      query_string: value.query_string?,
      range: value.range?,
      rank_feature: value.rank_feature?,
      regexp: value.regexp?,
      script: value.script?,
      script_score: value.script_score?,
      shape: value.shape?,
      simple_query_string: value.simple_query_string?,
      span_containing: value.span_containing?,
      span_first: value.span_first?,
      span_multi: value.span_multi?,
      span_near: value.span_near?,
      span_not: value.span_not?,
      span_or: value.span_or?,
      span_term: value.span_term?,
      span_within: value.span_within?,
      term: value.term?,
      terms: value.terms?,
      terms_set: value.terms_set?,
      wildcard: value.wildcard?,
      wrapper: value.wrapper?,
    })
  }
}

impl From<super::UserDefinedObjectStructure> for UserDefinedObjectStructure {
  fn from(value: super::UserDefinedObjectStructure) -> Self {
    Self {
      bool: Ok(value.bool),
      boosting: Ok(value.boosting),
      combined_fields: Ok(value.combined_fields),
      constant_score: Ok(value.constant_score),
      dis_max: Ok(value.dis_max),
      distance_feature: Ok(value.distance_feature),
      exists: Ok(value.exists),
      field_masking_span: Ok(value.field_masking_span),
      function_score: Ok(value.function_score),
      fuzzy: Ok(value.fuzzy),
      geo_bounding_box: Ok(value.geo_bounding_box),
      geo_distance: Ok(value.geo_distance),
      geo_polygon: Ok(value.geo_polygon),
      geo_shape: Ok(value.geo_shape),
      has_child: Ok(value.has_child),
      has_parent: Ok(value.has_parent),
      ids: Ok(value.ids),
      intervals: Ok(value.intervals),
      knn: Ok(value.knn),
      match_: Ok(value.match_),
      match_all: Ok(value.match_all),
      match_bool_prefix: Ok(value.match_bool_prefix),
      match_none: Ok(value.match_none),
      match_phrase: Ok(value.match_phrase),
      match_phrase_prefix: Ok(value.match_phrase_prefix),
      more_like_this: Ok(value.more_like_this),
      multi_match: Ok(value.multi_match),
      nested: Ok(value.nested),
      parent_id: Ok(value.parent_id),
      percolate: Ok(value.percolate),
      pinned: Ok(value.pinned),
      prefix: Ok(value.prefix),
      query_string: Ok(value.query_string),
      range: Ok(value.range),
      rank_feature: Ok(value.rank_feature),
      regexp: Ok(value.regexp),
      script: Ok(value.script),
      script_score: Ok(value.script_score),
      shape: Ok(value.shape),
      simple_query_string: Ok(value.simple_query_string),
      span_containing: Ok(value.span_containing),
      span_first: Ok(value.span_first),
      span_multi: Ok(value.span_multi),
      span_near: Ok(value.span_near),
      span_not: Ok(value.span_not),
      span_or: Ok(value.span_or),
      span_term: Ok(value.span_term),
      span_within: Ok(value.span_within),
      term: Ok(value.term),
      terms: Ok(value.terms),
      terms_set: Ok(value.terms_set),
      wildcard: Ok(value.wildcard),
      wrapper: Ok(value.wrapper),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UserDefinedStructure {
  alias: Result<Option<String>, String>,
  aliases: Result<Vec<String>, String>,
  filter: Result<Option<serde_json::Value>, String>,
  index: Result<Option<String>, String>,
  index_routing: Result<Option<String>, String>,
  indices: Result<Vec<String>, String>,
  is_hidden: Result<Option<bool>, String>,
  is_write_index: Result<Option<bool>, String>,
  must_exist: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
  search_routing: Result<Option<String>, String>,
}

impl Default for UserDefinedStructure {
  fn default() -> Self {
    Self {
      alias: Ok(Default::default()),
      aliases: Ok(Default::default()),
      filter: Ok(Default::default()),
      index: Ok(Default::default()),
      index_routing: Ok(Default::default()),
      indices: Ok(Default::default()),
      is_hidden: Ok(Default::default()),
      is_write_index: Ok(Default::default()),
      must_exist: Ok(Default::default()),
      routing: Ok(Default::default()),
      search_routing: Ok(Default::default()),
    }
  }
}

impl UserDefinedStructure {
  pub fn alias<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.alias = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for alias: {}", e));
    self
  }

  pub fn aliases<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.aliases = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for aliases: {}", e));
    self
  }

  pub fn filter<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<serde_json::Value>>,
    T::Error: std::fmt::Display, {
    self.filter = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for filter: {}", e));
    self
  }

  pub fn index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index: {}", e));
    self
  }

  pub fn index_routing<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.index_routing = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for index_routing: {}", e));
    self
  }

  pub fn indices<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Vec<String>>,
    T::Error: std::fmt::Display, {
    self.indices = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for indices: {}", e));
    self
  }

  pub fn is_hidden<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.is_hidden = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for is_hidden: {}", e));
    self
  }

  pub fn is_write_index<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.is_write_index = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for is_write_index: {}", e));
    self
  }

  pub fn must_exist<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.must_exist = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for must_exist: {}", e));
    self
  }

  pub fn routing<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.routing = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for routing: {}", e));
    self
  }

  pub fn search_routing<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<String>>,
    T::Error: std::fmt::Display, {
    self.search_routing = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for search_routing: {}", e));
    self
  }
}

impl std::convert::TryFrom<UserDefinedStructure> for super::UserDefinedStructure {
  type Error = String;

  fn try_from(value: UserDefinedStructure) -> Result<Self, String> {
    Ok(Self {
      alias: value.alias?,
      aliases: value.aliases?,
      filter: value.filter?,
      index: value.index?,
      index_routing: value.index_routing?,
      indices: value.indices?,
      is_hidden: value.is_hidden?,
      is_write_index: value.is_write_index?,
      must_exist: value.must_exist?,
      routing: value.routing?,
      search_routing: value.search_routing?,
    })
  }
}

impl From<super::UserDefinedStructure> for UserDefinedStructure {
  fn from(value: super::UserDefinedStructure) -> Self {
    Self {
      alias: Ok(value.alias),
      aliases: Ok(value.aliases),
      filter: Ok(value.filter),
      index: Ok(value.index),
      index_routing: Ok(value.index_routing),
      indices: Ok(value.indices),
      is_hidden: Ok(value.is_hidden),
      is_write_index: Ok(value.is_write_index),
      must_exist: Ok(value.must_exist),
      routing: Ok(value.routing),
      search_routing: Ok(value.search_routing),
    }
  }
}

#[derive(Clone, Debug)]
pub struct UserTenants {
  admin: Result<Option<bool>, String>,
  admin_tenant: Result<Option<bool>, String>,
  global_tenant: Result<Option<bool>, String>,
}

impl Default for UserTenants {
  fn default() -> Self {
    Self {
      admin: Ok(Default::default()),
      admin_tenant: Ok(Default::default()),
      global_tenant: Ok(Default::default()),
    }
  }
}

impl UserTenants {
  pub fn admin<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.admin = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for admin: {}", e));
    self
  }

  pub fn admin_tenant<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.admin_tenant = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for admin_tenant: {}", e));
    self
  }

  pub fn global_tenant<T>(mut self, value: T) -> Self
  where
    T: std::convert::TryInto<Option<bool>>,
    T::Error: std::fmt::Display, {
    self.global_tenant = value
      .try_into()
      .map_err(|e| format!("error converting supplied value for global_tenant: {}", e));
    self
  }
}

impl std::convert::TryFrom<UserTenants> for super::UserTenants {
  type Error = String;

  fn try_from(value: UserTenants) -> Result<Self, String> {
    Ok(Self {
      admin: value.admin?,
      admin_tenant: value.admin_tenant?,
      global_tenant: value.global_tenant?,
    })
  }
}

impl From<super::UserTenants> for UserTenants {
  fn from(value: super::UserTenants) -> Self {
    Self {
      admin: Ok(value.admin),
      admin_tenant: Ok(value.admin_tenant),
      global_tenant: Ok(value.global_tenant),
    }
  }
}
